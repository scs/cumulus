
//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_preimage.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3556`
		// Minimum execution time: 64_018_000 picoseconds.
		Weight::from_parts(64_742_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			// Standard Error: 13
			.saturating_add(Weight::from_parts(2_079, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3556`
		// Minimum execution time: 13_391_000 picoseconds.
		Weight::from_parts(13_844_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(2_176, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3556`
		// Minimum execution time: 13_312_000 picoseconds.
		Weight::from_parts(386_550_663, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			// Standard Error: 16
			.saturating_add(Weight::from_parts(1_803, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unnote_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3556`
		// Minimum execution time: 48_310_000 picoseconds.
		Weight::from_parts(58_772_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unnote_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3556`
		// Minimum execution time: 16_952_000 picoseconds.
		Weight::from_parts(18_277_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn request_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `222`
		//  Estimated: `3556`
		// Minimum execution time: 15_797_000 picoseconds.
		Weight::from_parts(22_185_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn request_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3556`
		// Minimum execution time: 10_349_000 picoseconds.
		Weight::from_parts(11_629_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn request_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3556`
		// Minimum execution time: 9_159_000 picoseconds.
		Weight::from_parts(9_688_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn request_requested_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3556`
		// Minimum execution time: 7_601_000 picoseconds.
		Weight::from_parts(7_933_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unrequest_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3556`
		// Minimum execution time: 15_466_000 picoseconds.
		Weight::from_parts(18_832_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn unrequest_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3556`
		// Minimum execution time: 7_599_000 picoseconds.
		Weight::from_parts(7_924_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3556`
		// Minimum execution time: 7_146_000 picoseconds.
		Weight::from_parts(7_737_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1023 w:1023)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1023 w:1023)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1023 w:1023)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:0 w:1023)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 1024]`.
	fn ensure_updated(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + n * (227 ±0)`
		//  Estimated: `990 + n * (2603 ±0)`
		// Minimum execution time: 42_713_000 picoseconds.
		Weight::from_parts(43_344_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 78_572
			.saturating_add(Weight::from_parts(47_242_344, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
}
