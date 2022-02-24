use crate::{mock::*, Error};

use frame_support::{ assert_noop, assert_ok};

const TEST_ORGANIZATION: &str = "Northwind";
const TEST_ALICE: &str = "Alice";
const TEST_BOB: &str = "Bob";

#[test]
fn create_org() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner), org.to_vec());

        assert_ok!(result);
    });
}
#[test]
fn create_existing_org() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner), org.to_vec());

        assert_ok!(result);

        let result = Registrar::create_organization(Origin::signed(owner), org.to_vec());

        assert_noop!(result,Error::<Test>::OrganizationExists);
    });
}
#[test]
fn add_user_to_org() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner), org.to_vec());

        assert_ok!(result);
        let user = account_key(TEST_BOB);
        let result = Registrar::add_to_organization(Origin::signed(owner),user);
        assert_ok!(result);
    });
}
#[test]
fn add_user_to_invalid_org() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner), org.to_vec());

        assert_ok!(result);
        let user = account_key(TEST_BOB);
        let result = Registrar::add_to_organization(Origin::signed(user),user);
        assert_noop!(result,Error::<Test>::InvalidOrganization);
    });
}
#[test]
fn add_existing_user_to_org() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner), org.to_vec());

        assert_ok!(result);
        let user = account_key(TEST_BOB);
        let result = Registrar::add_to_organization(Origin::signed(owner),user);
        assert_ok!(result);
        let result = Registrar::add_to_organization(Origin::signed(owner),user);
        assert_noop!(result,Error::<Test>::MemberOfOrganization);
    });
}
#[test]
fn verify_if_user_is_part_of_organization() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner.clone()), org.to_vec());

        assert_ok!(result);
        let result = Registrar::part_of_organization(&owner);
        assert!(result);
    });
}
#[test]
fn verify_if_user_is_not_part_of_organization() {
    new_test_ext().execute_with(|| {
        let owner = account_key(TEST_ALICE);
        let org = account_key(TEST_ORGANIZATION);

        let result = Registrar::create_organization(Origin::signed(owner.clone()), org.to_vec());
        assert_ok!(result);
        let user = account_key(TEST_ORGANIZATION);
        let result = Registrar::part_of_organization(&user);
        assert!(!result);
    });
}
