//! Event impact data structures.
//! DRAFT for review - not yet integrated into the codebase.

/// Macro to apply an operation to all 22 fields.
macro_rules! for_each_field {
    ($self:ident, $other:ident, $op:tt) => {
        Self {
            valence: $self.valence $op $other.valence,
            arousal: $self.arousal $op $other.arousal,
            dominance: $self.dominance $op $other.dominance,
            fatigue: $self.fatigue $op $other.fatigue,
            stress: $self.stress $op $other.stress,
            purpose: $self.purpose $op $other.purpose,
            loneliness: $self.loneliness $op $other.loneliness,
            prc: $self.prc $op $other.prc,
            perceived_liability: $self.perceived_liability $op $other.perceived_liability,
            self_hate: $self.self_hate $op $other.self_hate,
            perceived_competence: $self.perceived_competence $op $other.perceived_competence,
            depression: $self.depression $op $other.depression,
            self_worth: $self.self_worth $op $other.self_worth,
            hopelessness: $self.hopelessness $op $other.hopelessness,
            interpersonal_hopelessness: $self.interpersonal_hopelessness $op $other.interpersonal_hopelessness,
            acquired_capability: $self.acquired_capability $op $other.acquired_capability,
            impulse_control: $self.impulse_control $op $other.impulse_control,
            empathy: $self.empathy $op $other.empathy,
            aggression: $self.aggression $op $other.aggression,
            grievance: $self.grievance $op $other.grievance,
            reactance: $self.reactance $op $other.reactance,
            trust_propensity: $self.trust_propensity $op $other.trust_propensity,
        }
    };
}

macro_rules! scale_fields {
    ($self:ident, $scalar:ident) => {
        Self {
            valence: $self.valence * $scalar,
            arousal: $self.arousal * $scalar,
            dominance: $self.dominance * $scalar,
            fatigue: $self.fatigue * $scalar,
            stress: $self.stress * $scalar,
            purpose: $self.purpose * $scalar,
            loneliness: $self.loneliness * $scalar,
            prc: $self.prc * $scalar,
            perceived_liability: $self.perceived_liability * $scalar,
            self_hate: $self.self_hate * $scalar,
            perceived_competence: $self.perceived_competence * $scalar,
            depression: $self.depression * $scalar,
            self_worth: $self.self_worth * $scalar,
            hopelessness: $self.hopelessness * $scalar,
            interpersonal_hopelessness: $self.interpersonal_hopelessness * $scalar,
            acquired_capability: $self.acquired_capability * $scalar,
            impulse_control: $self.impulse_control * $scalar,
            empathy: $self.empathy * $scalar,
            aggression: $self.aggression * $scalar,
            grievance: $self.grievance * $scalar,
            reactance: $self.reactance * $scalar,
            trust_propensity: $self.trust_propensity * $scalar,
        }
    };
}

macro_rules! mask_fields {
    ($flags:ident, $impact:ident, $keep_when:expr, $ac_value:expr) => {
        EventImpact {
            valence: if $flags.valence == $keep_when { $impact.valence } else { 0.0 },
            arousal: if $flags.arousal == $keep_when { $impact.arousal } else { 0.0 },
            dominance: if $flags.dominance == $keep_when { $impact.dominance } else { 0.0 },
            fatigue: if $flags.fatigue == $keep_when { $impact.fatigue } else { 0.0 },
            stress: if $flags.stress == $keep_when { $impact.stress } else { 0.0 },
            purpose: if $flags.purpose == $keep_when { $impact.purpose } else { 0.0 },
            loneliness: if $flags.loneliness == $keep_when { $impact.loneliness } else { 0.0 },
            prc: if $flags.prc == $keep_when { $impact.prc } else { 0.0 },
            perceived_liability: if $flags.perceived_liability == $keep_when { $impact.perceived_liability } else { 0.0 },
            self_hate: if $flags.self_hate == $keep_when { $impact.self_hate } else { 0.0 },
            perceived_competence: if $flags.perceived_competence == $keep_when { $impact.perceived_competence } else { 0.0 },
            depression: if $flags.depression == $keep_when { $impact.depression } else { 0.0 },
            self_worth: if $flags.self_worth == $keep_when { $impact.self_worth } else { 0.0 },
            hopelessness: if $flags.hopelessness == $keep_when { $impact.hopelessness } else { 0.0 },
            interpersonal_hopelessness: if $flags.interpersonal_hopelessness == $keep_when { $impact.interpersonal_hopelessness } else { 0.0 },
            acquired_capability: $ac_value,
            impulse_control: if $flags.impulse_control == $keep_when { $impact.impulse_control } else { 0.0 },
            empathy: if $flags.empathy == $keep_when { $impact.empathy } else { 0.0 },
            aggression: if $flags.aggression == $keep_when { $impact.aggression } else { 0.0 },
            grievance: if $flags.grievance == $keep_when { $impact.grievance } else { 0.0 },
            reactance: if $flags.reactance == $keep_when { $impact.reactance } else { 0.0 },
            trust_propensity: if $flags.trust_propensity == $keep_when { $impact.trust_propensity } else { 0.0 },
        }
    };
}

/// Impact values for all 22 psychological dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EventImpact {
    // Mood (PAD)
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,

    // Needs
    pub fatigue: f32,
    pub stress: f32,
    pub purpose: f32,

    // Social Cognition
    pub loneliness: f32,
    pub prc: f32,
    pub perceived_liability: f32,
    pub self_hate: f32,
    pub perceived_competence: f32,

    // Mental Health
    pub depression: f32,
    pub self_worth: f32,
    pub hopelessness: f32,
    pub interpersonal_hopelessness: f32,
    pub acquired_capability: f32,

    // Disposition
    pub impulse_control: f32,
    pub empathy: f32,
    pub aggression: f32,
    pub grievance: f32,
    pub reactance: f32,
    pub trust_propensity: f32,
}

