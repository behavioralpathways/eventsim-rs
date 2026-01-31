//! Relationship slot for entity relationship hooks.
//!
//! This type represents a slot where a relationship can be attached
//! to an entity.

use crate::types::RelationshipId;

/// Slot for attaching relationships to an entity.
///
/// Each slot can hold a reference to a single relationship. Slots are
/// initially unattached.
///
/// # API
///
/// - `is_empty()` - Returns true if no relationship attached
/// - `is_attached()` - Returns true if a relationship is attached
/// - `get_attached()` - Returns the attached relationship ID, if any
///
/// # Examples
///
/// ```
/// use eventsim_rs::types::RelationshipSlot;
///
/// let slot = RelationshipSlot::new();
/// assert!(slot.is_empty());
/// assert!(!slot.is_attached());
/// assert!(slot.get_attached().is_none());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RelationshipSlot {
    /// The attached relationship ID, if any.
    attached: Option<RelationshipId>,
}

impl RelationshipSlot {
    /// Creates a new empty relationship slot.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventsim_rs::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(slot.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        RelationshipSlot { attached: None }
    }

    /// Returns true if this slot is empty (no relationship attached).
    ///
    /// This is the inverse of `is_attached()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventsim_rs::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(slot.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.attached.is_none()
    }

    /// Returns true if a relationship is attached to this slot.
    ///
    /// This is the inverse of `is_empty()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventsim_rs::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(!slot.is_attached());
    /// ```
    #[must_use]
    pub const fn is_attached(&self) -> bool {
        self.attached.is_some()
    }

    /// Returns the attached relationship ID, if any.
    ///
    /// Returns `None` if the slot is empty. Returns an owned copy of the ID
    /// to avoid lifetime complications in external code.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventsim_rs::types::RelationshipSlot;
    ///
    /// let slot = RelationshipSlot::new();
    /// assert!(slot.get_attached().is_none());
    /// ```
    #[must_use]
    pub fn get_attached(&self) -> Option<RelationshipId> {
        self.attached.clone()
    }

    #[cfg(test)]
    pub(crate) fn attach_for_test(&mut self, id: RelationshipId) {
        self.attached = Some(id);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_slot_is_empty_when_new() {
        let slot = RelationshipSlot::new();
        assert!(slot.is_empty());
        assert!(!slot.is_attached());

        // Verify default also works
        let default_slot = RelationshipSlot::default();
        assert!(default_slot.is_empty());
        assert!(!default_slot.is_attached());
    }

    #[test]
    fn get_attached_returns_none_when_empty() {
        let slot = RelationshipSlot::new();
        assert!(slot.get_attached().is_none());
    }

    #[test]
    fn relationship_slot_equality() {
        let slot1 = RelationshipSlot::new();
        let slot2 = RelationshipSlot::new();
        assert_eq!(slot1, slot2);
    }

    #[test]
    fn relationship_slot_clone() {
        let original = RelationshipSlot::new();
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.get_attached(), cloned.get_attached());
    }

    #[test]
    fn relationship_slot_debug() {
        let slot = RelationshipSlot::new();
        let debug = format!("{:?}", slot);
        assert!(debug.contains("RelationshipSlot"));
    }

    #[test]
    fn is_attached_and_is_empty_are_inverses() {
        let slot = RelationshipSlot::new();
        assert_eq!(slot.is_attached(), !slot.is_empty());
    }
}
