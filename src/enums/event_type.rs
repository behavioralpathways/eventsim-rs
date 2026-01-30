//! Event type enums for compile-time validated event classification.
//!
//! EventType provides type-safe event categorization. Each variant maps to an
//! EventSpec that defines impact across all 22 psychological dimensions.

use crate::event::event_spec::EventSpec;
use crate::event::types;
use serde::{Deserialize, Serialize};

/// Primary event classification for compile-time validation.
///
/// Each event type has an associated EventSpec that defines its impact
/// across all 22 psychological dimensions, chronic flags, and permanence values.
///
/// # Examples
///
/// ```
/// use eventsim_rs::enums::EventType;
///
/// let event_type = EventType::EndRelationshipRomantic;
/// let spec = event_type.spec();
/// assert!(spec.impact.valence < 0.0); // Breakups have negative valence
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// Achievement of a major life goal (graduating, career milestone, etc.)
    AchieveGoalMajor,

    /// Development of a chronic illness (diabetes, heart disease, etc.)
    DevelopIllnessChronic,

    /// End of a romantic relationship (breakup, divorce)
    EndRelationshipRomantic,

    /// Non-suicidal self-injury (cutting, burning, etc.)
    EngageSelfharmNonsuicidal,

    /// Acute awareness of one's own mortality
    ExperienceAwarenessMortality,

    /// Significant betrayal of trust by someone close
    ExperienceBetrayalTrust,

    /// Direct participation in military combat
    ExperienceCombatMilitary,

    /// Conflict within the family unit (arguments, estrangement)
    ExperienceConflictFamily,

    /// Interpersonal conflict outside of family (disputes, disagreements)
    ExperienceConflictInterpersonal,

    /// Exclusion from a group or organization
    ExperienceExclusionGroup,

    /// Exclusion by peers (social rejection)
    ExperienceExclusionPeer,

    /// Being formally charged with a crime in legal proceedings
    FaceChargesLegal,

    /// Facing housing eviction (served notice, imminent loss of housing)
    FaceEvictionHousing,

    /// Failure to achieve a major life goal (failing exam, missing promotion, etc.)
    FailGoalMajor,

    /// Gaining personal power (promotion, authority, wealth, influence, leadership)
    GainPowerPersonal,

    /// Public humiliation or embarrassment
    ExperienceHumiliationPublic,

    /// Inclusion and acceptance by peers
    ExperienceInclusionPeer,

    /// Chronic social isolation over extended period
    ExperienceIsolationChronic,

    /// Loss of access to healthcare services (insurance loss, provider departure, facility closure, affordability barrier)
    LoseAccessHealthcare,

    /// Loss of government benefits (unemployment, disability, social security, welfare, food assistance, housing vouchers)
    LoseBenefitsGovernment,

    /// Involuntary job termination for cause (being fired)
    LoseJobFired,

    /// Involuntary job loss due to organizational downsizing, restructuring, or economic conditions
    LoseJobLayoff,

    /// Voluntary resignation from employment
    LoseJobResigned,

    /// Death of a loved one (spouse, child, parent, or close friend)
    LosePersonDeath,

    /// Rejection by family members (disownment, estrangement, explicit exclusion)
    ExperienceRejectionFamily,

    /// Active rejection by peers (direct dismissal or refusal by classmates, coworkers, friends)
    ExperienceRejectionPeer,

    /// Romantic rejection (being rejected by a desired romantic interest)
    ExperienceRejectionRomantic,

    /// Public shaming (being publicly called out, criticized, or shamed)
    ExperienceShamingPublic,

    /// Financial strain or hardship (ongoing economic stress from insufficient resources)
    ExperienceStrainFinancial,

    /// Regional war exposure (armed conflict affecting a geographic region)
    ExperienceWarRegional,

    /// Custom event with developer-provided EventSpec.
    /// Use `Event::custom(spec)` to create events with this type.
    Custom,
}

