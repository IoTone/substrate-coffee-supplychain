#![cfg_attr(not(feature = "std"), no_std)]
mod types;
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
    use frame_support::{dispatch::DispatchResultWithPostInfo, ensure, pallet_prelude::*};
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
    #[pallet::getter(fn materials_by_id)]
    pub type RawMaterials<T: Config> =
        StorageMap<_, Blake2_128Concat, RawMaterialId, RawMaterial<T::Moment>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn materials_of_org)]
    pub type RawMaterialsOfOrganization<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<RawMaterialId>, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RawMaterialRegistered(T::AccountId, RawMaterialId, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        RawMaterialIdMissing,
        RawMaterialIdTooLong,
        RawMaterialIdExists,
        RawMaterialTooManyProps,
        RawMaterialInvalidPropName,
        RawMaterialInvalidPropValue,
        InvalidAmount,
        RawMaterialDontHaveEnoughAmount,
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
        pub fn register_raw_material(
            origin: OriginFor<T>,
            id: RawMaterialId,
            state: State,
            owner: T::AccountId,
            origin_process: Option<OriginProcess>,
            amount: Amount,
        ) -> DispatchResultWithPostInfo {
            T::CreateRoleOrigin::ensure_origin(origin.clone())?;
            let who = ensure_signed(origin)?;
            Self::verify_amount(amount.clone())?;
            // Create a product instance
            let raw_material = RawMaterial::new(
                id,
                amount,
                origin_process,
                state,
                <pallet_timestamp::Pallet<T>>::now(),
            );
            let id = raw_material.id.clone();
            // Add product & ownerOf (3 DB writes)
            <RawMaterials<T>>::insert(&id, raw_material);
            <RawMaterialsOfOrganization<T>>::append(&owner, &id);

            Self::deposit_event(Event::RawMaterialRegistered(who, id, owner));

            Ok(().into())
        }
    }

    #[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        pub fn verify_amount(amount: Amount) -> Result<(), Error<T>> {
            ensure!(amount > 0, Error::<T>::InvalidAmount);
            Ok(())
        }

        pub fn update_remaining_amount(amount: Amount, id: RawMaterialId) -> Result<(), Error<T>> {
            Self::verify_amount(amount.clone())?;

            let raw_material = <RawMaterials<T>>::get(&id);

            let mut raw_material = raw_material.unwrap();

            ensure!(
                raw_material.remaining_amount - amount >= 0,
                Error::<T>::RawMaterialIdMissing
            );

            raw_material = raw_material.subtract_amount(amount);
            <RawMaterials<T>>::insert(&id, raw_material);

            Ok(())
        }
    }
}
