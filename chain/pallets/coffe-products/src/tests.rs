use super::Event as RawEvent;
use super::{Config, Products, ProductsOfOrganization};
use crate::types::*;
use crate::{mock::*, Error};

use fixed::types::extra::U16;
use fixed::FixedI32;
use frame_support::{assert_noop, assert_ok, dispatch};

pub fn store_test_product<T: Config>(
    product_id: ProductId,
    date: T::Moment,
    kind: Kind,
    sku: Vec<u8>,
    lb: Decimal,
    amount: Decimal,
    remaining_amount: Decimal,
    price: Decimal,
    packaging_id: Identifier,
) {
    Products::<T>::insert(
        product_id.clone(),
        Product {
            product_id,
            date,
            kind,
            sku,
            lb,
            amount,
            remaining_amount,
            price,
            packaging_id,
        },
    );
}

const TEST_PRODUCT_ID: &str = "00012345600012";
const TEST_PACKAGING_ID: &str = "00012345678911";
const TEST_ORGANIZATION: &str = "Northwind";
const TEST_SENDER: &str = "Alice";
const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

#[test]
fn create_product() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let id = TEST_PRODUCT_ID.as_bytes().to_owned();
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let org = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let result = CoffeProducts::register_product(
            Origin::signed(sender),
            id.clone(),
            Kind::Grinded,
            sku,
            FixedI32::<U16>::from_bits(11 << 16),
            org,
            packaging_id,
            FixedI32::<U16>::from_bits(100 << 16),
            FixedI32::<U16>::from_bits(11 << 16),
        );

        assert_ok!(result);

        assert_eq!(
            CoffeProducts::product_by_id(&id),
            Some(Product {
                product_id: id.clone(),
                date: now,
                kind: Kind::Grinded,
                sku: vec![1, 2, 3, 4],
                lb: FixedI32::<U16>::from_bits(11 << 16),
                amount: FixedI32::<U16>::from_bits(100 << 16),
                remaining_amount: FixedI32::<U16>::from_bits(100 << 16),
                price: FixedI32::<U16>::from_bits(11 << 16),
                packaging_id: TEST_PACKAGING_ID.as_bytes().to_owned()
            })
        );

        assert_eq!(<ProductsOfOrganization<Test>>::get(org), vec![id.clone()]);
 
        assert!(System::events().iter().any(|er| er.event
            == Event::CoffeProducts(RawEvent::ProductRegistered(sender, id.clone(), org))));
    });
}

#[test]
fn create_product_aux() {
    new_test_ext().execute_with(|| {
        let id = TEST_PRODUCT_ID.as_bytes().to_owned();
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let org = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);
        let result = CoffeProducts::register_product_aux(
            id.clone(),
            Kind::Grinded,
            sku,
            FixedI32::<U16>::from_bits(11 << 16),
            org,
            packaging_id,
            FixedI32::<U16>::from_bits(100 << 16),
            FixedI32::<U16>::from_bits(11 << 16),
        );

        assert_ok!(result);

        assert_eq!(
            CoffeProducts::product_by_id(&id),
            Some(Product {
                product_id: id.clone(),
                date: now,
                kind: Kind::Grinded,
                sku: vec![1, 2, 3, 4],
                lb: FixedI32::<U16>::from_bits(11 << 16),
                amount: FixedI32::<U16>::from_bits(100 << 16),
                remaining_amount: FixedI32::<U16>::from_bits(100 << 16),
                price: FixedI32::<U16>::from_bits(11 << 16),
                packaging_id: TEST_PACKAGING_ID.as_bytes().to_owned()
            })
        );

        assert_eq!(<ProductsOfOrganization<Test>>::get(org), vec![id.clone()]);

     })
}

