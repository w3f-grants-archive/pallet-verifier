//! `rustc` callbacks for analyzing FRAME pallets.

mod entry_points;
mod utils;
mod verifier;

pub use {entry_points::EntryPointsCallbacks, verifier::VerifierCallbacks};
