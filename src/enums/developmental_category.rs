//! Developmental category enum for sensitive period processing.
//!
//! Maps event types to developmental domains based on Erikson's
//! psychosocial stages. Used internally by developmental processing
//! to determine sensitive period amplification.

use crate::enums::EventType;

/// Developmental category based on Erikson's psychosocial stages.
///
/// Each category represents a developmental domain that may be
/// amplified during specific life stages (sensitive periods).
///
/// # Internal Use Only
///
/// This enum is used internally by developmental processing and is
/// not exposed in the public API. Consumers interact with developmental
/// effects implicitly through `state_at()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevelopmentalCategory {
    /// Trust vs Mistrust (infancy) - attachment and trust-building events.
    Attachment,

    /// Autonomy vs Shame (toddler) - independence and self-control events.
    /// No EventType currently maps here; retained for Erikson stage completeness.
    #[allow(dead_code)]
    Autonomy,

    /// Initiative vs Guilt (preschool) - purpose and exploration events.
    /// No EventType currently maps here; retained for Erikson stage completeness.
    #[allow(dead_code)]
    Initiative,

    /// Industry vs Inferiority (school age) - competence and achievement events.
    Industry,

    /// Identity vs Role Confusion (adolescence) - identity formation events.
    Identity,

    /// Intimacy vs Isolation (young adult) - close relationship events.
    Intimacy,

    /// Generativity vs Stagnation (middle adult) - nurturing and mentoring events.
    #[allow(dead_code)]
    Generativity,

    /// Integrity vs Despair (late adult) - meaning-making and life review events.
    Integrity,

    /// No developmental amplification - cross-stage or neutral events.
    Neutral,
}

impl DevelopmentalCategory {
    /// Returns all developmental category variants.
    #[must_use]
    #[allow(dead_code)]
    pub const fn all() -> [DevelopmentalCategory; 9] {
        [
            DevelopmentalCategory::Attachment,
            DevelopmentalCategory::Autonomy,
            DevelopmentalCategory::Initiative,
            DevelopmentalCategory::Industry,
            DevelopmentalCategory::Identity,
            DevelopmentalCategory::Intimacy,
            DevelopmentalCategory::Generativity,
            DevelopmentalCategory::Integrity,
            DevelopmentalCategory::Neutral,
        ]
    }

    /// Returns a human-readable name for this category.
    #[must_use]
    #[allow(dead_code)]
    pub const fn name(&self) -> &'static str {
        match self {
            DevelopmentalCategory::Attachment => "Attachment",
            DevelopmentalCategory::Autonomy => "Autonomy",
            DevelopmentalCategory::Initiative => "Initiative",
            DevelopmentalCategory::Industry => "Industry",
            DevelopmentalCategory::Identity => "Identity",
            DevelopmentalCategory::Intimacy => "Intimacy",
            DevelopmentalCategory::Generativity => "Generativity",
            DevelopmentalCategory::Integrity => "Integrity",
            DevelopmentalCategory::Neutral => "Neutral",
        }
    }
}

