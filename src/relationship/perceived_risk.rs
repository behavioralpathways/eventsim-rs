//! Perceived risk assessment for trust decisions.
//!
//! Perceived risk represents a trustor's subjective assessment of the
//! potential negative consequences of trusting a specific trustee.

use crate::state::StateValue;
use crate::types::Duration;

/// Decay half-life for perceived risk (7 days).
/// Risk assessments fade faster without reinforcement.
const PERCEIVED_RISK_DECAY_HALF_LIFE: Duration = Duration::days(7);

/// Default base value for perceived risk.
const DEFAULT_BASE: f32 = 0.3;

/// Perceived risk of trusting another entity.
///
/// Perceived risk is subjective and varies based on:
/// - Relationship history (past betrayals increase risk)
/// - Relationship stage (strangers are higher risk)
/// - Entity personality (neuroticism increases risk perception)
///
/// # Examples
///
/// ```
/// use eventsim_rs::relationship::PerceivedRisk;
///
/// let mut risk = PerceivedRisk::new();
///
/// // Apply risk from a betrayal
/// risk.add_delta(0.3);
///
/// assert!(risk.effective() > 0.3);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PerceivedRisk {
    /// The base/accumulated perceived risk.
    risk: StateValue,
}

impl PerceivedRisk {
    /// Creates a new PerceivedRisk with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use eventsim_rs::relationship::PerceivedRisk;
    ///
    /// let risk = PerceivedRisk::new();
    /// assert!((risk.effective() - 0.3).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        PerceivedRisk {
            risk: StateValue::new(DEFAULT_BASE)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(PERCEIVED_RISK_DECAY_HALF_LIFE),
        }
    }

    /// Creates a PerceivedRisk with a custom base value.
    ///
    /// # Arguments
    ///
    /// * `base` - Base risk level (0-1)
    ///
    /// # Examples
    ///
    /// ```
    /// use eventsim_rs::relationship::PerceivedRisk;
    ///
    /// let risk = PerceivedRisk::with_base(0.5);
    /// assert!((risk.effective() - 0.5).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn with_base(base: f32) -> Self {
        PerceivedRisk {
            risk: StateValue::new(base)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(PERCEIVED_RISK_DECAY_HALF_LIFE),
        }
    }

    /// Returns the effective perceived risk (base + delta), clamped to [0, 1].
    #[must_use]
    pub fn effective(&self) -> f32 {
        self.risk.effective()
    }

    /// Returns the base risk level.
    #[must_use]
    pub fn base(&self) -> f32 {
        self.risk.base()
    }

    /// Returns the current delta.
    #[must_use]
    pub fn delta(&self) -> f32 {
        self.risk.delta()
    }

    /// Returns a reference to the underlying StateValue.
    #[must_use]
    pub fn state_value(&self) -> &StateValue {
        &self.risk
    }

    /// Returns a mutable reference to the underlying StateValue.
    pub fn state_value_mut(&mut self) -> &mut StateValue {
        &mut self.risk
    }

    /// Adds to the risk delta.
    pub fn add_delta(&mut self, amount: f32) {
        self.risk.add_delta(amount);
    }

    /// Sets the risk delta directly.
    pub fn set_delta(&mut self, delta: f32) {
        self.risk.set_delta(delta);
    }

    /// Sets the base risk level.
    pub fn set_base(&mut self, base: f32) {
        self.risk.set_base(base);
    }

    /// Applies decay to the risk delta over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.risk.apply_decay(elapsed);
    }

    /// Resets the delta to zero.
    pub fn reset_delta(&mut self) {
        self.risk.reset_delta();
    }
}

impl Default for PerceivedRisk {
    fn default() -> Self {
        PerceivedRisk::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_default_values() {
        let risk = PerceivedRisk::new();
        assert!((risk.effective() - DEFAULT_BASE).abs() < f32::EPSILON);
    }

    #[test]
    fn with_base_creates_custom_value() {
        let risk = PerceivedRisk::with_base(0.5);
        assert!((risk.effective() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn base_accessor() {
        let risk = PerceivedRisk::with_base(0.6);
        assert!((risk.base() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn delta_accessor() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.2);
        assert!((risk.delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn state_value_accessor() {
        let risk = PerceivedRisk::new();
        assert!((risk.state_value().effective() - DEFAULT_BASE).abs() < f32::EPSILON);
    }

    #[test]
    fn state_value_mut_accessor() {
        let mut risk = PerceivedRisk::new();
        risk.state_value_mut().add_delta(0.1);
        assert!((risk.delta() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.2);
        assert!((risk.effective() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn set_delta() {
        let mut risk = PerceivedRisk::new();
        risk.set_delta(0.3);
        assert!((risk.delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn set_base() {
        let mut risk = PerceivedRisk::new();
        risk.set_base(0.7);
        assert!((risk.base() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn decay_over_7_days() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.4);

        // After 7 days, delta should be halved
        risk.apply_decay(Duration::days(7));
        assert!((risk.delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn reset_delta() {
        let mut risk = PerceivedRisk::new();
        risk.add_delta(0.5);
        risk.reset_delta();
        assert!(risk.delta().abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = PerceivedRisk::default();
        let n = PerceivedRisk::new();
        assert_eq!(d, n);
    }

    #[test]
    fn clone_and_equality() {
        let r1 = PerceivedRisk::with_base(0.5);
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    #[test]
    fn debug_format() {
        let risk = PerceivedRisk::new();
        let debug = format!("{:?}", risk);
        assert!(debug.contains("PerceivedRisk"));
    }
}
