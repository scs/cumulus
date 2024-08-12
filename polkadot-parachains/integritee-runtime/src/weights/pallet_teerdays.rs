
//! Autogenerated weights for `pallet_teerdays`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 39.0.0
//! DATE: 2024-08-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("integritee-rococo-local-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/integritee-collator
// benchmark
// pallet
// --chain=integritee-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_teerdays
// --extrinsic=*
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_teerdays.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_teerdays`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teerdays::WeightInfo for WeightInfo<T> {
	/// Storage: `TeerDays::TeerDayBonds` (r:1 w:1)
	/// Proof: `TeerDays::TeerDayBonds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `344`
		//  Estimated: `4764`
		// Minimum execution time: 25_534_000 picoseconds.
		Weight::from_parts(26_907_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `TeerDays::PendingUnlock` (r:1 w:1)
	/// Proof: `TeerDays::PendingUnlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TeerDays::TeerDayBonds` (r:1 w:1)
	/// Proof: `TeerDays::TeerDayBonds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn unbond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3822`
		// Minimum execution time: 18_153_000 picoseconds.
		Weight::from_parts(18_960_000, 0)
			.saturating_add(Weight::from_parts(0, 3822))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `TeerDays::TeerDayBonds` (r:1 w:1)
	/// Proof: `TeerDays::TeerDayBonds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn update_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3822`
		// Minimum execution time: 12_938_000 picoseconds.
		Weight::from_parts(14_024_000, 0)
			.saturating_add(Weight::from_parts(0, 3822))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `TeerDays::PendingUnlock` (r:1 w:1)
	/// Proof: `TeerDays::PendingUnlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn withdraw_unbonded() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `504`
		//  Estimated: `4764`
		// Minimum execution time: 27_893_000 picoseconds.
		Weight::from_parts(29_110_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
