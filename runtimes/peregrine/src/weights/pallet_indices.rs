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

//! //! Autogenerated weights for pallet_indices
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-10, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE:
//! {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN:
//! Some("spiritnet-dev"), DB CACHE: 128

// Executed Command:
// /home/willi/mashnet-node/target/release/kilt-parachain
// benchmark
// --chain=spiritnet-dev
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --extrinsic=*
// --pallet=pallet_indices
// --steps=50
// --repeat=20
// --output
// ../../runtimes/spiritnet/src/weights/pallet_indices.rs
// --template
// ../../.maintain/weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_indices using the Substrate node and recommended
/// hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_indices::WeightInfo for WeightInfo<T> {
	fn claim() -> Weight {
		(66_605_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn transfer() -> Weight {
		(77_706_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn free() -> Weight {
		(63_439_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn force_transfer() -> Weight {
		(63_439_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn freeze() -> Weight {
		(60_743_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
