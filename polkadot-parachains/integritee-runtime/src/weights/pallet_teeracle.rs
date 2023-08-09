
//! Autogenerated weights for `pallet_teeracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/integritee-collator
// benchmark
// pallet
// --chain=integritee-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_teeracle
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_teeracle.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_teeracle`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teeracle::WeightInfo for WeightInfo<T> {
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teeracle Whitelists (r:1 w:0)
	/// Proof Skipped: Teeracle Whitelists (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teeracle ExchangeRates (r:1 w:1)
	/// Proof Skipped: Teeracle ExchangeRates (max_values: None, max_size: None, mode: Measured)
	fn update_exchange_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `491`
		//  Estimated: `3956`
		// Minimum execution time: 36_732_000 picoseconds.
		Weight::from_parts(40_409_000, 0)
			.saturating_add(Weight::from_parts(0, 3956))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teeracle Whitelists (r:1 w:0)
	/// Proof Skipped: Teeracle Whitelists (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teeracle OracleData (r:0 w:1)
	/// Proof Skipped: Teeracle OracleData (max_values: None, max_size: None, mode: Measured)
	fn update_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482`
		//  Estimated: `3947`
		// Minimum execution time: 28_204_000 picoseconds.
		Weight::from_parts(30_318_000, 0)
			.saturating_add(Weight::from_parts(0, 3947))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teeracle Whitelists (r:1 w:1)
	/// Proof Skipped: Teeracle Whitelists (max_values: None, max_size: None, mode: Measured)
	fn add_to_whitelist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 16_591_000 picoseconds.
		Weight::from_parts(16_885_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teeracle Whitelists (r:1 w:1)
	/// Proof Skipped: Teeracle Whitelists (max_values: None, max_size: None, mode: Measured)
	fn remove_from_whitelist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3609`
		// Minimum execution time: 19_536_000 picoseconds.
		Weight::from_parts(20_247_000, 0)
			.saturating_add(Weight::from_parts(0, 3609))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
