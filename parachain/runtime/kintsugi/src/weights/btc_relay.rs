
//! Autogenerated weights for btc_relay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sander-dell`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 100
// --repeat
// 10
// --output
// /tmp/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for btc_relay using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> btc_relay::WeightInfo for WeightInfo<T> {
	/// Storage: BTCRelay BestBlock (r:1 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:1)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: BTCRelay StartBlockHeight (r:0 w:1)
	/// Proof: BTCRelay StartBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:0 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:0 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	fn initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `4520`
		// Minimum execution time: 40_296_000 picoseconds.
		Weight::from_parts(41_141_000, 4520)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:1 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:1 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:0 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn store_block_header() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `965`
		//  Estimated: `11399`
		// Minimum execution time: 45_804_000 picoseconds.
		Weight::from_parts(49_633_000, 11399)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	/// Proof: BTCRelay DisableInclusionCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:1 w:0)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableParachainConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn verify_and_validate_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `999`
		//  Estimated: `7167`
		// Minimum execution time: 48_864_000 picoseconds.
		Weight::from_parts(50_083_000, 7167)
			.saturating_add(T::DbWeight::get().reads(6_u64))
	}
	/// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	/// Proof: BTCRelay DisableInclusionCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:1 w:0)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableParachainConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn verify_transaction_inclusion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `999`
		//  Estimated: `7167`
		// Minimum execution time: 34_887_000 picoseconds.
		Weight::from_parts(40_247_000, 7167)
			.saturating_add(T::DbWeight::get().reads(6_u64))
	}
	fn validate_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_776_000 picoseconds.
		Weight::from_parts(8_316_000, 0)
	}
}