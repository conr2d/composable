
//! Autogenerated weights for `vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn claim(s: u32, ) -> Weight {
		(63_640_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	fn vested_transfer() -> Weight {
		(107_156_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Vesting VestingSchedules (r:0 w:1)
	fn update_vesting_schedules(_s: u32, ) -> Weight {
		(59_960_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn claim_for(_s: u32, ) -> Weight {
		(63_952_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
