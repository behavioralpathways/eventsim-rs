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
fn immigrant_relocation_stress_lingers_but_softens() {
    let birth_date = Timestamp::from_ymd_hms(1985, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(2015, 1, 1, 0, 0, 0);

    let (mut sim, entity_id) = setup_sim("immigration", birth_date, reference);
    let (control_sim, control_id) = setup_sim("immigration_control", birth_date, reference);

    let events = [
        (20, EventType::ExperienceExclusionGroup, 0.6),
        (21, EventType::ExperienceConflictInterpersonal, 0.6),
        (22, EventType::ExperienceStrainFinancial, 0.6),
        (23, EventType::ExperienceRejectionPeer, 0.6),
        (24, EventType::ExperienceInclusionPeer, 0.5),
        (26, EventType::AchieveGoalMajor, 0.6),
        (28, EventType::ExperienceInclusionPeer, 0.5),
    ];

    add_events(&mut sim, birth_date, &entity_id, &events);

    let state = state_at_age(&sim, &entity_id, birth_date, END_AGE);
    let control_state = state_at_age(&control_sim, &control_id, birth_date, END_AGE);

    let needs = state.individual_state().needs();
    let control_needs = control_state.individual_state().needs();
    let social = state.individual_state().social_cognition();
    let control_social = control_state.individual_state().social_cognition();

    println!(
        "stress: cohort {:.3} control {:.3}",
        needs.stress_effective(),
        control_needs.stress_effective()
    );
    println!(
        "loneliness: cohort {:.3} control {:.3}",
        social.loneliness_effective(),
        control_social.loneliness_effective()
    );
    println!(
        "prc: cohort {:.3} control {:.3}",
        social.perceived_reciprocal_caring_effective(),
        control_social.perceived_reciprocal_caring_effective()
    );

    assert!(
        needs.stress_effective() > control_needs.stress_effective() + 0.03,
        "relocation stress should elevate stress"
    );
    assert!(
        social.loneliness_effective() > control_social.loneliness_effective() + 0.03,
        "relocation should leave some loneliness residue"
    );
    assert!(
        social.perceived_reciprocal_caring_effective()
            > control_social.perceived_reciprocal_caring_effective() - 0.02,
        "later inclusion should partially restore perceived caring"
    );
}
