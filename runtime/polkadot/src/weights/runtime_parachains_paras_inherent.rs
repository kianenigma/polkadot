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
//! Autogenerated weights for `runtime_parachains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=runtime_parachains::paras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/runtime_parachains_paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for WeightInfo<T> {
	/// Storage: ParaInherent Included (r:1 w:1)
	/// Proof Skipped: ParaInherent Included (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	/// Proof: Babe AuthorVrfRandomness (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	/// Storage: ParaSessionInfo Sessions (r:1 w:0)
	/// Proof Skipped: ParaSessionInfo Sessions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasDisputes Disputes (r:1 w:1)
	/// Proof Skipped: ParasDisputes Disputes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasDisputes BackersOnDisputes (r:1 w:1)
	/// Proof Skipped: ParasDisputes BackersOnDisputes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	/// Proof Skipped: ParaScheduler AvailabilityCores (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Included (r:1 w:1)
	/// Proof Skipped: ParasDisputes Included (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasDisputes Frozen (r:1 w:0)
	/// Proof Skipped: ParasDisputes Frozen (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailability (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras Parachains (r:1 w:0)
	/// Proof Skipped: Paras Parachains (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailabilityCommitments (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	/// Proof Skipped: ParaSessionInfo AccountKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasRewardPoints (r:1 w:1)
	/// Proof Skipped: Staking ErasRewardPoints (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpChannelDigests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInherent OnChainVotes (r:1 w:1)
	/// Proof Skipped: ParaInherent OnChainVotes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	/// Proof Skipped: ParaScheduler SessionStartBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	/// Proof Skipped: ParaScheduler ParathreadQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler Scheduled (r:1 w:1)
	/// Proof Skipped: ParaScheduler Scheduled (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	/// Proof Skipped: ParaScheduler ValidatorGroups (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NeedsDispatch (r:1 w:1)
	/// Proof Skipped: Ump NeedsDispatch (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	/// Proof Skipped: Ump NextDispatchRoundStartWith (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpWatermarks (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeGoAheadSignal (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `51185`
		//  Estimated: `1520629 + v * (16 ±0)`
		// Minimum execution time: 982_056 nanoseconds.
		Weight::from_ref_time(448_379_359)
			.saturating_add(Weight::from_proof_size(1520629))
			// Standard Error: 15_625
			.saturating_add(Weight::from_ref_time(56_687_934).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(29))
			.saturating_add(T::DbWeight::get().writes(16))
			.saturating_add(Weight::from_proof_size(16).saturating_mul(v.into()))
	}
	/// Storage: ParaInherent Included (r:1 w:1)
	/// Proof Skipped: ParaInherent Included (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	/// Proof: Babe AuthorVrfRandomness (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	/// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	/// Proof Skipped: ParaScheduler AvailabilityCores (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Frozen (r:1 w:0)
	/// Proof Skipped: ParasDisputes Frozen (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras Parachains (r:1 w:0)
	/// Proof Skipped: Paras Parachains (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailability (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailabilityCommitments (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	/// Proof Skipped: ParaSessionInfo AccountKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasRewardPoints (r:1 w:1)
	/// Proof Skipped: Staking ErasRewardPoints (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpChannelDigests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInherent OnChainVotes (r:1 w:1)
	/// Proof Skipped: ParaInherent OnChainVotes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Disputes (r:1 w:0)
	/// Proof Skipped: ParasDisputes Disputes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	/// Proof Skipped: ParaScheduler SessionStartBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	/// Proof Skipped: ParaScheduler ParathreadQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler Scheduled (r:1 w:1)
	/// Proof Skipped: ParaScheduler Scheduled (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	/// Proof Skipped: ParaScheduler ValidatorGroups (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NeedsDispatch (r:1 w:1)
	/// Proof Skipped: Ump NeedsDispatch (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	/// Proof Skipped: Ump NextDispatchRoundStartWith (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaInclusion AvailabilityBitfields (r:0 w:1)
	/// Proof Skipped: ParaInclusion AvailabilityBitfields (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasDisputes Included (r:0 w:1)
	/// Proof Skipped: ParasDisputes Included (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpWatermarks (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeGoAheadSignal (max_values: None, max_size: None, mode: Measured)
	fn enter_bitfields() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42987`
		//  Estimated: `1234899`
		// Minimum execution time: 452_390 nanoseconds.
		Weight::from_ref_time(475_746_000)
			.saturating_add(Weight::from_proof_size(1234899))
			.saturating_add(T::DbWeight::get().reads(27))
			.saturating_add(T::DbWeight::get().writes(17))
	}
	/// Storage: ParaInherent Included (r:1 w:1)
	/// Proof Skipped: ParaInherent Included (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	/// Proof: Babe AuthorVrfRandomness (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	/// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	/// Proof Skipped: ParaScheduler AvailabilityCores (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Frozen (r:1 w:0)
	/// Proof Skipped: ParasDisputes Frozen (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras Parachains (r:1 w:0)
	/// Proof Skipped: Paras Parachains (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailability (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailabilityCommitments (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	/// Proof Skipped: ParaSessionInfo AccountKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasRewardPoints (r:1 w:1)
	/// Proof Skipped: Staking ErasRewardPoints (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpChannelDigests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInherent OnChainVotes (r:1 w:1)
	/// Proof Skipped: ParaInherent OnChainVotes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Disputes (r:2 w:0)
	/// Proof Skipped: ParasDisputes Disputes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	/// Proof Skipped: ParaScheduler SessionStartBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	/// Proof Skipped: ParaScheduler ParathreadQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler Scheduled (r:1 w:1)
	/// Proof Skipped: ParaScheduler Scheduled (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	/// Proof Skipped: ParaScheduler ValidatorGroups (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:1 w:0)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	/// Proof Skipped: Ump RelayDispatchQueueSize (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump NeedsDispatch (r:1 w:1)
	/// Proof Skipped: Ump NeedsDispatch (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	/// Proof Skipped: Ump NextDispatchRoundStartWith (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Included (r:0 w:1)
	/// Proof Skipped: ParasDisputes Included (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpWatermarks (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeGoAheadSignal (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43019`
		//  Estimated: `1286239`
		// Minimum execution time: 6_758_415 nanoseconds.
		Weight::from_ref_time(1_113_645_249)
			.saturating_add(Weight::from_proof_size(1286239))
			// Standard Error: 20_724
			.saturating_add(Weight::from_ref_time(56_256_191).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(30))
			.saturating_add(T::DbWeight::get().writes(16))
	}
	/// Storage: ParaInherent Included (r:1 w:1)
	/// Proof Skipped: ParaInherent Included (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	/// Proof: Babe AuthorVrfRandomness (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	/// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	/// Proof Skipped: ParaScheduler AvailabilityCores (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Frozen (r:1 w:0)
	/// Proof Skipped: ParasDisputes Frozen (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras Parachains (r:1 w:0)
	/// Proof Skipped: Paras Parachains (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailability (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	/// Proof Skipped: ParaInclusion PendingAvailabilityCommitments (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	/// Proof Skipped: ParaSessionInfo AccountKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasRewardPoints (r:1 w:1)
	/// Proof Skipped: Staking ErasRewardPoints (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpChannelDigests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaInherent OnChainVotes (r:1 w:1)
	/// Proof Skipped: ParaInherent OnChainVotes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Disputes (r:2 w:0)
	/// Proof Skipped: ParasDisputes Disputes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	/// Proof Skipped: ParaScheduler SessionStartBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	/// Proof Skipped: ParaScheduler ParathreadQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler Scheduled (r:1 w:1)
	/// Proof Skipped: ParaScheduler Scheduled (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	/// Proof Skipped: ParaScheduler ValidatorGroups (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:1 w:0)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeHash (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeRestrictionSignal (r:1 w:0)
	/// Proof Skipped: Paras UpgradeRestrictionSignal (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	/// Proof Skipped: Ump RelayDispatchQueueSize (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump NeedsDispatch (r:1 w:1)
	/// Proof Skipped: Ump NeedsDispatch (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	/// Proof Skipped: Ump NextDispatchRoundStartWith (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasDisputes Included (r:0 w:1)
	/// Proof Skipped: ParasDisputes Included (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpWatermarks (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeGoAheadSignal (max_values: None, max_size: None, mode: Measured)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43046`
		//  Estimated: `1378064`
		// Minimum execution time: 46_447_376 nanoseconds.
		Weight::from_ref_time(46_875_346_000)
			.saturating_add(Weight::from_proof_size(1378064))
			.saturating_add(T::DbWeight::get().reads(32))
			.saturating_add(T::DbWeight::get().writes(16))
	}
}
