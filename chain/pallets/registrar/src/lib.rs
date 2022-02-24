#![cfg_attr(not(feature = "std"), no_std)]

/// Eddispatch::{self, DispatchResultWithPostInfo}o pallet_prelude:: * s  not needed.
/// }arn more about FRAME
//and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::OriginFor;
    use frame_system::pallet_prelude::*;
    use sp_std::{ prelude::*, vec::Vec};

    use frame_system::{ensure_signed, RawOrigin};
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_did::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn organizations)]
    pub type Organizations<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn members_of)]
    pub type MembersOf<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::AccountId>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub orgs: Vec<(T::AccountId, Vec<u8>)>,
        pub members: Vec<(T::AccountId, Vec<T::AccountId>)>,
    }
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                orgs: Default::default(),
                members: Default::default(),
            }
        }
    }
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for org in self.orgs.iter() {
                match Pallet::<T>::create_org(&org.0, org.1.clone()) {
                    Err(e) => panic!("{:?}", e),
                    Ok(_) => (),
                }
            }

            for (org, members) in self.members.iter() {
                for member in members.iter() {
                    match Pallet::<T>::add_to_org(org, member) {
                        Err(e) => panic!("{:?}", e),
                        Ok(_) => (),
                    }
                }
            }
        }
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An organization has been created. [creator, organization_id]
        CreatedOrganization(T::AccountId, Vec<u8>),
        /// An account was added to an organization. [account, organization_id]
        AddedToOrganization(T::AccountId, Vec<u8>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Cannot create the organization because it already exists.
        OrganizationExists,
        /// Cannot add users to a non-existent organization.
        InvalidOrganization,
        /// Cannot add a user to an organization to which they already belong.
        MemberOfOrganization,
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
        pub fn create_organization(
            origin: OriginFor<T>,
            org_name: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::create_org(&who, org_name.clone())?;
            Self::deposit_event(Event::CreatedOrganization(who, org_name));
            Ok(().into())
        }

        /// Add an account to an organization. Will return an InvalidOrganization error if the organization
        /// does not exist or the account is already a member. Will emit a AddedToOrganization event on success.
        ///
        /// The dispatch origin for this call must be Signed.
        #[pallet::weight(10_000)]

        pub fn add_to_organization(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::add_to_org(&who, &account)?;
            Self::deposit_event(Event::AddedToOrganization(who, b"OrgMember".to_vec()));
            Ok(().into())
        }
    }
    #[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        pub fn create_org(owner: &T::AccountId, org_name: Vec<u8>) -> DispatchResult {
            let mut orgs = <Pallet<T>>::organizations();
            ensure!(!orgs.contains(&owner), Error::<T>::OrganizationExists);
            orgs.push(owner.clone());
            <Organizations<T>>::put(orgs);

            // DID add attribute
            <pallet_did::Pallet<T>>::create_attribute(&owner, &owner, b"Org", &org_name, None)?;
            Ok(())
        }

        pub fn add_to_org(org: &T::AccountId, account: &T::AccountId) -> DispatchResult {
            // Organizations list.
            let orgs = Self::organizations();
            ensure!(orgs.contains(&org), Error::<T>::InvalidOrganization);

            // Accounts that belong to a certain organization.
            let mut members = Self::members_of(&org);

            // Validate organization and account should not be part.
            if !members.contains(&account) {
                members.push(account.clone());
                MembersOf::<T>::insert(&org, members);
            } else {
                return Err(Error::<T>::MemberOfOrganization.into());
            }

            // Add account as a DID delegate.
            <pallet_did::Pallet<T>>::create_delegate(
                &org,
                &org,
                &account,
                &b"OrgMember".to_vec(),
                None,
            )?;
            Ok(())
        }

        /// Returns true if and only if the account is a member of an organization.
        pub fn part_of_organization(account: &T::AccountId) -> bool {
            let orgs = <Pallet<T>>::organizations();
            for org in orgs.iter() {
                if <pallet_did::Pallet<T>>::valid_delegate(org, &b"OrgMember".to_vec(), &account)
                    .is_ok()
                {
                    return true;
                }
            }
            false
        }
    }

    pub struct EnsureOrg<T>(sp_std::marker::PhantomData<T>);
    impl<T: Config> EnsureOrigin<T::Origin> for EnsureOrg<T> {
        type Success = T::AccountId;
        fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
            o.into().and_then(|o| match o {
                RawOrigin::Signed(ref who) if <Pallet<T>>::part_of_organization(&who) => {
                    Ok(who.clone())
                }
                r => Err(T::Origin::from(r)),
            })
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn successful_origin() -> T::Origin {
            T::Origin::from(RawOrigin::Signed(Default::default()))
        }
    }
}
