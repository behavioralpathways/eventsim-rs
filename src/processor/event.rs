//! Event interpretation and application for entity state changes.
//!
//! This module provides functions for interpreting events based on
//! entity personality, applying state changes, and computing salience.
//! All event processing uses the EventSpec system for consistent
//! impact computation across all 22 psychological dimensions.

use crate::entity::Entity;
use crate::enums::{Attribution, AttributionStability, Direction, MoodPath, StatePath};
use crate::event::{compute_arousal_modulated_salience, AppliedDeltas, Event};
use crate::relationship::{Relationship, TrustAntecedent};
use crate::types::{EventId, Timestamp};

/// Interpretation of an event based on entity state and personality.
///
/// The interpretation determines how strongly the event affects different
/// psychological dimensions based on the entity's personality (HEXACO),
/// current state, and the event's properties.
#[derive(Debug, Clone)]
pub struct InterpretedEvent {
    /// Original event.
    pub event: Event,
    /// ID of the original event for reference.
    pub original_event: EventId,
    /// Computed attribution for this event.
    pub attribution: Attribution,
    /// Valence modifier based on event type and personality.
    pub valence_delta: f32,
    /// Arousal modifier based on event severity and emotionality.
    pub arousal_delta: f32,
    /// Dominance modifier for control-related events.
    pub dominance_delta: f32,
    /// Loneliness impact for belonging events.
    pub loneliness_delta: f32,
    /// Perceived reciprocal caring impact.
    pub prc_delta: f32,
    /// Perceived liability impact for burden events.
    pub perceived_liability_delta: f32,
    /// Self-hate impact.
    pub self_hate_delta: f32,
    /// Acquired capability impact for trauma events.
    pub acquired_capability_delta: f32,
    /// Interpersonal hopelessness impact.
    pub interpersonal_hopelessness_delta: f32,
    /// Computed salience for memory encoding.
    pub salience: f32,
    /// Perceived severity after personality modulation.
    pub perceived_severity: f64,
    /// Memory salience for encoding.
    pub memory_salience: f64,
    /// State changes to apply, as (path, delta) pairs.
    /// DEPRECATED: Use spec_deltas for new events.
    pub state_deltas: Vec<(StatePath, f64)>,
    /// Spec-based deltas for events with dedicated specs.
    /// Contains permanent (base), acute (delta), and chronic (chronic_delta) buckets.
    pub spec_deltas: Option<AppliedDeltas>,
}

impl InterpretedEvent {
    /// Creates a new interpreted event with all deltas scaled by a factor.
    ///
    /// This is used by developmental processing to apply plasticity and
    /// sensitive period multipliers to event impact.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scaling factor to apply to all deltas
    ///
    /// # Returns
    ///
    /// A new `InterpretedEvent` with scaled deltas.
    #[must_use]
    pub fn scaled_by(&self, factor: f64) -> Self {
        let factor_f32 = factor as f32;

        InterpretedEvent {
            event: self.event.clone(),
            original_event: self.original_event.clone(),
            attribution: self.attribution.clone(),
            valence_delta: self.valence_delta * factor_f32,
            arousal_delta: self.arousal_delta * factor_f32,
            dominance_delta: self.dominance_delta * factor_f32,
            loneliness_delta: self.loneliness_delta * factor_f32,
            prc_delta: self.prc_delta * factor_f32,
            perceived_liability_delta: self.perceived_liability_delta * factor_f32,
            self_hate_delta: self.self_hate_delta * factor_f32,
            acquired_capability_delta: self.acquired_capability_delta * factor_f32,
            interpersonal_hopelessness_delta: self.interpersonal_hopelessness_delta * factor_f32,
            salience: self.salience, // Salience is not scaled
            perceived_severity: self.perceived_severity * factor,
            memory_salience: self.memory_salience, // Memory salience is not scaled
            state_deltas: self
                .state_deltas
                .iter()
                .map(|(path, delta)| (*path, delta * factor))
                .collect(),
            spec_deltas: self.spec_deltas.map(|deltas| AppliedDeltas {
                permanent: deltas.permanent.scale(factor_f32),
                acute: deltas.acute.scale(factor_f32),
                chronic: deltas.chronic.scale(factor_f32),
            }),
        }
    }
}

