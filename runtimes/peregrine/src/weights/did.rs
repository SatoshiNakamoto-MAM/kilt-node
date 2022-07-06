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
//! DATE: 2022-06-23, STEPS: `1`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=1
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
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `did`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> did::WeightInfo for WeightInfo<T> {

	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	fn delete(c: u32, ) -> Weight {
		(33_940_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((1_128_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Did Did (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did DidBlacklist (r:0 w:1)
	// Storage: Did ServiceEndpoints (r:0 w:1)
	fn reclaim_deposit(c: u32, ) -> Weight {
		(37_019_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((1_119_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ed25519_key() -> Weight {
		(79_603_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_sr25519_key() -> Weight {
		(82_379_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Did Did (r:1 w:1)
	fn submit_did_call_ecdsa_key() -> Weight {
		(71_713_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	// Storage: Did Did (r:1 w:0)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	// Storage: Did ServiceEndpoints (r:1 w:1)
	fn add_service_endpoint() -> Weight {
		(37_336_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Did ServiceEndpoints (r:1 w:1)
	// Storage: Did DidEndpointsCount (r:1 w:1)
	fn remove_service_endpoint() -> Weight {
		(31_417_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		(66_480_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		(63_463_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Did Did (r:1 w:0)
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		(56_006_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}

fn create_ed25519_key() -> Weight {
        todo!()
    }

fn create_sr25519_key() -> Weight {
        todo!()
    }

fn create_ecdsa_key() -> Weight {
        todo!()
    }

fn set_authentication_key() -> Weight {
        todo!()
    }

fn set_delegation_key() -> Weight {
        todo!()
    }

fn remove_delegation_key() -> Weight {
        todo!()
    }

fn set_attestation_key() -> Weight {
        todo!()
    }

fn remove_attestation_key() -> Weight {
        todo!()
    }

fn add_key_agreement_key() -> Weight {
        todo!()
    }

fn remove_key_agreement_key() -> Weight {
        todo!()
    }
}
