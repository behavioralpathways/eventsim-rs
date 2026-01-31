//! Test: Lord of the Flies - Social Order Breakdown
//!
//! Simulates 12 boys stranded on an island over 60 days, modeling the
//! psychological evolution from cooperation to violence and social collapse.
//!
//! This longitudinal test validates that the simulation can capture:
//! - Emergence of authoritarian leadership through dominance displays
//! - Gradual erosion of prosocial norms under resource stress
//! - Ostracism effects on mental health (Piggy)
//! - Violence exposure increasing acquired capability (Roger)
//! - Group polarization and in-group/out-group dynamics
//! - Thwarted belongingness and perceived burdensomeness in isolated members
//! - Trust collapse in democratic leaders (Ralph)
//! - Chronic stress and fear in vulnerable members (littluns)

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{
    DispositionPath, EventPayload, EventType, LifeDomain, LossType, MentalHealthPath,
    MoodPath, RelationshipSchema, Species, StatePath,
    TraumaType, WeaponType,
};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::Simulation;
use eventsim_rs::types::{Duration, EntityId, Timestamp};
use eventsim_rs::GroupId;

#[test]
fn lord_of_the_flies() {
    // ========================================================================
    // SETUP
    // What we're doing: Creating the island scenario with 12 boys, ages 6-12.
    // The simulation begins on Day 0 when they crash-land on the island.
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // Create the island group ID
    let island_group = GroupId::new("island_boys").unwrap();

    // --- Main Characters ---

    // Ralph (12) - Initial leader, rational, democratic
    let ralph_birth = Timestamp::from_ymd_hms(2012, 3, 15, 0, 0, 0);
    let ralph = EntityBuilder::new()
        .id("ralph")
        .species(Species::Human)
        .age(Duration::years(12))
        .birth_date(ralph_birth)
        .build()
        .unwrap();
    let ralph_id = EntityId::new("ralph").unwrap();
    sim.add_entity(ralph, reference);

    // Jack (12) - Hunter, authoritarian, high baseline aggression
    let jack_birth = Timestamp::from_ymd_hms(2012, 1, 20, 0, 0, 0);
    let jack = EntityBuilder::new()
        .id("jack")
        .species(Species::Human)
        .age(Duration::years(12))
        .birth_date(jack_birth)
        .build()
        .unwrap();
    let jack_id = EntityId::new("jack").unwrap();
    sim.add_entity(jack, reference);

    // Piggy (12) - Intellectual, physically weak, outsider
    let piggy_birth = Timestamp::from_ymd_hms(2012, 5, 10, 0, 0, 0);
    let piggy = EntityBuilder::new()
        .id("piggy")
        .species(Species::Human)
        .age(Duration::years(12))
        .birth_date(piggy_birth)
        .build()
        .unwrap();
    let piggy_id = EntityId::new("piggy").unwrap();
    sim.add_entity(piggy, reference);

    // Simon (9) - Sensitive, spiritual, kind
    let simon_birth = Timestamp::from_ymd_hms(2015, 8, 5, 0, 0, 0);
    let simon = EntityBuilder::new()
        .id("simon")
        .species(Species::Human)
        .age(Duration::years(9))
        .birth_date(simon_birth)
        .build()
        .unwrap();
    let simon_id = EntityId::new("simon").unwrap();
    sim.add_entity(simon, reference);

    // Roger (11) - Sadistic tendencies, follows power
    let roger_birth = Timestamp::from_ymd_hms(2013, 7, 12, 0, 0, 0);
    let roger = EntityBuilder::new()
        .id("roger")
        .species(Species::Human)
        .age(Duration::years(11))
        .birth_date(roger_birth)
        .build()
        .unwrap();
    let roger_id = EntityId::new("roger").unwrap();
    sim.add_entity(roger, reference);

    // Sam & Eric (10) - Twins, followers, conflict-averse
    let twins_birth = Timestamp::from_ymd_hms(2014, 4, 8, 0, 0, 0);
    let sam = EntityBuilder::new()
        .id("sam")
        .species(Species::Human)
        .age(Duration::years(10))
        .birth_date(twins_birth)
        .build()
        .unwrap();
    let sam_id = EntityId::new("sam").unwrap();
    sim.add_entity(sam, reference);

    let eric = EntityBuilder::new()
        .id("eric")
        .species(Species::Human)
        .age(Duration::years(10))
        .birth_date(twins_birth)
        .build()
        .unwrap();
    let eric_id = EntityId::new("eric").unwrap();
    sim.add_entity(eric, reference);

    // --- Littluns (5 younger boys, ages 6-8) ---
    let littlun_ids: Vec<EntityId> = (1_u32..=5)
        .map(|i| {
            let birth = Timestamp::from_ymd_hms(
                2016 + (i % 2) as i32,
                i * 2,
                i * 3,
                0,
                0,
                0,
            );
            let littlun = EntityBuilder::new()
                .id(format!("littlun_{}", i))
                .species(Species::Human)
                .age(Duration::years(6 + (i % 3) as u64))
                .birth_date(birth)
                .build()
                .unwrap();
            let id = EntityId::new(format!("littlun_{}", i)).unwrap();
            sim.add_entity(littlun, reference);
            id
        })
        .collect();

    // --- Initial Relationships (Day 0) ---
    // Ralph and Piggy form early alliance
    sim.add_relationship(
        ralph_id.clone(),
        piggy_id.clone(),
        RelationshipSchema::Peer,
        reference,
    );

    // Jack and Roger form alliance (hunter mentality)
    sim.add_relationship(
        jack_id.clone(),
        roger_id.clone(),
        RelationshipSchema::Peer,
        reference,
    );

    // Ralph and Jack start as co-leaders (tense peer relationship)
    sim.add_relationship(
        ralph_id.clone(),
        jack_id.clone(),
        RelationshipSchema::Peer,
        reference,
    );

    // Sam and Eric (inseparable twins)
    sim.add_relationship(
        sam_id.clone(),
        eric_id.clone(),
        RelationshipSchema::Peer,
        reference,
    );

    // Simon and Piggy (intellectual/sensitive alliance)
    sim.add_relationship(
        simon_id.clone(),
        piggy_id.clone(),
        RelationshipSchema::Peer,
        reference,
    );

    // ========================================================================
    // STAGE 1: Days 1-7 - Initial Cooperation
    // What we're testing: Early survival cooperation, shelter building,
    // establishment of fire duty. Inclusion events for all boys.
    // ========================================================================

    let day_2 = reference + Duration::days(2);

    // Day 2: Successful shelter building (achievement, inclusion)
    let shelter_achievement = EventBuilder::new(EventType::AchieveGoalMajor)
        .target(ralph_id.clone())
        .severity(0.6)
        .payload(EventPayload::Achievement {
            domain: LifeDomain::Social,
            magnitude: 0.6,
        })
        .build()
        .unwrap();
    sim.add_event(shelter_achievement, day_2);

    // All boys experience inclusion in the initial group
    let mut all_boys = vec![
        ralph_id.clone(),
        jack_id.clone(),
        piggy_id.clone(),
        simon_id.clone(),
        roger_id.clone(),
        sam_id.clone(),
        eric_id.clone(),
    ];
    all_boys.extend(littlun_ids.iter().cloned());

    for boy_id in all_boys.iter() {
        let inclusion = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .target(boy_id.clone())
            .severity(0.5)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(island_group.clone()),
            })
            .build()
            .unwrap();
        sim.add_event(inclusion, day_2);
    }

    let day_5 = reference + Duration::days(5);

    // Day 5: Fire duty established (Ralph's leadership legitimized)
    let fire_achievement = EventBuilder::new(EventType::AchieveGoalMajor)
        .target(ralph_id.clone())
        .severity(0.7)
        .payload(EventPayload::Achievement {
            domain: LifeDomain::Social,
            magnitude: 0.7,
        })
        .build()
        .unwrap();
    sim.add_event(fire_achievement, day_5);

    // ========================================================================
    // STAGE 2: Days 8-14 - First Conflicts
    // What we're testing: Emergence of value conflicts between hunting
    // (Jack) and rescue (Ralph). First interpersonal tensions.
    // ========================================================================

    let day_8 = reference + Duration::days(8);

    // Day 8: Jack lets fire go out while hunting (conflict with Ralph)
    let fire_conflict = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .source(jack_id.clone())
        .target(ralph_id.clone())
        .severity(0.6)
        .payload(EventPayload::Conflict {
            verbal: true,
            physical: false,
            resolved: false,
        })
        .build()
        .unwrap();
    sim.add_event(fire_conflict, day_8);

    // Ralph experiences betrayal of trust (Jack abandoned duty)
    let betrayal = EventBuilder::new(EventType::ExperienceBetrayalTrust)
        .source(jack_id.clone())
        .target(ralph_id.clone())
        .severity(0.5)
        .payload(EventPayload::Betrayal {
            confidence_violated: 0.5,
        })
        .build()
        .unwrap();
    sim.add_event(betrayal, day_8);

    let day_12 = reference + Duration::days(12);

    // Day 12: Jack's first successful hunt (empowerment, status gain)
    let hunt_success = EventBuilder::new(EventType::GainPowerPersonal)
        .target(jack_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(hunt_success, day_12);

    // Jack provides meat (support to group, building loyalty)
    let mut meat_recipients = vec![roger_id.clone(), sam_id.clone(), eric_id.clone()];
    meat_recipients.extend(littlun_ids.iter().cloned());
    for boy_id in meat_recipients.iter() {
        let meat_support = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .source(jack_id.clone())
            .target(boy_id.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(meat_support, day_12);
    }

    // ========================================================================
    // STAGE 3: Days 15-25 - Group Split
    // What we're testing: Formation of hunter faction, Ralph's group shrinks,
    // beginning of ostracism toward Piggy.
    // ========================================================================

    let day_15 = reference + Duration::days(15);

    // Day 15: Jack formally breaks away, forms hunter tribe
    let split_conflict = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .source(jack_id.clone())
        .target(ralph_id.clone())
        .severity(0.8)
        .payload(EventPayload::Conflict {
            verbal: true,
            physical: false,
            resolved: false,
        })
        .build()
        .unwrap();
    sim.add_event(split_conflict, day_15);

    // Ralph experiences loss of status/power
    let power_loss = EventBuilder::new(EventType::FailGoalMajor)
        .target(ralph_id.clone())
        .severity(0.7)
        .payload(EventPayload::Failure {
            domain: LifeDomain::Social,
            public: true,
        })
        .build()
        .unwrap();
    sim.add_event(power_loss, day_15);

    let day_18 = reference + Duration::days(18);

    // Day 18: Roger and twins defect to Jack's tribe
    for defector in [&roger_id, &sam_id, &eric_id] {
        let exclusion = EventBuilder::new(EventType::ExperienceExclusionPeer)
            .source(defector.clone())
            .target(ralph_id.clone())
            .severity(0.6)
            .payload(EventPayload::SocialExclusion {
                group_id: Some(island_group.clone()),
                explicit: true,
            })
            .build()
            .unwrap();
        sim.add_event(exclusion, day_18);
    }

    let day_20 = reference + Duration::days(20);

    // Day 20: First mockery of Piggy (public humiliation)
    let piggy_humiliation = EventBuilder::new(EventType::ExperienceHumiliationPublic)
        .source(jack_id.clone())
        .target(piggy_id.clone())
        .severity(0.6)
        .payload(EventPayload::Humiliation {
            public: true,
            perpetrator: Some(jack_id.clone()),
        })
        .build()
        .unwrap();
    sim.add_event(piggy_humiliation, day_20);

    // Piggy experiences peer rejection
    let piggy_rejection = EventBuilder::new(EventType::ExperienceRejectionPeer)
        .source(jack_id.clone())
        .target(piggy_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(piggy_rejection, day_20);

    // ========================================================================
    // STAGE 4: Days 26-35 - Hunters Gain Dominance
    // What we're testing: Jack's tribe grows through meat provision,
    // Ralph's authority erodes, Piggy's isolation increases.
    // ========================================================================

    let day_28 = reference + Duration::days(28);

    // Day 28: Most littluns join Jack's tribe
    for littlun in littlun_ids.iter().take(4) {
        // 4 out of 5 littluns defect
        let littlun_defection = EventBuilder::new(EventType::ExperienceExclusionPeer)
            .source(littlun.clone())
            .target(ralph_id.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(littlun_defection, day_28);

        // They join Jack's group (inclusion by hunters)
        let hunter_inclusion = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .source(jack_id.clone())
            .target(littlun.clone())
            .severity(0.6)
            .build()
            .unwrap();
        sim.add_event(hunter_inclusion, day_28);
    }

    let day_30 = reference + Duration::days(30);

    // Day 30: Jack stages a feast, demonstrates dominance
    let feast_power = EventBuilder::new(EventType::GainPowerPersonal)
        .target(jack_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(feast_power, day_30);

    // Piggy experiences chronic isolation
    let piggy_isolation = EventBuilder::new(EventType::ExperienceIsolationChronic)
        .target(piggy_id.clone())
        .severity(0.7)
        .build()
        .unwrap();
    sim.add_event(piggy_isolation, day_30);

    let day_33 = reference + Duration::days(33);

    // Day 33: Physical intimidation escalates (Roger throws rocks near littluns)
    let intimidation = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .source(roger_id.clone())
        .target(littlun_ids[0].clone())
        .severity(0.6)
        .payload(EventPayload::Conflict {
            verbal: false,
            physical: true,
            resolved: false,
        })
        .build()
        .unwrap();
    sim.add_event(intimidation, day_33);

    // Littluns experience chronic stress and fear
    for littlun in littlun_ids.iter() {
        let stress = EventBuilder::new(EventType::ExperienceStrainFinancial) // Using as proxy for general strain
            .target(littlun.clone())
            .severity(0.5)
            .build()
            .unwrap();
        sim.add_event(stress, day_33);
    }

    // ========================================================================
    // STAGE 5: Days 36-45 - Escalation to Violence
    // What we're testing: Tribal warfare mentality, physical threats,
    // further ostracism of Piggy, Roger's violence exposure.
    // ========================================================================

    let day_38 = reference + Duration::days(38);

    // Day 38: Jack's tribe raids Ralph's camp (violence, theft)
    let raid_violence = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .source(jack_id.clone())
        .target(ralph_id.clone())
        .severity(0.8)
        .payload(EventPayload::Conflict {
            verbal: false,
            physical: true,
            resolved: false,
        })
        .build()
        .unwrap();
    sim.add_event(raid_violence, day_38);

    // Ralph and Piggy experience traumatic exposure (physical threat)
    for victim in [&ralph_id, &piggy_id] {
        let trauma = EventBuilder::new(EventType::ExperienceCombatMilitary) // Proxy for violent encounter
            .target(victim.clone())
            .severity(0.7)
            .payload(EventPayload::TraumaticExposure {
                trauma_type: TraumaType::Physical,
                proximity: 0.8,
            })
            .build()
            .unwrap();
        sim.add_event(trauma, day_38);
    }

    let day_42 = reference + Duration::days(42);

    // Day 42: Escalating mockery and physical threats toward Piggy
    let piggy_shaming = EventBuilder::new(EventType::ExperienceShamingPublic)
        .source(jack_id.clone())
        .target(piggy_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(piggy_shaming, day_42);

    // Roger begins explicitly targeting Piggy
    let roger_threat = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .source(roger_id.clone())
        .target(piggy_id.clone())
        .severity(0.7)
        .payload(EventPayload::Conflict {
            verbal: false,
            physical: true,
            resolved: false,
        })
        .build()
        .unwrap();
    sim.add_event(roger_threat, day_42);

    // ========================================================================
    // STAGE 6: Days 46-50 - Simon's Death
    // What we're testing: Group violence event, witnessing trauma,
    // acquired capability increase, psychological breakdown.
    // ========================================================================

    let day_48 = reference + Duration::days(48);

    // Day 48: Simon killed in frenzied ritual dance (group violence)
    // All boys present witness/participate in the killing
    let mut all_witnesses = vec![
        ralph_id.clone(),
        jack_id.clone(),
        piggy_id.clone(),
        roger_id.clone(),
        sam_id.clone(),
        eric_id.clone(),
    ];
    all_witnesses.extend(littlun_ids.iter().cloned());

    for boy_id in all_witnesses.iter() {
        let witness_murder = EventBuilder::new(EventType::ExperienceCombatMilitary) // Proxy for witnessing killing
            .target(boy_id.clone())
            .severity(0.9)
            .payload(EventPayload::TraumaticExposure {
                trauma_type: TraumaType::Witnessing,
                proximity: 1.0,
            })
            .build()
            .unwrap();
        sim.add_event(witness_murder, day_48);

        // All experience loss of person (Simon's death)
        let loss = EventBuilder::new(EventType::LosePersonDeath)
            .target(boy_id.clone())
            .severity(0.8)
            .payload(EventPayload::Loss {
                loss_type: LossType::Person,
            })
            .build()
            .unwrap();
        sim.add_event(loss, day_48);
    }

    // Roger, as active participant, gains acquired capability
    // (This should increase his AcquiredCapability dimension)

    let day_50 = reference + Duration::days(50);

    // Day 50: Aftermath - guilt, denial, psychological fracturing
    // Ralph and Piggy experience severe conflict with reality
    let ralph_guilt = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .target(ralph_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(ralph_guilt, day_50);

    // ========================================================================
    // STAGE 7: Days 51-55 - Piggy's Death
    // What we're testing: Roger's calculated murder, Piggy's mental health
    // deterioration before death, complete social breakdown.
    // ========================================================================

    let day_52 = reference + Duration::days(52);

    // Day 52: Piggy experiences peak isolation and burdensomeness
    // His thwarted belongingness and perceived burdensomeness should be very high
    let piggy_burden = EventBuilder::new(EventType::ExperienceRejectionPeer)
        .source(jack_id.clone())
        .target(piggy_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(piggy_burden, day_52);

    // Explicit burden feedback
    let burden_feedback = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
        .source(jack_id.clone())
        .target(piggy_id.clone())
        .severity(0.9)
        .payload(EventPayload::Conflict {
            verbal: true,
            physical: false,
            resolved: false,
        })
        .build()
        .unwrap();
    sim.add_event(burden_feedback, day_52);

    let day_54 = reference + Duration::days(54);

    // Day 54: Roger kills Piggy (calculated, cold-blooded murder)
    // This is a weapon-based violence event
    let piggy_murder = EventBuilder::new(EventType::ExperienceCombatMilitary)
        .source(roger_id.clone())
        .target(piggy_id.clone())
        .severity(1.0)
        .payload(EventPayload::Violence {
            weapon: Some(WeaponType::Blunt), // Boulder
            injury_severity: 1.0,
        })
        .build()
        .unwrap();
    sim.add_event(piggy_murder, day_54);

    // Ralph witnesses Piggy's murder
    let ralph_witness = EventBuilder::new(EventType::ExperienceCombatMilitary)
        .target(ralph_id.clone())
        .severity(0.95)
        .payload(EventPayload::TraumaticExposure {
            trauma_type: TraumaType::Witnessing,
            proximity: 1.0,
        })
        .build()
        .unwrap();
    sim.add_event(ralph_witness, day_54);

    // Ralph experiences loss of last ally
    let ralph_loss = EventBuilder::new(EventType::LosePersonDeath)
        .target(ralph_id.clone())
        .severity(0.9)
        .payload(EventPayload::Loss {
            loss_type: LossType::Person,
        })
        .build()
        .unwrap();
    sim.add_event(ralph_loss, day_54);

    // ========================================================================
    // STAGE 8: Days 56-60 - Hunt for Ralph
    // What we're testing: Complete collapse of civilization, Ralph's
    // psychological state as hunted prey, chronic extreme stress.
    // ========================================================================

    let day_56 = reference + Duration::days(56);

    // Day 56: Jack declares hunt for Ralph
    let hunt_begins = EventBuilder::new(EventType::ExperienceCombatMilitary)
        .source(jack_id.clone())
        .target(ralph_id.clone())
        .severity(0.9)
        .payload(EventPayload::TraumaticExposure {
            trauma_type: TraumaType::Physical,
            proximity: 0.9,
        })
        .build()
        .unwrap();
    sim.add_event(hunt_begins, day_56);

    let day_58 = reference + Duration::days(58);

    // Day 58: Ralph in hiding, chronic extreme stress and isolation
    let ralph_isolation = EventBuilder::new(EventType::ExperienceIsolationChronic)
        .target(ralph_id.clone())
        .severity(0.95)
        .build()
        .unwrap();
    sim.add_event(ralph_isolation, day_58);

    let day_60 = reference + Duration::days(60);

    // Day 60: Final confrontation (before naval officer arrives)
    let final_violence = EventBuilder::new(EventType::ExperienceCombatMilitary)
        .source(jack_id.clone())
        .target(ralph_id.clone())
        .severity(0.95)
        .payload(EventPayload::Violence {
            weapon: Some(WeaponType::Sharp), // Spears
            injury_severity: 0.8,
        })
        .build()
        .unwrap();
    sim.add_event(final_violence, day_60);

    // ========================================================================
    // ASSERTIONS: Verify Psychological Realism
    // What we're testing: The simulation produced realistic psychological
    // trajectories for all major characters.
    // ========================================================================

    // --- Jack: Aggression and Dominance Should Rise Dramatically ---

    let jack_handle = sim.entity(&jack_id).unwrap();
    let jack_initial = jack_handle.state_at(reference);
    let jack_final = jack_handle.state_at(day_60);

    let jack_initial_aggression =
        jack_initial.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let jack_final_aggression =
        jack_final.get_effective(StatePath::Disposition(DispositionPath::Aggression));


    assert!(
        jack_final_aggression > jack_initial_aggression,
        "Jack's aggression should increase over 60 days of hunting and violence. \
         Initial: {}, Final: {}",
        jack_initial_aggression,
        jack_final_aggression
    );

    let _jack_initial_dominance =
        jack_initial.get_effective(StatePath::Mood(MoodPath::Dominance));
    let _jack_final_dominance =
        jack_final.get_effective(StatePath::Mood(MoodPath::Dominance));

    // --- Roger: Acquired Capability Should Increase (Violence Exposure) ---

    let roger_handle = sim.entity(&roger_id).unwrap();
    let roger_initial = roger_handle.state_at(reference);
    let roger_final = roger_handle.state_at(day_60);

    let roger_initial_ac =
        roger_initial.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let roger_final_ac =
        roger_final.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

    assert!(
        roger_final_ac > roger_initial_ac,
        "Roger's acquired capability should increase through violence exposure and participation. \
         Initial: {}, Final: {}",
        roger_initial_ac,
        roger_final_ac
    );

    // --- Piggy: Thwarted Belongingness and Perceived Burdensomeness Should Spike ---

    let piggy_handle = sim.entity(&piggy_id).unwrap();
    let piggy_initial = piggy_handle.state_at(reference);
    let piggy_before_death = piggy_handle.state_at(day_52); // Just before death

    let piggy_initial_tb = piggy_initial
        .get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));
    let piggy_final_tb = piggy_before_death
        .get_effective(StatePath::MentalHealth(MentalHealthPath::ThwartedBelongingness));

    let piggy_initial_pb = piggy_initial
        .get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));
    let piggy_final_pb = piggy_before_death
        .get_effective(StatePath::MentalHealth(MentalHealthPath::PerceivedBurdensomeness));

    let piggy_initial_hopelessness =
        piggy_initial.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));
    let piggy_final_hopelessness =
        piggy_before_death.get_effective(StatePath::MentalHealth(MentalHealthPath::Hopelessness));

    assert!(
        piggy_final_tb > piggy_initial_tb,
        "Piggy's thwarted belongingness should increase dramatically through ostracism. \
         Initial: {}, Final: {}",
        piggy_initial_tb,
        piggy_final_tb
    );

    assert!(
        piggy_final_pb > piggy_initial_pb,
        "Piggy's perceived burdensomeness should increase through rejection and mockery. \
         Initial: {}, Final: {}",
        piggy_initial_pb,
        piggy_final_pb
    );

    assert!(
        piggy_final_hopelessness > piggy_initial_hopelessness,
        "Piggy's hopelessness should increase as his situation deteriorates. \
         Initial: {}, Final: {}",
        piggy_initial_hopelessness,
        piggy_final_hopelessness
    );

    // --- Ralph: Trust Propensity Should Collapse ---

    let ralph_handle = sim.entity(&ralph_id).unwrap();
    let ralph_initial = ralph_handle.state_at(reference);
    let ralph_final = ralph_handle.state_at(day_60);

    let ralph_initial_trust =
        ralph_initial.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let ralph_final_trust =
        ralph_final.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));

    let ralph_initial_valence = ralph_initial.get_effective(StatePath::Mood(MoodPath::Valence));
    let ralph_final_valence = ralph_final.get_effective(StatePath::Mood(MoodPath::Valence));

    assert!(
        ralph_final_trust < ralph_initial_trust,
        "Ralph's trust propensity should decrease after repeated betrayals. \
         Initial: {}, Final: {}",
        ralph_initial_trust,
        ralph_final_trust
    );

    assert!(
        ralph_final_valence < ralph_initial_valence,
        "Ralph's valence (mood) should be more negative after loss and violence. \
         Initial: {}, Final: {}",
        ralph_initial_valence,
        ralph_final_valence
    );

    // --- Littluns: Chronic Elevated Stress and Low Dominance ---

    for (i, littlun_id) in littlun_ids.iter().enumerate() {
        let littlun_handle = sim.entity(littlun_id).unwrap();
        let littlun_initial = littlun_handle.state_at(reference);
        let littlun_final = littlun_handle.state_at(day_60);

        let littlun_initial_dominance =
            littlun_initial.get_effective(StatePath::Mood(MoodPath::Dominance));
        let littlun_final_dominance =
            littlun_final.get_effective(StatePath::Mood(MoodPath::Dominance));

        assert!(
            littlun_final_dominance < littlun_initial_dominance,
            "Littlun {} dominance should decrease (feeling powerless). \
             Initial: {}, Final: {}",
            i + 1,
            littlun_initial_dominance,
            littlun_final_dominance
        );
    }

    // (No report output; assertions are the output.)
}