#[test]
fn sell_product() {
    new_test_ext().execute_with(|| {
        let id = TEST_PRODUCT_ID.as_bytes().to_owned();
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let now = 42;
        Timestamp::set_timestamp(now);

        store_test_product::<Test>(
            id.clone(),
            now.clone(),
            Kind::Grinded,
            sku.clone(),
            FixedI32::<U16>::from_bits(11 << 16),
            FixedI32::<U16>::from_bits(100 << 16),
            FixedI32::<U16>::from_bits(100 << 16),
            FixedI32::<U16>::from_bits(11 << 16),
            packaging_id.clone(),
        );
        let result = CoffeProducts::sell_product(id.clone(), FixedI32::<U16>::from_bits(60 << 16));

        assert_ok!(result);

        assert_eq!(
            CoffeProducts::product_by_id(&id),
            Some(Product {
                product_id: id.clone(),
                date: now,
                kind: Kind::Grinded,
                sku: vec![1, 2, 3, 4],
                lb: FixedI32::<U16>::from_bits(11 << 16),
                amount: FixedI32::<U16>::from_bits(100 << 16),
                remaining_amount: FixedI32::<U16>::from_bits(40 << 16),
                price: FixedI32::<U16>::from_bits(11 << 16),
                packaging_id: TEST_PACKAGING_ID.as_bytes().to_owned()
            })
        )
    });
}

#[test]
fn create_product_with_invalid_sender() {
    new_test_ext().execute_with(|| {
        let id = TEST_PRODUCT_ID.as_bytes().to_owned();
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let org = account_key(TEST_ORGANIZATION);
        assert_noop!(
            CoffeProducts::register_product(
                Origin::none(),
                id.clone(),
                Kind::Grinded,
                sku,
                FixedI32::<U16>::from_bits(11 << 16),
                org,
                packaging_id,
                FixedI32::<U16>::from_bits(100 << 16),
                FixedI32::<U16>::from_bits(11 << 16),
            ),
            dispatch::DispatchError::BadOrigin
        );
    });
}

#[test]
fn create_product_with_missing_id() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let org = account_key(TEST_ORGANIZATION);
        assert_noop!(
            CoffeProducts::register_product(
                Origin::signed(sender),
                vec!(),
                Kind::Grinded,
                sku,
                FixedI32::<U16>::from_bits(11 << 16),
                org,
                packaging_id,
                FixedI32::<U16>::from_bits(100 << 16),
                FixedI32::<U16>::from_bits(11 << 16),
            ),
            Error::<Test>::InvalidOrMissingIdentifier
        );
    });
}

#[test]
fn create_product_with_long_id() {
    new_test_ext().execute_with(|| {
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let org = account_key(TEST_ORGANIZATION);
        assert_noop!(
            CoffeProducts::register_product(
                Origin::signed(account_key(TEST_SENDER)),
                LONG_VALUE.as_bytes().to_owned(),
                Kind::Grinded,
                sku,
                FixedI32::<U16>::from_bits(11 << 16),
                org,
                packaging_id,
                FixedI32::<U16>::from_bits(100 << 16),
                FixedI32::<U16>::from_bits(11 << 16),
            ),
            Error::<Test>::InvalidOrMissingIdentifier
        );
    })
}

#[test]
fn create_product_with_existing_id() {
    new_test_ext().execute_with(|| {
        let existing_product = TEST_PRODUCT_ID.as_bytes().to_owned();
        let sender = account_key(TEST_SENDER);
        let packaging_id = TEST_PACKAGING_ID.as_bytes().to_owned();
        let sku = vec![1, 2, 3, 4];
        let org = account_key(TEST_ORGANIZATION);

        store_test_product::<Test>(
            existing_product.clone(),
            123,
            Kind::Grinded,
            sku.clone(),
            FixedI32::<U16>::from_bits(11 << 16),
            FixedI32::<U16>::from_bits(100 << 16),
            FixedI32::<U16>::from_bits(100 << 16),
            FixedI32::<U16>::from_bits(11 << 16),
            packaging_id.clone(),
        );

        assert_noop!(
            CoffeProducts::register_product(
                Origin::signed(sender),
                existing_product.clone(),
                Kind::Grinded,
                sku,
                FixedI32::<U16>::from_bits(11 << 16),
                org,
                packaging_id,
                FixedI32::<U16>::from_bits(100 << 16),
                FixedI32::<U16>::from_bits(11 << 16),
            ),
            Error::<Test>::ProductIdExists
        );
    })
}
