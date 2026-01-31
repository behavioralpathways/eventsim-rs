//! ITS (Interpersonal Theory of Suicide) types and convergence tracking.
//!
//! This module provides types for tracking ITS factor convergence.
//! The actual ITS computation formulas are in MentalHealth (Phase 2).
//!
//! # ITS Components
//!
//! - **Thwarted Belongingness (TB)**: Feeling disconnected from others
//! - **Perceived Burdensomeness (PB)**: Believing oneself to be a burden
//! - **Acquired Capability (AC)**: Habituation to pain/fear of death
//!
//! # Convergence Model (Joiner's Risk Matrix)
//!
//! The three-factor convergence model distinguishes:
//! - Single-factor elevation (TB only, PB only, AC only)
//! - Dual-factor convergence (desire without capability, or capability without desire)
//! - Three-factor convergence (TB + PB + AC all elevated = highest risk)

use crate::state::{PB_PRESENT_THRESHOLD, TB_PRESENT_THRESHOLD};
use serde::{Deserialize, Serialize};

/// Threshold for Acquired Capability to be considered elevated.
pub const AC_ELEVATED_THRESHOLD: f32 = 0.3;

/// The three proximal factors in the ITS model.
///
/// These are the immediate causes of suicidal ideation and behavior:
/// - TB (Thwarted Belongingness): Unmet need to belong
/// - PB (Perceived Burdensomeness): Belief of being a burden
/// - AC (Acquired Capability): Habituation to pain and reduced fear of death
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItsProximalFactor {
    /// Thwarted Belongingness - feeling disconnected from others.
    ThwartedBelongingness,
    /// Perceived Burdensomeness - believing oneself to be a burden.
    PerceivedBurdensomeness,
    /// Acquired Capability - habituation to pain/fear of death.
    AcquiredCapability,
}

impl ItsProximalFactor {
    /// Returns a human-readable name for this factor.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            ItsProximalFactor::ThwartedBelongingness => "Thwarted Belongingness",
            ItsProximalFactor::PerceivedBurdensomeness => "Perceived Burdensomeness",
            ItsProximalFactor::AcquiredCapability => "Acquired Capability",
        }
    }

    /// Returns all proximal factor variants.
    #[must_use]
    pub const fn all() -> [ItsProximalFactor; 3] {
        [
            ItsProximalFactor::ThwartedBelongingness,
            ItsProximalFactor::PerceivedBurdensomeness,
            ItsProximalFactor::AcquiredCapability,
        ]
    }

    /// Returns the short code for this factor.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            ItsProximalFactor::ThwartedBelongingness => "TB",
            ItsProximalFactor::PerceivedBurdensomeness => "PB",
            ItsProximalFactor::AcquiredCapability => "AC",
        }
    }
}

impl std::fmt::Display for ItsProximalFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Convergence status tracking which ITS factors are elevated.
///
/// Per Joiner's ITS, the critical clinical question is which factors
/// have converged. Three-factor convergence (TB + PB + AC all elevated)
/// represents the highest risk state.
///
/// # Risk Matrix
///
/// | TB | PB | AC | Risk Level |
/// |----|----|----|------------|
/// | X  |    |    | Low - passive ideation possible |
/// |    | X  |    | Low - passive ideation possible |
/// |    |    | X  | Low - capability without desire |
/// | X  | X  |    | Moderate - desire without capability |
/// | X  |    | X  | Moderate - belongingness + capability |
/// |    | X  | X  | Moderate - burdensomeness + capability |
/// | X  | X  | X  | HIGH - three-factor convergence |
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct ConvergenceStatus {
    /// True if all three factors (TB, PB, AC) are elevated.
    pub is_three_factor_convergent: bool,

    /// The factor with the highest elevation above threshold.
    pub highest_factor: Option<ItsProximalFactor>,

    /// Count of factors currently elevated (0-3).
    pub elevated_factor_count: u8,

    /// True if TB is above its threshold.
    pub tb_elevated: bool,

    /// True if PB is above its threshold.
    pub pb_elevated: bool,

    /// True if AC is above its threshold.
    pub ac_elevated: bool,
}