/// Interprets an event based on entity state and personality.
///
/// This function computes how an event should modify the entity's state
/// based on their personality traits (HEXACO), current emotional state,
/// and the event's properties. All events use the EventSpec system.
///
/// # Arguments
///
/// * `event` - The event to interpret
/// * `entity` - The entity interpreting the event
///
/// # Returns
///
/// An interpreted event with computed deltas for each state dimension
#[must_use]
pub(crate) fn interpret_event(event: &Event, entity: &Entity) -> InterpretedEvent {
    // Get personality for modulation
    let hexaco = entity.individual_state().hexaco();
    let emotionality = hexaco.emotionality();
    let honesty_humility = hexaco.honesty_humility();

    // Get current arousal for salience computation
    let current_arousal = entity
        .get_effective(StatePath::Mood(MoodPath::Arousal))
        .unwrap_or(0.0) as f32;

    let severity = event.severity() as f32;

    // Get the spec and apply it at the given severity
    let spec = event.spec();
    let applied_deltas = spec.apply(severity);

    // Extract key deltas for InterpretedEvent fields (sum of all buckets for display)
    let total_valence = applied_deltas.permanent.valence
        + applied_deltas.acute.valence
        + applied_deltas.chronic.valence;
    let total_arousal = applied_deltas.permanent.arousal
        + applied_deltas.acute.arousal
        + applied_deltas.chronic.arousal;
    let total_dominance = applied_deltas.permanent.dominance
        + applied_deltas.acute.dominance
        + applied_deltas.chronic.dominance;
    let total_loneliness = applied_deltas.permanent.loneliness
        + applied_deltas.acute.loneliness
        + applied_deltas.chronic.loneliness;
    let total_prc =
        applied_deltas.permanent.prc + applied_deltas.acute.prc + applied_deltas.chronic.prc;
    let total_perceived_liability = applied_deltas.permanent.perceived_liability
        + applied_deltas.acute.perceived_liability
        + applied_deltas.chronic.perceived_liability;
    let total_self_hate = applied_deltas.permanent.self_hate
        + applied_deltas.acute.self_hate
        + applied_deltas.chronic.self_hate;
    let total_ac = applied_deltas.permanent.acquired_capability
        + applied_deltas.acute.acquired_capability
        + applied_deltas.chronic.acquired_capability;
    let total_interpersonal_hopelessness = applied_deltas.permanent.interpersonal_hopelessness
        + applied_deltas.acute.interpersonal_hopelessness
        + applied_deltas.chronic.interpersonal_hopelessness;

    // Apply emotionality modulation to affect dimensions
    let emotionality_factor = 1.0 + (emotionality * 0.3);
    let modulated_valence = total_valence * emotionality_factor;
    let modulated_arousal = total_arousal * emotionality_factor;

    // Compute attribution (simplified model)
    let attribution = compute_attribution(event, honesty_humility);

    // Determine if this is a trauma event (AC > 0 means trauma)
    let is_trauma = total_ac > 0.0;

    // Compute salience with arousal modulation
    let base_salience = compute_base_salience(event);
    let salience = compute_arousal_modulated_salience(
        base_salience,
        current_arousal + modulated_arousal,
        modulated_valence,
        is_trauma,
        entity.species(),
    );

    // Compute perceived severity (modulated by emotionality)
    let perceived_severity = (severity * emotionality_factor) as f64;

    InterpretedEvent {
        event: event.clone(),
        original_event: event.id().clone(),
        attribution,
        valence_delta: modulated_valence,
        arousal_delta: modulated_arousal,
        dominance_delta: total_dominance,
        loneliness_delta: total_loneliness,
        prc_delta: total_prc,
        perceived_liability_delta: total_perceived_liability,
        self_hate_delta: total_self_hate,
        acquired_capability_delta: total_ac,
        interpersonal_hopelessness_delta: total_interpersonal_hopelessness,
        salience,
        perceived_severity,
        memory_salience: salience as f64,
        state_deltas: Vec::new(), // Deprecated - use spec_deltas
        spec_deltas: Some(applied_deltas),
    }
}

