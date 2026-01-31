//! Longitudinal scenario test modeling cult formation and radicalization.
//!
//! Scenario: A charismatic cult leader recruits and radicalizes 15 vulnerable
//! followers over 3 years, demonstrating how isolation, manipulation, and
//! escalating control lead to psychological convergence toward tragedy.
//!
//! Inspired by historical cases like Jonestown and Heaven's Gate, this test
//! models the psychological mechanisms of cult indoctrination:
//! - Love bombing (reducing thwarted belongingness initially)
//! - Isolation (severing outside relationships)
//! - Exploitation (financial, sleep deprivation)
//! - Thought reform (confession sessions, public humiliation)
//! - Capability building (suicide drills increasing acquired capability)
//!
//! Expected dynamics:
//! - TB decreases initially (love bombing creates belonging)
//! - TB increases later (isolation destroys outside relationships)
//! - PB increases (economic dependence, self-blame)
//! - AC increases (exposure to death, practice drills)
//! - Trust propensity toward outsiders collapses
//! - Purpose increases initially (cult provides meaning)
//! - Hopelessness increases as autonomy is destroyed

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{
    EventPayload, EventType, MentalHealthPath, PersonalityProfile, RelationshipSchema, Species,
    StatePath,
};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::Simulation;
use eventsim_rs::types::{Duration, EntityId, Timestamp};
use eventsim_rs::GroupId;

const LEADER_AGE: u64 = 45;
/// Creates the cult leader entity.
fn create_leader(birth_date: Timestamp, _reference: Timestamp) -> (EntityBuilder, EntityId) {
    let entity = EntityBuilder::new()
        .id("jim_leader")
        .species(Species::Human)
        .age(Duration::years(LEADER_AGE))
        .birth_date(birth_date)
        .personality(PersonalityProfile::Leader);

    let entity_id = EntityId::new("jim_leader").unwrap();
    (entity, entity_id)
}

/// Creates vulnerable follower profiles.
/// Each follower has specific vulnerabilities that make them susceptible to cult recruitment.
fn create_followers(_birth_date: Timestamp) -> Vec<(&'static str, u64, PersonalityProfile)> {
    vec![
        ("sarah", 28, PersonalityProfile::Anxious),     // Recent divorce, lonely
        ("michael", 35, PersonalityProfile::Neurotic),  // Lost job, burden
        ("linda", 22, PersonalityProfile::Anxious),     // Abusive childhood
        ("tom", 40, PersonalityProfile::Neurotic),      // Wife died, grieving
        ("jessica", 32, PersonalityProfile::Agreeable), // Trusting, seeking belonging
        ("david", 26, PersonalityProfile::Anxious),     // Purposeless, drifting
        ("maria", 38, PersonalityProfile::Agreeable),   // Lonely immigrant
        ("robert", 29, PersonalityProfile::Neurotic),   // Chronic illness, isolated
        ("emily", 24, PersonalityProfile::Anxious),     // Recent rejection
        ("james", 42, PersonalityProfile::Agreeable),   // Seeking meaning
        ("lisa", 31, PersonalityProfile::Neurotic),     // Financial strain
        ("kevin", 27, PersonalityProfile::Anxious),     // Family estrangement
        ("amy", 36, PersonalityProfile::Agreeable),     // Seeking community
        ("brian", 30, PersonalityProfile::Neurotic),    // Social isolation
        ("rachel", 25, PersonalityProfile::Anxious),    // Low self-worth
    ]
}

/// Applies events for the love bombing phase (Year 1, Months 1-3).
/// Goal: Create intense belonging, "you're special", reduce initial loneliness.
fn apply_love_bombing_phase(
    sim: &mut Simulation,
    followers: &[EntityId],
    reference: Timestamp,
    cult_group: &GroupId,
    leader_id: &EntityId,
) {
    for follower_id in followers.iter() {
        // Month 1: Intense inclusion (love bombing)
        let event = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.8)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(cult_group.clone()),
            })
            .timestamp(Duration::months(1))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(1));

        // Month 2: More inclusion, belonging
        let event = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.8)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(cult_group.clone()),
            })
            .timestamp(Duration::months(2))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(2));

        // Month 3: Achievement (feeling valued)
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.7)
            .timestamp(Duration::months(3))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(3));
    }
}

