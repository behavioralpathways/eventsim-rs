//! Longitudinal scenario test using only the public API.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 30;
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
fn chronic_isolation_is_worse_than_intermittent() {
    let birth_date = Timestamp::from_ymd_hms(1968, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(1998, 1, 1, 0, 0, 0);

    let (mut chronic_sim, chronic_id) = setup_sim("chronic_isolation", birth_date, reference);
    let (mut intermittent_sim, intermittent_id) = setup_sim("intermittent_isolation", birth_date, reference);

    let chronic_events = [
        (25, EventType::ExperienceIsolationChronic, 0.8),
        (30, EventType::ExperienceIsolationChronic, 0.8),
        (35, EventType::ExperienceIsolationChronic, 0.7),
        (40, EventType::ExperienceIsolationChronic, 0.7),
    ];

    let intermittent_events = [
        (25, EventType::ExperienceIsolationChronic, 0.7),
        (35, EventType::ExperienceInclusionPeer, 0.5),
        (45, EventType::ExperienceIsolationChronic, 0.6),
    ];

    add_events(&mut chronic_sim, birth_date, &chronic_id, &chronic_events);
    add_events(&mut intermittent_sim, birth_date, &intermittent_id, &intermittent_events);

    let chronic_state = state_at_age(&chronic_sim, &chronic_id, birth_date, END_AGE);
    let intermittent_state = state_at_age(&intermittent_sim, &intermittent_id, birth_date, END_AGE);

    let chronic_social = chronic_state.individual_state().social_cognition();
    let intermittent_social = intermittent_state.individual_state().social_cognition();
    let chronic_mh = chronic_state.individual_state().mental_health();
    let intermittent_mh = intermittent_state.individual_state().mental_health();

    assert!(
        chronic_social.loneliness_effective() > intermittent_social.loneliness_effective() + 0.05,
        "chronic isolation should exceed intermittent isolation in loneliness"
    );
    assert!(
        chronic_mh.depression_effective() > intermittent_mh.depression_effective() + 0.05,
        "chronic isolation should exceed intermittent isolation in depression"
    );
}
