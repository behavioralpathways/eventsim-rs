//! Tests for state_at() API - the core consumer query interface.
//!
//! These tests validate that state_at() correctly computes entity state
//! at any timestamp via forward projection or backward regression.

mod state_at_20_years_future_shows_decay;
mod state_at_childhood_regresses_from_adult;