impl EventImpact {
    pub fn scale(&self, s: f32) -> Self {
        scale_fields!(self, s)
    }

    pub fn add(&self, other: &Self) -> Self {
        for_each_field!(self, other, +)
    }

    /// Component-wise multiply with permanence. AC always keeps full value.
    pub fn mul_permanence(&self, p: &PermanenceValues) -> Self {
        Self {
            valence: self.valence * p.valence,
            arousal: self.arousal * p.arousal,
            dominance: self.dominance * p.dominance,
            fatigue: self.fatigue * p.fatigue,
            stress: self.stress * p.stress,
            purpose: self.purpose * p.purpose,
            loneliness: self.loneliness * p.loneliness,
            prc: self.prc * p.prc,
            perceived_liability: self.perceived_liability * p.perceived_liability,
            self_hate: self.self_hate * p.self_hate,
            perceived_competence: self.perceived_competence * p.perceived_competence,
            depression: self.depression * p.depression,
            self_worth: self.self_worth * p.self_worth,
            hopelessness: self.hopelessness * p.hopelessness,
            interpersonal_hopelessness: self.interpersonal_hopelessness * p.interpersonal_hopelessness,
            acquired_capability: self.acquired_capability, // AC ignores permanence
            impulse_control: self.impulse_control * p.impulse_control,
            empathy: self.empathy * p.empathy,
            aggression: self.aggression * p.aggression,
            grievance: self.grievance * p.grievance,
            reactance: self.reactance * p.reactance,
            trust_propensity: self.trust_propensity * p.trust_propensity,
        }
    }
}

/// Per-dimension chronic flags.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ChronicFlags {
    pub valence: bool,
    pub arousal: bool,
    pub dominance: bool,
    pub fatigue: bool,
    pub stress: bool,
    pub purpose: bool,
    pub loneliness: bool,
    pub prc: bool,
    pub perceived_liability: bool,
    pub self_hate: bool,
    pub perceived_competence: bool,
    pub depression: bool,
    pub self_worth: bool,
    pub hopelessness: bool,
    pub interpersonal_hopelessness: bool,
    pub impulse_control: bool,
    pub empathy: bool,
    pub aggression: bool,
    pub grievance: bool,
    pub reactance: bool,
    pub trust_propensity: bool,
}

impl ChronicFlags {
    /// Keep values where chronic is true. AC always 0.
    pub fn mask(&self, impact: &EventImpact) -> EventImpact {
        mask_fields!(self, impact, true, 0.0)
    }

    /// Keep values where chronic is false. AC always 0.
    pub fn mask_inverse(&self, impact: &EventImpact) -> EventImpact {
        mask_fields!(self, impact, false, 0.0)
    }
}

/// Per-dimension permanence values (0.0 to 1.0).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PermanenceValues {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub fatigue: f32,
    pub stress: f32,
    pub purpose: f32,
    pub loneliness: f32,
    pub prc: f32,
    pub perceived_liability: f32,
    pub self_hate: f32,
    pub perceived_competence: f32,
    pub depression: f32,
    pub self_worth: f32,
    pub hopelessness: f32,
    pub interpersonal_hopelessness: f32,
    pub impulse_control: f32,
    pub empathy: f32,
    pub aggression: f32,
    pub grievance: f32,
    pub reactance: f32,
    pub trust_propensity: f32,
}

impl PermanenceValues {
    /// Returns 1.0 - self (the temporary portion)
    pub fn inverse(&self) -> Self {
        Self {
            valence: 1.0 - self.valence,
            arousal: 1.0 - self.arousal,
            dominance: 1.0 - self.dominance,
            fatigue: 1.0 - self.fatigue,
            stress: 1.0 - self.stress,
            purpose: 1.0 - self.purpose,
            loneliness: 1.0 - self.loneliness,
            prc: 1.0 - self.prc,
            perceived_liability: 1.0 - self.perceived_liability,
            self_hate: 1.0 - self.self_hate,
            perceived_competence: 1.0 - self.perceived_competence,
            depression: 1.0 - self.depression,
            self_worth: 1.0 - self.self_worth,
            hopelessness: 1.0 - self.hopelessness,
            interpersonal_hopelessness: 1.0 - self.interpersonal_hopelessness,
            impulse_control: 1.0 - self.impulse_control,
            empathy: 1.0 - self.empathy,
            aggression: 1.0 - self.aggression,
            grievance: 1.0 - self.grievance,
            reactance: 1.0 - self.reactance,
            trust_propensity: 1.0 - self.trust_propensity,
        }
    }
}

/// Complete event specification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventSpec {
    pub impact: EventImpact,
    pub chronic: ChronicFlags,
    pub permanence: PermanenceValues,
}

/// Result of applying an event at a given severity.
#[derive(Debug, Clone, Copy, Default)]
pub struct AppliedDeltas {
    pub permanent: EventImpact,
    pub acute: EventImpact,
    pub chronic: EventImpact,
}

impl EventSpec {
    pub fn apply(&self, severity: f32) -> AppliedDeltas {
        let s = severity.clamp(0.0, 1.0);
        let scaled = self.impact.scale(s);
        let temp_portion = self.permanence.inverse();
        let temp_scaled = scaled.mul_permanence(&temp_portion);

        AppliedDeltas {
            permanent: scaled.mul_permanence(&self.permanence),
            acute: self.chronic.mask_inverse(&temp_scaled),
            chronic: self.chronic.mask(&temp_scaled),
        }
    }
}