impl From<&EventType> for DevelopmentalCategory {
    /// Maps event types to developmental categories.
    ///
    /// # Mapping Rationale
    ///
    /// - **Attachment**: Betrayal affects trust building/breaking
    /// - **Industry**: Achievement - competence development
    /// - **Identity**: Chronic illness - identity transformation
    /// - **Intimacy**: Relationship end - close relationship events
    /// - **Integrity**: Mortality awareness - meaning-making
    /// - **Neutral**: Combat, self-harm - cross-stage events (trauma/violence)
    fn from(event_type: &EventType) -> Self {
        match event_type {
            // Attachment category - trust building/breaking
            EventType::ExperienceBetrayalTrust => DevelopmentalCategory::Attachment,

            // Industry category - competence
            EventType::AchieveGoalMajor => DevelopmentalCategory::Industry,
            EventType::FailGoalMajor => DevelopmentalCategory::Industry,
            EventType::GainPowerPersonal => DevelopmentalCategory::Industry,
            EventType::LoseJobFired => DevelopmentalCategory::Industry,
            EventType::LoseJobLayoff => DevelopmentalCategory::Industry,
            EventType::LoseJobResigned => DevelopmentalCategory::Industry,

            // Identity category - identity transformation
            EventType::DevelopIllnessChronic => DevelopmentalCategory::Identity,

            // Intimacy category - close relationships
            EventType::EndRelationshipRomantic => DevelopmentalCategory::Intimacy,
            EventType::ExperienceConflictFamily => DevelopmentalCategory::Intimacy,
            EventType::ExperienceConflictInterpersonal => DevelopmentalCategory::Intimacy,

            // Integrity category - meaning-making
            EventType::ExperienceAwarenessMortality => DevelopmentalCategory::Integrity,

            // Attachment category - trust building/breaking (additional)
            EventType::ExperienceExclusionGroup => DevelopmentalCategory::Attachment,
            EventType::ExperienceExclusionPeer => DevelopmentalCategory::Attachment,
            EventType::ExperienceInclusionPeer => DevelopmentalCategory::Attachment,
            EventType::ExperienceIsolationChronic => DevelopmentalCategory::Attachment,
            EventType::ExperienceRejectionFamily => DevelopmentalCategory::Attachment,
            EventType::ExperienceRejectionPeer => DevelopmentalCategory::Attachment,
            EventType::LosePersonDeath => DevelopmentalCategory::Attachment,

            // Intimacy category - romantic relationships
            EventType::ExperienceRejectionRomantic => DevelopmentalCategory::Intimacy,

            // Neutral category - cross-stage events (trauma/violence)
            EventType::EngageSelfharmNonsuicidal => DevelopmentalCategory::Neutral,
            EventType::ExperienceCombatMilitary => DevelopmentalCategory::Neutral,
            EventType::ExperienceHumiliationPublic => DevelopmentalCategory::Neutral,
            EventType::ExperienceShamingPublic => DevelopmentalCategory::Neutral,
            EventType::ExperienceStrainFinancial => DevelopmentalCategory::Neutral,
            EventType::ExperienceWarRegional => DevelopmentalCategory::Neutral,
            EventType::FaceChargesLegal => DevelopmentalCategory::Neutral,
            EventType::FaceEvictionHousing => DevelopmentalCategory::Neutral,
            EventType::LoseAccessHealthcare => DevelopmentalCategory::Neutral,
            EventType::LoseBenefitsGovernment => DevelopmentalCategory::Neutral,

            // Custom events are neutral by default
            EventType::Custom => DevelopmentalCategory::Neutral,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_nine_categories() {
        let all = DevelopmentalCategory::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn all_categories_have_names() {
        for category in DevelopmentalCategory::all() {
            assert!(!category.name().is_empty());
        }
    }

    #[test]
    fn category_equality() {
        assert_eq!(
            DevelopmentalCategory::Attachment,
            DevelopmentalCategory::Attachment
        );
        assert_ne!(
            DevelopmentalCategory::Attachment,
            DevelopmentalCategory::Identity
        );
    }

    #[test]
    fn category_is_copy() {
        let c1 = DevelopmentalCategory::Attachment;
        let c2 = c1; // Copy
        assert_eq!(c1, c2);
    }

    #[test]
    fn category_is_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DevelopmentalCategory::Attachment);
        set.insert(DevelopmentalCategory::Attachment);
        assert_eq!(set.len(), 1);

        set.insert(DevelopmentalCategory::Identity);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn category_debug_format() {
        let category = DevelopmentalCategory::Attachment;
        let debug = format!("{:?}", category);
        assert!(debug.contains("Attachment"));
    }

    // --- EventType to DevelopmentalCategory mapping tests ---

    #[test]
    fn betrayal_maps_to_attachment() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::ExperienceBetrayalTrust),
            DevelopmentalCategory::Attachment
        );
    }

    #[test]
    fn achievement_maps_to_industry() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::AchieveGoalMajor),
            DevelopmentalCategory::Industry
        );
    }

    #[test]
    fn chronic_illness_maps_to_identity() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::DevelopIllnessChronic),
            DevelopmentalCategory::Identity
        );
    }

    #[test]
    fn relationship_end_maps_to_intimacy() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::EndRelationshipRomantic),
            DevelopmentalCategory::Intimacy
        );
    }

    #[test]
    fn mortality_awareness_maps_to_integrity() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::ExperienceAwarenessMortality),
            DevelopmentalCategory::Integrity
        );
    }

    #[test]
    fn neutral_events_map_to_neutral() {
        assert_eq!(
            DevelopmentalCategory::from(&EventType::EngageSelfharmNonsuicidal),
            DevelopmentalCategory::Neutral
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::ExperienceCombatMilitary),
            DevelopmentalCategory::Neutral
        );
        assert_eq!(
            DevelopmentalCategory::from(&EventType::Custom),
            DevelopmentalCategory::Neutral
        );
    }

    #[test]
    fn all_event_types_have_category_mapping() {
        // Ensure every event type maps to a developmental category
        for event_type in EventType::all() {
            let _ = DevelopmentalCategory::from(&event_type);
        }
    }

    #[test]
    fn neutral_category_name_is_neutral() {
        assert_eq!(DevelopmentalCategory::Neutral.name(), "Neutral");
    }

    #[test]
    fn attachment_name() {
        assert_eq!(DevelopmentalCategory::Attachment.name(), "Attachment");
    }

    #[test]
    fn identity_name() {
        assert_eq!(DevelopmentalCategory::Identity.name(), "Identity");
    }

    #[test]
    fn intimacy_name() {
        assert_eq!(DevelopmentalCategory::Intimacy.name(), "Intimacy");
    }

    #[test]
    fn generativity_name() {
        assert_eq!(DevelopmentalCategory::Generativity.name(), "Generativity");
    }

    #[test]
    fn integrity_name() {
        assert_eq!(DevelopmentalCategory::Integrity.name(), "Integrity");
    }

    #[test]
    fn autonomy_name() {
        assert_eq!(DevelopmentalCategory::Autonomy.name(), "Autonomy");
    }

    #[test]
    fn initiative_name() {
        assert_eq!(DevelopmentalCategory::Initiative.name(), "Initiative");
    }

    #[test]
    fn industry_name() {
        assert_eq!(DevelopmentalCategory::Industry.name(), "Industry");
    }
}
