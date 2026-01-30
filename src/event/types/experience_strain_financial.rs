//! ExperienceStrainFinancial event specification.
//!
//! Experiencing financial strain or hardship - ongoing economic stress from insufficient
//! resources to meet needs. This differs from acute job loss or economic recession as it
//! represents a persistent state of financial insecurity creating chronic psychological
//! burden through resource scarcity, reduced autonomy, and identity threat.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Financial strain produces severe negative valence through loss of control and future uncertainty, with moderate hedonic adaptation over 12-18 months as individuals adjust to new circumstances.
        // Vohs, K.D., Mead, N.L., & Goode, M.R. (2006). The Psychological Consequences of Money; Kahneman & Tversky (1979). Prospect Theory on loss aversion.
        valence: -0.55,

        // Mood - Arousal
        // Financial strain activates sustained anxiety and vigilance without reaching crisis-level arousal; chronic stressor creating moderate-to-high activation with modest permanent baseline shift due to hedonic adaptation.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Russell, J.A. (1980). A circumplex model of affect.
        arousal: 0.55,

        // Mood - Dominance
        // Financial strain significantly reduces perceived control and agency as resources become constrained and options narrow, with chronic effects if strain persists, but shows partial adaptation as individuals develop coping strategies.
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change; Deci, E.L. & Ryan, R.M. (1985). Intrinsic motivation and self-determination.
        dominance: -0.55,

        // Needs - Fatigue
        // Financial strain triggers sustained rumination and sleep disruption creating moderate mental and physical fatigue with substantial recovery once financial stability returns.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load; Hockey, G.R.J. (2013). The psychology of fatigue.
        fatigue: 0.45,

        // Needs - Stress
        // Financial strain is a major stressor triggering HPA axis activation and sustained cortisol elevation through unpredictability and loss of control, with significant but largely reversible effects.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load.
        stress: 0.55,

        // Needs - Purpose
        // Financial strain moderately disrupts purpose by blocking goal progression and creating uncertainty about future direction, but typically resolves with economic recovery.
        // Steger, M.F. et al. (2006). The Meaning in Life Questionnaire; Frankl, V.E. (1959). Man's Search for Meaning.
        purpose: -0.28,

        // Social Cognition - Loneliness
        // Financial strain creates social withdrawal through shame, reduced participation in activities, and economic constraints on maintaining relationships, leading to significant but temporally-bounded loneliness increase.
        // Cacioppo & Patrick (2008). Loneliness: Human nature and the need for social connection; Joiner, T. (2005). Why People Die by Suicide.
        loneliness: 0.35,

        // Social Cognition - PRC
        // Financial strain reduces perceived reciprocal caring through shame-driven social withdrawal and internalized burden beliefs, with substantial recovery upon financial stabilization.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        prc: -0.22,

        // Social Cognition - Perceived Liability
        // Financial strain creates significant perceived liability through inability to contribute and dependency dynamics, with chronic effects from ongoing economic uncertainty, but most people show substantial recovery when financial circumstances improve.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2012). The Interpersonal Needs Questionnaire.
        perceived_liability: 0.42,

        // Social Cognition - Self Hate
        // Financial strain triggers moderate self-blame and shame about personal financial management, but the attribution is mixed (partly circumstantial), and hedonic adaptation occurs over 1-2 years despite chronicity.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Financial strain undermines mastery experiences and goal-accomplishment beliefs across domains, creating significant but moderately recoverable doubt in one's ability to manage life circumstances.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Conger & Conger (2002). Financial stress cascade model.
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Financial strain triggers moderate-to-significant depressive symptoms through loss of control, identity threat, and sustained uncertainty; its chronic nature and rumination component produce greater impact than acute stressors, but recovery occurs as financial stability improves.
        // Kendler, K.S. et al. (1999). Causal relationship between stressful life events and depression; Brown, G.W. & Harris, T. (1978). Social origins of depression.
        depression: 0.38,

        // Mental Health - Self Worth
        // Financial strain threatens identity-relevant financial competence and self-sufficiency with sustained rumination, producing significant temporary self-worth reduction with modest permanence through hedonic adaptation and eventual economic recovery.
        // Crocker, J. & Wolfe, C.T. (2001). Contingencies of self-worth; Baumeister, R.F. et al. (2003). Does high self-esteem cause better performance?
        self_worth: -0.38,

        // Mental Health - Hopelessness
        // Financial strain creates significant hopelessness through direct personal threat, perceived entrapment, and attribution of hardship to uncontrollable future circumstances, with moderate permanence due to sustained financial pressure and limited hedonic adaptation.
        // Abramson, L.Y. et al. (1989). Hopelessness depression: A theory-based subtype of depression; Joiner et al. (2009). Financial hardship and thwarted belongingness.
        hopelessness: 0.42,

        // Mental Health - Interpersonal Hopelessness
        // Financial strain increases interpersonal hopelessness through shame-based withdrawal from social support and beliefs that others cannot or will not help with economic problems.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        interpersonal_hopelessness: 0.30,

        // Mental Health - Acquired Capability
        // Financial strain causes psychological distress and perceived burden but does not expose individuals to physical pain, injury, or death proximity - the mechanisms by which acquired capability develops according to Joiner's ITS framework.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Financial strain creates significant acute impairment through ego depletion and scarcity-induced cognitive load, with chronic stress component impairing executive function, but shows substantial recovery upon financial resolution.
        // Mullainathan, S. & Shafir, E. (2013). Scarcity: Why Having Too Little Means So Much; Baumeister, R.F. & Muraven, M. (2000). Self-regulation and depletion of limited resources.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Financial strain induces temporary self-focus that moderately reduces empathy through cognitive scarcity and stress, with small chronic effects through repeated activation of threat response and emotional depletion.
        // Mullainathan, S. & Shafir, E. (2013). Scarcity: Why having too little means so much; Singer, T. & Klimecki, O.M. (2014). Empathy and compassion.
        empathy: -0.15,

        // Disposition - Aggression
        // Financial strain creates sustained frustration and status threat that increase hostile tendencies through multiple pathways, but shows significant recovery with resolution due to hedonic adaptation.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson, C.A. & Bushman, B.J. (2002). Human aggression.
        aggression: 0.32,

        // Disposition - Grievance
        // Financial strain triggers moderate grievance through relative deprivation and systemic attribution of unfairness, with chronicity during ongoing hardship but substantial recovery as financial conditions improve.
        // Stouffer et al. (1949). Relative Deprivation Theory; Lind & Tyler (1988). The social psychology of procedural justice.
        grievance: 0.38,

        // Disposition - Reactance
        // Financial strain imposes significant restrictions on freedom and autonomy (triggering reactance), but most people show substantial adaptation within 1-2 years with diminished chronic effects.
        // Brehm, J.W. (1966). A theory of psychological reactance.
        reactance: 0.30,

        // Disposition - Trust Propensity
        // Financial strain reduces generalized trust through resource scarcity concerns and increased wariness of exploitation, but does not involve direct interpersonal betrayal, resulting in moderate negative impact with limited permanence due to hedonic adaptation.
        // Rotter, J.B. (1967). A new scale for the measurement of interpersonal trust; Putnam & Feldstein (2003). Social capital erosion during economic downturns.
        trust_propensity: -0.25,
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
        valence: 0.12,
        arousal: 0.12,
        dominance: 0.12,
        fatigue: 0.09,
        stress: 0.12,
        purpose: 0.08,
        loneliness: 0.12,
        prc: 0.09,
        perceived_liability: 0.12,
        self_hate: 0.12,
        perceived_competence: 0.08,
        depression: 0.10,
        self_worth: 0.11,
        hopelessness: 0.15,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.06,
        empathy: 0.08,
        aggression: 0.08,
        grievance: 0.18,
        reactance: 0.08,
        trust_propensity: 0.08,
    },
};
