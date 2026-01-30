//! Longitudinal lifespan test using only the public API.
//!
//! Scenario: A single person experiences ~40 events across age 2 to 65.
//! Expectation: By age 65, long-term social and needs metrics differ from
//! a no-event control in sensible directions, and life stage matches age.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, LifeStage, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::Simulation;
use eventsim_rs::types::{Duration, EntityId, Timestamp};

#[test]
fn lifespan_trajectory_with_many_events_looks_plausible() {
    let birth_date = Timestamp::from_ymd_hms(1960, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(1990, 1, 1, 0, 0, 0); // age 30 anchor

    let entity_id = EntityId::new("person_001").unwrap();

    let mut sim = Simulation::new(reference);
    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .birth_date(birth_date)
        .build()
        .unwrap();
    sim.add_entity(entity, reference);

    let events: [(u16, EventType, f64); 40] = [
        (2, EventType::ExperienceInclusionPeer, 0.6),
        (4, EventType::ExperienceExclusionPeer, 0.6),
        (5, EventType::ExperienceConflictFamily, 0.7),
        (6, EventType::ExperienceBetrayalTrust, 0.5),
        (7, EventType::ExperienceInclusionPeer, 0.5),
        (8, EventType::ExperienceExclusionGroup, 0.6),
        (9, EventType::ExperienceRejectionPeer, 0.6),
        (10, EventType::ExperienceHumiliationPublic, 0.7),
        (11, EventType::AchieveGoalMajor, 0.6),
        (12, EventType::ExperienceShamingPublic, 0.6),
        (13, EventType::ExperienceConflictInterpersonal, 0.5),
        (14, EventType::ExperienceRejectionFamily, 0.7),
        (15, EventType::AchieveGoalMajor, 0.6),
        (16, EventType::ExperienceInclusionPeer, 0.6),
        (17, EventType::ExperienceRejectionRomantic, 0.6),
        (18, EventType::ExperienceIsolationChronic, 0.8),
        (19, EventType::AchieveGoalMajor, 0.6),
        (20, EventType::ExperienceStrainFinancial, 0.7),
        (21, EventType::EndRelationshipRomantic, 0.7),
        (22, EventType::ExperienceConflictInterpersonal, 0.6),
        (23, EventType::ExperienceBetrayalTrust, 0.6),
        (24, EventType::ExperienceExclusionGroup, 0.6),
        (25, EventType::AchieveGoalMajor, 0.7),
        (26, EventType::ExperienceInclusionPeer, 0.6),
        (27, EventType::ExperienceStrainFinancial, 0.7),
        (28, EventType::FaceChargesLegal, 0.6),
        (29, EventType::ExperienceHumiliationPublic, 0.7),
        (30, EventType::EndRelationshipRomantic, 0.6),
        (31, EventType::ExperienceConflictFamily, 0.6),
        (32, EventType::ExperienceAwarenessMortality, 0.7),
        (33, EventType::ExperienceWarRegional, 0.6),
        (34, EventType::AchieveGoalMajor, 0.7),
        (35, EventType::ExperienceIsolationChronic, 0.7),
        (36, EventType::DevelopIllnessChronic, 0.8),
        (40, EventType::ExperienceStrainFinancial, 0.6),
        (45, EventType::ExperienceBetrayalTrust, 0.6),
        (50, EventType::ExperienceAwarenessMortality, 0.7),
        (55, EventType::DevelopIllnessChronic, 0.7),
        (60, EventType::ExperienceIsolationChronic, 0.7),
        (65, EventType::ExperienceAwarenessMortality, 0.8),
    ];

    for (age_years, event_type, severity) in events {
        let event = EventBuilder::new(event_type)
            .target(entity_id.clone())
            .severity(severity)
            .timestamp(Duration::years(age_years))
            .build()
            .unwrap();

        let event_time = birth_date + Duration::years(age_years);
        sim.add_event(event, event_time);
    }

    assert_eq!(sim.all_events().count(), 40);

    let handle = sim.entity(&entity_id).unwrap();
    let end_time = birth_date + Duration::years(65);
    let state = handle.state_at(end_time);

    assert_eq!(state.age_at_timestamp().as_years(), 65);
    assert_eq!(state.life_stage(), LifeStage::MatureAdult);

    let mut control_sim = Simulation::new(reference);
    let control_entity = EntityBuilder::new()
        .id("control_001")
        .species(Species::Human)
        .age(Duration::years(30))
        .birth_date(birth_date)
        .build()
        .unwrap();
    control_sim.add_entity(control_entity, reference);

    let control_id = EntityId::new("control_001").unwrap();
    let control_handle = control_sim.entity(&control_id).unwrap();
    let control_state = control_handle.state_at(end_time);

    let needs = state.individual_state().needs();
    let social = state.individual_state().social_cognition();

    let control_needs = control_state.individual_state().needs();
    let control_social = control_state.individual_state().social_cognition();

    assert!(
        needs.fatigue_effective() > control_needs.fatigue_effective() + 0.05,
        "fatigue should be higher after chronic stressors"
    );
    assert!(
        needs.stress_effective() > control_needs.stress_effective() + 0.05,
        "stress should be higher after repeated strain"
    );
    assert!(
        needs.purpose_effective() < control_needs.purpose_effective() - 0.05,
        "purpose should be lower after repeated setbacks"
    );

    assert!(
        social.loneliness_effective() > control_social.loneliness_effective() + 0.05,
        "loneliness should be higher after isolation and rejection"
    );
    assert!(
        social.perceived_reciprocal_caring_effective()
            < control_social.perceived_reciprocal_caring_effective() - 0.05,
        "perceived caring should be lower after betrayal and exclusion"
    );
    assert!(
        social.self_hate_effective() > control_social.self_hate_effective() + 0.05,
        "self-hate should be higher after prolonged negative social experiences"
    );
}
