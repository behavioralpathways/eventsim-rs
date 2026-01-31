//! State evolution functions for temporal state computation.
//!
//! This module provides pure functions for advancing and regressing
//! IndividualState over time, and for applying/reversing events.
//!
//! These functions are the core building blocks for the `state_at()` API.

use crate::state::IndividualState;
use crate::types::Duration;

/// Advances state forward in time by applying decay.
///
/// This is a pure function that returns a new state with decay applied.
/// The original state is not modified.
///
/// # Arguments
///
/// * `state` - The starting state
/// * `duration` - The time to advance
///
/// # Returns
///
/// A new `IndividualState` with decay applied.
#[must_use]
pub(crate) fn advance_state(state: IndividualState, duration: Duration) -> IndividualState {
    let mut new_state = state;
    new_state.apply_decay(duration);
    new_state
}

/// Regresses state backward in time by reversing decay.
///
/// This is a pure function that returns a new state with decay reversed.
/// The original state is not modified.
///
/// Note: Some dimensions (like Acquired Capability) cannot be regressed
/// because they have no decay. These dimensions remain unchanged.
///
/// # Arguments
///
/// * `state` - The current state
/// * `duration` - The time to regress
///
/// # Returns
///
/// A new `IndividualState` with decay reversed where possible.
#[must_use]
pub(crate) fn regress_state(state: IndividualState, duration: Duration) -> IndividualState {
    let mut new_state = state;

    // Reverse decay on all decayable dimensions
    // Note: This is an approximation - we reverse by applying the inverse of decay
    reverse_decay_on_state(&mut new_state, duration);

    new_state
}

/// Applies decay reversal to all reversible dimensions of a state.
fn reverse_decay_on_state(state: &mut IndividualState, duration: Duration) {
    // Mood dimensions have 6-hour half-life
    reverse_dimension_decay(state.mood_mut().valence_mut(), duration, Duration::hours(6));
    reverse_dimension_decay(state.mood_mut().arousal_mut(), duration, Duration::hours(6));
    reverse_dimension_decay(
        state.mood_mut().dominance_mut(),
        duration,
        Duration::hours(6),
    );

    // Social cognition dimensions have various half-lives
    reverse_dimension_decay(
        state.social_cognition_mut().loneliness_mut(),
        duration,
        Duration::days(1),
    );
    reverse_dimension_decay(
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut(),
        duration,
        Duration::days(2),
    );
    reverse_dimension_decay(
        state.social_cognition_mut().perceived_liability_mut(),
        duration,
        Duration::days(3),
    );
    reverse_dimension_decay(
        state.social_cognition_mut().self_hate_mut(),
        duration,
        Duration::days(3),
    );
    reverse_dimension_decay(
        state.needs_mut().stress_mut(),
        duration,
        Duration::hours(12),
    );
    reverse_dimension_decay(
        state.needs_mut().fatigue_mut(),
        duration,
        Duration::hours(8),
    );
    reverse_dimension_decay(state.needs_mut().purpose_mut(), duration, Duration::days(3));

    // Mental health dimensions (except AC which has no decay)
    reverse_dimension_decay(
        state.mental_health_mut().depression_mut(),
        duration,
        Duration::weeks(2),
    );
    reverse_dimension_decay(
        state.mental_health_mut().interpersonal_hopelessness_mut(),
        duration,
        Duration::weeks(2),
    );
    // Acquired Capability has no decay - cannot be reversed

    // Disposition dimensions
    reverse_dimension_decay(
        state.disposition_mut().empathy_mut(),
        duration,
        Duration::weeks(4),
    );
    reverse_dimension_decay(
        state.disposition_mut().aggression_mut(),
        duration,
        Duration::weeks(1),
    );
    reverse_dimension_decay(
        state.disposition_mut().grievance_mut(),
        duration,
        Duration::weeks(1),
    );

    // Person characteristics - social capital
    reverse_dimension_decay(
        state.person_characteristics_mut().social_capital_mut(),
        duration,
        Duration::weeks(4),
    );
}

