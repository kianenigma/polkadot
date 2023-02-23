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
//! Autogenerated weights for `pallet_offences`
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
// --pallet=pallet_offences
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

/// Weight functions for `pallet_offences`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_offences::WeightInfo for WeightInfo<T> {
	/// Storage: Offences ReportsByKindIndex (r:1 w:1)
	/// Proof Skipped: Offences ReportsByKindIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	/// Proof Skipped: Offences ConcurrentReportsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences Reports (r:100 w:100)
	/// Proof Skipped: Offences Reports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SlashRewardFraction (r:1 w:0)
	/// Proof: Staking SlashRewardFraction (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	/// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Staking Invulnerables (r:1 w:0)
	/// Proof Skipped: Staking Invulnerables (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ValidatorSlashInEra (r:100 w:100)
	/// Proof: Staking ValidatorSlashInEra (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:1700 w:1700)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SpanSlash (r:1700 w:1700)
	/// Proof: Staking SpanSlash (max_values: None, max_size: Some(76), added: 2551, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:100 w:100)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking CounterForValidators (r:1 w:1)
	/// Proof: Staking CounterForValidators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:299 w:299)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:100 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking OffendingValidators (r:1 w:1)
	/// Proof Skipped: Staking OffendingValidators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking NominatorSlashInEra (r:1600 w:1600)
	/// Proof: Staking NominatorSlashInEra (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// Proof Skipped: Staking UnappliedSlashes (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[1, 100]`.
	/// The range of component `o` is `[2, 100]`.
	/// The range of component `n` is `[0, 16]`.
	fn report_offence_im_online(_r: u32, o: u32, n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + o * (1138 ±0) + n * (3454 ±0)`
		//  Estimated: `316158 + o * (101513 ±6) + n * (481567 ±41)`
		// Minimum execution time: 462_530 nanoseconds.
		Weight::from_ref_time(463_729_000)
			.saturating_add(Weight::from_proof_size(316158))
			// Standard Error: 3_207_756
			.saturating_add(Weight::from_ref_time(322_787_535).saturating_mul(o.into()))
			// Standard Error: 19_491_766
			.saturating_add(Weight::from_ref_time(254_899_504).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(125))
			.saturating_add(T::DbWeight::get().reads((37_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().reads((187_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(118))
			.saturating_add(T::DbWeight::get().writes((36_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().writes((187_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(101513).saturating_mul(o.into()))
			.saturating_add(Weight::from_proof_size(481567).saturating_mul(n.into()))
	}
	/// Storage: Offences ReportsByKindIndex (r:1 w:1)
	/// Proof Skipped: Offences ReportsByKindIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	/// Proof Skipped: Offences ConcurrentReportsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences Reports (r:1 w:1)
	/// Proof Skipped: Offences Reports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SlashRewardFraction (r:1 w:0)
	/// Proof: Staking SlashRewardFraction (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	/// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Staking Invulnerables (r:1 w:0)
	/// Proof Skipped: Staking Invulnerables (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ValidatorSlashInEra (r:1 w:1)
	/// Proof: Staking ValidatorSlashInEra (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:17 w:17)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SpanSlash (r:17 w:17)
	/// Proof: Staking SpanSlash (max_values: None, max_size: Some(76), added: 2551, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:1)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking CounterForValidators (r:1 w:1)
	/// Proof: Staking CounterForValidators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:2 w:2)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking OffendingValidators (r:1 w:1)
	/// Proof Skipped: Staking OffendingValidators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking NominatorSlashInEra (r:16 w:16)
	/// Proof: Staking NominatorSlashInEra (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// Proof Skipped: Staking UnappliedSlashes (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 16]`.
	fn report_offence_grandpa(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1869 + n * (66 ±0)`
		//  Estimated: `50870 + n * (8150 ±0)`
		// Minimum execution time: 88_814 nanoseconds.
		Weight::from_ref_time(101_198_049)
			.saturating_add(Weight::from_proof_size(50870))
			// Standard Error: 29_057
			.saturating_add(Weight::from_ref_time(9_346_872).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(14))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(8150).saturating_mul(n.into()))
	}
	/// Storage: Offences ReportsByKindIndex (r:1 w:1)
	/// Proof Skipped: Offences ReportsByKindIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	/// Proof Skipped: Offences ConcurrentReportsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences Reports (r:1 w:1)
	/// Proof Skipped: Offences Reports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SlashRewardFraction (r:1 w:0)
	/// Proof: Staking SlashRewardFraction (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	/// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Staking Invulnerables (r:1 w:0)
	/// Proof Skipped: Staking Invulnerables (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ValidatorSlashInEra (r:1 w:1)
	/// Proof: Staking ValidatorSlashInEra (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:17 w:17)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SpanSlash (r:17 w:17)
	/// Proof: Staking SpanSlash (max_values: None, max_size: Some(76), added: 2551, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:1)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking CounterForValidators (r:1 w:1)
	/// Proof: Staking CounterForValidators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:2 w:2)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking OffendingValidators (r:1 w:1)
	/// Proof Skipped: Staking OffendingValidators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking NominatorSlashInEra (r:16 w:16)
	/// Proof: Staking NominatorSlashInEra (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// Proof Skipped: Staking UnappliedSlashes (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 16]`.
	fn report_offence_babe(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1869 + n * (66 ±0)`
		//  Estimated: `50870 + n * (8150 ±0)`
		// Minimum execution time: 88_328 nanoseconds.
		Weight::from_ref_time(100_343_495)
			.saturating_add(Weight::from_proof_size(50870))
			// Standard Error: 29_114
			.saturating_add(Weight::from_ref_time(9_466_628).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(14))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(8150).saturating_mul(n.into()))
	}
}
