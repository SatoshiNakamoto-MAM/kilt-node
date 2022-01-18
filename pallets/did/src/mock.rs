// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

#![allow(clippy::from_over_into)]
#![allow(dead_code)]

use frame_support::{
	parameter_types,
	traits::{Currency, OnUnbalanced, ReservableCurrency},
	weights::constants::RocksDbWeight,
};
use frame_system::EnsureSigned;
use pallet_balances::NegativeImbalance;
use runtime_common::{constants::MICRO_KILT, AccountId, Balance};
use sp_core::{ecdsa, ed25519, sr25519, Pair};
use sp_keystore::{testing::KeyStore, KeystoreExt};
use sp_runtime::{
	testing::{Header, H256},
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	MultiSigner, SaturatedConversion,
};
use sp_std::sync::Arc;

use crate::{
	self as did,
	did_details::{
		DeriveDidCallAuthorizationVerificationKeyRelationship, DeriveDidCallKeyRelationshipResult,
		DidAuthorizedCallOperation, DidAuthorizedCallOperationWithVerificationRelationship, DidDetails,
		DidEncryptionKey, DidPublicKey, DidPublicKeyDetails, DidVerificationKey, DidVerificationKeyRelationship,
		RelationshipDeriveError,
	},
	service_endpoints::DidEndpoint,
	utils as crate_utils, AccountIdOf, Config, CurrencyOf, DidBlacklist, DidEndpointsCount, DidStorageVersion, KeyIdOf,
	ServiceEndpoints, StorageVersion,
};
#[cfg(not(feature = "runtime-benchmarks"))]
use crate::{DidRawOrigin, EnsureDidOrigin};

pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

pub type TestDidIdentifier = runtime_common::AccountId;
pub type TestKeyId = KeyIdOf<Test>;
pub type TestBlockNumber = runtime_common::BlockNumber;
pub type TestCtypeOwner = TestDidIdentifier;
pub type TestCtypeHash = runtime_common::Hash;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		Did: did::{Pallet, Call, Storage, Event<T>, Origin<T>},
		Ctype: ctype::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
	}
);

parameter_types! {
	pub const SS58Prefix: u8 = 38;
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = runtime_common::Hash;
	type Hashing = BlakeTwo256;
	type AccountId = <<runtime_common::Signature as Verify>::Signer as IdentifyAccount>::AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = ();

	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const MaxNewKeyAgreementKeys: u32 = 10u32;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxUrlLength: u32 = 200u32;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxTotalKeyAgreementKeys: u32 = 10u32;
	// IMPORTANT: Needs to be at least MaxTotalKeyAgreementKeys + 3 (auth, delegation, attestation keys) for benchmarks!
	#[derive(Debug, Clone)]
	pub const MaxPublicKeysPerDid: u32 = 13u32;
	pub const MaxBlocksTxValidity: u64 = 300u64;
	pub const Deposit: Balance = 10 * MICRO_KILT;
	pub const DidFee: Balance = MICRO_KILT;
	pub const MaxNumberOfServicesPerDid: u32 = 25u32;
	pub const MaxServiceIdLength: u32 = 50u32;
	pub const MaxServiceTypeLength: u32 = 50u32;
	pub const MaxServiceUrlLength: u32 = 100u32;
	pub const MaxNumberOfTypesPerService: u32 = 1u32;
	pub const MaxNumberOfUrlsPerService: u32 = 1u32;
}

pub struct ToAccount<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAccount<R>
where
	R: pallet_balances::Config,
	<R as frame_system::Config>::AccountId: From<AccountId>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		pallet_balances::Pallet::<R>::resolve_creating(&ACCOUNT_FEE.into(), amount);
	}
}

impl Config for Test {
	type DidIdentifier = TestDidIdentifier;
	type Origin = Origin;
	type Call = Call;
	type EnsureOrigin = EnsureSigned<TestDidIdentifier>;
	type OriginSuccess = AccountId;
	type Event = ();
	type Currency = Balances;
	type Deposit = Deposit;
	type Fee = DidFee;
	type FeeCollector = ToAccount<Test>;
	type MaxNewKeyAgreementKeys = MaxNewKeyAgreementKeys;
	type MaxTotalKeyAgreementKeys = MaxTotalKeyAgreementKeys;
	type MaxPublicKeysPerDid = MaxPublicKeysPerDid;
	type MaxBlocksTxValidity = MaxBlocksTxValidity;
	type WeightInfo = ();
	type MaxNumberOfServicesPerDid = MaxNumberOfServicesPerDid;
	type MaxServiceIdLength = MaxServiceIdLength;
	type MaxServiceTypeLength = MaxServiceTypeLength;
	type MaxServiceUrlLength = MaxServiceUrlLength;
	type MaxNumberOfTypesPerService = MaxNumberOfTypesPerService;
	type MaxNumberOfUrlsPerService = MaxNumberOfUrlsPerService;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 500;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const Fee: Balance = 0;
}

