//! School shooter pathway - demonstrating how ITS factors converge over adolescence
//! and critically, what interventions at different stages could have prevented violence.
//!
//! This test models a 4-year descent (ages 14-18) showing how Thwarted Belongingness,
//! Perceived Burdensomeness, Interpersonal Hopelessness, and Acquired Capability
//! converge to create lethal risk - and how interventions at different stages could
//! have changed the trajectory.
//!
//! IMPORTANT: This is for understanding prevention pathways, not glorification.
//! The simulation demonstrates that the outcome was preventable at multiple points.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::state::{MentalHealth, SocialCognition};
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 14;

fn setup_sim(
    id: &str,
    birth_date: Timestamp,
    reference: Timestamp,
    baseline_loneliness: f32,
    baseline_self_worth: f32,
) -> (Simulation, EntityId) {
    let mut sim = Simulation::new(reference);
    let sc = SocialCognition::new().with_loneliness_base(baseline_loneliness);
    let mh = MentalHealth::new().with_self_worth_base(baseline_self_worth);
    let entity = EntityBuilder::new()
        .id(id)
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .social_cognition(sc)
        .mental_health(mh)
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
fn shooter_pathway_shows_its_convergence_and_intervention_effects() {
    // ========================================================================
    // SETUP - Alex, age 14, slightly elevated loneliness, slightly low self-worth
    // Not a "bad kid" - just struggling after parents' divorce
    // ========================================================================

    let birth_date = Timestamp::from_ymd_hms(2006, 1, 1, 0, 0, 0);
    let reference = birth_date + Duration::years(ANCHOR_AGE);

    // Baseline: slightly elevated loneliness (0.4), slightly low self-worth (0.45)
    // Normal baseline would be ~0.2 loneliness, ~0.6 self-worth
    let baseline_loneliness = 0.4;
    let baseline_self_worth = 0.45;

    // ========================================================================
    // PATH 1: NO INTERVENTION - Full trajectory to violence
    // ========================================================================

    let (mut sim_no_intervention, alex_no_intervention) = setup_sim(
        "alex_no_intervention",
        birth_date,
        reference,
        baseline_loneliness,
        baseline_self_worth,
    );

    // Age 14 - Early warning signs: social difficulties at new school
    let age_14_events = [
        (14, EventType::ExperienceExclusionPeer, 0.5),
        (14, EventType::ExperienceExclusionGroup, 0.5),
        (14, EventType::ExperienceRejectionPeer, 0.4),
    ];
    add_events(
        &mut sim_no_intervention,
        birth_date,
        &alex_no_intervention,
        &age_14_events,
    );

    // Age 15 - Escalation: bullying intensifies, public humiliation
    let age_15_events = [
        (15, EventType::ExperienceHumiliationPublic, 0.7),
        (15, EventType::ExperienceShamingPublic, 0.6),
        (15, EventType::ExperienceExclusionPeer, 0.6),
        (15, EventType::FailGoalMajor, 0.5), // Grades dropping
    ];
    add_events(
        &mut sim_no_intervention,
        birth_date,
        &alex_no_intervention,
        &age_15_events,
    );

    // Age 16 - Critical period: complete isolation, self-harm begins (AC building)
    let age_16_events = [
        (16, EventType::ExperienceIsolationChronic, 0.8),
        (16, EventType::ExperienceRejectionFamily, 0.6), // Mom overwhelmed, distant
        (16, EventType::EngageSelfharmNonsuicidal, 0.7), // AC increases
        (16, EventType::ExperienceHumiliationPublic, 0.6),
    ];
    add_events(
        &mut sim_no_intervention,
        birth_date,
        &alex_no_intervention,
        &age_16_events,
    );

    // Age 17 - Danger zone: grievance crystallizes, capability building
    let age_17_events = [
        (17, EventType::ExperienceIsolationChronic, 0.8),
        (17, EventType::EngageSelfharmNonsuicidal, 0.6), // Repeated self-harm
        (17, EventType::ExperienceShamingPublic, 0.7),
        (17, EventType::ExperienceRejectionPeer, 0.7),
    ];
    add_events(
        &mut sim_no_intervention,
        birth_date,
        &alex_no_intervention,
        &age_17_events,
    );

    // Age 18 - Final triggering event
    let age_18_events = [
        (18, EventType::ExperienceHumiliationPublic, 0.9), // Final humiliation
        (18, EventType::ExperienceRejectionRomantic, 0.8), // Romantic rejection
    ];
    add_events(
        &mut sim_no_intervention,
        birth_date,
        &alex_no_intervention,
        &age_18_events,
    );

    // ========================================================================
    // PATH 2: AGE 15 INTERVENTION - Teacher notices, connects to counselor
    // Early intervention during escalation phase
    // ========================================================================

    let (mut sim_age_15_intervention, alex_age_15_intervention) = setup_sim(
        "alex_age_15_intervention",
        birth_date,
        reference,
        baseline_loneliness,
        baseline_self_worth,
    );

    // Same events through age 14
    add_events(
        &mut sim_age_15_intervention,
        birth_date,
        &alex_age_15_intervention,
        &age_14_events,
    );

    // Age 15 - Same negative events BUT also intervention begins
    add_events(
        &mut sim_age_15_intervention,
        birth_date,
        &alex_age_15_intervention,
        &age_15_events,
    );

    // Intervention: teacher mentorship, counseling, peer inclusion program
    let age_15_intervention_events = [
        (15, EventType::ExperienceInclusionPeer, 0.6), // Mentorship program
        (15, EventType::AchieveGoalMajor, 0.5),        // Small success in supported activity
        (15, EventType::ExperienceInclusionPeer, 0.5), // Belonging starts to improve
    ];
    add_events(
        &mut sim_age_15_intervention,
        birth_date,
        &alex_age_15_intervention,
        &age_15_intervention_events,
    );

    // Age 16-18: Continued support, fewer negative events (trajectory changed)
    let age_16_intervention_events = [
        (16, EventType::ExperienceInclusionPeer, 0.6),
        (16, EventType::AchieveGoalMajor, 0.4),
    ];
    add_events(
        &mut sim_age_15_intervention,
        birth_date,
        &alex_age_15_intervention,
        &age_16_intervention_events,
    );

    let age_17_intervention_events = [
        (17, EventType::ExperienceInclusionPeer, 0.5),
        (17, EventType::AchieveGoalMajor, 0.5),
    ];
    add_events(
        &mut sim_age_15_intervention,
        birth_date,
        &alex_age_15_intervention,
        &age_17_intervention_events,
    );

    // ========================================================================
    // PATH 3: AGE 16 INTERVENTION - Mom gets therapy after self-harm discovery
    // Mid-trajectory intervention after crisis
    // ========================================================================

    let (mut sim_age_16_intervention, alex_age_16_intervention) = setup_sim(
        "alex_age_16_intervention",
        birth_date,
        reference,
        baseline_loneliness,
        baseline_self_worth,
    );

    // Same events through age 15
    add_events(
        &mut sim_age_16_intervention,
        birth_date,
        &alex_age_16_intervention,
        &age_14_events,
    );
    add_events(
        &mut sim_age_16_intervention,
        birth_date,
        &alex_age_16_intervention,
        &age_15_events,
    );

    // Age 16 - Same negative events BUT therapy intervention after self-harm
    add_events(
        &mut sim_age_16_intervention,
        birth_date,
        &alex_age_16_intervention,
        &age_16_events,
    );

    // Intervention: intensive therapy, family reconnection
    let age_16_therapy_events = [
        (16, EventType::ExperienceInclusionPeer, 0.5), // Therapy group
        (16, EventType::AchieveGoalMajor, 0.4),        // Progress in treatment
    ];
    add_events(
        &mut sim_age_16_intervention,
        birth_date,
        &alex_age_16_intervention,
        &age_16_therapy_events,
    );

    // Age 17-18: Continued treatment, reduced negative events
    let age_17_therapy_events = [
        (17, EventType::ExperienceInclusionPeer, 0.5),
        (17, EventType::AchieveGoalMajor, 0.4),
    ];
    add_events(
        &mut sim_age_16_intervention,
        birth_date,
        &alex_age_16_intervention,
        &age_17_therapy_events,
    );

    // ========================================================================
    // PATH 4: AGE 17 INTERVENTION - "Leak" reported, threat assessment
    // Late intervention, crisis response
    // ========================================================================

    let (mut sim_age_17_intervention, alex_age_17_intervention) = setup_sim(
        "alex_age_17_intervention",
        birth_date,
        reference,
        baseline_loneliness,
        baseline_self_worth,
    );

    // Same events through age 16
    add_events(
        &mut sim_age_17_intervention,
        birth_date,
        &alex_age_17_intervention,
        &age_14_events,
    );
    add_events(
        &mut sim_age_17_intervention,
        birth_date,
        &alex_age_17_intervention,
        &age_15_events,
    );
    add_events(
        &mut sim_age_17_intervention,
        birth_date,
        &alex_age_17_intervention,
        &age_16_events,
    );

    // Age 17 - Same events but intervention after "leak" (tells someone online)
    add_events(
        &mut sim_age_17_intervention,
        birth_date,
        &alex_age_17_intervention,
        &age_17_events,
    );

    // Late intervention: crisis response, intensive treatment
    let age_17_crisis_events = [
        (17, EventType::ExperienceInclusionPeer, 0.4), // Crisis intervention, support
        (17, EventType::AchieveGoalMajor, 0.3),        // Small wins in crisis treatment
    ];
    add_events(
        &mut sim_age_17_intervention,
        birth_date,
        &alex_age_17_intervention,
        &age_17_crisis_events,
    );

    // Age 18: Continued intensive support
    let age_18_crisis_events = [
        (18, EventType::ExperienceInclusionPeer, 0.4),
        (18, EventType::AchieveGoalMajor, 0.3),
    ];
    add_events(
        &mut sim_age_17_intervention,
        birth_date,
        &alex_age_17_intervention,
        &age_18_crisis_events,
    );

    // ========================================================================
    // ANALYZE PATH 1: NO INTERVENTION
    // Assert ITS convergence - TB, PB, Hopelessness all elevated, AC present
    // ========================================================================

    let baseline_no_intervention =
        state_at_age(&sim_no_intervention, &alex_no_intervention, birth_date, 14);
    let baseline_tb = baseline_no_intervention
        .individual_state()
        .compute_thwarted_belongingness();
    let baseline_pb = baseline_no_intervention
        .individual_state()
        .compute_perceived_burdensomeness();
    let baseline_hopelessness = baseline_no_intervention
        .individual_state()
        .mental_health()
        .interpersonal_hopelessness_effective();
    let baseline_ac = baseline_no_intervention
        .individual_state()
        .mental_health()
        .acquired_capability_effective();

    let state_no_intervention =
        state_at_age(&sim_no_intervention, &alex_no_intervention, birth_date, 18);
    let mh_no_intervention = state_no_intervention.individual_state().mental_health();

    let tb_no_intervention = state_no_intervention
        .individual_state()
        .compute_thwarted_belongingness();
    let pb_no_intervention = state_no_intervention
        .individual_state()
        .compute_perceived_burdensomeness();
    let hopelessness_no_intervention = mh_no_intervention.interpersonal_hopelessness_effective();
    let ac_no_intervention = mh_no_intervention.acquired_capability_effective();

    // Assert ITS convergence in no-intervention path (relative to baseline)
    assert!(
        tb_no_intervention > baseline_tb + 0.15,
        "No intervention: TB should increase (baseline {:.3}, age 18 {:.3})",
        baseline_tb,
        tb_no_intervention
    );
    assert!(
        pb_no_intervention > baseline_pb + 0.10,
        "No intervention: PB should increase (baseline {:.3}, age 18 {:.3})",
        baseline_pb,
        pb_no_intervention
    );
    assert!(
        hopelessness_no_intervention > baseline_hopelessness + 0.15,
        "No intervention: Hopelessness should increase (baseline {:.3}, age 18 {:.3})",
        baseline_hopelessness,
        hopelessness_no_intervention
    );
    assert!(
        ac_no_intervention > baseline_ac + 0.10,
        "No intervention: AC should increase from self-harm (baseline {:.3}, age 18 {:.3})",
        baseline_ac,
        ac_no_intervention
    );

    // ========================================================================
    // ANALYZE PATH 2: AGE 15 INTERVENTION
    // Assert early intervention significantly reduces TB and prevents trajectory
    // ========================================================================

    let state_age_15_intervention = state_at_age(&sim_age_15_intervention, &alex_age_15_intervention, birth_date, 18);
    let mh_age_15_intervention = state_age_15_intervention.individual_state().mental_health();
    let _social_age_15_intervention =
        state_age_15_intervention.individual_state().social_cognition();

    let tb_age_15_intervention = state_age_15_intervention.individual_state().compute_thwarted_belongingness();
    let _pb_age_15_intervention =
        state_age_15_intervention
            .individual_state()
            .compute_perceived_burdensomeness();
    let _hopelessness_age_15_intervention =
        mh_age_15_intervention.interpersonal_hopelessness_effective();
    let ac_age_15_intervention = mh_age_15_intervention.acquired_capability_effective();

    // Early intervention should show significant improvement
    assert!(
        tb_age_15_intervention < tb_no_intervention - 0.10,
        "Age 15 intervention: TB should be significantly lower than no intervention (got {:.3} vs {:.3}, diff {:.3})",
        tb_age_15_intervention, tb_no_intervention, tb_no_intervention - tb_age_15_intervention
    );
    assert!(
        ac_age_15_intervention < ac_no_intervention - 0.10,
        "Age 15 intervention: AC should be much lower (no self-harm path) (got {:.3} vs {:.3})",
        ac_age_15_intervention, ac_no_intervention
    );

    // ========================================================================
    // ANALYZE PATH 3: AGE 16 INTERVENTION
    // Assert mid-trajectory intervention shows some improvement but less than early
    // ========================================================================

    let state_age_16_intervention = state_at_age(&sim_age_16_intervention, &alex_age_16_intervention, birth_date, 18);
    let mh_age_16_intervention = state_age_16_intervention.individual_state().mental_health();

    let tb_age_16_intervention = state_age_16_intervention.individual_state().compute_thwarted_belongingness();
    let pb_age_16_intervention = state_age_16_intervention.individual_state().compute_perceived_burdensomeness();
    let _hopelessness_age_16_intervention =
        mh_age_16_intervention.interpersonal_hopelessness_effective();
    let _ac_age_16_intervention = mh_age_16_intervention.acquired_capability_effective();

    // Mid-trajectory intervention should show some improvement
    assert!(
        tb_age_16_intervention < tb_no_intervention - 0.05,
        "Age 16 intervention: TB should be lower than no intervention (got {:.3} vs {:.3})",
        tb_age_16_intervention, tb_no_intervention
    );
    assert!(
        pb_age_16_intervention < pb_no_intervention - 0.05,
        "Age 16 intervention: PB should be lower than no intervention (got {:.3} vs {:.3})",
        pb_age_16_intervention, pb_no_intervention
    );

    // ========================================================================
    // ANALYZE PATH 4: AGE 17 INTERVENTION
    // Assert late intervention shows some effect but less than earlier interventions
    // ========================================================================

    let state_age_17_intervention = state_at_age(&sim_age_17_intervention, &alex_age_17_intervention, birth_date, 18);
    let mh_age_17_intervention = state_age_17_intervention.individual_state().mental_health();

    let tb_age_17_intervention = state_age_17_intervention.individual_state().compute_thwarted_belongingness();
    let _pb_age_17_intervention =
        state_age_17_intervention
            .individual_state()
            .compute_perceived_burdensomeness();
    let _hopelessness_age_17_intervention =
        mh_age_17_intervention.interpersonal_hopelessness_effective();
    let _ac_age_17_intervention = mh_age_17_intervention.acquired_capability_effective();

    // Late intervention should show some improvement
    assert!(
        tb_age_17_intervention < tb_no_intervention - 0.02,
        "Age 17 intervention: TB should be somewhat lower than no intervention (got {:.3} vs {:.3})",
        tb_age_17_intervention, tb_no_intervention
    );

    // ========================================================================
    // COMPARE INTERVENTION TIMING
    // Assert earlier intervention = better outcomes
    // ========================================================================

    // Earlier intervention should be more effective
    assert!(
        tb_age_15_intervention < tb_age_16_intervention,
        "Age 15 intervention should have lower TB than age 16 intervention (15: {:.3}, 16: {:.3})",
        tb_age_15_intervention, tb_age_16_intervention
    );
    assert!(
        tb_age_16_intervention <= tb_age_17_intervention + 0.01,
        "Age 16 intervention should have lower TB than age 17 intervention (16: {:.3}, 17: {:.3})",
        tb_age_16_intervention, tb_age_17_intervention
    );
}
