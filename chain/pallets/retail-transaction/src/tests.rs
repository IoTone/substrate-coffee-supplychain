use super::types::*;
use crate::{mock::*, Error};

use fixed::FixedI32;
use frame_support::{ assert_noop, assert_ok};

const TEST_RETAIL_PACKAGING_ID: &str = "000123456789";
const TEST_RETAIL_PACKAGING_ID2: &str = "0001234567892";
const TEST_ORIGIN_PROCESS_ID: &str = "09275029357";
const TEST_RETAIL_SALE_ID: &str = "000123456788";
const TEST_SERIAL_NUMBER: &str = "00098235872375";
 const TEST_SKU: &str = "SFE-2345";
const TEST_BRAND: &str = "SUPER COFFEE BRAND";
 const TEST_SENDER: &str = "Alice";
const TEST_BUYER: &str = "Bob";
 
#[test]
fn create_retail_packaging() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_ok!(result);
    });
}

#[test]
fn create_retail_packaging_with_invalid_id() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = Vec::new();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidOrMissingIdentifier);
    });
}

#[test]
fn create_retail_packaging_with_invalid_amount() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(0),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidQuantity);
    });
}

#[test]
fn create_retail_packaging_with_invalid_amount_for_products() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(-10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidQuantity);
    });
}
#[test]
fn create_retail_packaging_with_invalid_amount_of_products() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(-5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidQuantity);
    });
}
#[test]
fn create_retail_packaging_with_invalid_price_for_products() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(-50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidCost);
    });
}
#[test]
fn create_retail_packaging_with_existing_id() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id.clone(),
            certs.clone(),
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku.clone(),
            serial_number.clone(),
            brand.clone(),
            origin_process.clone(),
            coffe_products::types::Kind::Bag,
            owner.clone(),
        );
        assert_ok!(result);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::RetailPackagingIdAlreadyExists);
    });
}

#[test]
fn create_retail_packaging_with_existing_sku() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id.clone(),
            certs.clone(),
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku.clone(),
            serial_number.clone(),
            brand.clone(),
            origin_process.clone(),
            coffe_products::types::Kind::Bag,
            owner.clone(),
        );
        assert_ok!(result);
        let id = TEST_RETAIL_PACKAGING_ID2.as_bytes().to_vec();

        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku,
            serial_number,
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_noop!(result,Error::<Test>::InvalidSku);
    });
}

#[test]
fn create_retail_transaction() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let buyer = account_key(TEST_BUYER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku.clone(),
            serial_number.clone(),
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_ok!(result);
        let id = TEST_RETAIL_SALE_ID.as_bytes().to_vec();
        let currency="USD".as_bytes().to_vec();
        let result = RetailTransaction::create_sale(Origin::signed(owner), id, currency, FixedI32::from_num(5), sku, serial_number, buyer, owner);
        assert_ok!(result);

    });
}

#[test]
fn create_retail_transaction_whith_invalid_amount() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let buyer = account_key(TEST_BUYER);
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let sku =TEST_SKU.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku.clone(),
            serial_number.clone(),
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_ok!(result);
        let id = TEST_RETAIL_SALE_ID.as_bytes().to_vec();
        let currency="USD".as_bytes().to_vec();
        let result = RetailTransaction::create_sale(Origin::signed(owner), id, currency, FixedI32::from_num(0), sku, serial_number, buyer, owner);
        assert_noop!(result,Error::<Test>::InvalidQuantity);

    });
}
#[test]
fn create_retail_transaction_whith_invalid_id() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_SENDER);
        let buyer = account_key(TEST_BUYER);
        let sku =TEST_SKU.as_bytes().to_vec();
        let id = TEST_RETAIL_PACKAGING_ID.as_bytes().to_vec();
        let serial_number=TEST_SERIAL_NUMBER.as_bytes().to_vec();
        let origin_process=TEST_ORIGIN_PROCESS_ID.as_bytes().to_vec();
        let brand=TEST_BRAND.as_bytes().to_vec();
        let mut certs: Certifications = Vec::new();
        let cert = "gluten free".as_bytes().to_vec();
        certs.push(cert);
        let result = RetailTransaction::create_retail_packaging(
            Origin::signed(owner),
            id,
            certs,
            FixedI32::from_num(50),
            FixedI32::from_num(5),
            FixedI32::from_num(10),
            FixedI32::from_num(50),
            sku.clone(),
            serial_number.clone(),
            brand,
            origin_process,
            coffe_products::types::Kind::Bag,
            owner,
        );

        assert_ok!(result);
        let id = Vec::new();

        let currency="USD".as_bytes().to_vec();
        let result = RetailTransaction::create_sale(Origin::signed(owner), id, currency, FixedI32::from_num(5), sku, serial_number, buyer, owner);
        assert_noop!(result,Error::<Test>::InvalidOrMissingIdentifier);

    });
}