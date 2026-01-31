//! Event module for behavioral pathways.
//!
//! This module provides event types, construction, and salience
//! computation for the simulation. Events are the primary mechanism for
//! state changes.
//!
//! # Key Types
//!
//! - [`Event`] - Core event structure
//! - [`EventBuilder`] - Fluent construction
//! - [`EventSpec`] - Event impact specification across 22 dimensions
//!
//! # Usage
//!
//! ```ignore
//! use eventsim_rs::event::{Event, EventBuilder};
//! use eventsim_rs::enums::EventType;
//!
//! // Build an event
//! let event = EventBuilder::new(EventType::EndRelationshipRomantic)
//!     .severity(0.8)
//!     .build()
//!     .unwrap();
//! ```

#[allow(clippy::module_inception)]
mod event;
mod event_builder;
pub mod event_spec;
mod salience;
pub mod types;

pub use event::Event;
pub use event_builder::{EventBuildError, EventBuilder};
pub use event_spec::{AppliedDeltas, ChronicFlags, EventImpact, EventSpec, PermanenceValues};
pub use salience::{
    arousal_weight_for_species, compute_arousal_modulated_salience, AROUSAL_CEILING,
    AROUSAL_THRESHOLD, AROUSAL_WEIGHT_ANIMAL, AROUSAL_WEIGHT_HUMAN, AROUSAL_WEIGHT_ROBOTIC,
    EXTREME_AROUSAL_IMPAIRMENT, NEGATIVITY_BIAS_MULTIPLIER,
};
