
//! Autogenerated weights for `pallet_utility`
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
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	// Storage: Migration Status (r:1 w:0)
	fn batch(c: u32, ) -> Weight {
		Weight::from_ref_time(245_817_000)
			// Standard Error: 132_000
			.saturating_add(Weight::from_ref_time(10_683_000).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Migration Status (r:1 w:0)
	fn as_derivative() -> Weight {
		Weight::from_ref_time(17_472_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Migration Status (r:1 w:0)
	fn batch_all(c: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 150_000
			.saturating_add(Weight::from_ref_time(11_944_000).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn dispatch_as() -> Weight {
		Weight::from_ref_time(59_832_000)
	}
	
	fn force_batch(c: u32, ) -> Weight {
		Weight::from_ref_time(13_470_000)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(4_229_000).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