/// Computes base salience from event properties.
fn compute_base_salience(event: &Event) -> f32 {
    let severity = event.severity() as f32;
    let spec = event.spec();

    // Events with high AC impact have boosted salience (trauma)
    let ac_boost = if spec.impact.acquired_capability > 0.5 {
        0.2
    } else if spec.impact.acquired_capability > 0.0 {
        0.1
    } else {
        0.0
    };

    // Events with strong social impact have boosted salience
    let social_boost = if spec.impact.loneliness.abs() > 0.3 || spec.impact.prc.abs() > 0.2 {
        0.1
    } else {
        0.0
    };

    (0.3 + severity * 0.5 + ac_boost + social_boost).clamp(0.0, 1.0)
}

/// Computes attribution based on event and personality.
fn compute_attribution(event: &Event, honesty_humility: f32) -> Attribution {
    // Simple model: higher honesty-humility = more internal attribution
    // Event source affects attribution
    if let Some(source) = event.source() {
        // There's a clear external cause
        let stability = if event.severity() > 0.7 {
            AttributionStability::Stable
        } else {
            AttributionStability::Unstable
        };
        return Attribution::Other(source.clone(), stability);
    }

    // No clear source - attribution based on personality
    let stability = if event.severity() > 0.7 {
        AttributionStability::Stable
    } else {
        AttributionStability::Unstable
    };

    // Higher honesty-humility = more likely to self-attribute
    if honesty_humility > 0.3 {
        Attribution::SelfCaused(stability)
    } else if honesty_humility < -0.3 {
        Attribution::Situational(stability)
    } else {
        Attribution::Unknown
    }
}

/// Applies an interpreted event to an entity, modifying their state.
///
/// This function uses the spec_deltas to apply changes to the entity's
/// state dimensions, routing to base, delta, or chronic_delta as appropriate.
///
/// # Arguments
///
/// * `interpreted` - The interpreted event with computed deltas
/// * `entity` - The entity to modify
#[cfg(test)]
pub(crate) fn apply_interpreted_event(interpreted: &InterpretedEvent, entity: &mut Entity) {
    if let Some(deltas) = &interpreted.spec_deltas {
        // Apply permanent changes to base
        apply_permanent_deltas(deltas, entity);

        // Apply acute changes to delta
        apply_acute_deltas(deltas, entity);

        // Apply chronic changes to chronic_delta
        apply_chronic_deltas(deltas, entity);
    }
}