/// Reverses decay on a single state value.
fn reverse_dimension_decay(
    state_value: &mut crate::state::StateValue,
    duration: Duration,
    half_life: Duration,
) {
    let current_delta = state_value.delta();

    // Skip if delta is effectively zero
    if current_delta.abs() < f32::EPSILON {
        return;
    }

    // Compute reversal factor: exp(ln(2) * t / half_life)
    let elapsed_ms = duration.as_millis() as f64;
    let half_life_ms = half_life.as_millis() as f64;

    if half_life_ms <= 0.0 {
        return;
    }

    let ln2 = std::f64::consts::LN_2;
    let exponent = ln2 * elapsed_ms / half_life_ms;

    // Guard against overflow
    if exponent > 700.0 {
        return;
    }

    let reversal_factor = exponent.exp();
    let original_delta = (current_delta as f64) * reversal_factor;

    // Clamp to reasonable range to avoid numerical issues
    let clamped = original_delta.clamp(-100.0, 100.0) as f32;
    state_value.set_delta(clamped);
}

/// Applies an interpreted event's effects to state using the spec-based deltas.
///
/// This function applies the deltas from an `InterpretedEvent` to an `IndividualState`.
/// It uses the spec_deltas (permanent, acute, chronic buckets) for proper routing.
///
/// # Arguments
///
/// * `state` - The current state
/// * `interpreted` - The interpreted event with computed deltas
///
/// # Returns
///
/// A new `IndividualState` with event effects applied.
#[must_use]
pub(crate) fn apply_interpreted_event_to_state(
    state: IndividualState,
    interpreted: &crate::processor::InterpretedEvent,
) -> IndividualState {
    let mut new_state = state;

    if let Some(deltas) = &interpreted.spec_deltas {
        // Apply permanent changes to base
        apply_permanent_deltas_to_state(&mut new_state, deltas);

        // Apply acute changes to delta
        apply_acute_deltas_to_state(&mut new_state, deltas);

        // Apply chronic changes to chronic_delta
        apply_chronic_deltas_to_state(&mut new_state, deltas);
    }

    new_state
}

/// Applies permanent (base-shifting) deltas from spec_deltas to state.
fn apply_permanent_deltas_to_state(
    state: &mut IndividualState,
    deltas: &crate::event::AppliedDeltas,
) {
    let p = &deltas.permanent;

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
        state
            .social_cognition_mut()
            .loneliness_mut()
            .shift_base(p.loneliness);
    }
    if p.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .shift_base(p.prc);
    }
    if p.perceived_competence.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_competence_mut()
            .shift_base(p.perceived_competence);
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
    if p.depression.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .depression_mut()
            .shift_base(p.depression);
    }
    if p.hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .hopelessness_mut()
            .shift_base(p.hopelessness);
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
        state
            .disposition_mut()
            .grievance_mut()
            .shift_base(p.grievance);
    }
    if p.impulse_control.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .impulse_control_mut()
            .shift_base(p.impulse_control);
    }
    if p.empathy.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .empathy_mut()
            .shift_base(p.empathy);
    }
    if p.aggression.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .aggression_mut()
            .shift_base(p.aggression);
    }
    if p.reactance.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .reactance_mut()
            .shift_base(p.reactance);
    }
    if p.trust_propensity.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .trust_propensity_mut()
            .shift_base(p.trust_propensity);
    }
}

/// Applies acute (normal delta) changes from spec_deltas to state.
fn apply_acute_deltas_to_state(state: &mut IndividualState, deltas: &crate::event::AppliedDeltas) {
    let a = &deltas.acute;

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
        state
            .social_cognition_mut()
            .add_loneliness_delta(a.loneliness);
    }
    if a.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_reciprocal_caring_delta(a.prc);
    }
    if a.perceived_competence.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_competence_delta(a.perceived_competence);
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
    if a.depression.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_depression_delta(a.depression);
    }
    if a.hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_hopelessness_delta(a.hopelessness);
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
    if a.impulse_control.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .add_impulse_control_delta(a.impulse_control);
    }
    if a.empathy.abs() > f32::EPSILON {
        state.disposition_mut().add_empathy_delta(a.empathy);
    }
    if a.aggression.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .add_aggression_delta(a.aggression);
    }
    if a.reactance.abs() > f32::EPSILON {
        state.disposition_mut().add_reactance_delta(a.reactance);
    }
    if a.trust_propensity.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .add_trust_propensity_delta(a.trust_propensity);
    }
}