/// Applies events for establishing control (Year 1, Months 4-12).
/// Isolation from family begins, loyalty demands, small boundary violations.
fn apply_establishing_control_phase(
    sim: &mut Simulation,
    followers: &[EntityId],
    reference: Timestamp,
    outside_group: &GroupId,
    leader_id: &EntityId,
) {
    for follower_id in followers.iter() {
        // Month 5: Family conflict (begin isolation)
        let event = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(follower_id.clone())
            .severity(0.6)
            .timestamp(Duration::months(5))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(5));

        // Month 7: Betrayal from outside (manufactured distrust)
        let event = EventBuilder::new(EventType::ExperienceBetrayalTrust)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.5)
            .payload(EventPayload::Betrayal {
                confidence_violated: 0.5,
            })
            .timestamp(Duration::months(7))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(7));

        // Month 9: Exclusion from outside groups (cut off)
        let event = EventBuilder::new(EventType::ExperienceExclusionGroup)
            .target(follower_id.clone())
            .severity(0.7)
            .payload(EventPayload::SocialExclusion {
                group_id: Some(outside_group.clone()),
                explicit: true,
            })
            .timestamp(Duration::months(9))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(9));

        // Month 11: Family rejection (or perceived rejection)
        let event = EventBuilder::new(EventType::ExperienceRejectionFamily)
            .target(follower_id.clone())
            .severity(0.7)
            .timestamp(Duration::months(11))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(11));
    }
}

/// Applies events for escalating control (Year 2, Months 13-24).
/// Financial exploitation, sleep deprivation, confession sessions, public humiliation.
fn apply_escalating_control_phase(
    sim: &mut Simulation,
    followers: &[EntityId],
    reference: Timestamp,
    leader_id: &EntityId,
) {
    for follower_id in followers.iter() {
        // Month 14: Financial strain (exploitation begins)
        let event = EventBuilder::new(EventType::ExperienceStrainFinancial)
            .target(follower_id.clone())
            .severity(0.8)
            .timestamp(Duration::months(14))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(14));

        // Month 16: Chronic isolation (cut off from world)
        let event = EventBuilder::new(EventType::ExperienceIsolationChronic)
            .target(follower_id.clone())
            .severity(0.9)
            .timestamp(Duration::months(16))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(16));

        // Month 18: Public humiliation (confession sessions)
        let event = EventBuilder::new(EventType::ExperienceHumiliationPublic)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.7)
            .payload(EventPayload::Humiliation {
                public: true,
                perpetrator: Some(leader_id.clone()),
            })
            .timestamp(Duration::months(18))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(18));

        // Month 20: Public shaming (punishing doubters)
        let event = EventBuilder::new(EventType::ExperienceShamingPublic)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.7)
            .timestamp(Duration::months(20))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(20));

        // Month 22: Interpersonal conflict (infighting)
        let event = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
            .target(follower_id.clone())
            .severity(0.6)
            .timestamp(Duration::months(22))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(22));
    }
}

/// Applies events for total control and capability building (Year 3, Months 25-36).
/// Apocalyptic messaging, us vs them, suicide drills, final act.
fn apply_total_control_phase(
    sim: &mut Simulation,
    followers: &[EntityId],
    reference: Timestamp,
    leader_id: &EntityId,
) {
    for follower_id in followers.iter() {
        // Month 26: Mortality awareness (apocalyptic messaging)
        let event = EventBuilder::new(EventType::ExperienceAwarenessMortality)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.8)
            .timestamp(Duration::months(26))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(26));

        // Month 28: More isolation (total separation)
        let event = EventBuilder::new(EventType::ExperienceIsolationChronic)
            .target(follower_id.clone())
            .severity(0.9)
            .timestamp(Duration::months(28))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(28));

        // Month 30: Financial strain continues
        let event = EventBuilder::new(EventType::ExperienceStrainFinancial)
            .target(follower_id.clone())
            .severity(0.8)
            .timestamp(Duration::months(30))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(30));

        // Month 32: Non-suicidal self-harm drills (capability building)
        let event = EventBuilder::new(EventType::EngageSelfharmNonsuicidal)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.7)
            .timestamp(Duration::months(32))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(32));

        // Month 34: More mortality awareness (practice drills)
        let event = EventBuilder::new(EventType::ExperienceAwarenessMortality)
            .source(leader_id.clone())
            .target(follower_id.clone())
            .severity(0.9)
            .timestamp(Duration::months(34))
            .build()
            .unwrap();
        sim.add_event(event, reference + Duration::months(34));
    }
}

