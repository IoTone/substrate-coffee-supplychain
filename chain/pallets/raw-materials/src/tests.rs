use super::Event as RawEvent;
use super::{Config, RawMaterialsOfOrganization, };
use crate::types::*;
use crate::{mock::*, Error};

use fixed::types::extra::U16;
use fixed::FixedI32;
use frame_support::{assert_noop, assert_ok, dispatch};

const TEST_RAW_MATERIAL_ID: &str = "000123456789";
const TEST_ORGANIZATION: &str = "Northwind";
const TEST_SENDER: &str = "Alice";

#[test]
fn create_raw_material() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();
        let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let result = RawMaterials::register_raw_material(
            Origin::signed(sender), 
            id.clone(), 
            State::Grinded, 
            owner, 
            Some(vec![1,2,3,4]), 
            FixedI32::<U16>::from_bits(100 << 16),
        );

        assert_ok!(result);
    });
}

#[test]
fn create_raw_material_with_invalid_sender() {
    new_test_ext().execute_with(|| {
        let id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();
        let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let result = RawMaterials::register_raw_material(
            Origin::none(), 
            id.clone(), 
            State::Grinded, 
            owner, 
            Some(vec![1,2,3,4]), 
            FixedI32::<U16>::from_bits(100 << 16),
        );

        assert_noop!(result, dispatch::DispatchError::BadOrigin);
    });
}
