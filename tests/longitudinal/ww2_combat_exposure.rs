//! Longitudinal scenario test using only the public API.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 18;
const END_AGE: u64 = 65;

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
            .timestamp(Duration::years(*age_years))
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
#[ignore]
fn ww2_combat_exposure_shifts_aggression_and_stress() {
    let birth_date = Timestamp::from_ymd_hms(1920, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(1938, 1, 1, 0, 0, 0);

    let (mut sim, entity_id) = setup_sim("combat", birth_date, reference);
    let (control_sim, control_id) = setup_sim("combat_control", birth_date, reference);

    let events = [
        (22, EventType::ExperienceCombatMilitary, 0.9),
        (23, EventType::ExperienceWarRegional, 0.8),
        (24, EventType::ExperienceAwarenessMortality, 0.7),
        (25, EventType::ExperienceIsolationChronic, 0.6),
        (27, EventType::ExperienceConflictInterpersonal, 0.6),
    ];

    add_events(&mut sim, birth_date, &entity_id, &events);

    let early_state = state_at_age(&sim, &entity_id, birth_date, 23);
    let early_control_state = state_at_age(&control_sim, &control_id, birth_date, 23);
    let state = state_at_age(&sim, &entity_id, birth_date, END_AGE);
    let control_state = state_at_age(&control_sim, &control_id, birth_date, END_AGE);

    let early_needs = early_state.individual_state().needs();
    let early_control_needs = early_control_state.individual_state().needs();
    let early_disposition = early_state.individual_state().disposition();
    let early_control_disposition = early_control_state.individual_state().disposition();
    let early_mh = early_state.individual_state().mental_health();
    let early_control_mh = early_control_state.individual_state().mental_health();

    let needs = state.individual_state().needs();
    let control_needs = control_state.individual_state().needs();
    let disposition = state.individual_state().disposition();
    let control_disposition = control_state.individual_state().disposition();
    let mh = state.individual_state().mental_health();
    let control_mh = control_state.individual_state().mental_health();

    assert!(
        early_disposition.aggression_effective()
            > early_control_disposition.aggression_effective() + 0.05,
        "combat exposure should elevate aggression soon after exposure"
    );
    assert!(
        early_needs.stress_effective() > early_control_needs.stress_effective() + 0.05,
        "combat exposure should elevate stress soon after exposure"
    );
    assert!(
        early_mh.depression_effective() > early_control_mh.depression_effective() + 0.05,
        "combat exposure should elevate depression soon after exposure"
    );

    assert!(
        disposition.aggression_effective() > control_disposition.aggression_effective() + 0.05,
        "combat exposure should elevate aggression long-term"
    );
    assert!(
        needs.stress_effective() > control_needs.stress_effective() + 0.05,
        "combat exposure should elevate stress long-term"
    );
    assert!(
        mh.depression_effective() > control_mh.depression_effective() + 0.05,
        "combat exposure should elevate depression long-term"
    );
    assert!(
        disposition.trust_propensity_effective()
            < control_disposition.trust_propensity_effective() - 0.05,
        "combat exposure should reduce trust propensity long-term"
    );
}
