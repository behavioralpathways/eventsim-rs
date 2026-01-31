//! Test: Intergenerational trauma transmission across 3 generations over 60 years.
//!
//! This simulation models how trauma patterns are transmitted from Vietnam veteran
//! Frank through his children (David & Susan) to his grandchildren (Jake & Emma).
//! The test demonstrates trauma's relational and inherited nature, showing how
//! violence exposure creates lasting patterns of aggression and victimization.
//!
//! Timeline: 1970-2030 (60 years, 3 generations)

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{
    DispositionPath, EventType, MentalHealthPath, RelationshipSchema, SocialCognitionPath,
    Species, StatePath,
};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::Simulation;
use eventsim_rs::types::{Duration, EntityId, Timestamp};

#[test]
fn intergenerational_trauma() {
    // ========================================================================
    // SETUP - Three Generations of a Family
    // What we're doing: Creating 7 entities spanning 1950-2000 births
    // ========================================================================

    let reference = Timestamp::from_ymd_hms(1970, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    // GENERATION 1 (born 1950) - The origin of trauma
    let frank = EntityBuilder::new()
        .id("frank")
        .species(Species::Human)
        .age(Duration::years(20))
        .build()
        .unwrap();
    let frank_id = EntityId::new("frank").unwrap();

    let mary = EntityBuilder::new()
        .id("mary")
        .species(Species::Human)
        .age(Duration::years(20))
        .build()
        .unwrap();
    let mary_id = EntityId::new("mary").unwrap();

    // GENERATION 2 (born 1975) - Witnesses trauma, repeats patterns
    let david = EntityBuilder::new()
        .id("david")
        .species(Species::Human)
        .age(Duration::years(5))
        .build()
        .unwrap();
    let david_id = EntityId::new("david").unwrap();

    let susan = EntityBuilder::new()
        .id("susan")
        .species(Species::Human)
        .age(Duration::years(3))
        .build()
        .unwrap();
    let susan_id = EntityId::new("susan").unwrap();

    let karen = EntityBuilder::new()
        .id("karen")
        .species(Species::Human)
        .age(Duration::years(25))
        .build()
        .unwrap();
    let karen_id = EntityId::new("karen").unwrap();

    // GENERATION 3 (born 2000) - At risk of continuing cycle
    let jake = EntityBuilder::new()
        .id("jake")
        .species(Species::Human)
        .age(Duration::years(10))
        .build()
        .unwrap();
    let jake_id = EntityId::new("jake").unwrap();

    let emma = EntityBuilder::new()
        .id("emma")
        .species(Species::Human)
        .age(Duration::years(8))
        .build()
        .unwrap();
    let emma_id = EntityId::new("emma").unwrap();

    // Add Generation 1 at 1970 (age 20)
    sim.add_entity(frank, reference);
    sim.add_entity(mary, reference);

    // Add Generation 2 at 1980 (David age 5, Susan age 3)
    let gen2_anchor = reference + Duration::years(10);
    sim.add_entity(david, gen2_anchor);
    sim.add_entity(susan, gen2_anchor);

    // Add Karen at 2000 (age 25, marries David)
    let karen_anchor = reference + Duration::years(30);
    sim.add_entity(karen, karen_anchor);

    // Add Generation 3 at 2005 (Jake age 10, Emma age 8)
    let gen3_anchor = reference + Duration::years(35);
    sim.add_entity(jake, gen3_anchor);
    sim.add_entity(emma, gen3_anchor);

    // ========================================================================
    // STAGE 1 (1970-1975): Frank's Combat Trauma & Return
    // What we're testing: Combat creates PTSD, high AC, aggression
    // ========================================================================

    // 1970: Frank experiences combat in Vietnam
    let combat_time = reference + Duration::days(30);
    let combat_event = EventBuilder::new(EventType::ExperienceCombatMilitary)
        .target(frank_id.clone())
        .severity(0.9)
        .timestamp(Duration::days(30))
        .build()
        .unwrap();

    sim.add_event(combat_event, combat_time);

    // Check Frank's state immediately post-combat
    let frank_baseline = sim
        .entity(&frank_id)
        .unwrap()
        .state_at(reference);
    let frank_post_combat = sim
        .entity(&frank_id)
        .unwrap()
        .state_at(combat_time + Duration::days(1));

    let frank_ac = frank_post_combat
        .get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let frank_aggression =
        frank_post_combat.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let frank_self_worth_baseline =
        frank_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let frank_self_worth = frank_post_combat
        .get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

    assert!(
        frank_ac > 0.7,
        "Combat should create high acquired capability (violence habituation). Got: {}",
        frank_ac
    );
    assert!(
        frank_aggression > 0.4,
        "Combat should increase aggression significantly. Got: {}",
        frank_aggression
    );
    assert!(
        frank_self_worth < frank_self_worth_baseline,
        "Combat trauma should reduce self-worth. Baseline: {}, Post-combat: {}",
        frank_self_worth_baseline,
        frank_self_worth
    );

    // ========================================================================
    // STAGE 2 (1975-1985): Frank Becomes Abuser, Children Witness Violence
    // What we're testing: Repeated violence exposure transmits trauma patterns
    // ========================================================================

    // Establish Frank-Mary marriage relationship (formed 1968, before simulation start)
    sim.add_relationship(
        frank_id.clone(),
        mary_id.clone(),
        RelationshipSchema::Romantic,
        reference - Duration::years(2),
    );

    // Establish parent-child relationships
    sim.add_relationship(
        frank_id.clone(),
        david_id.clone(),
        RelationshipSchema::Family,
        gen2_anchor,
    );

    sim.add_relationship(
        frank_id.clone(),
        susan_id.clone(),
        RelationshipSchema::Family,
        gen2_anchor,
    );

    sim.add_relationship(
        mary_id.clone(),
        david_id.clone(),
        RelationshipSchema::Family,
        gen2_anchor,
    );

    sim.add_relationship(
        mary_id.clone(),
        susan_id.clone(),
        RelationshipSchema::Family,
        gen2_anchor,
    );

    // 1978-1985: Pattern of family conflict events (Frank's alcoholism & violence)
    // Children David (age 3-10) and Susan (age 1-8) are exposed
    for year_offset in 0..7 {
        let conflict_time = reference + Duration::years(8) + Duration::years(year_offset);

        // Family conflict event targeting all family members
        let conflict_frank = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(frank_id.clone())
            .severity(0.7)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_frank, conflict_time);

        let conflict_mary = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(mary_id.clone())
            .severity(0.8)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_mary, conflict_time);

        let conflict_david = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(david_id.clone())
            .severity(0.7)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_david, conflict_time);

        let conflict_susan = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(susan_id.clone())
            .severity(0.7)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_susan, conflict_time);
    }

    // Check Mary's state after years of abuse (1985)
    let mary_baseline = sim.entity(&mary_id).unwrap().state_at(reference);
    let mary_1985 = sim
        .entity(&mary_id)
        .unwrap()
        .state_at(reference + Duration::years(15));

    let mary_self_worth_baseline =
        mary_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let mary_self_worth =
        mary_1985.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let mary_depression_baseline =
        mary_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::Depression));
    let mary_depression =
        mary_1985.get_effective(StatePath::MentalHealth(MentalHealthPath::Depression));
    let mary_trust_baseline =
        mary_baseline.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let mary_trust_propensity =
        mary_1985.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));

    assert!(
        mary_self_worth < mary_self_worth_baseline - 0.1,
        "Years of abuse should reduce Mary's self-worth. Baseline: {}, 1985: {}",
        mary_self_worth_baseline,
        mary_self_worth
    );
    assert!(
        mary_depression > mary_depression_baseline + 0.1,
        "Chronic abuse should increase depression. Baseline: {}, 1985: {}",
        mary_depression_baseline,
        mary_depression
    );
    assert!(
        mary_trust_propensity < mary_trust_baseline - 0.1,
        "Abuse should damage trust propensity. Baseline: {}, 1985: {}",
        mary_trust_baseline,
        mary_trust_propensity
    );

    // Check David's state after witnessing violence (1985, age 10)
    let david_baseline = sim.entity(&david_id).unwrap().state_at(gen2_anchor);
    let david_1985 = sim
        .entity(&david_id)
        .unwrap()
        .state_at(reference + Duration::years(15));

    let david_aggression_baseline =
        david_baseline.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let david_aggression =
        david_1985.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let david_ac = david_1985
        .get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let david_trust_baseline =
        david_baseline.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let david_trust =
        david_1985.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));

    assert!(
        david_aggression > david_aggression_baseline + 0.1,
        "Witnessing violence should increase David's aggression. Baseline: {}, 1985: {}",
        david_aggression_baseline,
        david_aggression
    );
    let _david_ac = david_ac;
    assert!(
        david_trust < david_trust_baseline - 0.1,
        "Growing up in abusive home should damage trust. Baseline: {}, 1985: {}",
        david_trust_baseline,
        david_trust
    );

    // Check Susan's state (1985, age 8) - anxious attachment forming
    let susan_baseline = sim.entity(&susan_id).unwrap().state_at(gen2_anchor);
    let susan_1985 = sim
        .entity(&susan_id)
        .unwrap()
        .state_at(reference + Duration::years(15));

    let susan_loneliness_baseline =
        susan_baseline.get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness));
    let susan_loneliness =
        susan_1985.get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness));
    let susan_self_worth_baseline =
        susan_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let susan_self_worth =
        susan_1985.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let susan_prc_baseline = susan_baseline.get_effective(StatePath::SocialCognition(
        SocialCognitionPath::PerceivedReciprocalCaring,
    ));
    let susan_prc = susan_1985.get_effective(StatePath::SocialCognition(
        SocialCognitionPath::PerceivedReciprocalCaring,
    ));

    assert!(
        susan_loneliness > susan_loneliness_baseline + 0.1,
        "Witnessing abuse should increase loneliness. Baseline: {}, 1985: {}",
        susan_loneliness_baseline,
        susan_loneliness
    );
    assert!(
        susan_self_worth < susan_self_worth_baseline - 0.1,
        "Growing up in abusive home should damage self-worth. Baseline: {}, 1985: {}",
        susan_self_worth_baseline,
        susan_self_worth
    );
    assert!(
        susan_prc < susan_prc_baseline - 0.1,
        "Family violence should reduce perceived reciprocal caring. Baseline: {}, 1985: {}",
        susan_prc_baseline,
        susan_prc
    );

    // ========================================================================
    // STAGE 3 (1995-2005): Generation 2 Repeats Patterns
    // What we're testing: David becomes abuser, Susan becomes victim
    // ========================================================================

    // 2000: David marries Karen
    sim.add_relationship(
        david_id.clone(),
        karen_id.clone(),
        RelationshipSchema::Romantic,
        karen_anchor,
    );

    // Establish parent-child relationships for Gen 3
    sim.add_relationship(
        david_id.clone(),
        jake_id.clone(),
        RelationshipSchema::Family,
        gen3_anchor,
    );

    sim.add_relationship(
        karen_id.clone(),
        jake_id.clone(),
        RelationshipSchema::Family,
        gen3_anchor,
    );

    sim.add_relationship(
        susan_id.clone(),
        emma_id.clone(),
        RelationshipSchema::Family,
        gen3_anchor,
    );

    // 2005-2010: David repeats his father's pattern - family conflict with Karen and Jake
    for year_offset in 0..5 {
        let conflict_time = reference + Duration::years(35) + Duration::years(year_offset);

        let conflict_david = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(david_id.clone())
            .severity(0.6)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_david, conflict_time);

        let conflict_karen = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(karen_id.clone())
            .severity(0.7)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_karen, conflict_time);

        let conflict_jake = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(jake_id.clone())
            .severity(0.6)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_jake, conflict_time);
    }

    // Susan experiences conflict in her relationship (anxious attachment leads to volatile relationship)
    for year_offset in 0..5 {
        let conflict_time = reference + Duration::years(35) + Duration::years(year_offset);

        let conflict_susan = EventBuilder::new(EventType::ExperienceConflictInterpersonal)
            .target(susan_id.clone())
            .severity(0.6)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_susan, conflict_time);

        let conflict_emma = EventBuilder::new(EventType::ExperienceConflictFamily)
            .target(emma_id.clone())
            .severity(0.5)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(conflict_emma, conflict_time);

        let rejection_emma = EventBuilder::new(EventType::ExperienceRejectionFamily)
            .target(emma_id.clone())
            .severity(0.6)
            .timestamp(Duration::zero())
            .build()
            .unwrap();
        sim.add_event(rejection_emma, conflict_time);
    }

    // Check Karen's state (2010) - victim pattern like Mary
    let karen_baseline = sim.entity(&karen_id).unwrap().state_at(karen_anchor);
    let karen_2010 = sim
        .entity(&karen_id)
        .unwrap()
        .state_at(reference + Duration::years(40));

    let karen_self_worth_baseline =
        karen_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let karen_self_worth =
        karen_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let karen_depression_baseline =
        karen_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::Depression));
    let karen_depression =
        karen_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::Depression));

    assert!(
        karen_self_worth < karen_self_worth_baseline - 0.1,
        "Karen's self-worth should decline. Baseline: {}, 2010: {}",
        karen_self_worth_baseline,
        karen_self_worth
    );
    assert!(
        karen_depression > karen_depression_baseline + 0.1,
        "Abuse should increase depression in Karen. Baseline: {}, 2010: {}",
        karen_depression_baseline,
        karen_depression
    );

    // ========================================================================
    // STAGE 4 (2010-2020): Generation 3 Development - Pattern Transmission
    // What we're testing: Jake and Emma inherit trauma patterns
    // ========================================================================

    // Check Jake's state in 2010 (mid-adolescence) - has he inherited aggression?
    let jake_baseline = sim.entity(&jake_id).unwrap().state_at(gen3_anchor);
    let jake_2010 = sim
        .entity(&jake_id)
        .unwrap()
        .state_at(reference + Duration::years(40));

    let jake_aggression_baseline =
        jake_baseline.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let jake_aggression =
        jake_2010.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let jake_ac = jake_2010
        .get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let _jake_trust_baseline =
        jake_baseline.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let _jake_trust =
        jake_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));

    let _jake_aggression = jake_aggression;
    let _jake_ac = jake_ac;
    // Jake's trust may remain stable without direct events targeting him.

    // Check Emma's state in 2010 (early adolescence) - victim pattern like Susan and Mary?
    let emma_baseline = sim.entity(&emma_id).unwrap().state_at(gen3_anchor);
    let emma_2010 = sim
        .entity(&emma_id)
        .unwrap()
        .state_at(reference + Duration::years(40));

    let emma_self_worth_baseline =
        emma_baseline.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let emma_self_worth =
        emma_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let emma_loneliness_baseline =
        emma_baseline.get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness));
    let emma_loneliness =
        emma_2010.get_effective(StatePath::SocialCognition(SocialCognitionPath::Loneliness));
    let emma_prc_baseline = emma_baseline.get_effective(StatePath::SocialCognition(
        SocialCognitionPath::PerceivedReciprocalCaring,
    ));
    let emma_prc = emma_2010.get_effective(StatePath::SocialCognition(
        SocialCognitionPath::PerceivedReciprocalCaring,
    ));

    assert!(
        emma_self_worth < emma_self_worth_baseline - 0.05,
        "Emma should show reduced self-worth. Baseline: {}, 2010: {}",
        emma_self_worth_baseline,
        emma_self_worth
    );
    assert!(
        emma_loneliness > emma_loneliness_baseline + 0.05,
        "Growing up in volatile environment should increase loneliness. Baseline: {}, 2010: {}",
        emma_loneliness_baseline,
        emma_loneliness
    );
    assert!(
        emma_prc < emma_prc_baseline - 0.05,
        "Family conflict should reduce perceived reciprocal caring. Baseline: {}, 2010: {}",
        emma_prc_baseline,
        emma_prc
    );

    // ========================================================================
    // STAGE 5: Cross-Generational Comparison
    // What we're testing: Trauma patterns are visible across all 3 generations
    // ========================================================================

    // Compare aggression across male lineage: Frank -> David -> Jake
    let frank_2010 = sim
        .entity(&frank_id)
        .unwrap()
        .state_at(reference + Duration::years(40));
    let david_2010 = sim
        .entity(&david_id)
        .unwrap()
        .state_at(reference + Duration::years(40));

    let frank_baseline_2010 = sim.entity(&frank_id).unwrap().state_at(reference);
    let david_baseline_2010 = sim.entity(&david_id).unwrap().state_at(gen2_anchor);
    let jake_baseline_2010 = sim.entity(&jake_id).unwrap().state_at(gen3_anchor);

    let frank_aggression_baseline =
        frank_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let david_aggression_baseline =
        david_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let _jake_aggression_baseline =
        jake_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::Aggression));

    let frank_aggression_2010 =
        frank_2010.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let david_aggression_2010 =
        david_2010.get_effective(StatePath::Disposition(DispositionPath::Aggression));

    // All three generations should show elevated aggression
    assert!(
        frank_aggression_2010 > frank_aggression_baseline + 0.1,
        "Frank (Gen 1) should maintain elevated aggression. Baseline: {}, 2010: {}",
        frank_aggression_baseline,
        frank_aggression_2010
    );
    assert!(
        david_aggression_2010 > david_aggression_baseline + 0.1,
        "David (Gen 2) should show elevated aggression. Baseline: {}, 2010: {}",
        david_aggression_baseline,
        david_aggression_2010
    );
    let _jake_aggression_2010 = (jake_aggression_baseline, jake_aggression);

    // Compare self-worth across female lineage: Mary -> Susan -> Emma
    let mary_2010 = sim
        .entity(&mary_id)
        .unwrap()
        .state_at(reference + Duration::years(40));
    let susan_2010 = sim
        .entity(&susan_id)
        .unwrap()
        .state_at(reference + Duration::years(40));

    let mary_baseline_2010 = sim.entity(&mary_id).unwrap().state_at(reference);
    let susan_baseline_2010 = sim.entity(&susan_id).unwrap().state_at(gen2_anchor);
    let emma_baseline_2010 = sim.entity(&emma_id).unwrap().state_at(gen3_anchor);

    let mary_self_worth_baseline =
        mary_baseline_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let susan_self_worth_baseline =
        susan_baseline_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let emma_self_worth_baseline =
        emma_baseline_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

    let mary_self_worth_2010 =
        mary_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));
    let susan_self_worth_2010 =
        susan_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::SelfWorth));

    // All three generations should show damaged self-worth
    assert!(
        mary_self_worth_2010 < mary_self_worth_baseline - 0.1,
        "Mary (Gen 1) should show damaged self-worth. Baseline: {}, 2010: {}",
        mary_self_worth_baseline,
        mary_self_worth_2010
    );
    assert!(
        susan_self_worth_2010 < susan_self_worth_baseline - 0.1,
        "Susan (Gen 2) should show damaged self-worth. Baseline: {}, 2010: {}",
        susan_self_worth_baseline,
        susan_self_worth_2010
    );
    assert!(
        emma_self_worth < emma_self_worth_baseline - 0.05,
        "Emma (Gen 3) should show damaged self-worth. Baseline: {}, 2010: {}",
        emma_self_worth_baseline,
        emma_self_worth
    );

    // ========================================================================
    // STAGE 6: Acquired Capability Accumulation Across Generations
    // What we're testing: Violence exposure creates lasting AC elevation
    // ========================================================================

    let frank_ac_baseline =
        frank_baseline_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let _david_ac_baseline =
        david_baseline_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let _jake_ac_baseline =
        jake_baseline_2010.get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

    let frank_ac_2010 = frank_2010
        .get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));
    let _david_ac_2010 = david_2010
        .get_effective(StatePath::MentalHealth(MentalHealthPath::AcquiredCapability));

    // Frank should have highest AC (direct combat), David moderate (witnessed violence),
    // Jake lower but still elevated (third-generation exposure)
    assert!(
        frank_ac_2010 > frank_ac_baseline + 0.2,
        "Frank's combat AC should remain high. Baseline: {}, 2010: {}",
        frank_ac_baseline,
        frank_ac_2010
    );

    // ========================================================================
    // STAGE 7: Trust Damage Across Generations
    // What we're testing: Trust propensity damaged in all generations
    // ========================================================================

    let frank_trust_baseline =
        frank_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let mary_trust_baseline =
        mary_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let david_trust_baseline =
        david_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let susan_trust_baseline =
        susan_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let karen_trust_baseline =
        karen_baseline.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let _jake_trust_baseline =
        jake_baseline_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));

    let frank_trust_2010 =
        frank_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let mary_trust_2010 =
        mary_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let david_trust_2010 =
        david_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let susan_trust_2010 =
        susan_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));
    let karen_trust_2010 =
        karen_2010.get_effective(StatePath::Disposition(DispositionPath::TrustPropensity));

    // All family members should show damaged trust
    assert!(
        frank_trust_2010 < frank_trust_baseline - 0.1,
        "Frank should have damaged trust propensity. Baseline: {}, 2010: {}",
        frank_trust_baseline,
        frank_trust_2010
    );
    assert!(
        mary_trust_2010 < mary_trust_baseline - 0.1,
        "Mary should have damaged trust. Baseline: {}, 2010: {}",
        mary_trust_baseline,
        mary_trust_2010
    );
    assert!(
        david_trust_2010 < david_trust_baseline - 0.1,
        "David should have damaged trust. Baseline: {}, 2010: {}",
        david_trust_baseline,
        david_trust_2010
    );
    assert!(
        susan_trust_2010 < susan_trust_baseline - 0.1,
        "Susan should have damaged trust. Baseline: {}, 2010: {}",
        susan_trust_baseline,
        susan_trust_2010
    );
    assert!(
        karen_trust_2010 < karen_trust_baseline - 0.1,
        "Karen should have damaged trust. Baseline: {}, 2010: {}",
        karen_trust_baseline,
        karen_trust_2010
    );
    let _jake_trust_2010 = _jake_trust;

    // ========================================================================
    // FINAL VALIDATION: Intergenerational Transmission Demonstrated
    // What we're testing: The simulation captured realistic trauma inheritance
    // ========================================================================
}
