//! Integration test: Memory priming persists after event deltas decay
//!
//! This test verifies that the memory priming mechanism continues to influence
//! mood even after direct event deltas have fully decayed. This isolates the
//! memory system's contribution to long-term psychological effects.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 18;
const TRAUMA_AGE: u64 = 19;
const QUERY_AGE: u64 = 69;

fn setup_sim(id: &str, birth_date: Timestamp, reference: Timestamp) -> (Simulation, EntityId) {
    let mut sim = Simulation::new(reference);
    let entity = EntityBuilder::new()
        .id(id)
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .build()
        .unwrap();

    sim.add_entity(entity, reference);
    let entity_id = EntityId::new(id).unwrap();
    (sim, entity_id)
}

fn add_events(
    sim: &mut Simulation,
    birth_date: Timestamp,
    target: &EntityId,
    events: &[(u64, EventType, f64)],
) {
    for (age_years, event_type, severity) in events {
        let event = EventBuilder::new(*event_type)
            .target(target.clone())
            .severity(*severity)
            .build()
            .unwrap();

        let event_time = birth_date + Duration::years(*age_years);
        sim.add_event(event, event_time);
    }
}

fn state_at_age(
    sim: &Simulation,
    entity_id: &EntityId,
    birth_date: Timestamp,
    age: u64,
) -> ComputedState {
    let handle = sim.entity(entity_id).unwrap();
    let time = birth_date + Duration::years(age);
    handle.state_at(time)
}

#[test]
fn memory_priming_persists_after_decay() {
    let birth_date = Timestamp::from_ymd_hms(1949, 1, 1, 0, 0, 0);
    let reference = birth_date + Duration::years(ANCHOR_AGE);

    let (mut test_sim, test_id) = setup_sim("test", birth_date, reference);
    let (control_sim, control_id) = setup_sim("control", birth_date, reference);

    // Add trauma events that create high-salience memories
    let trauma_events = [
        (TRAUMA_AGE, EventType::ExperienceCombatMilitary, 0.95),
        (TRAUMA_AGE, EventType::ExperienceAwarenessMortality, 0.9),
        (20, EventType::ExperienceCombatMilitary, 0.9),
        (20, EventType::SufferInjuryAccidental, 0.85),
    ];

    add_events(&mut test_sim, birth_date, &test_id, &trauma_events);

    // Query 50 years later - deltas have fully decayed
    let test_state = state_at_age(&test_sim, &test_id, birth_date, QUERY_AGE);
    let control_state = state_at_age(&control_sim, &control_id, birth_date, QUERY_AGE);

    let test_arousal = test_state.individual_state().mood().arousal_effective();
    let control_arousal = control_state.individual_state().mood().arousal_effective();

    // Memory priming should still contribute to elevated arousal
    assert!(
        test_arousal > control_arousal,
        "After 50 years, test arousal ({}) should exceed control ({}) due to memory priming",
        test_arousal,
        control_arousal
    );

    let test_valence = test_state.individual_state().mood().valence_effective();
    let control_valence = control_state.individual_state().mood().valence_effective();

    // Negative memories should prime negative valence
    assert!(
        test_valence < control_valence,
        "After 50 years, test valence ({}) should be lower than control ({}) due to memory priming",
        test_valence,
        control_valence
    );
}
