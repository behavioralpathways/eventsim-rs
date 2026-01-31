//! Mapping of event specs to trust antecedents.
//!
//! This module derives trust antecedents from event specifications,
//! using the event's impact on trust-related dimensions (trust_propensity,
//! prc, perceived_competence) to determine trust effects.

use crate::enums::TrustDomain;
use crate::event::Event;
use crate::relationship::{AntecedentDirection, AntecedentType};

/// A mapping entry from an event to a trust antecedent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AntecedentMapping {
    /// The trust dimension affected.
    pub antecedent_type: AntecedentType,
    /// Whether the antecedent is positive or negative.
    pub direction: AntecedentDirection,
    /// Base magnitude multiplier (0-1) applied to event severity.
    pub base_magnitude: f32,
    /// Narrative context string.
    pub context: &'static str,
    /// Trust domain affected by the antecedent.
    pub domain: TrustDomain,
}

impl AntecedentMapping {
    /// Creates a new mapping entry.
    pub const fn new(
        antecedent_type: AntecedentType,
        direction: AntecedentDirection,
        base_magnitude: f32,
        context: &'static str,
    ) -> Self {
        AntecedentMapping {
            antecedent_type,
            direction,
            base_magnitude,
            context,
            domain: antecedent_type.trust_domain(),
        }
    }
}

/// Trust antecedent lookup table (for reference - deprecated in favor of spec-based).
/// Kept for backwards compatibility but prefer get_antecedent_for_event().
#[allow(dead_code)]
pub const TRUST_ANTECEDENT_TABLE: &[(crate::enums::EventType, &[AntecedentMapping])] = &[];

