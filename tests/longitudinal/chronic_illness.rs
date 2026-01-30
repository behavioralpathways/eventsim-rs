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
fn chronic_illness_increases_burden_and_hopelessness() {
    let birth_date = Timestamp::from_ymd_hms(1955, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(1985, 1, 1, 0, 0, 0);

    let (mut sim, entity_id) = setup_sim("illness", birth_date, reference);
    let (control_sim, control_id) = setup_sim("illness_control", birth_date, reference);

    let events = [
        (42, EventType::DevelopIllnessChronic, 0.9),
        (45, EventType::ExperienceAwarenessMortality, 0.7),
        (52, EventType::DevelopIllnessChronic, 0.8),
        (58, EventType::ExperienceIsolationChronic, 0.6),
    ];

    add_events(&mut sim, birth_date, &entity_id, &events);

    let state = state_at_age(&sim, &entity_id, birth_date, END_AGE);
    let control_state = state_at_age(&control_sim, &control_id, birth_date, END_AGE);

    let needs = state.individual_state().needs();
    let control_needs = control_state.individual_state().needs();
    let mh = state.individual_state().mental_health();
    let control_mh = control_state.individual_state().mental_health();

    assert!(
        needs.fatigue_effective() > control_needs.fatigue_effective() + 0.05,
        "chronic illness should elevate fatigue"
    );
    assert!(
        needs.stress_effective() > control_needs.stress_effective() + 0.05,
        "chronic illness should elevate stress"
    );
    assert!(
        needs.purpose_effective() < control_needs.purpose_effective() - 0.05,
        "chronic illness should reduce purpose"
    );
    assert!(
        mh.hopelessness_effective() > control_mh.hopelessness_effective() + 0.05,
        "chronic illness should elevate hopelessness"
    );
}
