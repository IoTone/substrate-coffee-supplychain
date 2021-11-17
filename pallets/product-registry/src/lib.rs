#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::{prelude::*, vec::Vec};

    // General constraints to limit data size
    // Note: these could also be passed as trait config parameters
    pub const PRODUCT_ID_MAX_LENGTH: usize = 36;
    pub const PRODUCT_PROP_NAME_MAX_LENGTH: usize = 10;
    pub const PRODUCT_PROP_VALUE_MAX_LENGTH: usize = 20;
    pub const PRODUCT_MAX_PROPS: usize = 3;

    // Custom types
    pub type ProductId = Vec<u8>;
    pub type PropName = Vec<u8>;
    pub type PropValue = Vec<u8>;

    // Product contains master data (aka class-level) about a trade item.
    // This data is typically registered once by the product's manufacturer / supplier,
    // to be shared with other network participants, and remains largely static.
    // It can also be used for instance-level (lot) master data.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
    pub struct Product<AccountId, Moment> {
        // The product ID would typically be a GS1 GTIN (Global Trade Item Number),
        // or ASIN (Amazon Standard Identification Number), or similar,
        // a numeric or alpha-numeric code with a well-defined data structure.
        id: ProductId,
        // This is account that represents the owner of this product, as in
        // the manufacturer or supplier providing this product within the value chain.
        owner: AccountId,
        // This a series of properties describing the product.
        // Typically, there would at least be a textual description, and SKU.
        // It could also contain instance / lot master data e.g. expiration, weight, harvest date.
        props: Option<Vec<ProductProperty>>,
        // Timestamp (approximate) at which the prodct was registered on-chain.
        registered: Moment,
    }

    // Contains a name-value pair for a product property e.g. description: Ingredient ABC
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
    pub struct ProductProperty {
        // Name of the product property e.g. desc or description
        name: PropName,
        // Value of the product property e.g. Ingredient ABC
        value: PropValue,
    }

    impl ProductProperty {
        pub fn new(name: &[u8], value: &[u8]) -> Self {
            Self {
                name: name.to_vec(),
                value: value.to_vec(),
            }
        }

        pub fn name(&self) -> &[u8] {
            self.name.as_ref()
        }

        pub fn value(&self) -> &[u8] {
            self.value.as_ref()
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CreateRoleOrigin: EnsureOrigin<Self::Origin>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn product_by_id)]
    pub type Products<T: Config> =
        StorageMap<_, Blake2_128Concat, ProductId, Product<T::AccountId, T::Moment>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn products_of_org)]
    pub type ProductsOfOrganization<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<ProductId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn owner_of)]
    pub type OwnerOf<T: Config> = StorageMap<_, Blake2_128Concat, ProductId, T::AccountId, OptionQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProductRegistered(T::AccountId, ProductId, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        ProductIdMissing,
        ProductIdTooLong,
        ProductIdExists,
        ProductTooManyProps,
        ProductInvalidPropName,
        ProductInvalidPropValue,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
       

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000 )] 
            pub fn register_product(origin:OriginFor<T>, id: ProductId, owner: T::AccountId, props: Option<Vec<ProductProperty>>) -> DispatchResultWithPostInfo {
                T::CreateRoleOrigin::ensure_origin(origin.clone())?;
                let who = ensure_signed(origin)?;
    
                // Validate product ID
                Self::validate_product_id(&id)?;
    
                // Validate product props
                Self::validate_product_props(&props)?;
    
                // Check product doesn't exist yet (1 DB read)
                Self::validate_new_product(&id)?;
    
                // TODO: if organization has an attribute w/ GS1 Company prefix,
                //       additional validation could be applied to the product ID
                //       to ensure its validity (same company prefix as org).
    
                // Create a product instance
                let product = Self::new_product()
                    .identified_by(id.clone())
                    .owned_by(owner.clone())
                    .registered_on(<pallet_timestamp::Pallet<T>>::now())
                    .with_props(props)
                    .build();
    
                // Add product & ownerOf (3 DB writes)
                <Products<T>>::insert(&id, product);
                <ProductsOfOrganization<T>>::append(&owner, &id);
                <OwnerOf<T>>::insert(&id, &owner);
    
                Self::deposit_event(Event::ProductRegistered(who, id, owner));
    
                Ok(().into())
            }
        
    }

    #[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        fn new_product() -> ProductBuilder<T::AccountId, T::Moment> {
            ProductBuilder::<T::AccountId, T::Moment>::default()
        }

        pub fn validate_product_id(id: &[u8]) -> Result<(), Error<T>> {
            // Basic product ID validation
            ensure!(!id.is_empty(), Error::<T>::ProductIdMissing);
            ensure!(
                id.len() <= PRODUCT_ID_MAX_LENGTH,
                Error::<T>::ProductIdTooLong
            );
            Ok(())
        }

        pub fn validate_new_product(id: &[u8]) -> Result<(), Error<T>> {
            // Product existence check
            ensure!(
                !<Products<T>>::contains_key(id),
                Error::<T>::ProductIdExists
            );
            Ok(())
        }

        pub fn validate_product_props(
            props: &Option<Vec<ProductProperty>>,
        ) -> Result<(), Error<T>> {
            if let Some(props) = props {
                ensure!(
                    props.len() <= PRODUCT_MAX_PROPS,
                    Error::<T>::ProductTooManyProps,
                );
                for prop in props {
                    ensure!(
                        prop.name().len() <= PRODUCT_PROP_NAME_MAX_LENGTH,
                        Error::<T>::ProductInvalidPropName
                    );
                    ensure!(
                        prop.value().len() <= PRODUCT_PROP_VALUE_MAX_LENGTH,
                        Error::<T>::ProductInvalidPropValue
                    );
                }
            }
            Ok(())
        }
    }

    #[derive(Default)]
    pub struct ProductBuilder<AccountId, Moment>
    where
        AccountId: Default,
        Moment: Default,
    {
        id: ProductId,
        owner: AccountId,
        props: Option<Vec<ProductProperty>>,
        registered: Moment,
    }

    impl<AccountId, Moment> ProductBuilder<AccountId, Moment>
    where
        AccountId: Default,
        Moment: Default,
    {
        pub fn identified_by(mut self, id: ProductId) -> Self {
            self.id = id;
            self
        }

        pub fn owned_by(mut self, owner: AccountId) -> Self {
            self.owner = owner;
            self
        }

        pub fn with_props(mut self, props: Option<Vec<ProductProperty>>) -> Self {
            self.props = props;
            self
        }

        pub fn registered_on(mut self, registered: Moment) -> Self {
            self.registered = registered;
            self
        }

        pub fn build(self) -> Product<AccountId, Moment> {
            Product::<AccountId, Moment> {
                id: self.id,
                owner: self.owner,
                props: self.props,
                registered: self.registered,
            }
        }
    }
}
