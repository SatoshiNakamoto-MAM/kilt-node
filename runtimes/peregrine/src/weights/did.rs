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

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/did.rs
// --template=.maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for did using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> did::WeightInfo for WeightInfo<T> {
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight {
		(155_554_000_u64)
			// Standard Error: 40_000
			.saturating_add((2_340_000_u64).saturating_mul(n as Weight))
			// Standard Error: 12_000
			.saturating_add((10_183_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		(157_851_000_u64)
			// Standard Error: 27_000
			.saturating_add((2_477_000_u64).saturating_mul(n as Weight))
			// Standard Error: 8_000
			.saturating_add((10_523_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_ecdsa_keys(n: u32, c: u32, ) -> Weight {
		(275_184_000_u64)
			// Standard Error: 62_000
			.saturating_add((2_307_000_u64).saturating_mul(n as Weight))
			// Standard Error: 19_000
			.saturating_add((9_953_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn delete(c: u32, ) -> Weight {
		(40_970_000_u64)
			// Standard Error: 4_000
			.saturating_add((1_039_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn reclaim_deposit(c: u32, ) -> Weight {
		(45_659_000_u64)
			// Standard Error: 5_000
			.saturating_add((1_037_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(85_657_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(88_121_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(203_208_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		(47_523_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		(47_139_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		(47_445_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		(47_109_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		(47_153_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		(47_180_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		(43_349_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		(43_782_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		(43_469_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		(47_008_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		(46_791_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		(46_654_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		(43_206_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		(43_339_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		(43_217_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		(45_677_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		(45_880_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		(45_968_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		(43_709_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		(43_659_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		(43_747_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_service_endpoint() -> Weight {
		(43_232_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn remove_service_endpoint() -> Weight {
		(34_532_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		(25_823_000_u64)
			// Standard Error: 0
			.saturating_add((4_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		(23_103_000_u64)
			// Standard Error: 0
			.saturating_add((2_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		(141_265_000_u64)
			// Standard Error: 0
			.saturating_add((1_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}
