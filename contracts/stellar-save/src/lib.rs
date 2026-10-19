#![no_std]
// ── Clippy policy (see docs/clippy-policy.md) ─────────────────────────────────
//
// The attributes below suppress lint categories that routinely fire in Soroban
// smart-contract crates for justified, unavoidable reasons.  Each category has
// an inline comment explaining why it is allowed at the crate level rather than
// being fixed item-by-item.
//
// Lint: dead_code
//   Test-helper modules (admin_actions_tests, cei_tests, gas_benchmark, …) are
//   declared as unconditional `pub mod` entries rather than `#[cfg(test)] mod`
//   because the Soroban testutils SDK requires the contract types to be compiled
//   in the same crate as the tests.  This means items used only in test code
//   appear "dead" to the non-test build.
#![allow(dead_code)]
//
// Lint: unused_imports
//   Re-export blocks and test modules pull in names that are consumed only under
//   testutils compilation.  Removing them would break test builds.
#![allow(unused_imports)]
//
// Lint: clippy::module_inception
//   The `contract` module defines `StellarSaveContract`, mirroring the crate
//   name.  This is the canonical Soroban contract layout and cannot be renamed
//   without breaking the generated `StellarSaveContractClient`.
#![allow(clippy::module_inception)]
//
// Lint: clippy::too_many_arguments
//   Soroban contract entry-points must accept `Env` plus all domain parameters
//   in a single function signature; there is no way to group them differently
//   without violating the ABI.
#![allow(clippy::too_many_arguments)]
//
// Lint: clippy::type_complexity
//   Complex return types (e.g. `Result<Vec<(Address, bool)>, StellarSaveError>`)
//   are intrinsic to the contract query API and cannot be simplified without
//   introducing new wrapper types that would change the on-chain ABI.
#![allow(clippy::type_complexity)]
//
// Lint: clippy::comparison_chain
//   The Soroban SDK `Vec` does not implement `Ord`, so `if a < b … else if a > b`
//   chains are the only available comparison pattern in some helpers.
#![allow(clippy::comparison_chain)]

//! # Stellar-Save Smart Contract
//!
//! A decentralized rotational savings and credit association (ROSCA) built on Stellar Soroban.
//!
//! ## Module layout
//! - `types`:   Core contract-level types (`ContractConfig`, `MemberProfile`, etc.)
//! - `contract`: All `#[contractimpl]` entry-point methods (thin facades to domain modules)
//! - `group`, `contribution`, `payout`, `storage`, …: Domain modules

pub mod admin_actions_tests;
pub mod auth;
pub mod clone;
pub mod constants;
pub mod contribution;
pub mod contract;
pub mod cycle_advancement;
pub mod deadline;
pub mod error;
pub mod errors;
pub mod events;
pub mod governance;
pub mod group;
pub mod helpers;
pub mod migration;
pub mod migrations;
pub mod payout;
pub mod payout_executor;
pub mod penalty;
pub mod pool;
pub mod rating;
pub mod refund;
pub mod repository;
pub mod search;
pub mod status;
pub mod storage;
pub mod storage_benchmark;
pub mod storage_optimization;
pub mod token;
pub mod types;

// mod auto_contribution_tests;
pub mod cei_tests;
pub mod gas_benchmark;
pub mod insurance_integration_tests;
pub mod test_utils;
// mod invitation_tests;
// mod merge_tests;
// mod migration_matrix_tests;
// mod migration_tests;
// mod milestone_tests;
pub mod milestones;
// mod multi_token_tests;
// mod mutation_tests;
// mod upgrade_tests;
pub mod wrapping_audit;
pub mod zk_tests;

// ── Re-exports ────────────────────────────────────────────────────────────────
pub use auth::{is_active_member, require_admin, require_creator, require_member};
pub use contract::{StellarSaveContract, StellarSaveContractClient};
pub use contribution::{ContributionPage, ContributionRecord};
pub use error::{ContractResult, ErrorCategory, StellarSaveError};
pub use errors::{ContractError, ErrorRecoveryStrategy};
pub use events::EventEmitter;
pub use events::*;
pub use group::{Group, GroupStatus};
pub use payout::PayoutRecord;
pub use storage::StorageKeyBuilder;
pub use types::{ContractConfig, MemberProfile};

