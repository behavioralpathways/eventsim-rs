//! Longitudinal scenario: Trust amplifies betrayal
//!
//! Two women, Sarah and Emma, both experience betrayal. Sarah's betrayal comes
//! from her best friend of 15 years, Rachel, who she trusted completely. Emma's
//! betrayal comes from a recent acquaintance, Lisa, with whom she had minimal
//! rapport.
//!
//! The betrayal event is identical: both discover their confidant shared deeply
//! personal information about them with others. But the psychological impact
//! differs dramatically.
//!
//! Sarah's world shatters. She trusted Rachel with everything - her fears, her
//! insecurities, her marriage troubles. The betrayal doesn't just hurt, it
//! fundamentally damages her ability to trust anyone. Years later, she still
//! struggles to open up to new friends.
//!
//! Emma is disappointed, even angry, but she recovers faster. She never really
//! trusted Lisa anyway. The betrayal confirms her instinct that Lisa wasn't
//! trustworthy. It stings, but it doesn't change her fundamental beliefs about
//! relationships.
//!
//! This test validates that the system correctly models how prior trust level
//! amplifies the psychological impact of betrayal.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, RelationshipSchema, Species};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::Simulation;
use eventsim_rs::types::{Duration, EntityId, Timestamp};

const ANCHOR_AGE: u64 = 35;

