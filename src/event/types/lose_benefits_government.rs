//! LoseBenefitsGovernment event specification.
//!
//! Losing government benefits such as unemployment, disability, social security,
//! welfare, food assistance, housing vouchers, or other public assistance programs.
//! This represents a significant loss of institutional support creating financial
//! strain, reduced autonomy, and institutional betrayal through withdrawn promises.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Losing government benefits causes acute severe emotional distress through institutional rejection, income loss, and material deprivation with chronic ongoing effects during deprivation, but shows substantial hedonic adaptation within 1-2 years.
        // Kahneman & Tversky (1979) Prospect Theory on loss aversion; Sommers et al. (2017) on insurance transitions.
        valence: -0.60,

        // Mood - Arousal
        // Losing government benefits triggers sustained high physiological arousal through loss of control and existential financial threat, activating fight-or-flight response with heightened anxiety and vigilance.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load; Mullainathan & Shafir (2013). Scarcity.
        arousal: 0.52,

        // Mood - Dominance
        // Losing government benefits imposes external loss of control over basic needs and financial security, creating sustained helplessness; individuals lack agency in bureaucratic determinations but retain potential for appeal.
        // Bandura, A. (1977). Self-efficacy; Seligman (1975) learned helplessness; Deci & Ryan (1985) self-determination theory.
        dominance: -0.58,

        // Needs - Fatigue
        // Losing government benefits triggers sustained effort navigating alternatives, sleep disruption from financial anxiety, and chronic rumination over survival needs creating moderate-to-severe fatigue.
        // Hockey, G.R.J. (2013). The psychology of fatigue; Baumeister & Vohs (1998) on ego depletion from sustained self-regulation under financial threat.
        fatigue: 0.55,

        // Needs - Stress
        // Losing government benefits triggers significant physiological stress through financial threat, HPA axis activation, and sustained cortisol elevation from unpredictability and loss of control.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load; Kessler et al. (1988); Dooley et al. (2000).
        stress: 0.65,

        // Needs - Purpose
        // Losing government benefits creates significant disruption to life direction through loss of structural supports and future uncertainty, but most individuals recover within 1-2 years.
        // Frankl, V.E. (1959). Man's Search for Meaning; Ryff, C.D. (1989). Happiness is Everything, or Is It?
        purpose: -0.42,

        // Social Cognition - Loneliness
        // Loss of government benefits increases loneliness through housing instability, reduced participation in activities, and shame-driven social withdrawal creating significant temporary isolation.
        // Berkman, L.F. & Syme, S.L. (1979). Social networks, host resistance, and mortality; House et al. (1988).
        loneliness: 0.28,

        // Social Cognition - PRC
        // Loss of government benefits signals institutional withdrawal of care and creates financial stress that reduces social participation, producing significant but temporary decrease in perceived reciprocal caring.
        // Joiner, T. (2005). Why People Die by Suicide; Baumeister & Leary (1995). The need to belong.
        prc: -0.25,

        // Social Cognition - Perceived Liability
        // Loss of government benefits creates significant financial dependency and loss of autonomy, increasing perception of burdening family/support network with substantial recovery potential.
        // Van Orden et al. (2010). The Interpersonal Theory of Suicide on perceived burdensomeness.
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Benefit loss triggers moderate self-blame and shame through internalized attribution of inadequacy, but shows near-complete adaptation within 12-24 months absent repeated losses.
        // Joiner, T. (2005). Why People Die by Suicide; Gilbert & Irons (2005) on shame and self-directed criticism.
        self_hate: 0.32,

        // Social Cognition - Perceived Competence
        // Benefits loss reduces autonomy and resources for mastery, creating significant but temporary competence doubt in economic/self-sufficiency domains with partial hedonic adaptation.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; economic strain literature.
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Loss events combined with ongoing financial strain and identity disruption create substantial but partially-adaptive depressive response.
        // Brown, G.W. & Harris, T. (1978). Social Origins of Depression; Kendler, K.S. et al. (1999).
        depression: 0.35,

        // Mental Health - Self Worth
        // Loss of government benefits constitutes a significant self-worth threat through material deprivation, identity disruption, and potential welfare stigma reactivation.
        // Crocker & Wolfe (2001) on contingencies of self-worth; Mani et al. (2013) on scarcity; Lott & Bullock (2007) on class-based stigma.
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Losing government benefits creates severe hopelessness through material uncertainty and perceived entrapment, with effects lasting months to years but with significant hedonic adaptation.
        // Abramson, L.Y. et al. (1989). Hopelessness depression: A theory-based subtype of depression; Moffitt et al. (2011).
        hopelessness: 0.55,

        // Mental Health - Interpersonal Hopelessness
        // Loss of government benefits increases interpersonal hopelessness through shame-driven help-avoidance and reduction in social participation capacity, but remains reversible upon benefits restoration.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010).
        interpersonal_hopelessness: 0.25,

        // Mental Health - Acquired Capability
        // Losing government benefits increases suicidal desire through thwarted belongingness and perceived burdensomeness, but does not directly expose individuals to physical pain, death, or violence.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Loss of benefits creates significant financial stress and cognitive load that impairs self-regulation through scarcity-induced executive dysfunction, but recovers substantially once financial stability returns.
        // Mullainathan & Shafir (2013). Scarcity: Why Having Too Little Means So Much; Baumeister & Vohs (2007). Self-Regulation and Ego Depletion.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Loss of government benefits creates acute financial stress that narrows cognitive focus and increases self-concern, temporarily reducing empathic capacity; most individuals show hedonic adaptation.
        // Mullainathan & Shafir (2013). Scarcity; Decety & Jackson (2004). The Functional Architecture of Human Empathy.
        empathy: -0.15,

        // Disposition - Aggression
        // Loss of government benefits creates sustained financial strain and blocked goals triggering moderate frustration-aggression increase, with most effects reversing upon benefit restoration.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis.
        aggression: 0.35,

        // Disposition - Grievance
        // Losing government benefits creates significant perceived injustice through procedural unfairness and systemic blame attribution; approximately 18% becomes permanent through narrative internalization.
        // Lind & Tyler (1988). The social psychology of procedural justice; Miller (2001). Disrespect and the experience of injustice.
        grievance: 0.55,

        // Disposition - Reactance
        // Involuntary loss of benefits triggers moderate reactance through restricted autonomy and perceived control deprivation, but most emotional reactance adapts within 1-2 years if circumstances stabilize.
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological reactance: A theory of freedom and control.
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Institutional betrayal through benefit loss significantly damages trust in systems and generalizes partially to interpersonal trust, but most effects are temporary due to hedonic adaptation.
        // Putnam (2000). Bowling Alone; Mayer, Davis & Schoorman (1995) on institutional trustworthiness factors.
        trust_propensity: -0.35,
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
        perceived_liability: true,
        self_hate: true,
        perceived_competence: true,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.15,
        arousal: 0.14,
        dominance: 0.13,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.10,
        loneliness: 0.06,
        prc: 0.08,
        perceived_liability: 0.08,
        self_hate: 0.08,
        perceived_competence: 0.08,
        depression: 0.08,
        self_worth: 0.12,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.08,
        empathy: 0.05,
        aggression: 0.07,
        grievance: 0.18,
        reactance: 0.12,
        trust_propensity: 0.12,
    },
};
