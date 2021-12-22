#![cfg_attr(not(feature = "std"), no_std)]
mod types;

use frame_support::log::debug;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::*;
    use codec::alloc::string::ToString;
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_support::pallet_prelude::EnsureOrigin;
    use frame_support::pallet_prelude::IsType;
    use frame_support::pallet_prelude::OptionQuery;
    use frame_support::Blake2_128Concat;

    use frame_support::pallet_prelude::*;
    use frame_system::offchain::SendTransactionTypes;
    use frame_system::pallet_prelude::OriginFor;
    use frame_system::pallet_prelude::*;
    use sp_runtime::offchain::storage::StorageValueRef;
    use sp_std::vec;
    use sp_std::vec::Vec; 
    pub const IDENTIFIER_MAX_LENGTH: usize = 36;
    pub const SHIPMENT_MAX_PRODUCTS: usize = 10;
    pub const LISTENER_ENDPOINT: &str = "http://localhost:3005";
    pub const LOCK_TIMEOUT_EXPIRATION: u64 = 3000; // in milli-seconds

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_timestamp::Config
        + SendTransactionTypes<Call<Self>>
        + raw_materials::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CreateRoleOrigin: EnsureOrigin<Self::Origin>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn processes_by_id)]
    pub type Processes<T: Config> =
        StorageMap<_, Blake2_128Concat, SupplyProcessId, SupplyProcess<T::Moment>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn processes_of_org)]
    pub type ProcessesOfOrganization<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<SupplyProcessId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn processes_of_raw_material)]
    pub type ProcessesOfRawMaterial<T: Config> =
        StorageMap<_, Blake2_128Concat, RawMaterialId, Vec<SupplyProcessId>, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        ProcessRegistered(T::AccountId, SupplyProcessId, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        InvalidOrMissingIdentifier,
        ProcessAlreadyExists,
        ProcessHasBeenDelivered,
        ProcessIsInTransit,
        ProcessIsUnknown,
        ProcessHasTooManyProducts,
        ShippingEventAlreadyExists,
        ShippingEventMaxExceeded,
        OffchainWorkerAlreadyBusy,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_process(
            origin: OriginFor<T>,
            id:SupplyProcessId,
            attribute: ProcessAttribute,
            certifications: Certifications,
            amount: Amount,
            input_amount: Amount,
            process_type: ProcessType,
            raw_material_id: RawMaterialId,
            owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
          
            <raw_materials::Pallet<T>>::verify_amount(amount)?;
            <raw_materials::Pallet<T>>::verify_amount(input_amount)?;
            <raw_materials::Pallet<T>>::update_remaining_amount(input_amount, raw_material_id.clone())?;
            let process = SupplyProcess::new(
                id,
                attribute,
                certifications,
                <pallet_timestamp::Pallet<T>>::now(),
                amount,
                input_amount,
                process_type,
                raw_material_id.clone(),
            );
            let id =process.id.clone();
            <Processes<T>>::insert(&id, process);
            <ProcessesOfOrganization<T>>::append(&owner, &id);
            <ProcessesOfRawMaterial<T>>::append(&raw_material_id, &id);
  sp_std::if_std! {
                // This code is only being compiled and executed when the `std` feature is enabled.
                println!("Entro");
            }
            Ok(().into())
        }

      
    }

    #[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        pub fn get_bean_status(id:SupplyProcessId)->i8{
          let process=  <Processes<T>>::get(&id);
          match process{
              Some(p)=>{
                 match p.processType {
                    ProcessType::  Harvesing=>0,
                    ProcessType::Processing=>0, 
                    ProcessType::Roasting=>1,
                    ProcessType::Grinding=>2,
                    _=>0
                 }  
              },
              None=>0
          }
        }
    }
}
