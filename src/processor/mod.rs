//! Processing subsystems for entity state changes.
//!
//! This module contains processors that operate on entity state, including:
//! - State evolution (internal: advance/regress/apply/reverse)
//! - Event processing (internal: interpret/apply/process)
//! - Developmental processing (internal: plasticity, sensitive periods, turning points)
//!
//! # Key Types
//!
//! - [`InterpretedEvent`] - Interpreted event with computed deltas
//!
//! # Internal Functions (crate visibility)
//!
//! The following functions are internal to the crate and used by the Simulation API:
//! - State evolution: `advance_state`, `regress_state`, `apply_interpreted_event_to_state`, `reverse_interpreted_event_from_state`
//! - Event processing: `interpret_event`
//! - Developmental: `apply_developmental_effects`
//!
//! The following functions are internal to their modules and used only in tests:
//! - Event: `apply_interpreted_event`, `process_event`
//! - State evolution: `apply_event_to_state`

mod developmental;
mod event;
mod state_evolution;

pub(crate) use developmental::apply_developmental_effects;
pub(crate) use event::interpret_event;
pub(crate) use event::process_event_to_relationships;
pub use event::InterpretedEvent;
// apply_interpreted_event and process_event are internal to the event module and its tests
pub(crate) use state_evolution::{
    advance_state, apply_interpreted_event_to_state, regress_state,
    reverse_interpreted_event_from_state,
};
// apply_event_to_state is internal to the state_evolution module and its tests
