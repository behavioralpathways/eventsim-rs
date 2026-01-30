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
fn long_term_incarceration_erodes_social_functioning() {
    let birth_date = Timestamp::from_ymd_hms(1970, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0);

    let (mut sim, entity_id) = setup_sim("incarceration", birth_date, reference);
    let (control_sim, control_id) = setup_sim("incarceration_control", birth_date, reference);

    let events = [
        (20, EventType::FaceChargesLegal, 0.7),
        (21, EventType::ExperienceHumiliationPublic, 0.6),
        (22, EventType::ExperienceIsolationChronic, 0.8),
        (24, EventType::ExperienceConflictInterpersonal, 0.6),
        (26, EventType::ExperienceShamingPublic, 0.6),
        (28, EventType::ExperienceIsolationChronic, 0.8),
        (30, EventType::ExperienceConflictInterpersonal, 0.6),
        (33, EventType::ExperienceIsolationChronic, 0.7),
        (36, EventType::ExperienceShamingPublic, 0.6),
        (40, EventType::ExperienceIsolationChronic, 0.7),
        (43, EventType::ExperienceConflictFamily, 0.6),
    ];

    add_events(&mut sim, birth_date, &entity_id, &events);

    let state = state_at_age(&sim, &entity_id, birth_date, END_AGE);
    let control_state = state_at_age(&control_sim, &control_id, birth_date, END_AGE);

    let social = state.individual_state().social_cognition();
    let control_social = control_state.individual_state().social_cognition();
    let disposition = state.individual_state().disposition();
    let control_disposition = control_state.individual_state().disposition();

    assert!(
        social.loneliness_effective() > control_social.loneliness_effective() + 0.10,
        "incarceration should elevate loneliness"
    );
    assert!(
        social.self_hate_effective() > control_social.self_hate_effective() + 0.05,
        "incarceration should elevate self-hate"
    );
    assert!(
        disposition.grievance_effective() > control_disposition.grievance_effective() + 0.05,
        "incarceration should elevate grievance"
    );
    assert!(
        disposition.empathy_effective() < control_disposition.empathy_effective() - 0.05,
        "incarceration should reduce empathy"
    );
    assert!(
        disposition.trust_propensity_effective() < control_disposition.trust_propensity_effective() - 0.05,
        "incarceration should reduce trust propensity"
    );
}
