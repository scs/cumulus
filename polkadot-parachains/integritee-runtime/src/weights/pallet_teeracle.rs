
//! Autogenerated weights for `pallet_teeracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `DESKTOP-0F6V7QQ`, CPU: `Intel(R) Core(TM) i7-10875H CPU @ 2.30GHz`
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

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_teeracle`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teeracle::WeightInfo for WeightInfo<T> {
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Storage: Teerex EnclaveRegistry (r:1 w:0)
	// Storage: Teeracle Whitelists (r:1 w:0)
	// Storage: Teeracle ExchangeRates (r:1 w:1)
	fn update_exchange_rate() -> Weight {
		// Minimum execution time: 51_200 nanoseconds.
		Weight::from_ref_time(52_200_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Storage: Teerex EnclaveRegistry (r:1 w:0)
	// Storage: Teeracle Whitelists (r:1 w:0)
	// Storage: Teeracle OracleData (r:0 w:1)
	fn update_oracle() -> Weight {
		// Minimum execution time: 43_800 nanoseconds.
		Weight::from_ref_time(44_700_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Teeracle Whitelists (r:1 w:1)
	fn add_to_whitelist() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(26_500_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Teeracle Whitelists (r:1 w:1)
	fn remove_from_whitelist() -> Weight {
		// Minimum execution time: 28_200 nanoseconds.
		Weight::from_ref_time(28_800_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