impl EventType {
    /// Returns the EventSpec for this event type.
    ///
    /// Each event type has an associated specification that defines its impact
    /// across all 22 psychological dimensions, chronic flags, and permanence values.
    ///
    /// For `Custom` events, returns a default (zero-impact) spec. Custom events
    /// should be created with `Event::custom(spec)` which stores the spec directly.
    #[must_use]
    pub fn spec(&self) -> EventSpec {
        match self {
            EventType::AchieveGoalMajor => types::achieve_goal_major::SPEC,
            EventType::DevelopIllnessChronic => types::develop_illness_chronic::SPEC,
            EventType::EndRelationshipRomantic => types::end_relationship_romantic::SPEC,
            EventType::EngageSelfharmNonsuicidal => types::engage_selfharm_nonsuicidal::SPEC,
            EventType::ExperienceAwarenessMortality => types::experience_awareness_mortality::SPEC,
            EventType::ExperienceBetrayalTrust => types::experience_betrayal_trust::SPEC,
            EventType::ExperienceCombatMilitary => types::experience_combat_military::SPEC,
            EventType::ExperienceConflictFamily => types::experience_conflict_family::SPEC,
            EventType::ExperienceConflictInterpersonal => {
                types::experience_conflict_interpersonal::SPEC
            }
            EventType::ExperienceExclusionGroup => types::experience_exclusion_group::SPEC,
            EventType::ExperienceExclusionPeer => types::experience_exclusion_peer::SPEC,
            EventType::FaceChargesLegal => types::face_charges_legal::SPEC,
            EventType::FaceEvictionHousing => types::face_eviction_housing::SPEC,
            EventType::FailGoalMajor => types::fail_goal_major::SPEC,
            EventType::GainPowerPersonal => types::gain_power_personal::SPEC,
            EventType::ExperienceHumiliationPublic => types::experience_humiliation_public::SPEC,
            EventType::ExperienceInclusionPeer => types::experience_inclusion_peer::SPEC,
            EventType::ExperienceIsolationChronic => types::experience_isolation_chronic::SPEC,
            EventType::LoseAccessHealthcare => types::lose_access_healthcare::SPEC,
            EventType::LoseBenefitsGovernment => types::lose_benefits_government::SPEC,
            EventType::LoseJobFired => types::lose_job_fired::SPEC,
            EventType::LoseJobLayoff => types::lose_job_layoff::SPEC,
            EventType::LoseJobResigned => types::lose_job_resigned::SPEC,
            EventType::LosePersonDeath => types::lose_person_death::SPEC,
            EventType::ExperienceRejectionFamily => types::experience_rejection_family::SPEC,
            EventType::ExperienceRejectionPeer => types::experience_rejection_peer::SPEC,
            EventType::ExperienceRejectionRomantic => types::experience_rejection_romantic::SPEC,
            EventType::ExperienceShamingPublic => types::experience_shaming_public::SPEC,
            EventType::ExperienceStrainFinancial => types::experience_strain_financial::SPEC,
            EventType::ExperienceWarRegional => types::experience_war_regional::SPEC,
            EventType::Custom => EventSpec::default(),
        }
    }

