// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

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

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-17, STEPS: `[1, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --chain
// dev
// --heap-pages
// 4096
// --extrinsic
// *
// --pallet
// delegation
// --steps
// 1
// --repeat
// 20
// --execution
// wasm
// --wasm-execution
// Compiled
// --output
// runtimes/parachain/src/weights/delegation.rs
// --template
// .maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for delegation using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> delegation::WeightInfo for WeightInfo<T> {
	fn create_root() -> Weight {
		45_635_000_u64
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn revoke_root(r: u32, ) -> Weight {
		48_761_000_u64
			// Standard Error: 311_000
			.saturating_add(31_784_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(2_u64.saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(r as Weight)))
	}
	fn add_delegation() -> Weight {
		142_316_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn revoke_delegation_root_child(r: u32, _c: u32, ) -> Weight {
		21_746_000_u64
			// Standard Error: 60_000
			.saturating_add(32_601_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64.saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(r as Weight)))
	}
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight {
		52_521_000_u64
			// Standard Error: 45_000
			.saturating_add(93_000_u64.saturating_mul(r as Weight))
			// Standard Error: 45_000
			.saturating_add(8_110_000_u64.saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}