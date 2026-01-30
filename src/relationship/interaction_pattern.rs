//! Interaction pattern metadata for relationships.
//!
//! Captures how consistently entities interact.

/// Interaction pattern for a relationship.
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionPattern {
    /// Regularity of interactions (0 = erratic, 1 = consistent).
    pub consistency: f32,
}

impl Default for InteractionPattern {
    fn default() -> Self {
        InteractionPattern { consistency: 0.0 }
    }
}

impl InteractionPattern {
    /// Creates a new InteractionPattern with empty defaults.
    #[must_use]
    pub fn new() -> Self {
        InteractionPattern::default()
    }

    /// Sets the interaction consistency.
    #[must_use]
    pub fn with_consistency(mut self, consistency: f32) -> Self {
        self.consistency = consistency.clamp(0.0, 1.0);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction_pattern_defaults_empty() {
        let pattern = InteractionPattern::default();
        assert!((pattern.consistency - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn interaction_pattern_builder_sets_values() {
        let pattern = InteractionPattern::new().with_consistency(0.8);

        assert!((pattern.consistency - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn interaction_pattern_consistency_clamped() {
        let pattern = InteractionPattern::new().with_consistency(-0.2);
        assert!((pattern.consistency - 0.0).abs() < f32::EPSILON);

        let pattern2 = InteractionPattern::new().with_consistency(1.5);
        assert!((pattern2.consistency - 1.0).abs() < f32::EPSILON);
    }
}
