use crate::mock::*;
use hex_literal::hex;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

// took these from: https://github.com/CosmWasm/cosmwasm/blob/main/contracts/crypto-verify/tests/integration.rs
const SECP256K1_MESSAGE: &[u8] = &hex!("5c868fedb8026979ebd26f1ba07c27eedf4ff6d10443505a96ecaf21ba8c4f0937b3cd23ffdc3dd429d4cd1905fb8dbcceeff1350020e18b58d2ba70887baa3a9b783ad30d3fbf210331cdd7df8d77defa398cdacdfc2e359c7ba4cae46bb74401deb417f8b912a1aa966aeeba9c39c7dd22479ae2b30719dca2f2206c5eb4b7");
//const SECP256K1_SIGNATURE: &[u8] = &hex!("207082eb2c3dfa0b454e0906051270ba4074ac93760ba9e7110cd9471475111151eb0dbbc9920e72146fb564f99d039802bf6ef2561446eb126ef364d21ee9c4");
//const SECP256K1_PUBLIC_KEY: &[u8] = &hex!("04051c1ee2190ecfb174bfe4f90763f2b4ff7517b70a2aec1876ebcfd644c4633fb03f3cfbd94b1f376e34592d9d41ccaf640bb751b00a1fadeb0c01157769eb73");

// derived from sp_core::crypto::DEV_PHRASE
const SECP256K1_SIGNATURE: &[u8] = &hex!("5d4382a5de4b98ee1d9c729a5eda75c185c13698bd1911f487dc053cd464985a4156a9bbb14e4ca8cd7800246da504ceab3f7b4e991eb417586c074d0cb85cc9");
const SECP256K1_PUBLIC_KEY: &[u8] = &hex!("045b26108e8b97479c547da4860d862dc08ab2c29ada449c74d5a9a58a6c46a8c4bd893eee2e9653dbf593c468491ec30359376e0aab8c64f3e8c4aaf643727987");

// TEST 3 test vector from https://tools.ietf.org/html/rfc8032#section-7.1
const ED25519_MESSAGE: &[u8] = &hex!("af82");
const ED25519_SIGNATURE: &[u8] = &hex!("6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a");
const ED25519_PUBLIC_KEY: &[u8] =
	&hex!("fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025");

// Signed text "connect all the things" using MyEtherWallet with private key
// b5b1870957d373ef0eeffecc6e4812c0fd08f554b37b233526acc331bf1544f7
const ETHEREUM_MESSAGE: &str = "connect all the things";
const ETHEREUM_SIGNATURE: &[u8] = &hex!("dada130255a447ecf434a2df9193e6fbba663e4546c35c075cd6eea21d8c7cb1714b9b65a4f7f604ff6aad55fba73f8c36514a512bbbba03709b37069194f8a41b");
const ETHEREUM_SIGNER_ADDRESS: &[u8] = &hex!("12890D2cce102216644c59daE5baed380d84830c");

// TEST 2 test vector from https://tools.ietf.org/html/rfc8032#section-7.1
const ED25519_MESSAGE2: &[u8] = &hex!("72");
const ED25519_SIGNATURE2: &[u8] = &hex!("92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00");
const ED25519_PUBLIC_KEY2: &[u8] =
	&hex!("3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c");

#[test]
fn secp256k1_verify_verifies() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let message = SECP256K1_MESSAGE;
		let signature = SECP256K1_SIGNATURE;
		let public_key = SECP256K1_PUBLIC_KEY;
		let hash = Sha256::digest(message);

		assert!(Cosmwasm::do_secp256k1_verify(&hash, &signature, &public_key))
	})
}

#[test]
fn secp256k1_recover_pubkey_recovers() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let message = SECP256K1_MESSAGE;
		let signature = SECP256K1_SIGNATURE;
		let hash = Sha256::digest(message);

		assert_eq!(
			Cosmwasm::do_secp256k1_recover_pubkey(&hash, &signature, 1),
			Ok(SECP256K1_PUBLIC_KEY.to_vec())
		);
	})
}

#[test]
fn secp256k1_verify_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let message = SECP256K1_MESSAGE;
		let mut signature = SECP256K1_SIGNATURE.to_vec();
		let public_key = SECP256K1_PUBLIC_KEY;
		let hash = Sha256::digest(message);

		*signature.last_mut().unwrap() += 1;

		assert!(!Cosmwasm::do_secp256k1_verify(&hash, &signature, &public_key))
	})
}

