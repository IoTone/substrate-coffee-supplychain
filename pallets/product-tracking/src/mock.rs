// Creating mock runtime here
use crate as product_tracking;
use crate::*;
use core::marker::PhantomData;
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, traits::EnsureOrigin};
use frame_system as system;
use frame_system::RawOrigin;
use sp_core::{
    // offchain::{
    //     testing::{self, OffchainState, PoolState},
    //     OffchainExt, TransactionPoolExt,
    // },
    sr25519,
    Pair,
    H256,
};
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup},
};

pub use pallet_timestamp::Call as TimestampCall;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
        NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		ProductTracking: product_tracking::{Pallet, Call, Storage, Event<T>},
        ProductRegistry: product_registry::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub const MinimumPeriod: u64 = 1;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl product_tracking::Config for Test {
    type CreateRoleOrigin = MockOrigin<Test>;

    type Event = Event;
}
impl product_registry::Config for Test {
    type CreateRoleOrigin = MockOrigin<Test>;
    type Event = Event;
}

pub struct MockOrigin<T>(PhantomData<T>);


// pub type ProductTracking = Pallet<Test>;
// pub type Timestamp = pallet_timestamp::Pallet<Test>;


impl<T: Config> EnsureOrigin<T::Origin> for MockOrigin<T> {
    type Success = T::AccountId;
    fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
        o.into().and_then(|o| match o {
            RawOrigin::Signed(ref who) => Ok(who.clone()),
            r => Err(T::Origin::from(r)),
        })
    }
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.

pub fn account_key(s: &str) -> sr25519::Public {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}

// Offchain worker

type TestExtrinsic = TestXt<Call, ()>;

impl<C> system::offchain::SendTransactionTypes<C> for Test
where
    Call: From<C>,
{
    type OverarchingCall = Call;
    type Extrinsic = TestExtrinsic;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
    t.into()       
}