//! Relationship modeling for behavioral pathways.
//!
//! This module contains types and functions for modeling relationships
//! between entities, including:
//!
//! - Trust decomposition (competence, benevolence, integrity)
//! - Perceived risk assessment
//! - Relationship stages (Stranger -> Intimate)
//! - Shared and directional dimensions
//!
//! # Trust Model
//!
//! Based on Mayer's model, trust is decomposed into:
//!
//! - **Propensity**: The trustor's general willingness to trust (from Entity)
//! - **Trustworthiness**: Perceived competence, benevolence, and integrity of trustee
//! - **Perceived Risk**: Subjective assessment of potential negative consequences
//!
//! # Example
//!
//! ```
//! use eventsim_rs::relationship::{Relationship, RelationshipStage};
//! use eventsim_rs::types::EntityId;
//! use eventsim_rs::enums::Direction;
//!
//! // Create a relationship
//! let alice = EntityId::new("alice").unwrap();
//! let bob = EntityId::new("bob").unwrap();
//!
//! let rel = Relationship::try_between(alice, bob).unwrap()
//!     .with_stage(RelationshipStage::Acquaintance);
//!
//! // Access trustworthiness factors
//! let trustworthiness = rel.trustworthiness(Direction::AToB);
//! ```

mod antecedent;
mod directional_dimensions;
mod interaction_pattern;
mod perceived_risk;
#[allow(clippy::module_inception)]
mod relationship;
mod shared_dimensions;
mod stage;
mod trust_context;
mod trust_decision;
mod trustworthiness;

pub use antecedent::{AntecedentDirection, AntecedentType, TrustAntecedent};
pub use directional_dimensions::DirectionalDimensions;
pub use interaction_pattern::InteractionPattern;
pub use perceived_risk::PerceivedRisk;
pub use relationship::{Relationship, RelationshipError, StageTransitionError};
pub use shared_dimensions::SharedDimensions;
pub use stage::RelationshipStage;
pub use trust_context::TrustContext;
pub use trust_decision::TrustDecision;
pub use trustworthiness::TrustworthinessFactors;