#[test]
fn secp256k1_recover_pubkey_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let mut hasher = Keccak256::new();
		hasher.update(format!("\x19Ethereum Signed Message:\n{}", ETHEREUM_MESSAGE.len()));
		hasher.update(ETHEREUM_MESSAGE);
		let message_hash = hasher.finalize();
		let signature = ETHEREUM_SIGNATURE;
		let signer_address = ETHEREUM_SIGNER_ADDRESS;

		let (recovery, signature) = signature.split_last().unwrap();

		let recovered_pubkey =
			Cosmwasm::do_secp256k1_recover_pubkey(&message_hash, signature, *recovery - 27)
				.unwrap();
		let recovered_pubkey_hash = Keccak256::digest(&recovered_pubkey[1..]);

		assert_eq!(signer_address, &recovered_pubkey_hash[recovered_pubkey_hash.len() - 20..]);
	})
}

#[test]
fn ed25519_verify_verifies() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let message = ED25519_MESSAGE;
		let signature = ED25519_SIGNATURE;
		let public_key = ED25519_PUBLIC_KEY;

		assert!(Cosmwasm::do_ed25519_verify(&message, &signature, &public_key));
	})
}

#[test]
fn ed25519_verify_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let message = ED25519_MESSAGE;
		let mut signature = ED25519_SIGNATURE.to_vec();
		let public_key = ED25519_PUBLIC_KEY;

		*signature.last_mut().unwrap() += 1;

		assert!(!Cosmwasm::do_ed25519_verify(&message, &signature, &public_key));
	})
}

#[test]
fn ed25519_batch_verify_verifies() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		assert!(Cosmwasm::do_ed25519_batch_verify(
			&[ED25519_MESSAGE, ED25519_MESSAGE2],
			&[ED25519_SIGNATURE, ED25519_SIGNATURE2],
			&[ED25519_PUBLIC_KEY, ED25519_PUBLIC_KEY2],
		));
	})
}

#[test]
fn ed25519_batch_verify_verifies_multisig() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		assert!(Cosmwasm::do_ed25519_batch_verify(
			&[ED25519_MESSAGE],
			&[ED25519_SIGNATURE, ED25519_SIGNATURE],
			&[ED25519_PUBLIC_KEY, ED25519_PUBLIC_KEY],
		));
	})
}

#[test]
fn ed25519_batch_verify_verifies_with_single_pubkey_multi_msg() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		assert!(Cosmwasm::do_ed25519_batch_verify(
			&[ED25519_MESSAGE, ED25519_MESSAGE],
			&[ED25519_SIGNATURE, ED25519_SIGNATURE],
			&[ED25519_PUBLIC_KEY],
		));
	})
}

#[test]
fn ed25519_batch_verify_fails_if_one_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let mut bad_signature = ED25519_SIGNATURE2.to_vec();
		*bad_signature.last_mut().unwrap() += 1;

		assert!(!Cosmwasm::do_ed25519_batch_verify(
			&[ED25519_MESSAGE, ED25519_MESSAGE2],
			&[ED25519_SIGNATURE, bad_signature.as_slice()],
			&[ED25519_PUBLIC_KEY, ED25519_PUBLIC_KEY2],
		));
	})
}

#[test]
fn ed25519_batch_verify_fails_if_input_lengths_are_incorrect() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		assert!(!Cosmwasm::do_ed25519_batch_verify(
			&[ED25519_MESSAGE, ED25519_MESSAGE2],
			&[ED25519_SIGNATURE],
			&[ED25519_PUBLIC_KEY, ED25519_PUBLIC_KEY2],
		));
	})
}

#[test]
fn ss58_address_format_is_supported_correctly() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let valid_ss58_addresses = [
			(
				"5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL",
				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			),
			(
				"5txRkPpGeTRJyZ96t5aSxLKaQDa32ZY21rq8MDHaN7dLGCBe",
				"10dbdfc9a706a4cf96b9e9dfb25384a2cf25faeaddabd4c98079f8360bc4ad46",
			),
			(
				"5uawZPfyfP9hdowPJbeiR2GMSZatLq3b9wpWc6yWjSLeakgh",
				"2cb50f2480175397eb320e637fc56be1939e18fb2b326eab5fdeaad9d43ffc74",
			),
			(
				"5umjqLRoE5wrXUGyedwbATZjj1SukRC9eh8qGJPpVx47bUam",
				"34f149d3a32ff2afe4daee3f4c917b90a73b88ee84a2666b477cdd67d6c5d17b",
			),
		];
		for (ss58_addr, hex_addr) in valid_ss58_addresses {
			// ss58 string to AccountId works
			let lhs = Cosmwasm::cosmwasm_addr_to_account(ss58_addr.into()).unwrap();
			// address binary to canonical AccountId works
			let binary_addr = hex::decode(hex_addr).unwrap();
			let rhs = Cosmwasm::canonical_addr_to_account(binary_addr.into()).unwrap();
			assert_eq!(lhs, rhs);
		}

		let not_valid_ss58_addresses = [
			// length is correct but with some garbage string
			"5yasdX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL",
			// total garbage
			"someaddr",
		];

		for garbage_addr in not_valid_ss58_addresses {
			assert!(Cosmwasm::cosmwasm_addr_to_account(garbage_addr.into()).is_err());
		}
	})
}
