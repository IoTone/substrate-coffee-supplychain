#![cfg_attr(not(feature = "std"), no_std)]
pub mod types;

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
    use crate::types::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::{prelude::*, vec::Vec};
    // General constraints to limit data size
    // Note: these could also be passed as trait config parameters
    pub const PRODUCT_ID_MAX_LENGTH: usize = 36;
    pub const PRODUCT_PROP_NAME_MAX_LENGTH: usize = 10;
    pub const PRODUCT_PROP_VALUE_MAX_LENGTH: usize = 20;
    pub const PRODUCT_MAX_PROPS: usize = 3;

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
        StorageMap<_, Blake2_128Concat, ProductId, Product<T::Moment>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn products_of_org)]
    pub type ProductsOfOrganization<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<ProductId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn products_of_packaging)]
    pub type ProductsOfPackaging<T: Config> =
        StorageMap<_, Blake2_128Concat, Identifier, Vec<ProductId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn products_of_sku)]
    pub type ProductsOfSku<T: Config> =
        StorageMap<_, Blake2_128Concat, SKU, Vec<ProductId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn owner_of)]
    pub type OwnerOf<T: Config> =
        StorageMap<_, Blake2_128Concat, ProductId, T::AccountId, OptionQuery>;

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
        #[pallet::weight(10_000)]
        pub fn register_product(
            origin: OriginFor<T>,
            id: ProductId,
            kind: Kind,
            sku: SKU,
            lb: Decimal,
            org: T::AccountId,
            packaging_id: Identifier,
            amount: Decimal,
            price: Decimal,
        ) -> DispatchResultWithPostInfo {
            let product = Product::new(
                id,
                <pallet_timestamp::Pallet<T>>::now(),
                kind,
                sku.clone(),
                lb,
                amount,
                price,
                packaging_id.clone(),
            );
            let id = product.product_id.clone();
            let who = ensure_signed(origin)?;

            <Products<T>>::insert(&id, product);
            <ProductsOfOrganization<T>>::append(&org, &id);
            <ProductsOfPackaging<T>>::append(&packaging_id, &id);
            <ProductsOfSku<T>>::append(&sku, &id);

            Self::deposit_event(Event::ProductRegistered(who, id, org));

            Ok(().into())
        }
    }

    #[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        pub fn register_product_aux(
            id: ProductId,
            kind: Kind,
            sku: SKU,
            lb: Decimal,
            org: T::AccountId,
            packaging_id: Identifier,
            amount: Decimal,
            price: Decimal,
        ) -> DispatchResultWithPostInfo {
            let product = Product::new(
                id,
                <pallet_timestamp::Pallet<T>>::now(),
                kind,
                sku.clone(),
                lb,
                amount,
                price,
                packaging_id.clone(),
            );
            let id = product.product_id.clone();

            <Products<T>>::insert(&id, product);
            <ProductsOfOrganization<T>>::append(&org, &id);
            <ProductsOfPackaging<T>>::append(&packaging_id, &id);
            <ProductsOfSku<T>>::append(&sku, &id);

            Ok(().into())
        }
        pub fn sell_product(id: ProductId, amount: Decimal) -> Result<(), Error<T>> {
            let product = <Products<T>>::get(&id);
            let mut product = product.unwrap();

            product = product.sell(amount);

            <Products<T>>::insert(&id, product);
            Ok(())
        }
    }
}
