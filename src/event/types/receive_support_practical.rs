//! ReceiveSupportPractical event specification.
//!
//! Receiving practical/instrumental support - tangible help with tasks,
//! transportation, childcare, home repairs, moving assistance, etc.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Practical support creates moderate positive affect through stress relief and problem resolution, but lacks the identity validation of emotional support.
        // Cohen & Wills (1985); Thoits (1995); House (1981)
        valence: 0.28,

        // Mood - Arousal
        // Practical support reduces arousal through stress relief and cognitive load reduction, but less intensely than emotional support.
        // Cohen & Wills (1985); Thayer (1989)
        arousal: -0.18,

        // Mood - Dominance
        // Receiving practical support enhances perceived agency by enabling problem-solving capability over concrete challenges.
        // Deci & Ryan (1985)
        dominance: 0.25,

        // Needs - Fatigue
        // Practical support provides immediate energy restoration by eliminating task-related effort and cognitive burden.
        // Sonnentag & Zijlstra (2006)
        fatigue: -0.25,

        // Needs - Stress
        // Practical support reduces immediate physiological stress by removing objective stressors and restoring perceived control.
        // Cohen & Wills (1985)
        stress: -0.35,

        // Needs - Purpose
        // Practical support enables pursuit of meaningful goals by reducing barriers, producing mild temporary purpose enhancement.
        // Thoits (1995); Cohen & Wills (1985)
        purpose: 0.12,

        // Social Cognition - Loneliness
        // Practical support provides moderate temporary loneliness reduction by enabling social participation, though less direct than emotional support.
        // Cacioppo & Patrick (2008); Cutrona & Russell (1990)
        loneliness: -0.12,

        // Social Cognition - PRC
        // Receiving practical help signals genuine caring through concrete action rather than words, producing meaningful increase in perceived reciprocal care.
        // Van Orden et al. (2010); Cutrona & Russell (1990)
        prc: 0.35,

        // Social Cognition - Perceived Liability
        // Practical support creates visible dependency and reciprocity violation, increasing burden perception despite signals of care.
        // Van Orden et al. (2010)
        perceived_liability: 0.22,

        // Social Cognition - Self Hate
        // Practical support reduces self-blame by addressing external circumstances rather than personal failure.
        // Deci & Ryan (2000); Van Orden et al. (2010)
        self_hate: -0.25,

        // Social Cognition - Perceived Competence
        // Practical support provides temporary stress relief and social validation but limited mastery experience, resulting in small positive impact through buffering.
        // Bandura (1997)
        perceived_competence: 0.05,

        // Mental Health - Depression
        // Practical support significantly reduces depressive symptoms through stress relief, behavioral activation, and restored agency.
        // Cohen & Wills (1985); Cutrona & Russell (2008)
        depression: -0.30,

        // Mental Health - Self Worth
        // Practical support provides mild temporary self-worth boost through feeling valued, but risk of dependence-threat limits impact.
        // Cutrona & Russell (1990)
        self_worth: 0.08,

        // Mental Health - Hopelessness
        // Practical support reduces hopelessness by demonstrating concrete problem-solvability and restoring agency.
        // Abramson et al. (1989); House et al. (1988)
        hopelessness: -0.35,

        // Mental Health - Interpersonal Hopelessness
        // Receiving practical support provides concrete evidence that relationships can help, moderately reducing interpersonal hopelessness.
        // Joiner (2005); Van Orden et al. (2010)
        interpersonal_hopelessness: -0.25,

        // Mental Health - Acquired Capability
        // Practical support has no mechanism to habituate individuals to pain or death; operates on different psychological pathways.
        // Joiner (2005); Van Orden et al. (2010)
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Practical support reduces environmental stress and task load, temporarily preserving self-regulatory capacity through cognitive resource conservation.
        // Baumeister et al. (1998); Cohen & Wills (1985)
        impulse_control: 0.15,

        // Disposition - Empathy
        // Receiving practical support reduces cognitive load and stress, temporarily enhancing perspective-taking capacity.
        // Singer & Klimecki (2014)
        empathy: 0.12,

        // Disposition - Aggression
        // Practical support reduces frustration and goal-blocking stress, moderately decreasing aggressive response tendency.
        // Berkowitz (1989); Cohen & Wills (1985)
        aggression: -0.28,

        // Disposition - Grievance
        // Receiving practical support signals fairness and social care, reducing grievance narratives temporarily.
        // Lind & Tyler (1988)
        grievance: -0.18,

        // Disposition - Reactance
        // Receiving practical support typically enhances autonomy by enabling self-directed goals unless explicitly unsolicited or controlling.
        // Brehm & Brehm (1981); Deci & Ryan (1985)
        reactance: 0.05,

        // Disposition - Trust Propensity
        // Practical support demonstrates benevolence and competence through costly helping behavior, producing moderate increase in generalized trust.
        // Mayer, Davis, & Schoorman (1995)
        trust_propensity: 0.16,
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
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.04,
        arousal: 0.03,
        dominance: 0.02,
        fatigue: 0.04,
        stress: 0.04,
        purpose: 0.05,
        loneliness: 0.04,
        prc: 0.12,
        perceived_liability: 0.10,
        self_hate: 0.05,
        perceived_competence: 0.05,
        depression: 0.07,
        self_worth: 0.02,
        hopelessness: 0.04,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.05,
        empathy: 0.04,
        aggression: 0.06,
        grievance: 0.04,
        reactance: 0.02,
        trust_propensity: 0.09,
    },
};