    /// Returns a human-readable name for this event type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            EventType::AchieveGoalMajor => "Achieve Goal (Major)",
            EventType::DevelopIllnessChronic => "Develop Illness (Chronic)",
            EventType::EndRelationshipRomantic => "End Relationship (Romantic)",
            EventType::EngageSelfharmNonsuicidal => "Engage Self-Harm (Non-Suicidal)",
            EventType::ExperienceAwarenessMortality => "Experience Awareness (Mortality)",
            EventType::ExperienceBetrayalTrust => "Experience Betrayal (Trust)",
            EventType::ExperienceCombatMilitary => "Experience Combat (Military)",
            EventType::ExperienceConflictFamily => "Experience Conflict (Family)",
            EventType::ExperienceConflictInterpersonal => "Experience Conflict (Interpersonal)",
            EventType::ExperienceExclusionGroup => "Experience Exclusion (Group)",
            EventType::ExperienceExclusionPeer => "Experience Exclusion (Peer)",
            EventType::FaceChargesLegal => "Face Charges (Legal)",
            EventType::FaceEvictionHousing => "Face Eviction (Housing)",
            EventType::FailGoalMajor => "Fail Goal (Major)",
            EventType::GainPowerPersonal => "Gain Power (Personal)",
            EventType::ExperienceHumiliationPublic => "Experience Humiliation (Public)",
            EventType::ExperienceInclusionPeer => "Experience Inclusion (Peer)",
            EventType::ExperienceIsolationChronic => "Experience Isolation (Chronic)",
            EventType::LoseAccessHealthcare => "Lose Access (Healthcare)",
            EventType::LoseBenefitsGovernment => "Lose Benefits (Government)",
            EventType::LoseJobFired => "Lose Job (Fired)",
            EventType::LoseJobLayoff => "Lose Job (Layoff)",
            EventType::LoseJobResigned => "Lose Job (Resigned)",
            EventType::LosePersonDeath => "Lose Person (Death)",
            EventType::ExperienceRejectionFamily => "Experience Rejection (Family)",
            EventType::ExperienceRejectionPeer => "Experience Rejection (Peer)",
            EventType::ExperienceRejectionRomantic => "Experience Rejection (Romantic)",
            EventType::ExperienceShamingPublic => "Experience Shaming (Public)",
            EventType::ExperienceStrainFinancial => "Experience Strain (Financial)",
            EventType::ExperienceWarRegional => "Experience War (Regional)",
            EventType::Custom => "Custom Event",
        }
    }

    /// Returns all event type variants (excluding Custom).
    #[must_use]
    pub const fn all() -> [EventType; 30] {
        [
            EventType::AchieveGoalMajor,
            EventType::DevelopIllnessChronic,
            EventType::EndRelationshipRomantic,
            EventType::EngageSelfharmNonsuicidal,
            EventType::ExperienceAwarenessMortality,
            EventType::ExperienceBetrayalTrust,
            EventType::ExperienceCombatMilitary,
            EventType::ExperienceConflictFamily,
            EventType::ExperienceConflictInterpersonal,
            EventType::ExperienceExclusionGroup,
            EventType::ExperienceExclusionPeer,
            EventType::FaceChargesLegal,
            EventType::FaceEvictionHousing,
            EventType::FailGoalMajor,
            EventType::GainPowerPersonal,
            EventType::ExperienceHumiliationPublic,
            EventType::ExperienceInclusionPeer,
            EventType::ExperienceIsolationChronic,
            EventType::LoseAccessHealthcare,
            EventType::LoseBenefitsGovernment,
            EventType::LoseJobFired,
            EventType::LoseJobLayoff,
            EventType::LoseJobResigned,
            EventType::LosePersonDeath,
            EventType::ExperienceRejectionFamily,
            EventType::ExperienceRejectionPeer,
            EventType::ExperienceRejectionRomantic,
            EventType::ExperienceShamingPublic,
            EventType::ExperienceStrainFinancial,
            EventType::ExperienceWarRegional,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_type_spec_returns_correct_spec() {
        let spec = EventType::EndRelationshipRomantic.spec();
        assert!(spec.impact.valence < 0.0);
        assert!(spec.impact.loneliness > 0.0);
    }

    #[test]
    fn event_type_all_returns_all_variants() {
        let all = EventType::all();
        assert_eq!(all.len(), 30);
    }

    #[test]
    fn event_type_names_not_empty() {
        for event_type in EventType::all() {
            assert!(!event_type.name().is_empty());
        }
    }

    #[test]
    fn event_type_custom_returns_default_spec() {
        let spec = EventType::Custom.spec();
        assert!((spec.impact.valence - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn event_type_is_copy() {
        let t1 = EventType::AchieveGoalMajor;
        let t2 = t1; // Copy
        assert_eq!(t1, t2);
    }

    #[test]
    fn event_type_is_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(EventType::AchieveGoalMajor);
        set.insert(EventType::AchieveGoalMajor);
        assert_eq!(set.len(), 1);

        set.insert(EventType::EndRelationshipRomantic);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn event_type_debug_format() {
        let event_type = EventType::EndRelationshipRomantic;
        let debug = format!("{:?}", event_type);
        assert!(debug.contains("EndRelationshipRomantic"));
    }

    #[test]
    fn all_event_types_have_spec() {
        // All event types should be able to call spec() without panicking
        for event_type in EventType::all() {
            let _ = event_type.spec();
        }
    }

    #[test]
    fn achieve_goal_major_has_positive_valence() {
        let spec = EventType::AchieveGoalMajor.spec();
        assert!(spec.impact.valence > 0.0);
    }

    #[test]
    fn combat_exposure_has_high_acquired_capability() {
        let spec = EventType::ExperienceCombatMilitary.spec();
        assert!(spec.impact.acquired_capability > 0.5);
    }

    #[test]
    fn self_harm_increases_acquired_capability() {
        let spec = EventType::EngageSelfharmNonsuicidal.spec();
        assert!(spec.impact.acquired_capability > 0.0);
    }
}
