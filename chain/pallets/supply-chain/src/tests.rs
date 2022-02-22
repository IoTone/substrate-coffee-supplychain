
use crate::types::*;
use crate::{mock::*, Error};

use fixed::FixedI32;
use fixed::types::I16F16;
use fixed::types::extra::U16;
use frame_support::{assert_noop, assert_ok};

const TEST_PROCESS_ID: &str = "0000081408936590";
const TEST_RAW_MATERIAL_ID: &str = "000123456789";
const TEST_ALICE: &str = "Alice";

#[test]
fn create_process() {
    new_test_ext().execute_with(|| {
        create_raw_material();
        let owner = account_key(TEST_ALICE);
        let id = TEST_PROCESS_ID.as_bytes().to_vec();
        let raw_material_id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();

        let attribute = ProcessAttribute {
            name: AttributeName::Location,
            value: "west coast".as_bytes().to_vec(),
        };
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = SupplyChain::register_process(
            Origin::signed(owner),
            id,
            attribute,
            certs,
            I16F16::from_num(500),
            I16F16::from_num(50),
            ProcessType::Harvesting,
            raw_material_id,
            owner,
        );

        assert_ok!(result);
    });
}
#[test]
fn create_process_with_invalid_id() {
    new_test_ext().execute_with(|| {
        create_raw_material();
        let owner = account_key(TEST_ALICE);
        let id = Vec::new();
        let raw_material_id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();

        let attribute = ProcessAttribute {
            name: AttributeName::Location,
            value: "west coast".as_bytes().to_vec(),
        };
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = SupplyChain::register_process(
            Origin::signed(owner),
            id,
            attribute,
            certs,
            I16F16::from_num(500),
            I16F16::from_num(50),
            ProcessType::Harvesting,
            raw_material_id,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidOrMissingIdentifier);
    });
}
#[test]
fn create_process_with_existing_id() {
    new_test_ext().execute_with(|| {
        create_raw_material();
        let owner = account_key(TEST_ALICE);
         let raw_material_id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();
        let id = TEST_PROCESS_ID.as_bytes().to_vec();

        let attribute = ProcessAttribute {
            name: AttributeName::Location,
            value: "west coast".as_bytes().to_vec(),
        };
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = SupplyChain::register_process(
            Origin::signed(owner),
            id.clone(),
            attribute.clone(),
            certs.clone(),
            I16F16::from_num(500),
            I16F16::from_num(50),
            ProcessType::Harvesting,
            raw_material_id.clone(),
            owner,
        );
        assert_ok!(result);

        let result = SupplyChain::register_process(
            Origin::signed(owner),
            id,
            attribute,
            certs,
            I16F16::from_num(500),
            I16F16::from_num(50),
            ProcessType::Harvesting,
            raw_material_id,
            owner,
        );

        assert_noop!(result,Error::<Test>::ProcessIdAlreadyExists);
    });
}
#[test]
fn create_process_with_invalid_amount() {
    new_test_ext().execute_with(|| {
        create_raw_material();
        let owner = account_key(TEST_ALICE);
        let id = TEST_PROCESS_ID.as_bytes().to_vec();
        let raw_material_id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();

        let attribute = ProcessAttribute {
            name: AttributeName::Location,
            value: "west coast".as_bytes().to_vec(),
        };
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = SupplyChain::register_process(
            Origin::signed(owner),
            id,
            attribute,
            certs,
            I16F16::from_num(500),
            I16F16::from_num(2001),
            ProcessType::Harvesting,
            raw_material_id,
            owner,
        );

        assert!(result.is_err());
    });
}
fn create_raw_material() {
    
        let sender = account_key(TEST_ALICE);
        let id = TEST_RAW_MATERIAL_ID.as_bytes().to_owned();
         let now = 42;
        Timestamp::set_timestamp(now);
        let result = RawMaterials::register_raw_material(
            Origin::signed(sender),
            id.clone(),
             raw_materials::types::State::Grinded,
            sender,
            Some(vec![1, 2, 3, 4]),
            FixedI32::<U16>::from_num(2000),
        );

        assert_ok!(result);
   
}