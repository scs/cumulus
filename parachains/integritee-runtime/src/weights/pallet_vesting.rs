
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/integritee-collator
// benchmark
// --chain=integritee-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_vesting.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		(106_197_000 as Weight)
			// Standard Error: 46_000
			.saturating_add((544_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 95_000
			.saturating_add((705_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		(101_309_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((821_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 105_000
			.saturating_add((1_124_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		(111_156_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((318_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 105_000
			.saturating_add((1_156_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(l: u32, _s: u32, ) -> Weight {
		(147_469_000 as Weight)
			// Standard Error: 52_000
			.saturating_add((207_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vested_transfer(l: u32, _s: u32, ) -> Weight {
		(181_754_000 as Weight)
			// Standard Error: 89_000
			.saturating_add((888_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		(164_733_000 as Weight)
			// Standard Error: 78_000
			.saturating_add((367_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 160_000
			.saturating_add((536_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(103_096_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((256_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 109_000
			.saturating_add((1_090_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(125_159_000 as Weight)
			// Standard Error: 54_000
			.saturating_add((398_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 116_000
			.saturating_add((90_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}