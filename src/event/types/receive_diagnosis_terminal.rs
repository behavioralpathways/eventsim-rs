//! ReceiveDiagnosisTerminal event specification.
//!
//! Receiving a terminal medical diagnosis - being told by a doctor that you have
//! a fatal illness with a limited life expectancy. One of the most profound
//! psychological stressors with catastrophic impacts across multiple dimensions.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Terminal diagnosis is a catastrophic stressor causing severe negative affect with chronic baseline reduction due to existential burden and mortality awareness.
        // Kubler-Ross, E. (1969). On Death and Dying
        valence: -0.85,

        // Mood - Arousal
        // Terminal diagnosis triggers extreme physiological activation from existential threat, sustained over time as a chronic stressor.
        // Russell, J.A. (1980). A circumplex model of affect; Thayer, R.E. (1989). The Biopsychology of Mood and Arousal
        arousal: 0.80,

        // Mood - Dominance
        // Terminal diagnosis eliminates future autonomy and agency entirely, creating extreme and permanent loss of control more severe than chronic illness.
        // Kubler-Ross, E. (1969); Tedeschi, R.G. & Calhoun, L.G. (2004). Posttraumatic growth
        dominance: -0.80,

        // Needs - Fatigue
        // Terminal diagnosis triggers intense emotional processing and existential confrontation that depletes physical and mental energy reserves acutely.
        // Yalom, I.D. (1980). Existential Psychotherapy; Cancer-related fatigue literature
        fatigue: 0.65,

        // Needs - Stress
        // Terminal diagnosis triggers maximal acute stress response (threat to survival, loss of control, unpredictability) with persistent elevation.
        // Selye, H. (1956). The Stress of Life; McEwen, B.S. allostatic load framework
        stress: 0.95,

        // Needs - Purpose
        // Terminal diagnosis triggers acute existential crisis and severe purpose disruption from goal foreclosure, but activates meaning-making.
        // Steger, M.F. et al. (2006). The Meaning in Life Questionnaire; Frankl, V.E. (1959). Man's Search for Meaning
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Terminal diagnosis increases loneliness through disrupted routines, social withdrawal, fear-based distancing, and existential isolation.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600
        loneliness: 0.65,

        // Social Cognition - PRC
        // Terminal diagnosis increases visible care and social contact from loved ones, though existential anxiety about burdensomeness coexists.
        // Joiner, T. (2005). Why People Die by Suicide; palliative care literature
        prc: 0.25,

        // Social Cognition - Perceived Liability
        // Terminal diagnosis creates sustained perception of being a financial and caregiving burden through progressive dependency and ongoing medical needs.
        // Van Orden, K.A. et al. (2010). The Interpersonal Theory of Suicide; Joiner, T.E. (2005). Why People Die by Suicide
        perceived_liability: 0.72,

        // Social Cognition - Self Hate
        // Terminal diagnosis triggers moderate situational self-blame (guilt about lifestyle factors) and shame about dependency, but not severe self-loathing.
        // Kubler-Ross, E. (1969). On Death and Dying - emotional trajectory showing movement toward acceptance
        self_hate: 0.15,

        // Social Cognition - Perceived Competence
        // Terminal diagnosis creates significant loss of assumed future capability, particularly for health-dependent competencies.
        // Bandura, A. (1997). Self-efficacy: The exercise of control
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Terminal diagnosis is a severe loss event triggering major depressive symptoms including anhedonia and hopelessness.
        // Chochinov et al. palliative care literature; 40-50% depression prevalence in terminal cancer populations
        depression: 0.75,

        // Mental Health - Self Worth
        // Terminal diagnosis triggers identity-level crisis and loss of future self-narrative, causing significant self-worth reduction.
        // Stiegelis et al. (2004); Frankl's logotherapy framework
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Terminal diagnosis creates severe hopelessness due to irreversibility and loss of future possibilities.
        // Beck, A.T. et al. (1974). The Hopelessness Scale; Abramson, L.Y. et al. (1989). Hopelessness depression
        hopelessness: 0.85,

        // Mental Health - Interpersonal Hopelessness
        // Terminal diagnosis increases burden beliefs more than hopelessness about relationships; most patients actively seek connection.
        // Joiner, T.E. (2005). Why People Die by Suicide; Van Orden, K.A. et al. (2010). The Interpersonal Theory of Suicide
        interpersonal_hopelessness: 0.15,

        // Mental Health - Acquired Capability
        // Terminal diagnosis creates existential awareness of death but not immediate pain habituation; progressive disease exposure produces mild growth.
        // Joiner, T. (2005). Why People Die by Suicide - Chapter on acquired capability
        acquired_capability: 0.15,

        // Disposition - Impulse Control
        // Terminal diagnosis acutely depletes self-regulatory resources through extreme emotional distress.
        // Baumeister, R.F. & Vohs, K.D. (2001). Self-regulation and self-control
        impulse_control: -0.35,

        // Disposition - Empathy
        // Terminal diagnosis triggers acute self-focus reducing empathic capacity, but partial recovery occurs through existential processing.
        // Davis, M.H. (1983). Measuring individual differences in empathy; Singer, T. & Klimecki, O.M. (2014)
        empathy: -0.15,

        // Disposition - Aggression
        // Terminal diagnosis triggers acute anger via blocked goals and injustice perception (Kubler-Ross anger stage).
        // Kubler-Ross, E. (1969). On Death and Dying; Berkowitz, L. (1989). Frustration-aggression hypothesis
        aggression: 0.35,

        // Disposition - Grievance
        // Terminal diagnosis triggers acute existential grievance (undeserved suffering, cosmic unfairness).
        // Kubler-Ross, E. (1969). On Death and Dying; Breitbart, W. et al. (2000). Psychotherapeutic interventions at end of life
        grievance: 0.65,

        // Disposition - Reactance
        // Terminal diagnosis imposes severe, involuntary constraints on autonomy and future freedom, triggering strong reactance.
        // Brehm, J.W. (1966). A theory of psychological reactance
        reactance: 0.75,

        // Disposition - Trust Propensity
        // Terminal diagnosis triggers initial distrust and vulnerability in social contracts, with mild erosion of generalized trust propensity.
        // Kubler-Ross, E. (1969). On Death and Dying; end-of-life psychology literature
        trust_propensity: -0.15,
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
        self_hate: false,
        perceived_competence: true,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: true,
        aggression: false,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.25,
        arousal: 0.35,
        dominance: 0.65,
        fatigue: 0.35,
        stress: 0.60,
        purpose: 0.35,
        loneliness: 0.35,
        prc: 0.35,
        perceived_liability: 0.65,
        self_hate: 0.08,
        perceived_competence: 0.18,
        depression: 0.35,
        self_worth: 0.25,
        hopelessness: 0.45,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.12,
        empathy: 0.12,
        aggression: 0.12,
        grievance: 0.20,
        reactance: 0.35,
        trust_propensity: 0.12,
    },
};
