//! # pallet-verifier
//!
//! pallet-verifier is a tool for detecting [common security vulnerabilities][vulnerabilities] and
//! [insecure patterns] in [FRAME pallets][FRAME] using [static program analysis][static-analysis]
//! techniques like [data-flow analysis][data-flow], [abstract interpretation][abs-int] and
//! [symbolic execution][symbex].
//!
//! [FRAME]: https://docs.substrate.io/learn/runtime-development/#frame
//! [Substrate]: https://docs.substrate.io/
//! [vulnerabilities]: https://secure-contracts.com/not-so-smart-contracts/substrate/
//! [insecure patterns]: https://docs.substrate.io/build/troubleshoot-your-code/#unsafe-or-insecure-patterns
//! [static-analysis]: https://en.wikipedia.org/wiki/Static_program_analysis
//! [data-flow]: https://en.wikipedia.org/wiki/Data-flow_analysis
//! [abs-int]: https://en.wikipedia.org/wiki/Abstract_interpretation
//! [symbex]: https://en.wikipedia.org/wiki/Symbolic_execution

#![feature(rustc_private)]
#![feature(new_uninit)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_const_eval;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_transform;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;

mod callbacks;
mod file_loader;
mod providers;
mod utils;

pub use callbacks::{DefaultCallbacks, EntryPointsCallbacks, VerifierCallbacks};
pub use file_loader::VirtualFileLoader;

/// Name of generated `mod` for entry points.
pub const ENTRY_POINTS_MOD_NAME: &str = "__pallet_verifier_entry_points";
/// Prefix of generated `fn`s for entry points.
pub const ENTRY_POINT_FN_PREFIX: &str = "__pallet_verifier_entry_point__";

/// Name of generated `mod` for MIRAI contracts.
pub const CONTRACTS_MOD_NAME: &str = "foreign_contracts";

/// Tracks pointer width for target execution environment.
pub const ENV_TARGET_POINTER_WIDTH: &str = "PALLET_VERIFIER_TARGET_POINTER_WIDTH";

/// Kind of pallet `fn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    Dispatchable,
    PubAssocFn,
}

impl std::fmt::Display for CallKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CallKind::Dispatchable => "dispatchable",
                CallKind::PubAssocFn => "pub assoc fn",
            }
        )
    }
}
