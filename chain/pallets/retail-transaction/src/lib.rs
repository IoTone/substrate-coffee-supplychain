#![cfg_attr(not(feature = "std"), no_std)]
mod types;

/// Eddispatch::{self, DispatchResultWithPostInfo}o pallet_prelude:: * s  not needed.
/// }arn more about FRAME
//and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::*;
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        ensure,
        pallet_prelude::*,
    };

    use frame_system::pallet_prelude::OriginFor;
    use frame_system::pallet_prelude::*;
    use sp_std::{if_std, prelude::*, vec::Vec};
    pub const IDENTIFIER_MAX_LENGTH: usize = 36;

    use frame_system::{ensure_signed, RawOrigin};
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_timestamp::Config
        + supply_chain::Config
        + coffe_products::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn sales)]
    pub type Sales<T: Config> =
        StorageMap<_, Blake2_128Concat, Id, Sale<T::AccountId, T::Moment>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn sales_by_org)]
    pub type SalesByOrg<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Id>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn retail_packaging_by_org)]
    pub type RetailPackagingsByOrgs<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Id>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn skus)]
    pub type Skus<T: Config> = StorageMap<_, Blake2_128Concat, SKU, (), OptionQuery>;
    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewSale(T::AccountId, Vec<u8>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Cannot create the organization because it already exists.
        SaleIdAlreadyExists,
        /// Cannot add users to a non-existent organization.
        InvalidCost,
        /// Cannot add a user to an organization to which they already belong.
        InvalidQuantity,
        InvalidOrMissingIdentifier,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000)]
        pub fn create_sale(
            origin: OriginFor<T>,
            id: Id,
            currency: CurrencyType,
            cost: Cost,
            quantity: Quantity,
            sku: SKU,
            serial_number: SerialNumber,
            product_id: ProductId,
            buyer: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::validate_cost(cost.clone())?;
            Self::validate_identifier(&id)?;
            Self::validate_quantity(quantity.clone())?;
            Self::validate_new_sale(&id)?;

            let sale = Sale::new(
                id.clone(),
                <pallet_timestamp::Pallet<T>>::now(),
                currency,
                cost,
                quantity,
                sku,
                serial_number,
                product_id,
                who,
                buyer,
            );
            Sales::<T>::insert(&id, sale);
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn create_retail_packaging(
            origin: OriginFor<T>,
            id:Id,
            certifications: Certifications,
            amount: Quantity,
            amount_of_products: AmountOfProducts,
            amount_for_products: AmountForProducts,
            sku: SKU,
            serial_number: SerialNumber,
            brand: Brand,
            origin_process: OriginProcess,
            kind: coffe_products::types::Kind,
            org: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin.clone())?;
            let retail_packaging = RetailPackaging::new(
                id,
                <pallet_timestamp::Pallet<T>>::now(),
                certifications,
                amount,
                amount_of_products,
                amount_for_products,
                sku.clone(),
                serial_number,
                brand,
                origin_process,
            );

            let id = retail_packaging.id.clone();
            Skus::<T>::insert(&sku, ());
           let _= <coffe_products::Pallet<T>>::register_product(
               origin,
              id.clone(),
                kind,
                sku,
                amount_for_products,
                org,
                id.clone()
            );

            Ok(().into())
        }
    }
    #[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        pub fn validate_identifier(id: &[u8]) -> Result<(), Error<T>> {
            // Basic identifier validation
            ensure!(!id.is_empty(), Error::<T>::InvalidOrMissingIdentifier);
            ensure!(
                id.len() <= IDENTIFIER_MAX_LENGTH,
                Error::<T>::InvalidOrMissingIdentifier
            );
            Ok(())
        }
        pub fn validate_new_sale(id: &[u8]) -> Result<(), Error<T>> {
            // Shipment existence check
            ensure!(
                !<Sales<T>>::contains_key(id),
                Error::<T>::SaleIdAlreadyExists
            );
            Ok(())
        }
        pub fn validate_quantity(quantity: Quantity) -> Result<(), Error<T>> {
            ensure!(quantity > 0, Error::<T>::InvalidQuantity);

            Ok(())
        }
        pub fn validate_cost(cost: Cost) -> Result<(), Error<T>> {
            ensure!(cost > 0, Error::<T>::InvalidCost);
            Ok(())
        }
        pub fn validate_sku(sale_id: Id) -> Result<(), Error<T>> {
            Ok(())
        }
    }
}
