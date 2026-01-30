//! Core event structure for behavioral pathways.
//!
//! Events are occurrences that affect entity state. Each event has a type,
//! optional source and target, severity, and type-specific payload.

use crate::enums::{EventPayload, EventType, HexacoPath};
use crate::event::event_spec::EventSpec;
use crate::types::{Duration, EntityId, EventId, MicrosystemId};
use uuid::Uuid;

/// Generates a unique event ID using UUID v4.
fn generate_event_id() -> EventId {
    let uuid = Uuid::new_v4();
    // Safe to unwrap because UUIDs are never empty
    EventId::new(format!("evt_{uuid}")).unwrap()
}

/// An event that can affect entity state.
///
/// Events are the primary mechanism for state changes in the simulation.
/// Each event has a type that links to an EventSpec defining impact across
/// all 22 psychological dimensions.
///
/// # Examples
///
/// ```
/// use eventsim_rs::event::{Event, EventBuilder};
/// use eventsim_rs::enums::EventType;
///
/// let event = EventBuilder::new(EventType::EndRelationshipRomantic)
///     .severity(0.7)
///     .build()
///     .unwrap();
///
/// assert_eq!(event.event_type(), EventType::EndRelationshipRomantic);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// Unique identifier for this event.
    id: EventId,
    /// Primary classification of the event.
    event_type: EventType,
    /// Entity that caused the event (None for environmental).
    source: Option<EntityId>,
    /// Entity affected by the event (None for broadcast).
    target: Option<EntityId>,
    /// Intensity of the event (0.0 to 1.0).
    severity: f64,
    /// Type-specific event data (required, use EventPayload::Empty if none).
    payload: EventPayload,
    /// When the event occurred (entity age at event time).
    timestamp: Duration,
    /// Microsystem context where event occurred.
    microsystem_context: Option<MicrosystemId>,
    /// Personality base shifts triggered by this event.
    /// Each entry is (trait, shift_amount) to be processed during simulation.
    base_shifts: Vec<(HexacoPath, f32)>,
    /// Custom EventSpec (used when event_type is Custom).
    custom_spec: Option<EventSpec>,
}

impl Event {
    /// Creates a new event with the given type.
    ///
    /// This is an internal constructor. Use `EventBuilder` for public
    /// construction. Payload defaults to EventPayload::Empty.
    pub(crate) fn new(event_type: EventType) -> Self {
        Event {
            id: generate_event_id(),
            event_type,
            source: None,
            target: None,
            severity: 0.5,
            payload: EventPayload::Empty,
            timestamp: Duration::zero(),
            microsystem_context: None,
            base_shifts: Vec::new(),
            custom_spec: None,
        }
    }

    /// Creates a new event with a specific ID (for testing/loading).
    #[must_use]
    pub fn with_id(id: EventId, event_type: EventType) -> Self {
        Event {
            id,
            event_type,
            source: None,
            target: None,
            severity: 0.5,
            payload: EventPayload::Empty,
            timestamp: Duration::zero(),
            microsystem_context: None,
            base_shifts: Vec::new(),
            custom_spec: None,
        }
    }

    /// Creates a custom event with an explicit EventSpec.
    ///
    /// Use this when you need event impacts that don't match any predefined type.
    #[must_use]
    pub fn custom(spec: EventSpec) -> Self {
        Event {
            id: generate_event_id(),
            event_type: EventType::Custom,
            source: None,
            target: None,
            severity: 1.0,
            payload: EventPayload::Empty,
            timestamp: Duration::zero(),
            microsystem_context: None,
            base_shifts: Vec::new(),
            custom_spec: Some(spec),
        }
    }

    // Accessors

    /// Returns the event's unique identifier.
    #[must_use]
    pub fn id(&self) -> &EventId {
        &self.id
    }

    /// Returns the event type.
    #[must_use]
    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Returns the EventSpec for this event.
    ///
    /// For custom events, returns the custom spec. For other events,
    /// returns the spec from the event type.
    #[must_use]
    pub fn spec(&self) -> EventSpec {
        self.custom_spec.unwrap_or_else(|| self.event_type.spec())
    }