#[test]
fn cult_formation_demonstrates_its_convergence() {
    // ========================================================================
    // SETUP
    // Create cult leader and 15 vulnerable followers.
    // Reference date: January 1, 2000 (cult formation begins)
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2000, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Create leader
    let leader_birth_date = reference - Duration::years(LEADER_AGE);
    let (leader_entity, _leader_id) = create_leader(leader_birth_date, reference);
    let leader_id = EntityId::new("jim_leader").unwrap();
    sim.add_entity(leader_entity.build().unwrap(), reference);

    let cult_group = GroupId::new("cult_members").unwrap();
    let outside_group = GroupId::new("outsiders").unwrap();

    // Create followers with birth dates
    let follower_specs = create_followers(reference);
    let mut follower_ids = Vec::new();
    let mut follower_birth_dates = Vec::new();

    for (name, age, profile) in follower_specs {
        let birth_date = reference - Duration::years(age);
        let entity = EntityBuilder::new()
            .id(name)
            .species(Species::Human)
            .age(Duration::years(age))
            .birth_date(birth_date)
            .personality(profile)
            .build()
            .unwrap();

        sim.add_entity(entity, reference);
        follower_ids.push(EntityId::new(name).unwrap());
        follower_birth_dates.push((birth_date, age));
    }

    // Establish leader authority relationships
    for follower_id in &follower_ids {
        sim.add_relationship(
            leader_id.clone(),
            follower_id.clone(),
            RelationshipSchema::Mentor,
            reference,
        );
        sim.add_relationship(
            follower_id.clone(),
            leader_id.clone(),
            RelationshipSchema::Subordinate,
            reference,
        );
    }

    // ========================================================================
    // BASELINE: Measure initial state before cult involvement
    // Followers start vulnerable but not yet in crisis
    // ========================================================================

    let baseline_states: Vec<_> = follower_ids
        .iter()
        .map(|id| {
            let handle = sim.entity(id).unwrap();
            handle.state_at(reference)
        })
        .collect();

    // Calculate baseline averages
    let baseline_avg_loneliness: f64 = baseline_states
        .iter()
        .map(|s| {
            s.individual_state()
                .social_cognition()
                .loneliness_effective() as f64
        })
        .sum::<f64>()
        / baseline_states.len() as f64;

    let baseline_avg_purpose: f64 = baseline_states
        .iter()
        .map(|s| s.individual_state().needs().purpose_effective() as f64)
        .sum::<f64>()
        / baseline_states.len() as f64;

    let baseline_avg_trust: f64 = baseline_states
        .iter()
        .map(|s| {
            s.individual_state()
                .disposition()
                .trust_propensity_effective() as f64
        })
        .sum::<f64>()
        / baseline_states.len() as f64;

    // ========================================================================
    // YEAR 1: LOVE BOMBING (Months 1-3)
    // Intense inclusion and validation reduces initial loneliness
    // ========================================================================

    apply_love_bombing_phase(&mut sim, &follower_ids, reference, &cult_group, &leader_id);

    // ========================================================================
    // STAGE CHECK: After Month 3
    // Loneliness should drop as belonging is manufactured
    // ========================================================================

    let month3_states: Vec<_> = follower_ids
        .iter()
        .map(|id| {
            let handle = sim.entity(id).unwrap();
            handle.state_at(reference + Duration::months(3))
        })
        .collect();

    let month3_avg_loneliness: f64 = month3_states
        .iter()
        .map(|s| {
            s.individual_state()
                .social_cognition()
                .loneliness_effective() as f64
        })
        .sum::<f64>()
        / month3_states.len() as f64;

    let month3_avg_purpose: f64 = month3_states
        .iter()
        .map(|s| s.individual_state().needs().purpose_effective() as f64)
        .sum::<f64>()
        / month3_states.len() as f64;

    assert!(
        month3_avg_loneliness < baseline_avg_loneliness + 0.05,
        "Month 3: Loneliness should not spike during love bombing. Baseline: {:.3}, Month 3: {:.3}",
        baseline_avg_loneliness,
        month3_avg_loneliness
    );

    assert!(
        month3_avg_purpose > baseline_avg_purpose,
        "Month 3: Purpose should rise with love bombing. Baseline: {:.3}, Month 3: {:.3}",
        baseline_avg_purpose,
        month3_avg_purpose
    );

    // ========================================================================
    // YEAR 1: ESTABLISHING CONTROL (Months 4-12)
    // Isolation from family, manufactured conflicts with outsiders
    // ========================================================================

    apply_establishing_control_phase(
        &mut sim,
        &follower_ids,
        reference,
        &outside_group,
        &leader_id,
    );

    // ========================================================================
    // STAGE CHECK: After Year 1
    // Loneliness should be mixed - initial decrease from love bombing,
    // but starting to increase as isolation begins
    // ========================================================================

    let year1_states: Vec<_> = follower_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let handle = sim.entity(id).unwrap();
            let (_birth_date, _) = follower_birth_dates[idx];
            handle.state_at(reference + Duration::months(12))
        })
        .collect();

    let year1_avg_purpose: f64 = year1_states
        .iter()
        .map(|s| s.individual_state().needs().purpose_effective() as f64)
        .sum::<f64>()
        / year1_states.len() as f64;

    // Purpose should erode after control escalates
    assert!(
        year1_avg_purpose < month3_avg_purpose,
        "Year 1: Purpose should decline after control escalates. Month 3: {:.3}, Year 1: {:.3}",
        month3_avg_purpose,
        year1_avg_purpose
    );

    // ========================================================================
    // YEAR 2: ESCALATING CONTROL (Months 13-24)
    // Financial exploitation, chronic isolation, public humiliation
    // ========================================================================

    apply_escalating_control_phase(&mut sim, &follower_ids, reference, &leader_id);

    // ========================================================================
    // STAGE CHECK: After Year 2
    // Isolation should be severe, loneliness high, trust in outsiders low
    // ========================================================================

    let year2_states: Vec<_> = follower_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let handle = sim.entity(id).unwrap();
            let (_birth_date, _) = follower_birth_dates[idx];
            handle.state_at(reference + Duration::months(24))
        })
        .collect();

    let year2_avg_loneliness: f64 = year2_states
        .iter()
        .map(|s| {
            s.individual_state()
                .social_cognition()
                .loneliness_effective() as f64
        })
        .sum::<f64>()
        / year2_states.len() as f64;

    let year2_avg_trust: f64 = year2_states
        .iter()
        .map(|s| {
            s.individual_state()
                .disposition()
                .trust_propensity_effective() as f64
        })
        .sum::<f64>()
        / year2_states.len() as f64;

    // Loneliness should be elevated due to chronic isolation
    assert!(
        year2_avg_loneliness > baseline_avg_loneliness + 0.05,
        "Year 2: Loneliness should be elevated. Baseline: {:.3}, Year 2: {:.3}",
        baseline_avg_loneliness,
        year2_avg_loneliness
    );

    // Trust in outsiders should decrease
    assert!(
        year2_avg_trust < baseline_avg_trust - 0.05,
        "Year 2: Trust propensity should decrease. Baseline: {:.3}, Year 2: {:.3}",
        baseline_avg_trust,
        year2_avg_trust
    );

    // ========================================================================
    // YEAR 3: TOTAL CONTROL (Months 25-36)
    // Apocalyptic messaging, suicide drills, acquired capability building
    // ========================================================================

    apply_total_control_phase(&mut sim, &follower_ids, reference, &leader_id);

    // ========================================================================
    // FINAL STATE: After 3 Years
    // ITS factors should converge: high TB, high PB, elevated AC
    // ========================================================================

    let final_states: Vec<_> = follower_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            let handle = sim.entity(id).unwrap();
            let (_birth_date, _) = follower_birth_dates[idx];
            handle.state_at(reference + Duration::months(36))
        })
        .collect();

    // Calculate final averages across all followers
    let final_avg_tb: f64 = final_states
        .iter()
        .map(|s| {
            s.get_effective(StatePath::MentalHealth(
                MentalHealthPath::ThwartedBelongingness,
            )) as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let final_avg_pb: f64 = final_states
        .iter()
        .map(|s| {
            s.get_effective(StatePath::MentalHealth(
                MentalHealthPath::PerceivedBurdensomeness,
            )) as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let final_avg_ac: f64 = final_states
        .iter()
        .map(|s| {
            s.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability)) as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let final_avg_hopelessness: f64 = final_states
        .iter()
        .map(|s| {
            s.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness)) as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let final_avg_interpersonal_hopelessness: f64 = final_states
        .iter()
        .map(|s| {
            s.get_effective(StatePath::MentalHealth(
                MentalHealthPath::InterpersonalHopelessness,
            )) as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let final_avg_loneliness: f64 = final_states
        .iter()
        .map(|s| {
            s.individual_state()
                .social_cognition()
                .loneliness_effective() as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let final_avg_self_hate: f64 = final_states
        .iter()
        .map(|s| {
            s.individual_state()
                .social_cognition()
                .self_hate_effective() as f64
        })
        .sum::<f64>()
        / final_states.len() as f64;

    let year2_avg_ac: f64 = year2_states
        .iter()
        .map(|s| {
            s.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability)) as f64
        })
        .sum::<f64>()
        / year2_states.len() as f64;

    // ========================================================================
    // ASSERTIONS: Verify cult psychology captured realistically
    // ========================================================================

    // TB should be elevated (chronic isolation destroys belongingness)
    assert!(
        final_avg_tb > 0.4,
        "Final TB should be elevated due to isolation. Got: {:.3}",
        final_avg_tb
    );

    // PB should be elevated (financial dependence, self-blame)
    assert!(
        final_avg_pb > 0.25,
        "Final PB should be elevated due to exploitation. Got: {:.3}",
        final_avg_pb
    );

    // AC should be elevated (exposure to mortality, suicide drills)
    assert!(
        final_avg_ac > 0.3,
        "Final AC should be elevated due to suicide drills. Got: {:.3}",
        final_avg_ac
    );

    assert!(
        final_avg_ac > year2_avg_ac + 0.05,
        "Year 3: AC should rise from drills. Year 2: {:.3}, Final: {:.3}",
        year2_avg_ac,
        final_avg_ac
    );

    // Hopelessness should be elevated (trapped, no escape)
    assert!(
        final_avg_hopelessness > 0.4,
        "Final hopelessness should be elevated. Got: {:.3}",
        final_avg_hopelessness
    );

    // Loneliness should be high (isolated from outside world)
    assert!(
        final_avg_loneliness > baseline_avg_loneliness + 0.15,
        "Final loneliness should be much higher than baseline. Baseline: {:.3}, Final: {:.3}",
        baseline_avg_loneliness,
        final_avg_loneliness
    );

    // Self-hate should increase (humiliation, shaming)
    assert!(
        final_avg_self_hate > 0.3,
        "Final self-hate should be elevated. Got: {:.3}",
        final_avg_self_hate
    );

    // Interpersonal hopelessness should be elevated (belief that isolation is permanent)
    assert!(
        final_avg_interpersonal_hopelessness > 0.3,
        "Final interpersonal hopelessness should be elevated. Got: {:.3}",
        final_avg_interpersonal_hopelessness
    );
}
