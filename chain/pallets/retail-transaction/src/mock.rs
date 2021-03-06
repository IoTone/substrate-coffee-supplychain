// Creating mock runtime here
use crate as retail_transaction;
use crate::*;
use frame_support::parameter_types;
use frame_system as system;
use frame_system::RawOrigin;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup},
};

use core::marker::PhantomData;
use frame_support::traits::EnsureOrigin;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        RetailTransaction: retail_transaction::{Pallet, Call, Storage, Event<T>},
        CoffeProducts: coffe_products::{Pallet, Call, Storage, Event<T>},
        SupplyChain: supply_chain::{Pallet, Call, Storage, Event<T>},
        RawMaterials:raw_materials::{Event<T>}
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
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

impl supply_chain::Config for Test {
    type CreateRoleOrigin = MockOrigin<Test>;
    type Event = Event;
}
impl retail_transaction::Config for Test {
    type Event = Event;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}
impl coffe_products::Config for Test {
    type Event = Event;
    type CreateRoleOrigin = MockOrigin<Test>;
}
impl raw_materials::Config for Test {
    type Event = Event;
    type CreateRoleOrigin = MockOrigin<Test>;
}
type TestExtrinsic = TestXt<Call, ()>;

impl<C> system::offchain::SendTransactionTypes<C> for Test
where
    Call: From<C>,
{
    type OverarchingCall = Call;
    type Extrinsic = TestExtrinsic;
}

 
pub struct MockOrigin<T>(PhantomData<T>);

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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let mut ext = sp_io::TestExternalities::from(storage);
    // Events are not emitted on block 0 -> advance to block 1.
    // Any dispatchable calls made during genesis block will have no events emitted.
    ext.execute_with(|| System::set_block_number(1));
    ext
}
