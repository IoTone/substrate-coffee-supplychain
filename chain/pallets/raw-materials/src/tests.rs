
use crate::types::*;
use crate::{mock::*, Error};

 
use fixed::{types::extra::U16,FixedI32};

use frame_support::{assert_noop, assert_ok, dispatch};

const TEST_RAW_MATERIAL_ID: &str = "000123456789";
const TEST_ORGANIZATION: &str = "Northwind";
const TEST_SENDER: &str = "Alice";
const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

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
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_bits(100 << 16),
        );

        assert_ok!(result);
    });
}
#[test]
fn create_raw_material_with_existing_id() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();
        let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let _ = RawMaterials::register_raw_material(
            Origin::signed(sender),
            id.clone(),
            State::Grinded,
            owner,
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_bits(100 << 16),
        );
        let result = RawMaterials::register_raw_material(
            Origin::signed(sender),
            id.clone(),
            State::Grinded,
            owner,
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_bits(100 << 16),
        );
        assert_noop!(result, Error::<Test>::RawMaterialIdExists);
    });
}
#[test]
fn create_raw_material_with_missing_id() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
         let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let result = RawMaterials::register_raw_material(
            Origin::signed(sender),
            Vec::new(),
            State::Grinded,
            owner,
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_bits(100 << 16),
        );

        assert_noop!(result,Error::<Test>::InvalidOrMissingIdentifier);
    });
}
#[test]
fn create_raw_material_with_long_id() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
         let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let id = LONG_VALUE.as_bytes().to_owned();

        let result = RawMaterials::register_raw_material(
            Origin::signed(sender),
            id,
            State::Grinded,
            owner,
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_bits(100 << 16),
        );

        assert_noop!(result,Error::<Test>::InvalidOrMissingIdentifier);
    });
}
#[test]
fn create_raw_material_with_invalid_amount() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
         let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();

        let result = RawMaterials::register_raw_material(
            Origin::signed(sender),
            id,
            State::Grinded,
            owner,
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_num(0),
        );
        assert_noop!(result,Error::<Test>::InvalidAmount);
    });
}
#[test]
fn create_raw_material_and_update_amount() {
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
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_num(600),
        );
        assert_ok!(result);

        let result=RawMaterials::update_remaining_amount(FixedI32::<U16>::from_num(400), id);
        assert_ok!(result);
    });
}
#[test]
fn create_raw_material_and_update_with_invalid_amount() {
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
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_num(600),
        );
        assert_ok!(result);

        let result=RawMaterials::update_remaining_amount(FixedI32::<U16>::from_num(-5), id);
        assert!(result.is_err())
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
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_bits(100 << 16),
        );

        assert_noop!(result, dispatch::DispatchError::BadOrigin);
    });
}