impl ConvergenceStatus {
    /// Creates a new convergence status from factor values.
    #[must_use]
    pub fn from_factors(tb: f32, pb: f32, ac: f32) -> Self {
        let tb_elevated = tb >= TB_PRESENT_THRESHOLD;
        let pb_elevated = pb >= PB_PRESENT_THRESHOLD;
        let ac_elevated = ac >= AC_ELEVATED_THRESHOLD;

        let elevated_factor_count =
            u8::from(tb_elevated) + u8::from(pb_elevated) + u8::from(ac_elevated);
        let is_three_factor_convergent = elevated_factor_count == 3;

        // Determine highest factor (by how much it exceeds its threshold)
        let tb_excess = if tb_elevated {
            tb - TB_PRESENT_THRESHOLD
        } else {
            -1.0
        };
        let pb_excess = if pb_elevated {
            pb - PB_PRESENT_THRESHOLD
        } else {
            -1.0
        };
        let ac_excess = if ac_elevated {
            ac - AC_ELEVATED_THRESHOLD
        } else {
            -1.0
        };

        let highest_factor = if tb_excess >= pb_excess && tb_excess >= ac_excess && tb_elevated {
            Some(ItsProximalFactor::ThwartedBelongingness)
        } else if pb_excess >= tb_excess && pb_excess >= ac_excess && pb_elevated {
            Some(ItsProximalFactor::PerceivedBurdensomeness)
        } else if ac_elevated {
            Some(ItsProximalFactor::AcquiredCapability)
        } else {
            None
        };

        ConvergenceStatus {
            is_three_factor_convergent,
            highest_factor,
            elevated_factor_count,
            tb_elevated,
            pb_elevated,
            ac_elevated,
        }
    }

    /// Returns true if suicidal desire is present (TB + PB elevated).
    #[must_use]
    pub fn has_desire(&self) -> bool {
        self.tb_elevated && self.pb_elevated
    }

    /// Returns true if only capability is elevated (no desire).
    #[must_use]
    pub fn is_dormant_capability(&self) -> bool {
        self.ac_elevated && !self.has_desire()
    }

    /// Returns true if desire is present without capability.
    #[must_use]
    pub fn has_desire_without_capability(&self) -> bool {
        self.has_desire() && !self.ac_elevated
    }

