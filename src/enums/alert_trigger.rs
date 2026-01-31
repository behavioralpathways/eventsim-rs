//! Alert trigger types for threshold notifications.
//!
//! This module defines what conditions can trigger an alert.

use crate::enums::StatePath;

/// Type of feedback spiral detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpiralType {
    /// Stress-fatigue-impulse control spiral.
    Stress,

    /// Depression-loneliness spiral (Human only).
    Depression,
}

impl SpiralType {
    /// Returns a human-readable name for this spiral type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SpiralType::Stress => "Stress Spiral",
            SpiralType::Depression => "Depression Spiral",
        }
    }
}

impl std::fmt::Display for SpiralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The condition that triggered an alert.
///
/// Alerts can be triggered by thresholds being exceeded, feedback loops
/// activating, or custom application-specific conditions.
///
/// # Examples
///
/// ```
/// use eventsim_rs::enums::{AlertTrigger, StatePath, MentalHealthPath};
///
/// let trigger = AlertTrigger::ThresholdExceeded(
///     StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
///     0.7,
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum AlertTrigger {
    /// A state dimension exceeded a threshold value.
    ThresholdExceeded(StatePath, f64),

    /// A feedback spiral was detected as active.
    SpiralDetected(SpiralType),

    /// A custom trigger with a description.
    Custom(String),
}

impl AlertTrigger {
    /// Creates a threshold exceeded trigger.
    #[must_use]
    pub const fn threshold(path: StatePath, value: f64) -> Self {
        AlertTrigger::ThresholdExceeded(path, value)
    }

    /// Creates a spiral detected trigger.
    #[must_use]
    pub const fn spiral(spiral_type: SpiralType) -> Self {
        AlertTrigger::SpiralDetected(spiral_type)
    }

    /// Creates a custom trigger.
    #[must_use]
    pub fn custom(description: impl Into<String>) -> Self {
        AlertTrigger::Custom(description.into())
    }

    /// Returns true if this is a threshold trigger.
    #[must_use]
    pub const fn is_threshold(&self) -> bool {
        matches!(self, AlertTrigger::ThresholdExceeded(_, _))
    }

    /// Returns true if this is a spiral trigger.
    #[must_use]
    pub const fn is_spiral(&self) -> bool {
        matches!(self, AlertTrigger::SpiralDetected(_))
    }

    /// Returns true if this is a custom trigger.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, AlertTrigger::Custom(_))
    }
}