impl ctype::Config for Test {
	#[cfg(feature = "runtime-benchmarks")]
	type EnsureOrigin = EnsureSigned<TestDidIdentifier>;
	#[cfg(feature = "runtime-benchmarks")]
	type OriginSuccess = runtime_common::AccountId;

	#[cfg(not(feature = "runtime-benchmarks"))]
	type EnsureOrigin = EnsureDidOrigin<TestCtypeOwner, runtime_common::AccountId>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type OriginSuccess = DidRawOrigin<runtime_common::AccountId, TestCtypeOwner>;

	type CtypeCreatorId = TestCtypeOwner;
	type Event = ();
	type WeightInfo = ();
	type Currency = Balances;
	type Fee = Fee;
	type FeeCollector = ();
}

#[cfg(test)]
pub(crate) const ACCOUNT_00: runtime_common::AccountId = runtime_common::AccountId::new([1u8; 32]);
#[cfg(test)]
pub(crate) const ACCOUNT_01: runtime_common::AccountId = runtime_common::AccountId::new([2u8; 32]);
#[cfg(test)]
pub(crate) const ACCOUNT_FEE: runtime_common::AccountId = runtime_common::AccountId::new([u8::MAX; 32]);

const DEFAULT_AUTH_SEED: [u8; 32] = [4u8; 32];
const ALTERNATIVE_AUTH_SEED: [u8; 32] = [40u8; 32];
const DEFAULT_ENC_SEED: [u8; 32] = [254u8; 32];
const ALTERNATIVE_ENC_SEED: [u8; 32] = [255u8; 32];
const DEFAULT_ATT_SEED: [u8; 32] = [6u8; 32];
const ALTERNATIVE_ATT_SEED: [u8; 32] = [60u8; 32];
const DEFAULT_DEL_SEED: [u8; 32] = [7u8; 32];
const ALTERNATIVE_DEL_SEED: [u8; 32] = [70u8; 32];

/// Solely used to fill public keys in unit tests to check for correct error
/// throws. Thus, it does not matter whether the correct key types get added
/// such that we can use the ed25519 for all key types per default.
pub(crate) fn fill_public_keys(mut did_details: DidDetails<Test>) -> DidDetails<Test> {
	while (did_details.public_keys.len() as u32) < <Test as Config>::MaxPublicKeysPerDid::get() {
		did_details
			.public_keys
			.try_insert(
				H256::random(),
				DidPublicKeyDetails {
					key: DidPublicKey::from(DidVerificationKey::from(ed25519::Public::from_h256(H256::random()))),
					block_number: 0u64,
				},
			)
			.expect("Should not exceed BoundedBTreeMap size due to prior check");
	}
	did_details
}

pub fn get_did_identifier_from_ed25519_key(public_key: ed25519::Public) -> TestDidIdentifier {
	MultiSigner::from(public_key).into_account()
}

pub fn get_did_identifier_from_sr25519_key(public_key: sr25519::Public) -> TestDidIdentifier {
	MultiSigner::from(public_key).into_account()
}

pub fn get_did_identifier_from_ecdsa_key(public_key: ecdsa::Public) -> TestDidIdentifier {
	MultiSigner::from(public_key).into_account()
}

pub fn get_ed25519_authentication_key(default: bool) -> ed25519::Pair {
	if default {
		ed25519::Pair::from_seed(&DEFAULT_AUTH_SEED)
	} else {
		ed25519::Pair::from_seed(&ALTERNATIVE_AUTH_SEED)
	}
}

pub fn get_sr25519_authentication_key(default: bool) -> sr25519::Pair {
	if default {
		sr25519::Pair::from_seed(&DEFAULT_AUTH_SEED)
	} else {
		sr25519::Pair::from_seed(&ALTERNATIVE_AUTH_SEED)
	}
}

