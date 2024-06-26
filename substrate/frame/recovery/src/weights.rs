// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_recovery`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-anb7yjbi-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_recovery
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/recovery/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_recovery`.
pub trait WeightInfo {
	fn as_recovered() -> Weight;
	fn set_recovered() -> Weight;
	fn create_recovery(n: u32, ) -> Weight;
	fn initiate_recovery() -> Weight;
	fn vouch_recovery(n: u32, ) -> Weight;
	fn claim_recovery(n: u32, ) -> Weight;
	fn close_recovery(n: u32, ) -> Weight;
	fn remove_recovery(n: u32, ) -> Weight;
	fn cancel_recovered() -> Weight;
}

/// Weights for `pallet_recovery` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Recovery::Proxy` (r:1 w:0)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497`
		//  Estimated: `3997`
		// Minimum execution time: 15_318_000 picoseconds.
		Weight::from_parts(15_767_000, 3997)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	/// Storage: `Recovery::Proxy` (r:0 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_153_000 picoseconds.
		Weight::from_parts(7_578_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3816`
		// Minimum execution time: 23_303_000 picoseconds.
		Weight::from_parts(24_725_158, 3816)
			// Standard Error: 5_723
			.saturating_add(Weight::from_parts(13_638, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `3854`
		// Minimum execution time: 26_914_000 picoseconds.
		Weight::from_parts(28_041_000, 3854)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 17_695_000 picoseconds.
		Weight::from_parts(18_591_642, 3854)
			// Standard Error: 5_582
			.saturating_add(Weight::from_parts(188_668, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 22_580_000 picoseconds.
		Weight::from_parts(23_526_020, 3854)
			// Standard Error: 6_604
			.saturating_add(Weight::from_parts(134_340, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `584 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 32_017_000 picoseconds.
		Weight::from_parts(33_401_086, 3854)
			// Standard Error: 6_498
			.saturating_add(Weight::from_parts(95_507, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 28_641_000 picoseconds.
		Weight::from_parts(30_230_511, 3854)
			// Standard Error: 7_058
			.saturating_add(Weight::from_parts(61_004, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `352`
		//  Estimated: `3545`
		// Minimum execution time: 11_767_000 picoseconds.
		Weight::from_parts(12_275_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Recovery::Proxy` (r:1 w:0)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497`
		//  Estimated: `3997`
		// Minimum execution time: 15_318_000 picoseconds.
		Weight::from_parts(15_767_000, 3997)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
	}
	/// Storage: `Recovery::Proxy` (r:0 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_153_000 picoseconds.
		Weight::from_parts(7_578_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3816`
		// Minimum execution time: 23_303_000 picoseconds.
		Weight::from_parts(24_725_158, 3816)
			// Standard Error: 5_723
			.saturating_add(Weight::from_parts(13_638, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `3854`
		// Minimum execution time: 26_914_000 picoseconds.
		Weight::from_parts(28_041_000, 3854)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 17_695_000 picoseconds.
		Weight::from_parts(18_591_642, 3854)
			// Standard Error: 5_582
			.saturating_add(Weight::from_parts(188_668, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 22_580_000 picoseconds.
		Weight::from_parts(23_526_020, 3854)
			// Standard Error: 6_604
			.saturating_add(Weight::from_parts(134_340, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `584 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 32_017_000 picoseconds.
		Weight::from_parts(33_401_086, 3854)
			// Standard Error: 6_498
			.saturating_add(Weight::from_parts(95_507, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 28_641_000 picoseconds.
		Weight::from_parts(30_230_511, 3854)
			// Standard Error: 7_058
			.saturating_add(Weight::from_parts(61_004, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `352`
		//  Estimated: `3545`
		// Minimum execution time: 11_767_000 picoseconds.
		Weight::from_parts(12_275_000, 3545)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