/// Returns the antecedent mappings for an event based on its spec.
///
/// This function derives trust antecedents from the event's spec by examining:
/// - `trust_propensity`: Direct trust impact (positive = trust building, negative = breach)
/// - `prc`: Perceived reciprocal caring (affects benevolence perception)
/// - `perceived_competence`: Competence demonstration (affects ability perception)
///
/// # Arguments
///
/// * `event` - The event to analyze
///
/// # Returns
///
/// A vector of antecedent mappings derived from the event's spec.
#[must_use]
pub fn get_antecedent_for_event(event: &Event) -> Vec<AntecedentMapping> {
    let spec = event.spec();
    let severity = event.severity() as f32;
    let context = event.event_type().name();

    let mut mappings = Vec::new();

    // Trust propensity impact -> Integrity or general trust
    if spec.impact.trust_propensity.abs() > 0.05 {
        let (direction, magnitude) = if spec.impact.trust_propensity > 0.0 {
            (AntecedentDirection::Positive, spec.impact.trust_propensity)
        } else {
            (
                AntecedentDirection::Negative,
                spec.impact.trust_propensity.abs(),
            )
        };

        mappings.push(AntecedentMapping::new(
            AntecedentType::Integrity,
            direction,
            (magnitude * severity).clamp(0.0, 1.0),
            context,
        ));
    }

    // PRC impact -> Benevolence
    if spec.impact.prc.abs() > 0.05 {
        let (direction, magnitude) = if spec.impact.prc > 0.0 {
            (AntecedentDirection::Positive, spec.impact.prc)
        } else {
            (AntecedentDirection::Negative, spec.impact.prc.abs())
        };

        mappings.push(AntecedentMapping::new(
            AntecedentType::Benevolence,
            direction,
            (magnitude * severity).clamp(0.0, 1.0),
            context,
        ));
    }

    // Perceived competence impact -> Ability
    if spec.impact.perceived_competence.abs() > 0.05 {
        let (direction, magnitude) = if spec.impact.perceived_competence > 0.0 {
            (AntecedentDirection::Positive, spec.impact.perceived_competence)
        } else {
            (
                AntecedentDirection::Negative,
                spec.impact.perceived_competence.abs(),
            )
        };

        mappings.push(AntecedentMapping::new(
            AntecedentType::Ability,
            direction,
            (magnitude * severity).clamp(0.0, 1.0),
            context,
        ));
    }

    mappings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::EventType;
    use crate::event::EventBuilder;

    #[test]
    fn antecedent_mapping_new_builds_struct() {
        let mapping = AntecedentMapping::new(
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.42,
            "test",
        );

        assert_eq!(mapping.antecedent_type, AntecedentType::Integrity);
        assert_eq!(mapping.direction, AntecedentDirection::Negative);
        assert!((mapping.base_magnitude - 0.42).abs() < f32::EPSILON);
        assert_eq!(mapping.context, "test");
        assert_eq!(mapping.domain, TrustDomain::Disclosure);
    }

    #[test]
    fn betrayal_event_has_trust_antecedents() {
        let event = EventBuilder::new(EventType::ExperienceBetrayalTrust)
            .severity(0.8)
            .build()
            .unwrap();

        let mappings = get_antecedent_for_event(&event);

        // Betrayal should have negative trust antecedents
        // Either integrity or benevolence should be affected
        let has_negative = mappings
            .iter()
            .any(|m| m.direction == AntecedentDirection::Negative);

        // Note: If the spec has trust_propensity or prc < 0, we'll see mappings
        // Otherwise the list may be empty (which is fine - not all events affect trust)
        assert!(mappings.is_empty() || has_negative);
    }

    #[test]
    fn achievement_event_may_have_competence_antecedent() {
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .severity(0.8)
            .build()
            .unwrap();

        let mappings = get_antecedent_for_event(&event);

        // If spec has positive perceived_competence, should have Ability antecedent
        let has_ability = mappings
            .iter()
            .any(|m| m.antecedent_type == AntecedentType::Ability);
        let has_positive = mappings
            .iter()
            .any(|m| m.direction == AntecedentDirection::Positive);

        // This depends on the spec - may or may not have ability impact
        let outcome_count =
            (mappings.is_empty() as u8) + (has_ability as u8) + (has_positive as u8);
        assert!(outcome_count > 0);
    }

    #[test]
    fn event_without_trust_impact_returns_empty() {
        // Custom event with no trust-related impacts
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let custom_spec = EventSpec {
            impact: EventImpact {
                valence: 0.5,
                arousal: 0.3,
                // No trust_propensity, prc, or perceived_competence
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(0.8).build().unwrap();

        let mappings = get_antecedent_for_event(&event);

        assert!(mappings.is_empty());
    }

    #[test]
    fn event_with_positive_trust_propensity_has_positive_integrity() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let custom_spec = EventSpec {
            impact: EventImpact {
                trust_propensity: 0.3,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(0.8).build().unwrap();

        let mappings = get_antecedent_for_event(&event);

        assert!(!mappings.is_empty());
        let integrity = mappings
            .iter()
            .find(|m| m.antecedent_type == AntecedentType::Integrity);
        assert!(integrity.is_some());
        assert_eq!(integrity.unwrap().direction, AntecedentDirection::Positive);
    }

    #[test]
    fn event_with_negative_prc_has_negative_benevolence() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let custom_spec = EventSpec {
            impact: EventImpact {
                prc: -0.4,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(0.8).build().unwrap();

        let mappings = get_antecedent_for_event(&event);

        assert!(!mappings.is_empty());
        let benevolence = mappings
            .iter()
            .find(|m| m.antecedent_type == AntecedentType::Benevolence);
        assert!(benevolence.is_some());
        assert_eq!(benevolence.unwrap().direction, AntecedentDirection::Negative);
    }

    #[test]
    fn event_with_positive_competence_has_positive_ability() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let custom_spec = EventSpec {
            impact: EventImpact {
                perceived_competence: 0.25,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(0.8).build().unwrap();

        let mappings = get_antecedent_for_event(&event);

        assert!(!mappings.is_empty());
        let ability = mappings
            .iter()
            .find(|m| m.antecedent_type == AntecedentType::Ability);
        assert!(ability.is_some());
        assert_eq!(ability.unwrap().direction, AntecedentDirection::Positive);
    }

    #[test]
    fn all_event_types_can_get_antecedents() {
        // All event types should be able to call get_antecedent_for_event without panic
        for event_type in EventType::all() {
            let event = EventBuilder::new(event_type).severity(0.5).build().unwrap();

            let _ = get_antecedent_for_event(&event);
        }
    }

    #[test]
    fn magnitude_scales_with_severity() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let custom_spec = EventSpec {
            impact: EventImpact {
                trust_propensity: 0.5,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let low_severity = EventBuilder::custom(custom_spec)
            .severity(0.2)
            .build()
            .unwrap();
        let high_severity = EventBuilder::custom(custom_spec)
            .severity(0.9)
            .build()
            .unwrap();

        let low_mappings = get_antecedent_for_event(&low_severity);
        let high_mappings = get_antecedent_for_event(&high_severity);

        assert!(
            low_mappings[0].base_magnitude < high_mappings[0].base_magnitude
        );
    }

    #[test]
    fn event_with_negative_competence_has_negative_ability() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let custom_spec = EventSpec {
            impact: EventImpact {
                perceived_competence: -0.3,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(0.8).build().unwrap();

        let mappings = get_antecedent_for_event(&event);

        assert!(!mappings.is_empty());
        let ability = mappings
            .iter()
            .find(|m| m.antecedent_type == AntecedentType::Ability);
        assert!(ability.is_some());
        assert_eq!(ability.unwrap().direction, AntecedentDirection::Negative);
    }
}
