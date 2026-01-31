//! Tag derivation from event impact profiles.
//!
//! This module provides automatic derivation of memory tags from an event's
//! psychological impacts. This works for both built-in and custom events since
//! it operates on the EventSpec's impact values rather than event type names.

use crate::event::EventSpec;
use crate::memory::MemoryTag;

/// Derives memory tags from an event's psychological impact profile.
///
/// Tags are inferred from the severity and direction of various impact dimensions:
/// - Violence: high acquired_capability
/// - Crisis: high arousal with negative valence
/// - Loss: negative PRC or high loneliness
/// - Betrayal: strongly negative PRC with interpersonal hopelessness
/// - Achievement: positive valence with purpose
/// - Support: positive PRC with reduced loneliness
///
/// Falls back to `Personal` if no strong signals are detected.
///
/// # Arguments
///
/// * `spec` - The event specification containing impact values
/// * `severity` - The event severity (0.0 to 1.0) used for threshold scaling
///
/// # Returns
///
/// A vector of derived memory tags, never empty (at minimum contains `Personal`).
#[must_use]
pub fn derive_tags_from_impacts(spec: &EventSpec, severity: f64) -> Vec<MemoryTag> {
    // At severity 0, the event has no psychological weight - return Personal fallback
    if severity <= 0.0 {
        return vec![MemoryTag::Personal];
    }

    let mut tags = Vec::new();
    let impact = &spec.impact;
    let severity_f32 = severity as f32;

    // Violence/Trauma indicators
    // High acquired capability suggests exposure to violence or pain
    if impact.acquired_capability > 0.3 * severity_f32 {
        tags.push(MemoryTag::Violence);
    }

    // Crisis indicators
    // High arousal combined with negative valence suggests crisis
    if impact.arousal > 0.5 * severity_f32 && impact.valence < -0.3 * severity_f32 {
        tags.push(MemoryTag::Crisis);
    }

    // Loss indicators
    // Negative PRC or increased loneliness suggests loss
    if impact.prc < -0.2 * severity_f32 || impact.loneliness > 0.3 * severity_f32 {
        tags.push(MemoryTag::Loss);
    }

    // Betrayal indicators
    // Strongly negative PRC with interpersonal hopelessness suggests betrayal
    if impact.prc < -0.4 * severity_f32 && impact.interpersonal_hopelessness > 0.2 * severity_f32 {
        tags.push(MemoryTag::Betrayal);
    }

    // Achievement indicators
    // Positive valence with sense of purpose suggests achievement
    if impact.valence > 0.3 * severity_f32 && impact.purpose > 0.2 * severity_f32 {
        tags.push(MemoryTag::Achievement);
    }

    // Support indicators
    // Positive PRC with reduced loneliness suggests supportive experience
    if impact.prc > 0.3 * severity_f32 && impact.loneliness < -0.2 * severity_f32 {
        tags.push(MemoryTag::Support);
    }

    // Conflict indicators
    // Increased aggression or grievance suggests conflict
    if impact.aggression > 0.3 * severity_f32 || impact.grievance > 0.3 * severity_f32 {
        tags.push(MemoryTag::Conflict);
    }

    // Default fallback - every memory should have at least one tag
    if tags.is_empty() {
        tags.push(MemoryTag::Personal);
    }

    tags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::event_spec::{ChronicFlags, EventImpact, PermanenceValues};

    fn create_spec_with_impact(impact: EventImpact) -> EventSpec {
        EventSpec {
            impact,
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        }
    }

    #[test]
    fn empty_impact_returns_personal_tag() {
        let spec = EventSpec::default();
        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert_eq!(tags.len(), 1);
        assert!(tags.contains(&MemoryTag::Personal));
    }

    #[test]
    fn high_acquired_capability_returns_violence_tag() {
        let spec = create_spec_with_impact(EventImpact {
            acquired_capability: 0.5,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Violence));
    }

    #[test]
    fn high_arousal_negative_valence_returns_crisis_tag() {
        let spec = create_spec_with_impact(EventImpact {
            arousal: 0.7,
            valence: -0.5,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Crisis));
    }

    #[test]
    fn negative_prc_returns_loss_tag() {
        let spec = create_spec_with_impact(EventImpact {
            prc: -0.4,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Loss));
    }

    #[test]
    fn high_loneliness_returns_loss_tag() {
        let spec = create_spec_with_impact(EventImpact {
            loneliness: 0.5,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Loss));
    }

    #[test]
    fn negative_prc_with_interpersonal_hopelessness_returns_betrayal_tag() {
        let spec = create_spec_with_impact(EventImpact {
            prc: -0.6,
            interpersonal_hopelessness: 0.4,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Betrayal));
        // Also contains Loss because of negative PRC
        assert!(tags.contains(&MemoryTag::Loss));
    }

    #[test]
    fn positive_valence_with_purpose_returns_achievement_tag() {
        let spec = create_spec_with_impact(EventImpact {
            valence: 0.5,
            purpose: 0.4,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Achievement));
    }

    #[test]
    fn positive_prc_with_reduced_loneliness_returns_support_tag() {
        let spec = create_spec_with_impact(EventImpact {
            prc: 0.5,
            loneliness: -0.4,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Support));
    }

    #[test]
    fn high_aggression_returns_conflict_tag() {
        let spec = create_spec_with_impact(EventImpact {
            aggression: 0.5,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Conflict));
    }

    #[test]
    fn high_grievance_returns_conflict_tag() {
        let spec = create_spec_with_impact(EventImpact {
            grievance: 0.5,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Conflict));
    }

    #[test]
    fn severity_scales_thresholds() {
        // At severity 0.5, thresholds are halved
        let spec = create_spec_with_impact(EventImpact {
            acquired_capability: 0.2, // Below 0.3 but above 0.15 (0.3 * 0.5)
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 0.5);

        assert!(tags.contains(&MemoryTag::Violence));
    }

    #[test]
    fn zero_severity_returns_personal_tag() {
        // At severity 0, the event has no psychological weight
        // So we return Personal fallback regardless of impact values
        let spec = create_spec_with_impact(EventImpact {
            acquired_capability: 0.1,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 0.0);

        assert_eq!(tags.len(), 1);
        assert!(tags.contains(&MemoryTag::Personal));
    }

    #[test]
    fn multiple_tags_can_be_returned() {
        // Event with both violence and crisis characteristics
        let spec = create_spec_with_impact(EventImpact {
            acquired_capability: 0.5,
            arousal: 0.7,
            valence: -0.5,
            ..Default::default()
        });

        let tags = derive_tags_from_impacts(&spec, 1.0);

        assert!(tags.contains(&MemoryTag::Violence));
        assert!(tags.contains(&MemoryTag::Crisis));
        assert!(tags.len() >= 2);
    }

    #[test]
    fn tags_never_empty() {
        // Various specs should always return at least one tag
        let specs = vec![
            EventSpec::default(),
            create_spec_with_impact(EventImpact {
                valence: 0.1,
                ..Default::default()
            }),
            create_spec_with_impact(EventImpact {
                arousal: 0.2,
                ..Default::default()
            }),
        ];

        for spec in specs {
            let tags = derive_tags_from_impacts(&spec, 1.0);
            assert!(!tags.is_empty());
        }
    }
}