/// Applies permanent (base-shifting) deltas from an event.
#[cfg(test)]
fn apply_permanent_deltas(deltas: &AppliedDeltas, entity: &mut Entity) {
    let p = &deltas.permanent;
    let state = entity.individual_state_mut();

    if p.valence.abs() > f32::EPSILON {
        state.mood_mut().shift_valence_base(p.valence);
    }
    if p.arousal.abs() > f32::EPSILON {
        state.mood_mut().shift_arousal_base(p.arousal);
    }
    if p.dominance.abs() > f32::EPSILON {
        state.mood_mut().shift_dominance_base(p.dominance);
    }
    if p.loneliness.abs() > f32::EPSILON {
        state.social_cognition_mut().loneliness_mut().shift_base(p.loneliness);
    }
    if p.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .shift_base(p.prc);
    }
    if p.perceived_liability.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_liability_mut()
            .shift_base(p.perceived_liability);
    }
    if p.self_hate.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .self_hate_mut()
            .shift_base(p.self_hate);
    }
    if p.purpose.abs() > f32::EPSILON {
        state.needs_mut().purpose_mut().shift_base(p.purpose);
    }
    // AC is always 100% permanent - apply to base
    if p.acquired_capability.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .shift_acquired_capability_base(p.acquired_capability);
    }
    if p.interpersonal_hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .shift_interpersonal_hopelessness_base(p.interpersonal_hopelessness);
    }
    if p.self_worth.abs() > f32::EPSILON {
        state.mental_health_mut().shift_self_worth_base(p.self_worth);
    }
    if p.stress.abs() > f32::EPSILON {
        state.needs_mut().stress_mut().shift_base(p.stress);
    }
    if p.fatigue.abs() > f32::EPSILON {
        state.needs_mut().fatigue_mut().shift_base(p.fatigue);
    }
    if p.grievance.abs() > f32::EPSILON {
        state.disposition_mut().grievance_mut().shift_base(p.grievance);
    }
}

/// Applies acute (normal delta) changes from an event.
#[cfg(test)]
fn apply_acute_deltas(deltas: &AppliedDeltas, entity: &mut Entity) {
    let a = &deltas.acute;
    let state = entity.individual_state_mut();

    if a.valence.abs() > f32::EPSILON {
        state.mood_mut().add_valence_delta(a.valence);
    }
    if a.arousal.abs() > f32::EPSILON {
        state.mood_mut().add_arousal_delta(a.arousal);
    }
    if a.dominance.abs() > f32::EPSILON {
        state.mood_mut().add_dominance_delta(a.dominance);
    }
    if a.loneliness.abs() > f32::EPSILON {
        state.social_cognition_mut().add_loneliness_delta(a.loneliness);
    }
    if a.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_reciprocal_caring_delta(a.prc);
    }
    if a.perceived_liability.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_liability_delta(a.perceived_liability);
    }
    if a.self_hate.abs() > f32::EPSILON {
        state.social_cognition_mut().add_self_hate_delta(a.self_hate);
    }
    if a.purpose.abs() > f32::EPSILON {
        state.needs_mut().add_purpose_delta(a.purpose);
    }
    // AC never goes to acute - it's always permanent
    if a.interpersonal_hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_interpersonal_hopelessness_delta(a.interpersonal_hopelessness);
    }
    if a.self_worth.abs() > f32::EPSILON {
        state.mental_health_mut().add_self_worth_delta(a.self_worth);
    }
    if a.stress.abs() > f32::EPSILON {
        state.needs_mut().add_stress_delta(a.stress);
    }
    if a.fatigue.abs() > f32::EPSILON {
        state.needs_mut().add_fatigue_delta(a.fatigue);
    }
    if a.grievance.abs() > f32::EPSILON {
        state.disposition_mut().add_grievance_delta(a.grievance);
    }
}

/// Applies chronic (slow-decay) changes from an event.
#[cfg(test)]
fn apply_chronic_deltas(deltas: &AppliedDeltas, entity: &mut Entity) {
    let c = &deltas.chronic;
    let state = entity.individual_state_mut();

    if c.valence.abs() > f32::EPSILON {
        state.mood_mut().add_valence_chronic_delta(c.valence);
    }
    if c.arousal.abs() > f32::EPSILON {
        state.mood_mut().add_arousal_chronic_delta(c.arousal);
    }
    if c.dominance.abs() > f32::EPSILON {
        state.mood_mut().add_dominance_chronic_delta(c.dominance);
    }
    if c.loneliness.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .loneliness_mut()
            .add_chronic_delta(c.loneliness);
    }
    if c.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .add_chronic_delta(c.prc);
    }
    if c.perceived_liability.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_liability_mut()
            .add_chronic_delta(c.perceived_liability);
    }
    if c.self_hate.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .self_hate_mut()
            .add_chronic_delta(c.self_hate);
    }
    if c.purpose.abs() > f32::EPSILON {
        state.needs_mut().purpose_mut().add_chronic_delta(c.purpose);
    }
    // AC never goes to chronic - it's always permanent
    if c.interpersonal_hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_interpersonal_hopelessness_chronic_delta(c.interpersonal_hopelessness);
    }
    if c.self_worth.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_self_worth_chronic_delta(c.self_worth);
    }
    if c.stress.abs() > f32::EPSILON {
        state.needs_mut().stress_mut().add_chronic_delta(c.stress);
    }
    if c.fatigue.abs() > f32::EPSILON {
        state.needs_mut().fatigue_mut().add_chronic_delta(c.fatigue);
    }
    if c.grievance.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .grievance_mut()
            .add_chronic_delta(c.grievance);
    }
}