    /// Returns the source entity, if any.
    #[must_use]
    pub fn source(&self) -> Option<&EntityId> {
        self.source.as_ref()
    }

    /// Returns the target entity, if any.
    #[must_use]
    pub fn target(&self) -> Option<&EntityId> {
        self.target.as_ref()
    }

    /// Returns the severity (0.0 to 1.0).
    #[must_use]
    pub fn severity(&self) -> f64 {
        self.severity
    }

    /// Returns the payload.
    #[must_use]
    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }

    /// Returns whether the payload has data (not Empty).
    #[must_use]
    pub fn has_payload_data(&self) -> bool {
        !matches!(self.payload, EventPayload::Empty)
    }

    /// Returns the timestamp.
    #[must_use]
    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    /// Returns the microsystem context, if any.
    #[must_use]
    pub fn microsystem_context(&self) -> Option<&MicrosystemId> {
        self.microsystem_context.as_ref()
    }

    /// Returns the personality base shifts for this event.
    ///
    /// Each entry is a (trait, shift_amount) pair representing a permanent
    /// personality change to apply when this event is processed.
    #[must_use]
    pub fn base_shifts(&self) -> &[(HexacoPath, f32)] {
        &self.base_shifts
    }

    /// Returns true if this event has any personality base shifts.
    #[must_use]
    pub fn has_base_shifts(&self) -> bool {
        !self.base_shifts.is_empty()
    }

    // Builder-style setters for internal use

    #[allow(dead_code)]
    pub(crate) fn set_id(&mut self, id: EventId) {
        self.id = id;
    }

    pub(crate) fn set_source(&mut self, source: Option<EntityId>) {
        self.source = source;
    }

    pub(crate) fn set_target(&mut self, target: Option<EntityId>) {
        self.target = target;
    }

    pub(crate) fn set_severity(&mut self, severity: f64) {
        self.severity = severity.clamp(0.0, 1.0);
    }

    pub(crate) fn set_payload(&mut self, payload: EventPayload) {
        self.payload = payload;
    }

    pub(crate) fn set_timestamp(&mut self, timestamp: Duration) {
        self.timestamp = timestamp;
    }

    pub(crate) fn set_microsystem_context(&mut self, context: Option<MicrosystemId>) {
        self.microsystem_context = context;
    }

    pub(crate) fn set_base_shifts(&mut self, shifts: Vec<(HexacoPath, f32)>) {
        self.base_shifts = shifts;
    }

    pub(crate) fn set_custom_spec(&mut self, spec: EventSpec) {
        self.custom_spec = Some(spec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::event_spec::{ChronicFlags, EventImpact, PermanenceValues};

    #[test]
    fn event_creation_with_type() {
        let event = Event::new(EventType::EndRelationshipRomantic);
        assert_eq!(event.event_type(), EventType::EndRelationshipRomantic);
    }

    #[test]
    fn event_source_target_specification() {
        let mut event = Event::new(EventType::ExperienceBetrayalTrust);
        let source = EntityId::new("attacker").unwrap();
        let target = EntityId::new("victim").unwrap();

        event.set_source(Some(source.clone()));
        event.set_target(Some(target.clone()));

        assert_eq!(event.source(), Some(&source));
        assert_eq!(event.target(), Some(&target));
    }

    #[test]
    fn event_id_auto_generated() {
        let event1 = Event::new(EventType::AchieveGoalMajor);
        let event2 = Event::new(EventType::AchieveGoalMajor);

        assert_ne!(event1.id(), event2.id());
        assert!(event1.id().as_str().starts_with("evt_"));
    }

    #[test]
    fn event_set_id_overrides_existing_id() {
        let mut event = Event::new(EventType::AchieveGoalMajor);
        let new_id = EventId::new("custom_event_override").unwrap();

        event.set_id(new_id.clone());

        assert_eq!(event.id(), &new_id);
    }

    #[test]
    fn event_with_id_sets_specific_id() {
        let id = EventId::new("custom_event").unwrap();
        let event = Event::with_id(id.clone(), EventType::AchieveGoalMajor);
        assert_eq!(event.id(), &id);
    }

    #[test]
    fn event_severity_clamped() {
        let mut event = Event::new(EventType::ExperienceCombatMilitary);
        event.set_severity(1.5);
        assert!((event.severity() - 1.0).abs() < f64::EPSILON);

        event.set_severity(-0.5);
        assert!(event.severity().abs() < f64::EPSILON);
    }

    #[test]
    fn event_payload_access() {
        let mut event = Event::new(EventType::ExperienceCombatMilitary);
        let payload = EventPayload::Violence {
            weapon: None,
            injury_severity: 0.5,
        };
        event.set_payload(payload.clone());

        assert_eq!(event.payload(), &payload);
        assert!(event.has_payload_data());
    }

    #[test]
    fn event_empty_payload() {
        let event = Event::new(EventType::ExperienceCombatMilitary);
        assert!(!event.has_payload_data());
        assert_eq!(event.payload(), &EventPayload::Empty);
    }

    #[test]
    fn event_timestamp_works() {
        let mut event = Event::new(EventType::AchieveGoalMajor);
        event.set_timestamp(Duration::days(100));
        assert_eq!(event.timestamp().as_days(), 100);
    }

    #[test]
    fn event_microsystem_context_works() {
        let mut event = Event::new(EventType::AchieveGoalMajor);
        let context = MicrosystemId::new("work_001").unwrap();
        event.set_microsystem_context(Some(context.clone()));
        assert_eq!(event.microsystem_context(), Some(&context));
    }

    #[test]
    fn event_clone() {
        let mut event = Event::new(EventType::ExperienceCombatMilitary);
        event.set_severity(0.8);

        let cloned = event.clone();
        assert_eq!(event, cloned);
    }

    #[test]
    fn event_debug_format() {
        let event = Event::new(EventType::ExperienceCombatMilitary);
        let debug = format!("{:?}", event);
        assert!(debug.contains("Event"));
        assert!(debug.contains("ExperienceCombatMilitary"));
    }

    #[test]
    fn event_default_values() {
        let event = Event::new(EventType::AchieveGoalMajor);

        assert!(event.source().is_none());
        assert!(event.target().is_none());
        assert!((event.severity() - 0.5).abs() < f64::EPSILON);
        assert_eq!(event.payload(), &EventPayload::Empty);
        assert_eq!(event.timestamp().as_seconds(), 0);
        assert!(event.microsystem_context().is_none());
    }

    #[test]
    fn event_default_has_no_base_shifts() {
        let event = Event::new(EventType::ExperienceCombatMilitary);
        assert!(!event.has_base_shifts());
        assert!(event.base_shifts().is_empty());
    }

    #[test]
    fn event_with_base_shifts() {
        let mut event = Event::new(EventType::ExperienceCombatMilitary);
        event.set_base_shifts(vec![
            (HexacoPath::Neuroticism, 0.25),
            (HexacoPath::Agreeableness, -0.15),
        ]);

        assert!(event.has_base_shifts());
        assert_eq!(event.base_shifts().len(), 2);
        assert_eq!(event.base_shifts()[0], (HexacoPath::Neuroticism, 0.25));
        assert_eq!(event.base_shifts()[1], (HexacoPath::Agreeableness, -0.15));
    }

    #[test]
    fn event_spec_returns_type_spec() {
        let event = Event::new(EventType::EndRelationshipRomantic);
        let spec = event.spec();
        assert!(spec.impact.valence < 0.0);
    }

    #[test]
    fn custom_event_uses_custom_spec() {
        let custom_spec = EventSpec {
            impact: EventImpact {
                valence: 0.99,
                ..Default::default()
            },
            chronic: ChronicFlags::default(),
            permanence: PermanenceValues::default(),
        };

        let event = Event::custom(custom_spec);
        assert_eq!(event.event_type(), EventType::Custom);

        let spec = event.spec();
        assert!((spec.impact.valence - 0.99).abs() < f32::EPSILON);
    }

    #[test]
    fn custom_event_has_default_severity_one() {
        let custom_spec = EventSpec::default();
        let event = Event::custom(custom_spec);
        assert!((event.severity() - 1.0).abs() < f64::EPSILON);
    }
}