#[test]
#[ignore]
fn trust_amplifies_betrayal() {
    let birth_date = Timestamp::from_ymd_hms(1985, 3, 15, 0, 0, 0);
    let reference = birth_date + Duration::years(ANCHOR_AGE);

    // === SARAH'S SCENARIO: HIGH-TRUST BETRAYAL ===

    let (mut sarah_sim, sarah_id, rachel_id) = setup_high_trust_scenario(birth_date, reference);

    // Build trust between Sarah and Rachel through RECENT events
    // Temporal decay means trust is built through recent, consistent experiences
    // Most trust weight comes from the past 6 months (180-day half-life)
    //
    // Sarah has been best friends with Rachel for years, but what matters
    // for her psychological vulnerability is how much she currently trusts Rachel
    // This is built through recent positive experiences
    let trust_building_events: Vec<(u64, u64, EventType, f64)> = vec![
        // Recent intense trust-building period (past year)
        // Sarah went through a hard time, Rachel was there constantly
        // Using days offset from reference: (years, days_in_year, event, severity)

        // 11 months ago - Rachel there during a crisis
        (0, 330, EventType::ReceiveSupportEmotional, 0.9),
        (0, 325, EventType::ReceiveSupportEmotional, 0.8),

        // 10 months ago - continued support
        (0, 300, EventType::ReceiveSupportEmotional, 0.8),

        // 8 months ago - another difficult period
        (0, 240, EventType::ReceiveSupportEmotional, 0.9),
        (0, 235, EventType::ReceiveSupportEmotional, 0.8),

        // 6 months ago - regular friend time
        (0, 180, EventType::ReceiveSupportEmotional, 0.8),
        (0, 175, EventType::ReceiveSupportEmotional, 0.7),

        // 4 months ago - more support
        (0, 120, EventType::ReceiveSupportEmotional, 0.8),
        (0, 115, EventType::ReceiveSupportEmotional, 0.7),

        // 2 months ago - close calls and check-ins
        (0, 60, EventType::ReceiveSupportEmotional, 0.8),
        (0, 55, EventType::ReceiveSupportEmotional, 0.8),
        (0, 50, EventType::ReceiveSupportEmotional, 0.7),

        // 1 month ago - recent meaningful interactions
        (0, 30, EventType::ReceiveSupportEmotional, 0.9),
        (0, 25, EventType::ReceiveSupportEmotional, 0.8),
        (0, 20, EventType::ReceiveSupportEmotional, 0.8),

        // 2 weeks ago - very recent trust reinforcement
        (0, 14, EventType::ReceiveSupportEmotional, 0.9),
        (0, 10, EventType::ReceiveSupportEmotional, 0.8),

        // 1 week ago - just before the betrayal
        (0, 7, EventType::ReceiveSupportEmotional, 0.8),
        (0, 5, EventType::ReceiveSupportEmotional, 0.9),
        (0, 3, EventType::ReceiveSupportEmotional, 0.8),
    ];

    // Events are offset as days BEFORE the betrayal
    // Note: betrayal must happen AFTER anchor time to be included in forward projection
    // (anchor, target] - events at anchor are excluded
    let betrayal_time = reference + Duration::days(1); // 1 day after anchor

    for (_, days_before, event_type, severity) in trust_building_events {
        let event_time = betrayal_time - Duration::days(days_before);
        let event = EventBuilder::new(event_type)
            .source(rachel_id.clone())
            .target(sarah_id.clone())
            .severity(severity)
            .build()
            .unwrap();
        sarah_sim.add_event(event, event_time);
    }
    let sarah_betrayal = EventBuilder::new(EventType::ExperienceBetrayalTrust)
        .source(rachel_id.clone())
        .target(sarah_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sarah_sim.add_event(sarah_betrayal, betrayal_time);

    // === EMMA'S SCENARIO: STRANGER BETRAYAL (NO RELATIONSHIP) ===

    let (mut emma_sim, emma_id, lisa_id) = setup_stranger_scenario(birth_date, reference);

    // Emma and Lisa have NO relationship - Lisa is a stranger who Emma met briefly
    // at a social event. Emma mentioned something personal in passing and Lisa
    // spread it around. No prior trust was established.

    // Same betrayal event - but Lisa is a stranger (no relationship tracked)
    let emma_betrayal = EventBuilder::new(EventType::ExperienceBetrayalTrust)
        .source(lisa_id.clone())
        .target(emma_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    emma_sim.add_event(emma_betrayal, betrayal_time);

    // === COMPARE OUTCOMES ===
    //
    // Query state 6 months after betrayal
    let query_time = betrayal_time + Duration::days(180);

    let sarah_handle = sarah_sim.entity(&sarah_id).unwrap();
    let sarah_state = sarah_handle.state_at(query_time);

    let emma_handle = emma_sim.entity(&emma_id).unwrap();
    let emma_state = emma_handle.state_at(query_time);

    // === CORE ASSERTION: Trust modulation amplifies betrayal impact ===
    //
    // Sarah trusted Rachel (trust modulation factor ~1.11)
    // Emma had no relationship with Lisa (trust modulation factor = 1.0)
    // Sarah's impacts should be ~11% larger across all dimensions

    // Valence: Sarah should feel worse (more negative) than Emma
    let sarah_valence = sarah_state.individual_state().mood().valence_effective();
    let emma_valence = emma_state.individual_state().mood().valence_effective();
    assert!(
        sarah_valence < emma_valence,
        "Sarah's valence ({}) should be lower than Emma's ({}) - betrayal from trusted friend cuts deeper",
        sarah_valence,
        emma_valence
    );

    // Stress: Sarah should experience more stress than Emma
    let sarah_stress = sarah_state.individual_state().needs().stress_effective();
    let emma_stress = emma_state.individual_state().needs().stress_effective();
    assert!(
        sarah_stress > emma_stress,
        "Sarah's stress ({}) should exceed Emma's ({}) - vulnerability to trusted friend creates more distress",
        sarah_stress,
        emma_stress
    );

    // Interpersonal hopelessness: Sarah's belief in relationships should be more damaged
    let sarah_hopelessness = sarah_state
        .individual_state()
        .mental_health()
        .interpersonal_hopelessness_effective();
    let emma_hopelessness = emma_state
        .individual_state()
        .mental_health()
        .interpersonal_hopelessness_effective();
    assert!(
        sarah_hopelessness > emma_hopelessness,
        "Sarah's hopelessness ({}) should exceed Emma's ({}) - betrayal from trusted friend damages relationship beliefs more",
        sarah_hopelessness,
        emma_hopelessness
    );

    // === RECOVERY CHECK: 2 years later ===
    //
    // Check if the amplified effects persist. Sarah's base shifts were larger,
    // so even after decay, she should still show more damage.
    let later_query = betrayal_time + Duration::years(2);

    let sarah_later = sarah_sim.entity(&sarah_id).unwrap().state_at(later_query);
    let emma_later = emma_sim.entity(&emma_id).unwrap().state_at(later_query);

    // After 2 years, Sarah's hopelessness should still exceed Emma's
    // The permanent base shift from amplified betrayal persists
    let sarah_hopelessness_later = sarah_later
        .individual_state()
        .mental_health()
        .interpersonal_hopelessness_effective();
    let emma_hopelessness_later = emma_later
        .individual_state()
        .mental_health()
        .interpersonal_hopelessness_effective();

    assert!(
        sarah_hopelessness_later > emma_hopelessness_later,
        "After 2 years, Sarah's hopelessness ({}) should still exceed Emma's ({}) - deep wounds heal slowly",
        sarah_hopelessness_later,
        emma_hopelessness_later
    );
}

fn setup_high_trust_scenario(
    birth_date: Timestamp,
    reference: Timestamp,
) -> (Simulation, EntityId, EntityId) {
    let mut sim = Simulation::new(reference);

    // Sarah - the main character
    let sarah = EntityBuilder::new()
        .id("sarah")
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .build()
        .unwrap();
    sim.add_entity(sarah, reference);
    let sarah_id = EntityId::new("sarah").unwrap();

    // Rachel - Sarah's best friend of 15 years
    let rachel = EntityBuilder::new()
        .id("rachel")
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .build()
        .unwrap();
    sim.add_entity(rachel, reference);
    let rachel_id = EntityId::new("rachel").unwrap();

    // Relationship formed 15 years ago - trust will be built through events
    let formed = birth_date + Duration::years(ANCHOR_AGE - 15);
    sim.add_relationship(sarah_id.clone(), rachel_id.clone(), RelationshipSchema::Peer, formed);

    (sim, sarah_id, rachel_id)
}

fn setup_stranger_scenario(
    birth_date: Timestamp,
    reference: Timestamp,
) -> (Simulation, EntityId, EntityId) {
    let mut sim = Simulation::new(reference);

    // Emma - the main character
    let emma = EntityBuilder::new()
        .id("emma")
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .build()
        .unwrap();
    sim.add_entity(emma, reference);
    let emma_id = EntityId::new("emma").unwrap();

    // Lisa - a complete stranger Emma met at a party
    // Lisa is an entity but there's NO relationship with Emma
    let lisa = EntityBuilder::new()
        .id("lisa")
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .build()
        .unwrap();
    sim.add_entity(lisa, reference);
    let lisa_id = EntityId::new("lisa").unwrap();

    // NO relationship added - Lisa is a stranger
    // Events from strangers get no trust modulation

    (sim, emma_id, lisa_id)
}