/// Applies chronic (slow-decay) changes from spec_deltas to state.
fn apply_chronic_deltas_to_state(state: &mut IndividualState, deltas: &crate::event::AppliedDeltas) {
    let c = &deltas.chronic;

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
    if c.perceived_competence.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_competence_mut()
            .add_chronic_delta(c.perceived_competence);
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
    if c.depression.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .depression_mut()
            .add_chronic_delta(c.depression);
    }
    if c.hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .hopelessness_mut()
            .add_chronic_delta(c.hopelessness);
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
    if c.impulse_control.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .impulse_control_mut()
            .add_chronic_delta(c.impulse_control);
    }
    if c.empathy.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .empathy_mut()
            .add_chronic_delta(c.empathy);
    }
    if c.aggression.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .aggression_mut()
            .add_chronic_delta(c.aggression);
    }
    if c.reactance.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .reactance_mut()
            .add_chronic_delta(c.reactance);
    }
    if c.trust_propensity.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .trust_propensity_mut()
            .add_chronic_delta(c.trust_propensity);
    }
}

/// Reverses an interpreted event's effects from state using the spec-based deltas.
///
/// This function uses the spec_deltas from an `InterpretedEvent` to precisely reverse
/// the event's effects.
///
/// Note: Acquired Capability increases are NOT reversed (permanent per ITS theory).
/// Note: Permanent (base) changes are also NOT reversed as they represent lasting changes.
///
/// # Arguments
///
/// * `state` - The current state
/// * `interpreted` - The interpreted event with computed deltas
///
/// # Returns
///
/// A new `IndividualState` with event effects reversed (acute and chronic only).
#[must_use]
pub(crate) fn reverse_interpreted_event_from_state(
    state: IndividualState,
    interpreted: &crate::processor::InterpretedEvent,
) -> IndividualState {
    let mut new_state = state;

    if let Some(deltas) = &interpreted.spec_deltas {
        // Reverse acute changes (but NOT permanent or AC)
        reverse_acute_deltas_from_state(&mut new_state, deltas);

        // Reverse chronic changes (but NOT permanent or AC)
        reverse_chronic_deltas_from_state(&mut new_state, deltas);
    }

    new_state
}

/// Reverses acute (normal delta) changes from spec_deltas.
fn reverse_acute_deltas_from_state(
    state: &mut IndividualState,
    deltas: &crate::event::AppliedDeltas,
) {
    let a = &deltas.acute;

    if a.valence.abs() > f32::EPSILON {
        state.mood_mut().add_valence_delta(-a.valence);
    }
    if a.arousal.abs() > f32::EPSILON {
        state.mood_mut().add_arousal_delta(-a.arousal);
    }
    if a.dominance.abs() > f32::EPSILON {
        state.mood_mut().add_dominance_delta(-a.dominance);
    }
    if a.loneliness.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_loneliness_delta(-a.loneliness);
    }
    if a.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_reciprocal_caring_delta(-a.prc);
    }
    if a.perceived_competence.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_competence_delta(-a.perceived_competence);
    }
    if a.perceived_liability.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_perceived_liability_delta(-a.perceived_liability);
    }
    if a.self_hate.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .add_self_hate_delta(-a.self_hate);
    }
    if a.purpose.abs() > f32::EPSILON {
        state.needs_mut().add_purpose_delta(-a.purpose);
    }
    // AC is NOT reversed - it's permanent per ITS theory
    if a.interpersonal_hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_interpersonal_hopelessness_delta(-a.interpersonal_hopelessness);
    }
    if a.depression.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_depression_delta(-a.depression);
    }
    if a.hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_hopelessness_delta(-a.hopelessness);
    }
    if a.self_worth.abs() > f32::EPSILON {
        state.mental_health_mut().add_self_worth_delta(-a.self_worth);
    }
    if a.stress.abs() > f32::EPSILON {
        state.needs_mut().add_stress_delta(-a.stress);
    }
    if a.fatigue.abs() > f32::EPSILON {
        state.needs_mut().add_fatigue_delta(-a.fatigue);
    }
    if a.grievance.abs() > f32::EPSILON {
        state.disposition_mut().add_grievance_delta(-a.grievance);
    }
    if a.impulse_control.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .add_impulse_control_delta(-a.impulse_control);
    }
    if a.empathy.abs() > f32::EPSILON {
        state.disposition_mut().add_empathy_delta(-a.empathy);
    }
    if a.aggression.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .add_aggression_delta(-a.aggression);
    }
    if a.reactance.abs() > f32::EPSILON {
        state.disposition_mut().add_reactance_delta(-a.reactance);
    }
    if a.trust_propensity.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .add_trust_propensity_delta(-a.trust_propensity);
    }
}