impl std::fmt::Display for AlertTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertTrigger::ThresholdExceeded(path, value) => {
                write!(f, "Threshold exceeded: {} = {:.2}", path, value)
            }
            AlertTrigger::SpiralDetected(spiral) => {
                write!(f, "Spiral detected: {}", spiral)
            }
            AlertTrigger::Custom(desc) => {
                write!(f, "Custom: {}", desc)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::MentalHealthPath;

    fn matches_threshold(
        trigger: &AlertTrigger,
        expected_path: StatePath,
        expected_value: f64,
    ) -> bool {
        match trigger {
            AlertTrigger::ThresholdExceeded(path, value) => {
                let path_matches = *path == expected_path;
                let value_matches = (*value - expected_value).abs() < f64::EPSILON;
                path_matches && value_matches
            }
            _ => false,
        }
    }

    fn matches_spiral(trigger: &AlertTrigger, expected: SpiralType) -> bool {
        match trigger {
            AlertTrigger::SpiralDetected(spiral_type) => *spiral_type == expected,
            _ => false,
        }
    }

    fn matches_custom(trigger: &AlertTrigger, expected: &str) -> bool {
        match trigger {
            AlertTrigger::Custom(desc) => desc == expected,
            _ => false,
        }
    }

    #[test]
    fn alert_trigger_variants_exist() {
        // Verify all variants can be constructed
        let threshold = AlertTrigger::ThresholdExceeded(
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
            0.7,
        );
        let spiral = AlertTrigger::SpiralDetected(SpiralType::Stress);
        let custom = AlertTrigger::Custom("Test trigger".to_string());

        assert!(threshold.is_threshold());
        assert!(spiral.is_spiral());
        assert!(custom.is_custom());
    }

    #[test]
    fn threshold_trigger_constructor() {
        let trigger =
            AlertTrigger::threshold(StatePath::MentalHealth(MentalHealthPath::Depression), 0.8);

        assert!(matches_threshold(
            &trigger,
            StatePath::MentalHealth(MentalHealthPath::Depression),
            0.8
        ));
        assert!(!matches_threshold(
            &AlertTrigger::spiral(SpiralType::Stress),
            StatePath::MentalHealth(MentalHealthPath::Depression),
            0.8
        ));
    }

    #[test]
    fn spiral_trigger_constructor() {
        let trigger = AlertTrigger::spiral(SpiralType::Depression);

        assert!(matches_spiral(&trigger, SpiralType::Depression));
        assert!(!matches_spiral(
            &AlertTrigger::threshold(StatePath::MentalHealth(MentalHealthPath::Depression), 0.5),
            SpiralType::Depression
        ));
    }

    #[test]
    fn custom_trigger_constructor() {
        let trigger = AlertTrigger::custom("High risk detected");

        assert!(matches_custom(&trigger, "High risk detected"));
        assert!(!matches_custom(
            &AlertTrigger::spiral(SpiralType::Stress),
            "High risk detected"
        ));
    }

    #[test]
    fn spiral_type_names() {
        assert_eq!(SpiralType::Stress.name(), "Stress Spiral");
        assert_eq!(SpiralType::Depression.name(), "Depression Spiral");
    }

    #[test]
    fn spiral_type_display() {
        assert_eq!(format!("{}", SpiralType::Stress), "Stress Spiral");
        assert_eq!(format!("{}", SpiralType::Depression), "Depression Spiral");
    }

    #[test]
    fn trigger_display() {
        let threshold = AlertTrigger::ThresholdExceeded(
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
            0.75,
        );
        let display = format!("{}", threshold);
        assert!(display.contains("Threshold exceeded"));
        assert!(display.contains("0.75"));

        let spiral = AlertTrigger::SpiralDetected(SpiralType::Stress);
        let spiral_display = format!("{}", spiral);
        assert!(spiral_display.contains("Spiral detected"));
        assert!(spiral_display.contains("Stress"));

        let custom = AlertTrigger::Custom("Test".to_string());
        let custom_display = format!("{}", custom);
        assert!(custom_display.contains("Custom"));
        assert!(custom_display.contains("Test"));
    }

    #[test]
    fn trigger_debug() {
        let trigger = AlertTrigger::SpiralDetected(SpiralType::Depression);
        let debug = format!("{:?}", trigger);
        assert!(debug.contains("SpiralDetected"));
    }

    #[test]
    fn trigger_clone() {
        let original = AlertTrigger::Custom("Test".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn spiral_type_clone_copy() {
        let s1 = SpiralType::Stress;
        let s2 = s1; // Copy
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn spiral_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SpiralType::Stress);
        set.insert(SpiralType::Stress);
        assert_eq!(set.len(), 1);

        set.insert(SpiralType::Depression);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn is_methods_exclusive() {
        let threshold =
            AlertTrigger::threshold(StatePath::MentalHealth(MentalHealthPath::Depression), 0.5);
        assert!(threshold.is_threshold());
        assert!(!threshold.is_spiral());
        assert!(!threshold.is_custom());

        let spiral = AlertTrigger::spiral(SpiralType::Stress);
        assert!(!spiral.is_threshold());
        assert!(spiral.is_spiral());
        assert!(!spiral.is_custom());

        let custom = AlertTrigger::custom("test");
        assert!(!custom.is_threshold());
        assert!(!custom.is_spiral());
        assert!(custom.is_custom());
    }

    #[test]
    fn threshold_trigger_display_format() {
        let trigger = AlertTrigger::ThresholdExceeded(
            StatePath::MentalHealth(MentalHealthPath::SuicidalDesire),
            0.75,
        );
        let display = format!("{}", trigger);
        assert!(display.contains("Threshold exceeded"));
        assert!(display.contains("0.75"));
        assert!(display.contains("MentalHealth"));
    }

    #[test]
    fn spiral_trigger_display_format() {
        let trigger = AlertTrigger::SpiralDetected(SpiralType::Depression);
        let display = format!("{}", trigger);
        assert!(display.contains("Spiral detected"));
        assert!(display.contains("Depression"));
    }

    #[test]
    fn custom_trigger_display_format() {
        let trigger = AlertTrigger::Custom("High risk intervention needed".to_string());
        let display = format!("{}", trigger);
        assert!(display.contains("Custom"));
        assert!(display.contains("High risk intervention needed"));
    }
}
