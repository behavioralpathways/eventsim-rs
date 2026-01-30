//! Longitudinal scenario test using only the public API.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 2;
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
fn secure_childhood_buffers_later_trauma() {
    let birth_date = Timestamp::from_ymd_hms(1975, 1, 1, 0, 0, 0);
    let reference = birth_date + Duration::years(ANCHOR_AGE);

    let (mut secure_sim, secure_id) = setup_sim("secure_childhood", birth_date, reference);
    let (mut adverse_sim, adverse_id) = setup_sim("adverse_childhood_compare", birth_date, reference);

    let secure_events = [
        (4, EventType::ExperienceInclusionPeer, 0.6),
        (6, EventType::AchieveGoalMajor, 0.6),
        (9, EventType::ExperienceInclusionPeer, 0.6),
        (12, EventType::AchieveGoalMajor, 0.6),
        (32, EventType::ExperienceBetrayalTrust, 0.7),
        (35, EventType::DevelopIllnessChronic, 0.7),
        (38, EventType::ExperienceAwarenessMortality, 0.6),
    ];

    let adverse_events = [
        (4, EventType::ExperienceConflictFamily, 0.7),
        (6, EventType::ExperienceRejectionFamily, 0.7),
        (9, EventType::ExperienceHumiliationPublic, 0.6),
        (12, EventType::ExperienceExclusionPeer, 0.6),
        (32, EventType::ExperienceBetrayalTrust, 0.7),
        (35, EventType::DevelopIllnessChronic, 0.7),
        (38, EventType::ExperienceAwarenessMortality, 0.6),
    ];

    add_events(&mut secure_sim, birth_date, &secure_id, &secure_events);
    add_events(&mut adverse_sim, birth_date, &adverse_id, &adverse_events);

    let secure_state = state_at_age(&secure_sim, &secure_id, birth_date, END_AGE);
    let adverse_state = state_at_age(&adverse_sim, &adverse_id, birth_date, END_AGE);

    let secure_mh = secure_state.individual_state().mental_health();
    let adverse_mh = adverse_state.individual_state().mental_health();

    assert!(
        secure_mh.self_worth_effective() > adverse_mh.self_worth_effective() + 0.05,
        "secure childhood should buffer self-worth relative to adverse childhood"
    );
}
