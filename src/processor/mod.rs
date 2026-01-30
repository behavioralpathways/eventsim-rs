//! Processing subsystems for entity state changes.
//!
//! This module contains processors that operate on entity state, including:
//! - Decay processing for state values
//! - ITS (Interpersonal Theory of Suicide) computation
//! - Alert generation for threshold crossings (internal)
//! - State evolution (internal: advance/regress/apply/reverse)
//! - Event processing (internal: interpret/apply/process)
//! - Developmental processing (internal: plasticity, sensitive periods, turning points)
//!
//! # Key Types
//!
//! - [`DecayProcessor`] - Trait for applying decay to entity state
//! - [`StateDecayProcessor`] - Real implementation with exponential decay
//! - [`NoOpDecayProcessor`] - No-op implementation for testing/robotic entities
//! - [`InterpretedEvent`] - Interpreted event with computed deltas
//! - [`ItsFactors`] - Computed ITS risk factors
//!
//! # Internal Functions (crate visibility)
//!
//! The following functions are internal to the crate and used by the Simulation API:
//! - State evolution: `advance_state`, `regress_state`, `apply_interpreted_event_to_state`, `reverse_interpreted_event_from_state`
//! - Event processing: `interpret_event`
//! - Developmental: `apply_developmental_effects`
//!
//! The following functions are internal to their modules and used only in tests:
//! - Alerts: `check_its_thresholds`, `check_spiral_alerts`
//! - Event: `apply_interpreted_event`, `process_event`
//! - State evolution: `apply_event_to_state`

mod alerts;
mod decay;
mod developmental;
mod emotions;
mod event;
mod feedback;
mod its;
mod its_contributors;
mod reversibility;
mod state_evolution;

// Alert functions are internal to the alerts module and its tests
pub use decay::DecayProcessor;
#[allow(unused_imports)]
pub use decay::{NoOpDecayProcessor, StateDecayProcessor};
pub(crate) use developmental::apply_developmental_effects;
pub(crate) use event::interpret_event;
pub(crate) use event::process_event_to_relationships;
pub use event::InterpretedEvent;
// apply_interpreted_event and process_event are internal to the event module and its tests
#[allow(unused_imports)]
pub use its::{
    compute_its_factors, ConvergenceStatus, ItsFactors, ItsProximalFactor, AC_ELEVATED_THRESHOLD,
};
#[allow(unused_imports)]
pub use its_contributors::{
    AcContributor, ContributorActivation, ItsContributor, ItsContributors, PbContributor,
    TbContributor, ACUTE_CONTRIBUTOR_DECAY_HALF_LIFE, CONTRIBUTOR_ACTIVATION_THRESHOLD,
};
pub(crate) use state_evolution::{
    advance_state, apply_interpreted_event_to_state, regress_state,
    reverse_interpreted_event_from_state,
};
// apply_event_to_state is internal to the state_evolution module and its tests
