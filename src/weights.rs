//! Autogenerated weights for orml_auction
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-04, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// /Users/xiliangchen/projects/acala/target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=orml_auction
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./auction/src/weights.rs
// --template
// ../templates/orml-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for orml_auction.
pub trait WeightInfo {
	fn bid_collateral_auction() -> Weight;
	fn on_finalize(c: u32, ) -> Weight;
}

/// Default weights.
impl WeightInfo for () {
	fn bid_collateral_auction() -> Weight {
		Weight::from_parts(108_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	fn on_finalize(c: u32, ) -> Weight {
		Weight::from_parts(9_779_000, 0)
			// Standard Error: 13_000
			.saturating_add(Weight::from_parts(57_962_000, 0).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(c as u64)))
	}
}
