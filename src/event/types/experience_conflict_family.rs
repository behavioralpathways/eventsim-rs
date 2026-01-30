//! ExperienceConflictFamily event specification.
//!
//! Experiencing significant family conflict - arguments, disputes, hostility,
//! or ongoing tension between family members that creates a stressful home
//! environment and undermines relational security.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Family conflict creates moderate negative affect through sustained stress and threat to relational security
        // Davies & Cummings (1994) Psychological Bulletin - https://psycnet.apa.org/record/1994-28559-001
        valence: -0.30,

        // Mood - Arousal
        // Family conflict triggers fight-or-flight activation through threat perception and social stress
        // Posner, Russell & Peterson (2005) on circumplex affect and threat arousal
        arousal: 0.55,

        // Mood - Dominance
        // Family conflict creates moderate loss of control as an imposed, ongoing stressor that individuals typically cannot directly resolve
        // Cummings & Davies (2002); Bandura (1977) on self-efficacy and controllability
        dominance: -0.35,

        // Needs - Fatigue
        // Family conflict creates sustained emotional processing and stress-induced rumination causing moderate mental and physical fatigue
        // Baumeister et al. (1998) on ego depletion; Hockey (2013) on psychology of fatigue
        fatigue: 0.35,

        // Needs - Stress
        // Family conflict triggers significant HPA axis activation and cortisol release through threat perception and loss of environmental control
        // McEwen (1998) Annals of NY Academy of Sciences - allostasis and allostatic load
        stress: 0.55,

        // Needs - Purpose
        // Family conflict disrupts meaning systems and consumes emotional energy needed for goal pursuit
        // Cummings & Davies (2002) Journal of Child Psychology and Psychiatry
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Family conflict creates perceived emotional distance and withdrawal, increasing loneliness through disrupted safe attachment
        // Cacioppo & Patrick (2008) - https://psycnet.apa.org/record/2008-07214-000
        loneliness: 0.35,

        // Social Cognition - PRC
        // Family conflict signals that one's wellbeing is deprioritized relative to interpersonal disputes
        // Joiner (2005) Why People Die by Suicide - belongingness perception pathways
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Family conflict temporarily increases felt responsibility for family tension, but typically recovers as conflict resolves
        // Van Orden et al. (2010) Interpersonal Theory of Suicide - https://doi.org/10.1037/a0018697
        perceived_liability: 0.15,

        // Social Cognition - Self Hate
        // Family conflict triggers moderate self-blame and shame for being unable to prevent or stop the conflict
        // Van Orden et al. (2010) - https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2913990/
        self_hate: 0.28,

        // Social Cognition - Perceived Competence
        // Ongoing family hostility reduces performance feedback quality and creates cognitive load, moderately lowering self-efficacy beliefs
        // Bandura (1997) Self-Efficacy: The Exercise of Control - https://psycnet.apa.org/record/1997-08589-000
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Family conflict is a chronic stressor that reliably triggers depressive symptoms through rumination and social withdrawal
        // Brown & Harris (1978); Kendler et al. (1999) on stressful life events and depression
        depression: 0.35,

        // Mental Health - Self Worth
        // Ongoing family conflict creates chronic psychological threat that moderately reduces self-worth through perceived inadequacy
        // Cummings & Davies meta-analysis - https://psycnet.apa.org/doi/10.1037/0033-2909.121.3.406
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Family conflict increases pessimism about future family/relationship stability through loss of safety and control
        // Davies & Cummings (1994) - https://doi.org/10.1037/0012-1649.26.6.915
        hopelessness: 0.25,

        // Mental Health - Interpersonal Hopelessness
        // Family conflict increases interpersonal hopelessness by creating learned beliefs that relationships are unreliable and painful
        // Cummings & Davies (2010) on interparental conflict effects on relationship perception
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Family conflict is a psychosocial stressor that does not involve direct exposure to physical pain, injury, violence, or death
        // Joiner ITS model - AC requires habituation to pain/death, not emotional stress
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Family conflict creates sustained emotional stress that depletes self-regulatory capacity through cortisol elevation and cognitive load
        // Baumeister et al. (1998) Journal of Personality and Social Psychology - ego depletion
        impulse_control: -0.35,

        // Disposition - Empathy
        // Family conflict triggers self-focus and threat responses that temporarily reduce empathic capacity
        // Singer & Klimecki (2014) Current Biology - state vs trait empathy distinction
        empathy: -0.15,

        // Disposition - Aggression
        // Family conflict creates significant acute aggression elevation through frustration and injustice perception
        // Berkowitz (1989) Frustration-Aggression Hypothesis
        aggression: 0.35,

        // Disposition - Grievance
        // Ongoing family conflict creates moderate chronic grievance through perceived unfairness and betrayal by close relations
        // Mikula (1993) on experience of injustice in close relationships
        grievance: 0.38,

        // Disposition - Reactance
        // Family conflict creates ongoing threat to autonomy and freedom through environmental constraint and perceived loss of control
        // Brehm (1966) A Theory of Psychological Reactance
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Family conflict undermines foundational trust security, creating significant generalized distrust of others
        // Rotter (1967) Interpersonal Trust Scale - https://psycnet.apa.org/record/1968-05002-001
        trust_propensity: -0.40,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: true,
        loneliness: true,
        prc: true,
        perceived_liability: false,
        self_hate: true,
        perceived_competence: true,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.12,
        dominance: 0.08,
        fatigue: 0.12,
        stress: 0.08,
        purpose: 0.08,
        loneliness: 0.08,
        prc: 0.12,
        perceived_liability: 0.04,
        self_hate: 0.06,
        perceived_competence: 0.12,
        depression: 0.12,
        self_worth: 0.12,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.06,
        empathy: 0.05,
        aggression: 0.12,
        grievance: 0.14,
        reactance: 0.12,
        trust_propensity: 0.18,
    },
};
