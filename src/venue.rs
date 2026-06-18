//! Projection Finance venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Projection Finance";

/// Venue category.
pub const VENUE_TYPE: &str = "Volatility / sims";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Direction Hunting",
    "Spread Farming",
];
