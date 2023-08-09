
//! Autogenerated weights for `pallet_multisig`
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
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_428_000 picoseconds.
		Weight::from_parts(9_434_591, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(465), added: 2940, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159 + s * (11 ±0)`
		//  Estimated: `3930`
		// Minimum execution time: 32_030_000 picoseconds.
		Weight::from_parts(30_902_909, 0)
			.saturating_add(Weight::from_parts(0, 3930))
			// Standard Error: 8_894
			.saturating_add(Weight::from_parts(249_624, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_162, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(465), added: 2940, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 10]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `3930`
		// Minimum execution time: 17_652_000 picoseconds.
		Weight::from_parts(17_078_329, 0)
			.saturating_add(Weight::from_parts(0, 3930))
			// Standard Error: 5_738
			.saturating_add(Weight::from_parts(182_578, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_062, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(465), added: 2940, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `301 + s * (43 ±0)`
		//  Estimated: `3930`
		// Minimum execution time: 35_671_000 picoseconds.
		Weight::from_parts(33_222_478, 0)
			.saturating_add(Weight::from_parts(0, 3930))
			// Standard Error: 9_196
			.saturating_add(Weight::from_parts(346_157, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_181, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(465), added: 2940, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159 + s * (11 ±0)`
		//  Estimated: `3930`
		// Minimum execution time: 29_571_000 picoseconds.
		Weight::from_parts(30_323_391, 0)
			.saturating_add(Weight::from_parts(0, 3930))
			// Standard Error: 9_570
			.saturating_add(Weight::from_parts(291_970, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(465), added: 2940, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `3930`
		// Minimum execution time: 15_890_000 picoseconds.
		Weight::from_parts(16_436_811, 0)
			.saturating_add(Weight::from_parts(0, 3930))
			// Standard Error: 3_966
			.saturating_add(Weight::from_parts(170_864, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(465), added: 2940, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365 + s * (11 ±0)`
		//  Estimated: `3930`
		// Minimum execution time: 30_766_000 picoseconds.
		Weight::from_parts(31_005_857, 0)
			.saturating_add(Weight::from_parts(0, 3930))
			// Standard Error: 6_723
			.saturating_add(Weight::from_parts(272_297, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