/// Processes an event into trust antecedents for related relationships.
///
/// For events with a source and target, this updates the target's
/// trustworthiness perceptions of the source based on the event type
/// and its spec.
pub(crate) fn process_event_to_relationships(
    event: &Event,
    timestamp: Timestamp,
    relationships: &mut [Relationship],
) {
    let (Some(source), Some(target)) = (event.source(), event.target()) else {
        return;
    };

    let spec = event.spec();
    let severity = event.severity() as f32;

    // Determine trust impact from the event's spec
    // Trust propensity in spec indicates general trust effects
    // Negative trust_propensity = breach of trust
    // Positive trust_propensity = trust building
    let trust_impact = spec.impact.trust_propensity;
    if trust_impact.abs() < f32::EPSILON {
        return;
    }

    for relationship in relationships.iter_mut() {
        let Some(direction) = direction_for_relationship(relationship, target, source) else {
            continue;
        };

        let consistency = relationship.pattern().consistency.clamp(0.0, 1.0);
        let consistency_weight = 0.5 + (consistency * 0.5);

        // Determine antecedent type based on event characteristics
        let (antecedent_type, ant_direction) = if trust_impact < 0.0 {
            // Negative trust impact - breach
            use crate::relationship::{AntecedentDirection, AntecedentType};
            if spec.impact.prc < -0.1 {
                // Affects caring perception - benevolence breach
                (AntecedentType::Benevolence, AntecedentDirection::Negative)
            } else {
                // General breach - integrity
                (AntecedentType::Integrity, AntecedentDirection::Negative)
            }
        } else {
            // Positive trust impact - building
            use crate::relationship::{AntecedentDirection, AntecedentType};
            if spec.impact.perceived_competence > 0.1 {
                // Demonstrates competence
                (AntecedentType::Ability, AntecedentDirection::Positive)
            } else {
                // General trust building - benevolence
                (AntecedentType::Benevolence, AntecedentDirection::Positive)
            }
        };

        let raw_magnitude = (trust_impact.abs() * severity).clamp(0.0, 1.0);
        let magnitude = raw_magnitude * consistency_weight;
        if magnitude <= 0.0 {
            continue;
        }

        let context = event.event_type().name();
        let antecedent = TrustAntecedent::new(
            timestamp,
            antecedent_type,
            ant_direction,
            magnitude,
            context,
        );
        relationship.append_antecedent(direction, antecedent);

        let history = relationship.antecedent_history(direction).to_vec();
        relationship
            .trustworthiness_mut(direction)
            .recompute_from_antecedents(&history);
    }
}

fn direction_for_relationship(
    relationship: &Relationship,
    trustor: &crate::types::EntityId,
    trustee: &crate::types::EntityId,
) -> Option<Direction> {
    if relationship.entity_a() == trustor && relationship.entity_b() == trustee {
        Some(Direction::AToB)
    } else if relationship.entity_b() == trustor && relationship.entity_a() == trustee {
        Some(Direction::BToA)
    } else {
        None
    }
}