/// Reverses chronic (slow-decay) changes from spec_deltas.
fn reverse_chronic_deltas_from_state(
    state: &mut IndividualState,
    deltas: &crate::event::AppliedDeltas,
) {
    let c = &deltas.chronic;

    if c.valence.abs() > f32::EPSILON {
        state.mood_mut().add_valence_chronic_delta(-c.valence);
    }
    if c.arousal.abs() > f32::EPSILON {
        state.mood_mut().add_arousal_chronic_delta(-c.arousal);
    }
    if c.dominance.abs() > f32::EPSILON {
        state.mood_mut().add_dominance_chronic_delta(-c.dominance);
    }
    if c.loneliness.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .loneliness_mut()
            .add_chronic_delta(-c.loneliness);
    }
    if c.prc.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .add_chronic_delta(-c.prc);
    }
    if c.perceived_competence.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_competence_mut()
            .add_chronic_delta(-c.perceived_competence);
    }
    if c.perceived_liability.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .perceived_liability_mut()
            .add_chronic_delta(-c.perceived_liability);
    }
    if c.self_hate.abs() > f32::EPSILON {
        state
            .social_cognition_mut()
            .self_hate_mut()
            .add_chronic_delta(-c.self_hate);
    }
    if c.purpose.abs() > f32::EPSILON {
        state.needs_mut().purpose_mut().add_chronic_delta(-c.purpose);
    }
    // AC never goes to chronic - it's always permanent
    if c.interpersonal_hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_interpersonal_hopelessness_chronic_delta(-c.interpersonal_hopelessness);
    }
    if c.depression.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .depression_mut()
            .add_chronic_delta(-c.depression);
    }
    if c.hopelessness.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .hopelessness_mut()
            .add_chronic_delta(-c.hopelessness);
    }
    if c.self_worth.abs() > f32::EPSILON {
        state
            .mental_health_mut()
            .add_self_worth_chronic_delta(-c.self_worth);
    }
    if c.stress.abs() > f32::EPSILON {
        state.needs_mut().stress_mut().add_chronic_delta(-c.stress);
    }
    if c.fatigue.abs() > f32::EPSILON {
        state.needs_mut().fatigue_mut().add_chronic_delta(-c.fatigue);
    }
    if c.grievance.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .grievance_mut()
            .add_chronic_delta(-c.grievance);
    }
    if c.impulse_control.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .impulse_control_mut()
            .add_chronic_delta(-c.impulse_control);
    }
    if c.empathy.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .empathy_mut()
            .add_chronic_delta(-c.empathy);
    }
    if c.aggression.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .aggression_mut()
            .add_chronic_delta(-c.aggression);
    }
    if c.reactance.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .reactance_mut()
            .add_chronic_delta(-c.reactance);
    }
    if c.trust_propensity.abs() > f32::EPSILON {
        state
            .disposition_mut()
            .trust_propensity_mut()
            .add_chronic_delta(-c.trust_propensity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{EventType, Species};
    use crate::event::EventBuilder;
    use crate::processor::interpret_event;

    #[test]
    fn advance_state_applies_decay() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.8);

        let advanced = advance_state(state, Duration::weeks(1));

        // After 1 week with 6-hour half-life, delta should be nearly zero
        assert!(advanced.mood().valence_delta() < 0.01);
    }

    #[test]
    fn advance_state_zero_duration_unchanged() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.5);

        let advanced = advance_state(state, Duration::zero());

        assert!((advanced.mood().valence_delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn regress_state_reverses_decay() {
        let mut state = IndividualState::new();
        // Set a small delta that would result from decay
        state.mood_mut().add_valence_delta(0.25); // Half of 0.5 after one half-life

        let regressed = regress_state(state, Duration::hours(6));

        // After reversing 6 hours (one half-life), delta should approximately double
        assert!(regressed.mood().valence_delta() > 0.4);
    }

    #[test]
    fn regress_state_zero_duration_unchanged() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.5);

        let regressed = regress_state(state, Duration::zero());

        assert!((regressed.mood().valence_delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_interpreted_event_to_state_breakup() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(crate::types::Duration::years(30))
            .build()
            .unwrap();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let state = IndividualState::new();
        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        // Breakup should decrease valence (either via delta or base)
        let valence_change = new_state.mood().valence_delta()
            + (new_state.mood().valence_base() - 0.0); // Base starts at 0
        assert!(valence_change < 0.0);
    }

    #[test]
    fn apply_interpreted_event_to_state_combat_increases_ac_base() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(crate::types::Duration::years(30))
            .build()
            .unwrap();
        let event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let state = IndividualState::new();
        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        // Combat should increase AC base (permanent)
        assert!(new_state.mental_health().acquired_capability().base() > 0.0);
    }

    #[test]
    fn apply_interpreted_event_to_state_achievement_positive() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(crate::types::Duration::years(30))
            .build()
            .unwrap();
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let state = IndividualState::new();
        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        // Achievement should have positive valence impact
        let valence_change = new_state.mood().valence_delta()
            + (new_state.mood().valence_base() - 0.0);
        assert!(valence_change > 0.0);
    }

    #[test]
    fn reverse_interpreted_event_reverses_acute_deltas() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(crate::types::Duration::years(30))
            .build()
            .unwrap();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let state = IndividualState::new();

        // Apply event
        let after_event = apply_interpreted_event_to_state(state.clone(), &interpreted);

        // Reverse event
        let reversed = reverse_interpreted_event_from_state(after_event.clone(), &interpreted);

        // Acute deltas should be reversed (delta should be back to near zero)
        // Note: permanent changes are NOT reversed
        assert!(reversed.mood().valence_delta().abs() < 0.01);
    }

    #[test]
    fn reverse_interpreted_event_does_not_reverse_ac() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(crate::types::Duration::years(30))
            .build()
            .unwrap();
        let event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let state = IndividualState::new();

        // Apply event
        let after_event = apply_interpreted_event_to_state(state, &interpreted);
        let ac_after = after_event.mental_health().acquired_capability().base();

        // Reverse event
        let reversed = reverse_interpreted_event_from_state(after_event, &interpreted);
        let ac_reversed = reversed.mental_health().acquired_capability().base();

        // AC should NOT be reversed - it's permanent
        assert!((ac_after - ac_reversed).abs() < f32::EPSILON);
    }

    #[test]
    fn reverse_dimension_decay_doubles_after_half_life() {
        let mut state = IndividualState::new();
        state.mood_mut().add_valence_delta(0.25);

        reverse_dimension_decay(state.mood_mut().valence_mut(), Duration::hours(6), Duration::hours(6));

        // After one half-life reversal, delta should approximately double
        assert!(state.mood().valence_delta() > 0.4);
        assert!(state.mood().valence_delta() < 0.6);
    }

    #[test]
    fn reverse_dimension_decay_zero_delta_unchanged() {
        let mut state = IndividualState::new();
        // Delta is zero by default

        reverse_dimension_decay(state.mood_mut().valence_mut(), Duration::hours(6), Duration::hours(6));

        assert!(state.mood().valence_delta().abs() < f32::EPSILON);
    }

    #[test]
    fn apply_chronic_deltas_routes_to_chronic_bucket() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(crate::types::Duration::years(30))
            .build()
            .unwrap();
        // Chronic illness should have chronic flags set
        let event = EventBuilder::new(EventType::DevelopIllnessChronic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let state = IndividualState::new();
        let new_state = apply_interpreted_event_to_state(state, &interpreted);

        // Some dimensions should have chronic delta (checking stress or fatigue)
        let has_chronic = new_state.needs().stress().chronic_delta().abs() > f32::EPSILON
            || new_state.needs().fatigue().chronic_delta().abs() > f32::EPSILON
            || new_state.mental_health().self_worth().chronic_delta().abs() > f32::EPSILON;

        // At least some chronic impact expected from chronic illness
        assert!(has_chronic || new_state.needs().stress().delta().abs() > f32::EPSILON);
    }
}
