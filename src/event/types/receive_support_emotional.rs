//! ReceiveSupportEmotional event specification.
//!
//! Receiving genuine emotional support, validation, and care from others.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Emotional support creates substantial positive affect through satisfaction of belonging and validation needs.
        // Baumeister & Leary (1995); Van Orden et al. (2010)
        valence: 0.42,

        // Mood - Arousal
        // Emotional support activates parasympathetic calming through stress buffering and perceived safety.
        // Thayer (1989); Cohen & Wills (1985)
        arousal: -0.25,

        // Mood - Dominance
        // Support validates perspective and reduces isolation, temporarily increasing perceived control and agency.
        // Deci & Ryan (1985); Bandura (1977)
        dominance: 0.15,

        // Needs - Fatigue
        // Reduces acute fatigue through stress buffering and emotional regulation.
        // Cohen & Wills (1985); Baumeister et al. (1998)
        fatigue: -0.25,

        // Needs - Stress
        // Activates parasympathetic nervous system and reduces HPA axis activation through social buffering.
        // Cohen & Wills (1985)
        stress: -0.45,

        // Needs - Purpose
        // Temporarily enhances sense of purpose through restored social connectedness and validation.
        // Ryff (1989); Baumeister & Leary (1995)
        purpose: 0.25,

        // Social Cognition - Loneliness
        // Meaningfully reduces loneliness through direct affirmation of social connection.
        // Cacioppo & Patrick (2008); Joiner (2005)
        loneliness: -0.35,

        // Social Cognition - PRC
        // Directly signals others care, creating significant increase in perceived reciprocal care.
        // Joiner (2005); Van Orden et al. (2010); Baumeister & Leary (1995)
        prc: 0.42,

        // Social Cognition - Perceived Liability
        // Reduces perceived liability by validating worth and signaling reciprocal care.
        // Van Orden et al. (2010)
        perceived_liability: -0.32,

        // Social Cognition - Self Hate
        // Reduces self-directed negativity through external validation and compassionate response.
        // Gilbert & Irons (2005); Van Orden et al. (2010)
        self_hate: -0.40,

        // Social Cognition - Perceived Competence
        // Provides temporary reassurance that moderately increases belief in ability to handle challenges.
        // Bandura (1997)
        perceived_competence: 0.12,

        // Mental Health - Depression
        // Directly reduces depressive symptoms through social buffering and validation.
        // Brown & Harris (1978); Cohen & Wills (1985)
        depression: -0.25,

        // Mental Health - Self Worth
        // Provides direct validation that one is valued as a person, boosting identity-level self-worth.
        // Baumeister & Leary (1995); Crocker & Wolfe (2001)
        self_worth: 0.40,

        // Mental Health - Hopelessness
        // Reduces hopelessness by increasing perceived agency and belonging.
        // Abramson et al. (1989); Beck et al. (1974)
        hopelessness: -0.25,

        // Mental Health - Interpersonal Hopelessness
        // Directly contradicts belief that relationships cannot help, creating meaningful relief.
        // Van Orden et al. (2010); Joiner (2005)
        interpersonal_hopelessness: -0.35,

        // Mental Health - Acquired Capability
        // Does not expose individual to pain or death; addresses belonging/burdensomeness, not habituation.
        // Joiner (2005); Van Orden et al. (2010)
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Temporarily restores self-regulatory resources and reduces stress-induced impairment.
        // Baumeister et al. (1998); Muraven & Baumeister (2000)
        impulse_control: 0.25,

        // Disposition - Empathy
        // Mild positive effect by reducing self-focused concern and enabling perspective-taking.
        // Singer & Klimecki (2014)
        empathy: 0.15,

        // Disposition - Aggression
        // Reduces aggression through validation, belongingness satisfaction, and stress buffering.
        // Cohen & Wills (1985); Baumeister & Leary (1995)
        aggression: -0.28,

        // Disposition - Grievance
        // Moderate temporary reduction by validating experiences and reducing victimization narratives.
        // Tyler (1989); Lind & Tyler (1988)
        grievance: -0.28,

        // Disposition - Reactance
        // Respectfully-offered support reduces reactance by restoring psychological agency.
        // Brehm & Brehm (1981)
        reactance: -0.15,

        // Disposition - Trust Propensity
        // Demonstrates benevolence and care, producing moderate increase in generalized trust.
        // Mayer, Davis, & Schoorman (1995)
        trust_propensity: 0.15,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: false,
        loneliness: false,
        prc: true,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: false,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: true,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.05,
        dominance: 0.04,
        fatigue: 0.05,
        stress: 0.05,
        purpose: 0.08,
        loneliness: 0.06,
        prc: 0.20,
        perceived_liability: 0.08,
        self_hate: 0.06,
        perceived_competence: 0.05,
        depression: 0.08,
        self_worth: 0.12,
        hopelessness: 0.08,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.04,
        empathy: 0.08,
        aggression: 0.07,
        grievance: 0.06,
        reactance: 0.04,
        trust_propensity: 0.08,
    },
};
