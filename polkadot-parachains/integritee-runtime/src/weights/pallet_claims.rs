
//! Autogenerated weights for `pallet_claims`
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
// --pallet=pallet_claims
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_claims.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_claims`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_claims::WeightInfo for WeightInfo<T> {
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn claim() -> Weight {
		// Minimum execution time: 198_000 nanoseconds.
		Weight::from_ref_time(215_500_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:0 w:1)
	// Storage: Claims Claims (r:0 w:1)
	// Storage: Claims Signing (r:0 w:1)
	fn mint_claim() -> Weight {
		// Minimum execution time: 20_200 nanoseconds.
		Weight::from_ref_time(22_100_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn claim_attest() -> Weight {
		// Minimum execution time: 204_300 nanoseconds.
		Weight::from_ref_time(226_000_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Claims Preclaims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn attest() -> Weight {
		// Minimum execution time: 122_600 nanoseconds.
		Weight::from_ref_time(144_700_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Claims Claims (r:1 w:2)
	// Storage: Claims Vesting (r:1 w:2)
	// Storage: Claims Signing (r:1 w:2)
	// Storage: Claims Preclaims (r:1 w:1)
	fn move_claim() -> Weight {
		// Minimum execution time: 35_200 nanoseconds.
		Weight::from_ref_time(38_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