    /// Returns a list of currently elevated factors.
    #[must_use]
    pub fn elevated_factors(&self) -> Vec<ItsProximalFactor> {
        let mut factors = Vec::with_capacity(3);
        if self.tb_elevated {
            factors.push(ItsProximalFactor::ThwartedBelongingness);
        }
        if self.pb_elevated {
            factors.push(ItsProximalFactor::PerceivedBurdensomeness);
        }
        if self.ac_elevated {
            factors.push(ItsProximalFactor::AcquiredCapability);
        }
        factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- ItsProximalFactor tests ---

    #[test]
    fn proximal_factor_names() {
        assert_eq!(
            ItsProximalFactor::ThwartedBelongingness.name(),
            "Thwarted Belongingness"
        );
        assert_eq!(
            ItsProximalFactor::PerceivedBurdensomeness.name(),
            "Perceived Burdensomeness"
        );
        assert_eq!(
            ItsProximalFactor::AcquiredCapability.name(),
            "Acquired Capability"
        );
    }

    #[test]
    fn proximal_factor_codes() {
        assert_eq!(ItsProximalFactor::ThwartedBelongingness.code(), "TB");
        assert_eq!(ItsProximalFactor::PerceivedBurdensomeness.code(), "PB");
        assert_eq!(ItsProximalFactor::AcquiredCapability.code(), "AC");
    }

    #[test]
    fn proximal_factor_all() {
        let all = ItsProximalFactor::all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], ItsProximalFactor::ThwartedBelongingness);
        assert_eq!(all[1], ItsProximalFactor::PerceivedBurdensomeness);
        assert_eq!(all[2], ItsProximalFactor::AcquiredCapability);
    }

    #[test]
    fn proximal_factor_display() {
        assert_eq!(
            format!("{}", ItsProximalFactor::ThwartedBelongingness),
            "Thwarted Belongingness"
        );
    }

    #[test]
    fn proximal_factor_copy_clone() {
        let f1 = ItsProximalFactor::ThwartedBelongingness;
        let f2 = f1; // Copy
        let f3 = f1.clone();
        assert_eq!(f1, f2);
        assert_eq!(f1, f3);
    }

    #[test]
    fn proximal_factor_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ItsProximalFactor::ThwartedBelongingness);
        set.insert(ItsProximalFactor::ThwartedBelongingness);
        assert_eq!(set.len(), 1);
        set.insert(ItsProximalFactor::PerceivedBurdensomeness);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn proximal_factor_debug() {
        let debug = format!("{:?}", ItsProximalFactor::AcquiredCapability);
        assert!(debug.contains("AcquiredCapability"));
    }

    // --- ConvergenceStatus tests ---

    #[test]
    fn convergence_status_default() {
        let status = ConvergenceStatus::default();
        assert!(!status.is_three_factor_convergent);
        assert!(status.highest_factor.is_none());
        assert_eq!(status.elevated_factor_count, 0);
        assert!(!status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_no_factors_elevated() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert!(status.highest_factor.is_none());
        assert_eq!(status.elevated_factor_count, 0);
        assert!(!status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_tb_only() {
        let status = ConvergenceStatus::from_factors(0.7, 0.3, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(
            status.highest_factor,
            Some(ItsProximalFactor::ThwartedBelongingness)
        );
        assert_eq!(status.elevated_factor_count, 1);
        assert!(status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_pb_only() {
        let status = ConvergenceStatus::from_factors(0.3, 0.7, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(
            status.highest_factor,
            Some(ItsProximalFactor::PerceivedBurdensomeness)
        );
        assert_eq!(status.elevated_factor_count, 1);
        assert!(!status.tb_elevated);
        assert!(status.pb_elevated);
        assert!(!status.ac_elevated);
    }

    #[test]
    fn convergence_status_ac_only() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.5);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(
            status.highest_factor,
            Some(ItsProximalFactor::AcquiredCapability)
        );
        assert_eq!(status.elevated_factor_count, 1);
        assert!(!status.tb_elevated);
        assert!(!status.pb_elevated);
        assert!(status.ac_elevated);
    }

    #[test]
    fn convergence_status_tb_pb_dual() {
        let status = ConvergenceStatus::from_factors(0.6, 0.7, 0.1);
        assert!(!status.is_three_factor_convergent);
        assert_eq!(status.elevated_factor_count, 2);
        assert!(status.tb_elevated);
        assert!(status.pb_elevated);
        assert!(!status.ac_elevated);
        assert!(status.has_desire());
        assert!(status.has_desire_without_capability());
        assert!(!status.is_dormant_capability());
    }

    #[test]
    fn convergence_status_three_factor() {
        let status = ConvergenceStatus::from_factors(0.7, 0.6, 0.5);
        assert!(status.is_three_factor_convergent);
        assert_eq!(status.elevated_factor_count, 3);
        assert!(status.tb_elevated);
        assert!(status.pb_elevated);
        assert!(status.ac_elevated);
        assert!(status.has_desire());
        assert!(!status.has_desire_without_capability());
        assert!(!status.is_dormant_capability());
    }

    #[test]
    fn convergence_status_dormant_capability() {
        let status = ConvergenceStatus::from_factors(0.3, 0.3, 0.5);
        assert!(status.is_dormant_capability());
        assert!(!status.has_desire());
    }

    #[test]
    fn convergence_status_elevated_factors_list() {
        let status = ConvergenceStatus::from_factors(0.6, 0.7, 0.5);
        let factors = status.elevated_factors();
        assert_eq!(factors.len(), 3);
        assert!(factors.contains(&ItsProximalFactor::ThwartedBelongingness));
        assert!(factors.contains(&ItsProximalFactor::PerceivedBurdensomeness));
        assert!(factors.contains(&ItsProximalFactor::AcquiredCapability));
    }

    #[test]
    fn convergence_status_elevated_factors_empty() {
        let status = ConvergenceStatus::default();
        let factors = status.elevated_factors();
        assert!(factors.is_empty());
    }

    #[test]
    fn convergence_status_highest_factor_by_excess() {
        // TB has highest excess (0.7 - 0.5 = 0.2)
        // PB has lower excess (0.55 - 0.5 = 0.05)
        let status = ConvergenceStatus::from_factors(0.7, 0.55, 0.1);
        assert_eq!(
            status.highest_factor,
            Some(ItsProximalFactor::ThwartedBelongingness)
        );
    }

    #[test]
    fn convergence_status_copy_clone() {
        let s1 = ConvergenceStatus::from_factors(0.6, 0.6, 0.6);
        let s2 = s1; // Copy
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn convergence_status_debug() {
        let status = ConvergenceStatus::default();
        let debug = format!("{:?}", status);
        assert!(debug.contains("ConvergenceStatus"));
    }

    #[test]
    fn ac_elevated_threshold_constant() {
        assert!((AC_ELEVATED_THRESHOLD - 0.3).abs() < f32::EPSILON);
    }
}
