// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_indices`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ehxwxxsd-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_indices
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_indices`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_indices::WeightInfo for WeightInfo<T> {
	/// Storage: Indices Accounts (r:1 w:1)
	/// Proof: Indices Accounts (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `2544`
		// Minimum execution time: 17_924 nanoseconds.
		Weight::from_ref_time(18_402_000)
			.saturating_add(Weight::from_proof_size(2544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Indices Accounts (r:1 w:1)
	/// Proof: Indices Accounts (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `405`
		//  Estimated: `5147`
		// Minimum execution time: 23_187 nanoseconds.
		Weight::from_ref_time(23_892_000)
			.saturating_add(Weight::from_proof_size(5147))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Indices Accounts (r:1 w:1)
	/// Proof: Indices Accounts (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn free() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270`
		//  Estimated: `2544`
		// Minimum execution time: 18_897 nanoseconds.
		Weight::from_ref_time(19_380_000)
			.saturating_add(Weight::from_proof_size(2544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Indices Accounts (r:1 w:1)
	/// Proof: Indices Accounts (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `405`
		//  Estimated: `5147`
		// Minimum execution time: 21_855 nanoseconds.
		Weight::from_ref_time(23_037_000)
			.saturating_add(Weight::from_proof_size(5147))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Indices Accounts (r:1 w:1)
	/// Proof: Indices Accounts (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270`
		//  Estimated: `2544`
		// Minimum execution time: 20_277 nanoseconds.
		Weight::from_ref_time(21_421_000)
			.saturating_add(Weight::from_proof_size(2544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
