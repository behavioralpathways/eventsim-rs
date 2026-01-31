//! Longitudinal scenario: Vietnam veteran life trajectory
//!
//! A young man from rural Ohio enlists at 18 in 1967. He serves two tours in
//! Vietnam, experiencing intense combat. He returns home changed - struggles
//! with reintegration, goes through a failed marriage, battles isolation.
//! Eventually finds purpose through mentoring other veterans. At 70, we
//! examine his psychological state compared to a peer who never served.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 18;
const END_AGE: u64 = 70;

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
#[ignore]
fn vietnam_veteran_life_trajectory() {
    let birth_date = Timestamp::from_ymd_hms(1949, 6, 15, 0, 0, 0);
    let reference = birth_date + Duration::years(ANCHOR_AGE);

    let (mut veteran_sim, veteran_id) = setup_sim("veteran", birth_date, reference);
    let (mut control_sim, control_id) = setup_sim("control", birth_date, reference);

    // === VETERAN'S LIFE ===

    // Age 19-20: First tour in Vietnam - intense combat
    // The defining trauma that will echo through his life
    let first_tour = [
        (19, EventType::ExperienceCombatMilitary, 0.9),
        (19, EventType::ExperienceAwarenessMortality, 0.85),
        (20, EventType::ExperienceCombatMilitary, 0.95),
        (20, EventType::LosePersonDeath, 0.8), // Loses a buddy
    ];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &first_tour);

    // Age 21-22: Second tour - more combat, wounded
    let second_tour = [
        (21, EventType::ExperienceCombatMilitary, 0.85),
        (22, EventType::SufferInjuryAccidental, 0.7),
        (22, EventType::ExperienceAwarenessMortality, 0.9),
    ];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &second_tour);

    // Age 23-25: Returns home - struggles to reintegrate
    let reintegration = [
        (23, EventType::ExperienceIsolationChronic, 0.6),
        (24, EventType::ExperienceConflictFamily, 0.5),
        (25, EventType::ExperienceRejectionFamily, 0.6),
    ];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &reintegration);

    // Age 28: Marriage - brief hope
    let marriage = [(28, EventType::ReceiveSupportEmotional, 0.7)];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &marriage);

    // Age 32-35: Marriage falls apart
    let divorce = [
        (32, EventType::ExperienceConflictInterpersonal, 0.6),
        (34, EventType::ExperienceBetrayalTrust, 0.7),
        (35, EventType::EndRelationshipRomantic, 0.8),
    ];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &divorce);

    // Age 36-45: Dark years - isolation, struggles
    let dark_years = [
        (36, EventType::ExperienceIsolationChronic, 0.7),
        (40, EventType::ExperienceStrainFinancial, 0.6),
        (42, EventType::ExperienceConflictInterpersonal, 0.5),
        (45, EventType::ExperienceIsolationChronic, 0.6),
    ];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &dark_years);

    // Age 50+: Finds purpose mentoring younger veterans
    let recovery = [
        (50, EventType::ReceiveSupportEmotional, 0.6),
        (52, EventType::AchieveGoalMajor, 0.4),
        (55, EventType::ReceiveSupportEmotional, 0.7),
        (58, EventType::ExperienceInclusionPeer, 0.6),
        (60, EventType::AchieveGoalMajor, 0.7), // Recognition for service
    ];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &recovery);

    // Age 65: Loses wife of second marriage
    let late_loss = [(65, EventType::LosePersonDeath, 0.85)];
    add_events(&mut veteran_sim, birth_date, &veteran_id, &late_loss);

    // === CONTROL'S LIFE ===
    // Normal civilian life - some ups and downs but no trauma

    let control_life = [
        (22, EventType::AchieveGoalMajor, 0.5),           // College graduation
        (25, EventType::ReceiveSupportEmotional, 0.7),    // Marriage
        (28, EventType::AchieveGoalMajor, 0.7),           // Career success
        (35, EventType::ExperienceConflictFamily, 0.4),   // Normal family stress
        (45, EventType::LosePersonDeath, 0.6),            // Parent dies
        (50, EventType::AchieveGoalMajor, 0.7),           // Peak career
        (60, EventType::ExperienceStrainFinancial, 0.4),  // Retirement worries
        (65, EventType::ReceiveSupportEmotional, 0.5),    // Grandkids
    ];
    add_events(&mut control_sim, birth_date, &control_id, &control_life);

    // === COMPARE AT AGE 70 ===

    let veteran_state = state_at_age(&veteran_sim, &veteran_id, birth_date, END_AGE);
    let control_state = state_at_age(&control_sim, &control_id, birth_date, END_AGE);

    let veteran_mh = veteran_state.individual_state().mental_health();
    let control_mh = control_state.individual_state().mental_health();
    let veteran_mood = veteran_state.individual_state().mood();
    let control_mood = control_state.individual_state().mood();
    let veteran_disposition = veteran_state.individual_state().disposition();
    let control_disposition = control_state.individual_state().disposition();
    let veteran_social = veteran_state.individual_state().social_cognition();
    let control_social = control_state.individual_state().social_cognition();

    // Combat exposure leaves lasting acquired capability
    assert!(
        veteran_mh.acquired_capability_effective() > control_mh.acquired_capability_effective() + 0.05,
        "Veteran should have higher acquired capability from combat exposure"
    );

    // Chronic isolation and failed relationships affect trust
    assert!(
        veteran_disposition.trust_propensity_effective()
            < control_disposition.trust_propensity_effective() - 0.05,
        "Veteran should have lower trust propensity after betrayals and failed relationships"
    );

    // Despite recovery period, veteran still carries more stress
    assert!(
        veteran_state.individual_state().needs().stress_effective()
            > control_state.individual_state().needs().stress_effective(),
        "Veteran should have higher baseline stress from lifetime of trauma"
    );

    // The late-life losses compound with earlier trauma
    assert!(
        veteran_social.loneliness_effective() > control_social.loneliness_effective(),
        "Veteran's loneliness should be elevated from chronic isolation history"
    );

    // Combat veteran's mood still affected by trauma memories decades later
    // Even with recovery events, the weight of combat trauma persists
    assert!(
        veteran_mood.arousal_effective() > control_mood.arousal_effective(),
        "Veteran should have elevated arousal (hypervigilance) from combat trauma memories"
    );

    // Self-worth may be partially recovered through mentorship and recognition
    // but early trauma still leaves marks
    assert!(
        veteran_mh.self_worth_effective() < control_mh.self_worth_effective(),
        "Despite recovery, veteran's self-worth should be lower than control's stable life trajectory"
    );
}
