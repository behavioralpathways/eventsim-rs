//! Memory creation from interpreted events.
//!
//! This module provides functions to create MemoryEntry instances from
//! InterpretedEvent data during state computation. Memories are created
//! after an event is applied to capture the emotional context at encoding.

use crate::memory::event_tags::derive_tags_from_impacts;
use crate::memory::{EmotionalSnapshot, MemoryEntry, MemorySource};
use crate::processor::InterpretedEvent;
use crate::state::IndividualState;
use crate::types::{Duration, EntityId};

/// Creates a memory entry from an interpreted event.
///
/// This function captures the psychological impact of an event as a memory.
/// The memory includes:
/// - Tags derived from the event's psychological impacts
/// - Emotional snapshot captured AFTER the event is applied
/// - Salience from the interpreted event
/// - Participants from the event source/target
/// - Summary from the event type name
///
/// # Arguments
///
/// * `interpreted` - The interpreted event with computed deltas and salience
/// * `state` - The entity's state AFTER the event has been applied
/// * `age_at_event` - The entity's age when the event occurred
/// * `target_id` - Optional entity ID of the memory owner (target of the event)
///
/// # Returns
///
/// A new MemoryEntry representing this event.
#[must_use]
pub fn create_memory_from_event(
    interpreted: &InterpretedEvent,
    state: &IndividualState,
    age_at_event: Duration,
    target_id: Option<&EntityId>,
) -> MemoryEntry {
    let event = &interpreted.event;
    let spec = event.spec();

    // Derive tags from the event's psychological impacts
    let tags = derive_tags_from_impacts(&spec, event.severity());

    // Capture emotional snapshot from current state (after event applied)
    let emotional_snapshot = EmotionalSnapshot::from_mood(state.mood());

    // Build participant list
    let mut participants = Vec::new();
    if let Some(source) = event.source() {
        participants.push(source.clone());
    }
    if let Some(target) = target_id {
        if !participants.contains(target) {
            participants.push(target.clone());
        }
    }

    // Generate summary from event type
    let summary = format_event_summary(event.event_type().name());

    // Build the memory entry
    let mut entry = MemoryEntry::new(age_at_event, summary)
        .with_event_id(event.id().clone())
        .with_tags(tags)
        .with_emotional_snapshot(emotional_snapshot)
        .with_salience(interpreted.salience)
        .with_source(MemorySource::Self_);

    if !participants.is_empty() {
        entry = entry.with_participants(participants);
    }

    // Add microsystem context if present
    if let Some(context) = event.microsystem_context() {
        entry = entry.with_microsystem_context(context.clone());
    }

    entry
}

/// Formats an event type name into a human-readable summary.
///
/// Converts CamelCase event names like "EndRelationshipRomantic" into
/// readable summaries like "End relationship romantic".
fn format_event_summary(event_type_name: &str) -> String {
    let mut result = String::with_capacity(event_type_name.len() + 10);
    let mut chars = event_type_name.chars().peekable();
    let mut first = true;

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !first {
                result.push(' ');
            }
            if first {
                result.push(c);
            } else {
                result.push(c.to_ascii_lowercase());
            }
            first = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityBuilder;
    use crate::enums::{EventType, Species};
    use crate::event::EventBuilder;
    use crate::processor::interpret_event;

    fn create_human() -> crate::entity::Entity {
        EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(30))
            .build()
            .unwrap()
    }

    #[test]
    fn memory_has_event_id() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        assert_eq!(memory.event_id(), Some(event.id()));
    }

    #[test]
    fn memory_has_derived_tags() {
        let entity = create_human();
        // Combat event has high AC, should get Violence tag
        let event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        assert!(!memory.tags().is_empty());
    }

    #[test]
    fn memory_has_emotional_snapshot() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        // Snapshot should reflect the state's current mood
        let snapshot = memory.emotional_snapshot();
        let state_mood = entity.individual_state().mood();
        assert!((snapshot.valence() - state_mood.valence_effective()).abs() < f32::EPSILON);
    }

    #[test]
    fn memory_has_salience_from_interpreted_event() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::ExperienceCombatMilitary)
            .severity(0.9)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        assert!((memory.salience() - interpreted.salience).abs() < f32::EPSILON);
    }

    #[test]
    fn memory_includes_event_source_as_participant() {
        let entity = create_human();
        let source_id = EntityId::new("perpetrator").unwrap();
        let event = EventBuilder::new(EventType::ExperienceBetrayalTrust)
            .source(source_id.clone())
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        assert!(memory.involves_participant(&source_id));
    }

    #[test]
    fn memory_includes_target_as_participant() {
        let entity = create_human();
        let target_id = EntityId::new("victim").unwrap();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            Some(&target_id),
        );

        assert!(memory.involves_participant(&target_id));
    }

    #[test]
    fn memory_has_microsystem_context_when_present() {
        let entity = create_human();
        let context = crate::types::MicrosystemId::new("work_001").unwrap();
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .context(context.clone())
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        assert_eq!(memory.microsystem_context(), Some(&context));
    }

    #[test]
    fn memory_has_self_source() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            None,
        );

        assert_eq!(memory.source(), MemorySource::Self_);
    }

    #[test]
    fn memory_timestamp_is_age_at_event() {
        let entity = create_human();
        let event = EventBuilder::new(EventType::AchieveGoalMajor)
            .severity(0.7)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let age = Duration::years(35);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            age,
            None,
        );

        assert_eq!(memory.timestamp(), age);
    }

    #[test]
    fn format_event_summary_converts_camel_case() {
        assert_eq!(
            format_event_summary("EndRelationshipRomantic"),
            "End relationship romantic"
        );
        assert_eq!(
            format_event_summary("ExperienceCombatMilitary"),
            "Experience combat military"
        );
        assert_eq!(format_event_summary("AchieveGoalMajor"), "Achieve goal major");
    }

    #[test]
    fn format_event_summary_handles_single_word() {
        assert_eq!(format_event_summary("Custom"), "Custom");
    }

    #[test]
    fn format_event_summary_handles_empty_string() {
        assert_eq!(format_event_summary(""), "");
    }

    #[test]
    fn duplicate_participant_not_added_twice() {
        let entity = create_human();
        let entity_id = EntityId::new("self").unwrap();
        let event = EventBuilder::new(EventType::EndRelationshipRomantic)
            .source(entity_id.clone())
            .severity(0.8)
            .build()
            .unwrap();

        let interpreted = interpret_event(&event, &entity);
        let memory = create_memory_from_event(
            &interpreted,
            entity.individual_state(),
            Duration::years(30),
            Some(&entity_id), // Same as source
        );

        // Should only appear once
        assert_eq!(
            memory
                .participants()
                .iter()
                .filter(|p| *p == &entity_id)
                .count(),
            1
        );
    }
}