/// Processes an event completely: interprets and applies it.
///
/// This is a convenience function that combines interpretation and
/// application in a single call.
///
/// # Arguments
///
/// * `event` - The event to process
/// * `entity` - The entity to modify
///
/// # Returns
///
/// The interpreted event for logging/debugging
#[cfg(test)]
pub(crate) fn process_event(event: &Event, entity: &mut Entity) -> InterpretedEvent {
    let interpreted = interpret_event(event, entity);
    apply_interpreted_event(&interpreted, entity);
    interpreted
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{EventType, Species};
    use crate::event::EventBuilder;

    fn create_human() -> Entity {
        EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap()
    }

    #[test]
    fn interpret_event_returns_spec_deltas() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.spec_deltas.is_some());
        let deltas = interpreted.spec_deltas.unwrap();
        // Breakup has negative valence
        assert!(
            deltas.permanent.valence < 0.0
                || deltas.acute.valence < 0.0
                || deltas.chronic.valence < 0.0
        );
    }

    #[test]
    fn interpret_breakup_has_negative_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta < 0.0);
    }

    #[test]
    fn interpret_achievement_has_positive_valence() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta > 0.0);
    }

    #[test]
    fn interpret_combat_increases_ac() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.acquired_capability_delta > 0.0);
        // AC should be in permanent bucket
        let deltas = interpreted.spec_deltas.unwrap();
        assert!(deltas.permanent.acquired_capability > 0.0);
    }

    #[test]
    fn interpret_betrayal_has_negative_prc() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::ExperienceBetrayalTrust)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.prc_delta < 0.0);
    }

    #[test]
    fn interpret_self_harm_increases_ac() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EngageSelfharmNonsuicidal)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.acquired_capability_delta > 0.0);
    }

    #[test]
    fn interpret_chronic_illness_has_chronic_flags() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::DevelopIllnessChronic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let deltas = interpreted.spec_deltas.unwrap();

        // Chronic illness should route some impacts to chronic bucket
        let has_chronic = deltas.chronic.stress.abs() > f32::EPSILON
            || deltas.chronic.fatigue.abs() > f32::EPSILON
            || deltas.chronic.self_worth.abs() > f32::EPSILON;
        assert!(has_chronic);
    }

    #[test]
    fn interpret_mortality_awareness_affects_hopelessness() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::ExperienceAwarenessMortality)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        // Mortality awareness should affect either hopelessness or interpersonal hopelessness
        let deltas = interpreted.spec_deltas.unwrap();
        let has_hopelessness_impact = deltas.permanent.hopelessness.abs() > f32::EPSILON
            || deltas.acute.hopelessness.abs() > f32::EPSILON
            || deltas.chronic.hopelessness.abs() > f32::EPSILON
            || deltas.permanent.interpersonal_hopelessness.abs() > f32::EPSILON
            || deltas.acute.interpersonal_hopelessness.abs() > f32::EPSILON
            || deltas.chronic.interpersonal_hopelessness.abs() > f32::EPSILON;
        assert!(has_hopelessness_impact || interpreted.interpersonal_hopelessness_delta.abs() > 0.0);
    }

    #[test]
    fn scaled_by_scales_all_deltas() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let scaled = interpreted.scaled_by(0.5);

        // Check that valence was scaled
        let original_valence = interpreted.valence_delta.abs();
        let scaled_valence = scaled.valence_delta.abs();
        assert!((scaled_valence - original_valence * 0.5).abs() < 0.01);

        // Check spec_deltas were scaled
        if let (Some(orig), Some(sc)) = (interpreted.spec_deltas, scaled.spec_deltas) {
            let orig_perm_v = orig.permanent.valence.abs();
            let sc_perm_v = sc.permanent.valence.abs();
            assert!((sc_perm_v - orig_perm_v * 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn process_event_applies_deltas_to_entity() {
        let mut entity = create_human();
        let initial_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence)).unwrap_or(0.0);

        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        process_event(&event, &mut entity);

        let final_valence = entity.get_effective(StatePath::Mood(MoodPath::Valence)).unwrap_or(0.0);
        assert!(final_valence < initial_valence);
    }

    #[test]
    fn process_combat_event_increases_ac_base() {
        let mut entity = create_human();
        let initial_ac = entity
            .individual_state()
            .mental_health()
            .acquired_capability()
            .base();

        let event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.9)
            .build()
            .unwrap();

        process_event(&event, &mut entity);

        let final_ac = entity
            .individual_state()
            .mental_health()
            .acquired_capability()
            .base();
        assert!(final_ac > initial_ac);
    }

    #[test]
    fn attribution_with_source_attributes_to_other() {
        let entity = create_human();
        let source = crate::types::EntityId::new("perpetrator").unwrap();
        let event = EventBuilder::new(EventType::ExperienceBetrayalTrust)
            .source(source.clone())
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);

        match &interpreted.attribution {
            Attribution::Other(id, _) => assert_eq!(id, &source),
            _ => panic!("Expected Other attribution"),
        }
    }

    #[test]
    fn base_salience_increases_with_severity() {
        let low_event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.3)
            .build()
            .unwrap();
        let high_event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.9)
            .build()
            .unwrap();

        let low_salience = compute_base_salience(&low_event);
        let high_salience = compute_base_salience(&high_event);

        assert!(high_salience > low_salience);
    }

    #[test]
    fn trauma_events_have_higher_salience() {
        let trauma_event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.7)
            .build()
            .unwrap();
        let non_trauma_event = EventBuilder::new(EventType::AchieveGoalMajor)
            .severity(0.7)
            .build()
            .unwrap();

        let trauma_salience = compute_base_salience(&trauma_event);
        let non_trauma_salience = compute_base_salience(&non_trauma_event);

        assert!(trauma_salience > non_trauma_salience);
    }

    #[test]
    fn custom_event_with_spec_processes_correctly() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let entity = create_human();
        let custom_spec = EventSpec {
            impact: EventImpact {
                valence: 0.5,
                arousal: 0.3,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(1.0).build().unwrap();

        let interpreted = interpret_event(&event, &entity);

        assert!(interpreted.valence_delta > 0.0);
        assert!(interpreted.arousal_delta > 0.0);
    }

    #[test]
    fn all_event_types_can_be_interpreted() {
        let entity = create_human();

        for event_type in EventType::all() {
            let event = EventBuilder::new(event_type).severity(0.5).build().unwrap();

            let interpreted = interpret_event(&event, &entity);

            assert!(interpreted.spec_deltas.is_some());
        }
    }

    #[test]
    fn direction_for_relationship_returns_correct_direction() {
        use crate::types::EntityId;

        let a = EntityId::new("entity_a").unwrap();
        let b = EntityId::new("entity_b").unwrap();
        let c = EntityId::new("entity_c").unwrap();

        let rel = Relationship::try_between(a.clone(), b.clone()).unwrap();

        // A is trustor, B is trustee -> AToB
        assert_eq!(direction_for_relationship(&rel, &a, &b), Some(Direction::AToB));

        // B is trustor, A is trustee -> BToA
        assert_eq!(direction_for_relationship(&rel, &b, &a), Some(Direction::BToA));

        // C is not in relationship -> None
        assert_eq!(direction_for_relationship(&rel, &c, &a), None);
    }

    #[test]
    fn apply_all_permanent_delta_branches() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let mut entity = create_human();
        let custom_spec = EventSpec {
            impact: EventImpact {
                valence: 0.1,
                arousal: 0.1,
                dominance: 0.1,
                loneliness: 0.1,
                prc: 0.1,
                perceived_liability: 0.1,
                self_hate: 0.1,
                purpose: 0.1,
                acquired_capability: 0.1, // AC is always 100% permanent
                interpersonal_hopelessness: 0.1,
                self_worth: 0.1,
                stress: 0.1,
                fatigue: 0.1,
                grievance: 0.1,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues {
                valence: 1.0,
                arousal: 1.0,
                dominance: 1.0,
                loneliness: 1.0,
                prc: 1.0,
                perceived_liability: 1.0,
                self_hate: 1.0,
                purpose: 1.0,
                // AC has no permanence field - it's always 100% permanent
                interpersonal_hopelessness: 1.0,
                self_worth: 1.0,
                stress: 1.0,
                fatigue: 1.0,
                grievance: 1.0,
                ..Default::default()
            },
        };

        let event = EventBuilder::custom(custom_spec).severity(1.0).build().unwrap();
        process_event(&event, &mut entity);

        // All permanent dimensions should have been modified
        let state = entity.individual_state();
        assert!(state.mood().valence_base().abs() > f32::EPSILON);
        assert!(state.mental_health().acquired_capability().base().abs() > f32::EPSILON);
    }

    #[test]
    fn apply_all_acute_delta_branches() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let mut entity = create_human();
        // Create spec with no permanence (all acute)
        let custom_spec = EventSpec {
            impact: EventImpact {
                valence: 0.2,
                arousal: 0.2,
                dominance: 0.2,
                loneliness: 0.2,
                prc: 0.2,
                perceived_liability: 0.2,
                self_hate: 0.2,
                purpose: 0.2,
                interpersonal_hopelessness: 0.2,
                self_worth: 0.2,
                stress: 0.2,
                fatigue: 0.2,
                grievance: 0.2,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(), // 0.0 permanence = all acute
        };

        let event = EventBuilder::custom(custom_spec).severity(1.0).build().unwrap();
        process_event(&event, &mut entity);

        // Acute deltas should have been applied
        let state = entity.individual_state();
        assert!(state.mood().valence_delta().abs() > f32::EPSILON);
    }

    #[test]
    fn apply_all_chronic_delta_branches() {
        use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

        let mut entity = create_human();
        // Create spec with chronic flags set
        let custom_spec = EventSpec {
            impact: EventImpact {
                valence: 0.3,
                arousal: 0.3,
                dominance: 0.3,
                loneliness: 0.3,
                prc: 0.3,
                perceived_liability: 0.3,
                self_hate: 0.3,
                purpose: 0.3,
                interpersonal_hopelessness: 0.3,
                self_worth: 0.3,
                stress: 0.3,
                fatigue: 0.3,
                grievance: 0.3,
                ..Default::default()
            },
            chronic: ChronicFlags {
                valence: true,
                arousal: true,
                dominance: true,
                loneliness: true,
                prc: true,
                perceived_liability: true,
                self_hate: true,
                purpose: true,
                interpersonal_hopelessness: true,
                self_worth: true,
                stress: true,
                fatigue: true,
                grievance: true,
                ..Default::default()
            },
            permanence: PermanenceValues::default(),
        };

        let event = EventBuilder::custom(custom_spec).severity(1.0).build().unwrap();
        process_event(&event, &mut entity);

        // Chronic deltas should have been applied
        let state = entity.individual_state();
        // Chronic illness typically affects stress and fatigue chronically
        assert!(
            state.needs().stress().chronic_delta().abs() > f32::EPSILON
                || state.mood().valence().chronic_delta().abs() > f32::EPSILON
        );
    }

    #[test]
    fn scaled_by_with_state_deltas() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let mut interpreted = interpret_event(&event, &entity);
        // Add a state delta to test scaling of state_deltas vec
        interpreted.state_deltas.push((StatePath::Mood(MoodPath::Valence), -0.5));

        let scaled = interpreted.scaled_by(0.5);

        // Check state_deltas were scaled
        assert_eq!(scaled.state_deltas.len(), 1);
        assert!((scaled.state_deltas[0].1 - (-0.25)).abs() < 0.01);
    }
}