pub fn get_ecdsa_authentication_key(default: bool) -> ecdsa::Pair {
	if default {
		ecdsa::Pair::from_seed(&DEFAULT_AUTH_SEED)
	} else {
		ecdsa::Pair::from_seed(&ALTERNATIVE_AUTH_SEED)
	}
}

pub fn get_x25519_encryption_key(default: bool) -> DidEncryptionKey {
	if default {
		DidEncryptionKey::X25519(DEFAULT_ENC_SEED)
	} else {
		DidEncryptionKey::X25519(ALTERNATIVE_ENC_SEED)
	}
}

pub fn get_ed25519_attestation_key(default: bool) -> ed25519::Pair {
	if default {
		ed25519::Pair::from_seed(&DEFAULT_ATT_SEED)
	} else {
		ed25519::Pair::from_seed(&ALTERNATIVE_ATT_SEED)
	}
}

pub fn get_sr25519_attestation_key(default: bool) -> sr25519::Pair {
	if default {
		sr25519::Pair::from_seed(&DEFAULT_ATT_SEED)
	} else {
		sr25519::Pair::from_seed(&ALTERNATIVE_ATT_SEED)
	}
}

pub fn get_ecdsa_attestation_key(default: bool) -> ecdsa::Pair {
	if default {
		ecdsa::Pair::from_seed(&DEFAULT_ATT_SEED)
	} else {
		ecdsa::Pair::from_seed(&ALTERNATIVE_ATT_SEED)
	}
}

pub fn get_ed25519_delegation_key(default: bool) -> ed25519::Pair {
	if default {
		ed25519::Pair::from_seed(&DEFAULT_DEL_SEED)
	} else {
		ed25519::Pair::from_seed(&ALTERNATIVE_DEL_SEED)
	}
}

pub fn get_sr25519_delegation_key(default: bool) -> sr25519::Pair {
	if default {
		sr25519::Pair::from_seed(&DEFAULT_DEL_SEED)
	} else {
		sr25519::Pair::from_seed(&ALTERNATIVE_DEL_SEED)
	}
}

pub fn get_ecdsa_delegation_key(default: bool) -> ecdsa::Pair {
	if default {
		ecdsa::Pair::from_seed(&DEFAULT_DEL_SEED)
	} else {
		ecdsa::Pair::from_seed(&ALTERNATIVE_DEL_SEED)
	}
}

pub fn generate_key_id(key: &DidPublicKey) -> TestKeyId {
	crate_utils::calculate_key_id::<Test>(key)
}

pub(crate) fn get_attestation_key_test_input() -> Vec<u8> {
	[0u8; 32].to_vec()
}
pub(crate) fn get_attestation_key_call() -> Call {
	Call::Ctype(ctype::Call::add {
		ctype: get_attestation_key_test_input(),
	})
}
pub(crate) fn get_authentication_key_test_input() -> Vec<u8> {
	[1u8; 32].to_vec()
}
pub(crate) fn get_authentication_key_call() -> Call {
	Call::Ctype(ctype::Call::add {
		ctype: get_authentication_key_test_input(),
	})
}
pub(crate) fn get_delegation_key_test_input() -> Vec<u8> {
	[2u8; 32].to_vec()
}
pub(crate) fn get_delegation_key_call() -> Call {
	Call::Ctype(ctype::Call::add {
		ctype: get_delegation_key_test_input(),
	})
}
pub(crate) fn get_none_key_test_input() -> Vec<u8> {
	[3u8; 32].to_vec()
}
pub(crate) fn get_none_key_call() -> Call {
	Call::Ctype(ctype::Call::add {
		ctype: get_none_key_test_input(),
	})
}

impl DeriveDidCallAuthorizationVerificationKeyRelationship for Call {
	fn derive_verification_key_relationship(&self) -> DeriveDidCallKeyRelationshipResult {
		if *self == get_attestation_key_call() {
			Ok(DidVerificationKeyRelationship::AssertionMethod)
		} else if *self == get_authentication_key_call() {
			Ok(DidVerificationKeyRelationship::Authentication)
		} else if *self == get_delegation_key_call() {
			Ok(DidVerificationKeyRelationship::CapabilityDelegation)
		} else {
			#[cfg(feature = "runtime-benchmarks")]
			if *self == Self::get_call_for_did_call_benchmark() {
				// Always require an authentication key to dispatch calls during benchmarking
				return Ok(DidVerificationKeyRelationship::Authentication);
			}
			Err(RelationshipDeriveError::NotCallableByDid)
		}
	}

