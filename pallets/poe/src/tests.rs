use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;


// create claim test case
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim), 
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn create_claim_failed_when_bad_claim_length() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim),
			Error::<Test>::BadClaimLength
		);
	});
}