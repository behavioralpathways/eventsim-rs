//! ReceiveSupportFinancial event specification.
//!
//! Receiving financial support, assistance, or aid from others (family, friends, institutions).

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Financial support creates strong positive affect through immediate stress relief and restored agency.
        // Mullainathan & Shafir (2013); Kahneman & Tversky (1979); Cohen & Wills (1985)
        valence: 0.38,

        // Mood - Arousal
        // Reduces physiological arousal through stress relief and threat resolution, activating parasympathetic calming.
        // Thayer (1989); stress-buffering effects of social support research
        arousal: -0.25,

        // Mood - Dominance
        // Temporarily reduces dominance through induced dependency and compromised autonomy, despite alleviating material constraint.
        // Deci & Ryan (1985); Vohs, Mead, & Goode (2006)
        dominance: -0.18,

        // Needs - Fatigue
        // Reduces mental fatigue through stress relief and cognitive load reduction as financial worry dissipates.
        // Baumeister et al. (1998); ego depletion and self-regulatory resource restoration
        fatigue: -0.35,

        // Needs - Stress
        // Financial support directly reduces HPA axis activation and threat perception by addressing resource insufficiency.
        // McEwen (1998); Cohen & Wills (1985) stress buffering hypothesis
        stress: -0.52,

        // Needs - Purpose
        // Removes material stress and enables goal pursuit, providing mild positive impact on purpose.
        // Maslow (1943); Steger et al. (2006)
        purpose: 0.15,

        // Social Cognition - Loneliness
        // Reduces loneliness by alleviating shame-based social withdrawal and perceived burden, but lacks direct social affirmation.
        // Vohs, Mead, & Goode (2006); Joiner (2005)
        loneliness: -0.22,

        // Social Cognition - PRC
        // Financial support demonstrates concrete caring through material investment, signaling reciprocal care.
        // Joiner (2005); Van Orden et al. (2010)
        prc: 0.35,

        // Social Cognition - Perceived Liability
        // Increases perceived liability through creating dependency and violating reciprocity norms.
        // Van Orden et al. (2010, 2012); financial dependence operationalizes perceived burdensomeness
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Provides material security reducing existential threat, but stigma and autonomy loss create mild shame.
        // Stuber & Schlesinger (2006); Drentea & Lavrakas (2000)
        self_hate: -0.15,

        // Social Cognition - Perceived Competence
        // Provides immediate relief and validates financial problem-solving, but external rescue limits self-efficacy gains.
        // Deci & Ryan (2000); Bandura (1997)
        perceived_competence: 0.15,

        // Mental Health - Depression
        // Reduces acute economic stress and alleviates depressive symptoms through restored sense of control and security.
        // Kendler et al. (1999); Brown & Harris (1978)
        depression: -0.35,

        // Mental Health - Self Worth
        // Temporarily boosts self-worth by reducing scarcity threat and signaling care, but most gains adapt away.
        // Crocker & Wolfe (2001); Mullainathan & Shafir (2013)
        self_worth: 0.18,

        // Mental Health - Hopelessness
        // Provides immediate relief from resource scarcity and survival threat, improving near-term future outlook.
        // Abramson et al. (1989); Lyubomirsky et al. (2005) hedonic adaptation
        hopelessness: -0.35,

        // Mental Health - Interpersonal Hopelessness
        // Directly contradicts belief that relationships cannot help by providing concrete proof of meaningful assistance.
        // Joiner (2005); Van Orden et al. (2010); Cohen & Wills (1985)
        interpersonal_hopelessness: -0.38,

        // Mental Health - Acquired Capability
        // Does not expose individual to physical pain, injury, violence, or death; AC develops through habituation pathways only.
        // Joiner (2005); Van Orden et al. (2010)
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Reduces stress-induced cognitive load, restoring self-regulatory capacity through relief and security.
        // Baumeister & Vohs (2003); Shah & Mullainathan (2012) scarcity theory
        impulse_control: 0.25,

        // Disposition - Empathy
        // Reduces cognitive scarcity and stress-induced self-focus, enabling perspective-taking capacity.
        // Mullainathan & Shafir (2013); Singer & Klimecki (2014)
        empathy: 0.12,

        // Disposition - Aggression
        // Reduces aggression by eliminating resource scarcity-driven frustration and blocked goals.
        // Berkowitz (1989) frustration-aggression hypothesis; Mullainathan & Shafir (2013)
        aggression: -0.38,

        // Disposition - Grievance
        // Moderately reduces grievance by addressing material deprivation and signaling fairness.
        // Lind & Tyler (1988) social psychology of procedural justice
        grievance: -0.25,

        // Disposition - Reactance
        // Poses mild temporary reactance through perceived obligation, but autonomy is restored through goal achievement.
        // Brehm & Brehm (1981) psychological reactance theory
        reactance: 0.08,

        // Disposition - Trust Propensity
        // Demonstrates trustworthiness through benevolence and competence, producing moderate positive trust shift.
        // Rotter (1967); Mayer, Davis, & Schoorman (1995)
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
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.06,
        arousal: 0.04,
        dominance: 0.05,
        fatigue: 0.08,
        stress: 0.12,
        purpose: 0.05,
        loneliness: 0.04,
        prc: 0.12,
        perceived_liability: 0.12,
        self_hate: 0.05,
        perceived_competence: 0.04,
        depression: 0.08,
        self_worth: 0.06,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.05,
        empathy: 0.05,
        aggression: 0.09,
        grievance: 0.05,
        reactance: 0.02,
        trust_propensity: 0.08,
    },
};