	// Always return a System::remark() extrinsic call
	#[cfg(feature = "runtime-benchmarks")]
	fn get_call_for_did_call_benchmark() -> Self {
		Call::System(frame_system::Call::remark { remark: vec![] })
	}
}

pub fn generate_test_did_call(
	verification_key_required: DidVerificationKeyRelationship,
	caller: TestDidIdentifier,
	submitter: runtime_common::AccountId,
) -> DidAuthorizedCallOperationWithVerificationRelationship<Test> {
	let call = match verification_key_required {
		DidVerificationKeyRelationship::AssertionMethod => get_attestation_key_call(),
		DidVerificationKeyRelationship::Authentication => get_authentication_key_call(),
		DidVerificationKeyRelationship::CapabilityDelegation => get_delegation_key_call(),
		_ => get_none_key_call(),
	};
	DidAuthorizedCallOperationWithVerificationRelationship {
		operation: DidAuthorizedCallOperation {
			did: caller,
			call,
			tx_counter: 1u64,
			block_number: 0u64,
			submitter,
		},
		verification_key_relationship: verification_key_required,
	}
}

#[allow(unused_must_use)]
pub fn initialize_logger() {
	env_logger::builder().is_test(true).try_init();
}

#[derive(Clone, Default)]
pub struct ExtBuilder {
	dids_stored: Vec<(TestDidIdentifier, DidDetails<Test>)>,
	service_endpoints: Vec<(TestDidIdentifier, Vec<DidEndpoint<Test>>)>,
	deleted_dids: Vec<TestDidIdentifier>,
	storage_version: DidStorageVersion,
	ctypes_stored: Vec<(TestCtypeHash, TestCtypeOwner)>,
	balances: Vec<(AccountIdOf<Test>, Balance)>,
}

impl ExtBuilder {
	pub fn with_dids(mut self, dids: Vec<(TestDidIdentifier, DidDetails<Test>)>) -> Self {
		self.dids_stored = dids;
		self
	}

	pub fn with_endpoints(mut self, endpoints: Vec<(TestDidIdentifier, Vec<DidEndpoint<Test>>)>) -> Self {
		self.service_endpoints = endpoints;
		self
	}

	pub(crate) fn with_balances(mut self, balances: Vec<(AccountIdOf<Test>, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub fn with_ctypes(mut self, ctypes: Vec<(TestCtypeHash, TestCtypeOwner)>) -> Self {
		self.ctypes_stored = ctypes;
		self
	}

	pub fn with_deleted_dids(mut self, dids: Vec<TestDidIdentifier>) -> Self {
		self.deleted_dids = dids;
		self
	}

	pub fn with_storage_version(mut self, storage_version: DidStorageVersion) -> Self {
		self.storage_version = storage_version;
		self
	}

	pub fn build(self, ext: Option<sp_io::TestExternalities>) -> sp_io::TestExternalities {
		let mut ext = if let Some(ext) = ext {
			ext
		} else {
			let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
			pallet_balances::GenesisConfig::<Test> {
				balances: self.balances.clone(),
			}
			.assimilate_storage(&mut storage)
			.expect("assimilate should not fail");
			sp_io::TestExternalities::new(storage)
		};

		ext.execute_with(|| {
			for (ctype_hash, owner) in self.ctypes_stored.iter() {
				ctype::Ctypes::<Test>::insert(ctype_hash, owner);
			}

			for did in self.dids_stored.iter() {
				did::Did::<Test>::insert(&did.0, did.1.clone());
				CurrencyOf::<Test>::reserve(&did.1.deposit.owner, did.1.deposit.amount)
					.expect("Deposit owner should have enough balance");
			}
			for did in self.deleted_dids.iter() {
				DidBlacklist::<Test>::insert(&did, ());
			}
			for (did, endpoints) in self.service_endpoints.iter() {
				for endpoint in endpoints.iter() {
					ServiceEndpoints::<Test>::insert(&did, &endpoint.id, endpoint)
				}
				DidEndpointsCount::<Test>::insert(&did, endpoints.len().saturated_into::<u32>());
			}
			StorageVersion::<Test>::set(self.storage_version);
		});

		ext
	}

	// allowance only required for clippy, this function is actually used
	#[allow(dead_code)]
	pub fn build_with_keystore(self) -> sp_io::TestExternalities {
		let mut ext = self.build(None);

		let keystore = KeyStore::new();
		ext.register_extension(KeystoreExt(Arc::new(keystore)));

		ext
	}
}
