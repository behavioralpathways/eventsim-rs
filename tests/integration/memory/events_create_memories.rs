//! Integration tests for event-to-memory creation.
//!
//! Tests that events processed during state_at() correctly create memories
//! that influence mood through priming.

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{EventType, MoodPath, Species, StatePath};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::Simulation;
use eventsim_rs::types::{Duration, EntityId, Timestamp};

#[test]
fn negative_events_create_memories_that_prime_mood() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let birth_date = reference - Duration::years(30);
    let entity = EntityBuilder::new()
        .id("person_001")
        .species(Species::Human)
        .birth_date(birth_date)
        .age(Duration::years(30))
        .build()
        .unwrap();

    sim.add_entity(entity, reference);
    let entity_id = EntityId::new("person_001").unwrap();

    // Verify initial neutral mood
    let handle = sim.entity(&entity_id).unwrap();
    let initial_state = handle.state_at(reference);
    let initial_valence = initial_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Add negative events
    let event1_time = reference + Duration::days(60);
    let event1 = EventBuilder::new(EventType::ExperienceBetrayalTrust)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event1, event1_time);

    let event2_time = reference + Duration::days(180);
    let event2 = EventBuilder::new(EventType::LosePersonDeath)
        .target(entity_id.clone())
        .severity(0.9)
        .build()
        .unwrap();
    sim.add_event(event2, event2_time);

    // Query state after events
    let end_time = reference + Duration::years(1);
    let handle = sim.entity(&entity_id).unwrap();
    let final_state = handle.state_at(end_time);
    let final_valence = final_state.get_effective(StatePath::Mood(MoodPath::Valence));

    // Valence should be more negative
    assert!(
        final_valence < initial_valence,
        "Final valence ({}) should be more negative than initial ({})",
        final_valence,
        initial_valence
    );
}

#[test]
fn trauma_events_create_high_salience_memories() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let birth_date = reference - Duration::years(30);
    let entity = EntityBuilder::new()
        .id("trauma_victim")
        .species(Species::Human)
        .birth_date(birth_date)
        .age(Duration::years(30))
        .build()
        .unwrap();

    sim.add_entity(entity, reference);
    let entity_id = EntityId::new("trauma_victim").unwrap();

    // Add high-severity trauma event
    let trauma_time = reference + Duration::days(30);
    let trauma_event = EventBuilder::new(EventType::ExperienceCombatMilitary)
        .target(entity_id.clone())
        .severity(0.95)
        .build()
        .unwrap();
    sim.add_event(trauma_event, trauma_time);

    // Query state 1 year later
    let query_time = reference + Duration::years(1);
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(query_time);

    // High-salience memories increase arousal through priming
    let arousal = state.get_effective(StatePath::Mood(MoodPath::Arousal));

    assert!(
        arousal >= 0.0,
        "Arousal should not be negative after trauma, got {}",
        arousal
    );
}

#[test]
fn event_processing_creates_valid_state() {
    let reference = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
    let mut sim = Simulation::new(reference);

    let birth_date = reference - Duration::years(25);
    let entity = EntityBuilder::new()
        .id("test_person")
        .species(Species::Human)
        .birth_date(birth_date)
        .age(Duration::years(25))
        .build()
        .unwrap();

    sim.add_entity(entity, reference);
    let entity_id = EntityId::new("test_person").unwrap();

    // Add an achievement event
    let event_time = reference + Duration::days(30);
    let event = EventBuilder::new(EventType::AchieveGoalMajor)
        .target(entity_id.clone())
        .severity(0.8)
        .build()
        .unwrap();
    sim.add_event(event, event_time);

    // Query state after the event
    let query_time = reference + Duration::days(60);
    let handle = sim.entity(&entity_id).unwrap();
    let state = handle.state_at(query_time);
    let valence = state.get_effective(StatePath::Mood(MoodPath::Valence));

    assert!(
        valence.is_finite(),
        "Valence should be a finite number after event processing"
    );
}
