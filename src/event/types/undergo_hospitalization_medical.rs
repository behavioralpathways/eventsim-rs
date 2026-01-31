//! UndergoHospitalizationMedical event specification.
//!
//! Being hospitalized for medical treatment including acute illness, surgery,
//! injury recovery, or medical procedures requiring inpatient care.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Medical hospitalization creates sustained moderate-to-severe negative emotional tone due to loss of control, physical stress, and health uncertainty
        // Russell, J.A. (1980). A circumplex model of affect
        valence: -0.45,

        // Mood - Arousal
        // Medical hospitalization triggers sustained moderate-to-high arousal through acute stress response (elevated cortisol, anxiety, vigilance)
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Holmes & Rahe (1967) Social Readjustment Rating Scale
        arousal: 0.55,

        // Mood - Dominance
        // Medical hospitalization significantly reduces sense of control and autonomy through involuntary treatment procedures and dependency on medical staff
        // Rodin, J., & Langer, E. J. (1977). Long-term effects of a control-relevant intervention; Bandura, A. (1977). Self-efficacy theory
        dominance: -0.55,

        // Needs - Fatigue
        // Medical hospitalization causes acute physical exhaustion, medication effects, sleep disruption, and medical stress with moderate chronic elevation
        // Hockey, G.R.J. (2013). The psychology of fatigue; Baumeister, R.F. et al. (1998). Ego depletion
        fatigue: 0.48,

        // Needs - Stress
        // Medical hospitalization triggers acute physiological stress through genuine health threat, loss of control, and environmental stressors
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load
        stress: 0.62,

        // Needs - Purpose
        // Medical hospitalization disrupts life plans and roles, creating temporary existential questioning, but most recover purpose within months post-recovery
        // Frankl, V.E. (1959). Man's Search for Meaning; Steger, M.F. et al. (2006). The Meaning in Life Questionnaire
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Medical hospitalization temporarily increases loneliness through disrupted social connections and physical isolation
        // Cacioppo & Patrick (2008). Loneliness: Human nature and the need for social connection
        loneliness: 0.35,

        // Social Cognition - PRC
        // Medical hospitalization typically increases perceived caring during acute phase through intensified family presence and emotional support
        // Lepore, S.J., & Helgeson, V.S. (1998). Social constraints and mental health after prostate cancer
        prc: 0.25,

        // Social Cognition - Perceived Liability
        // Medical hospitalization creates significant temporary burden perception due to physical dependency and functional impairment
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2012). The Interpersonal Needs Questionnaire
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Medical hospitalization is typically externally attributed (circumstantial health event), producing minimal self-blame or self-directed negativity
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        self_hate: 0.05,

        // Social Cognition - Perceived Competence
        // Medical hospitalization temporarily reduces perceived competence through loss of autonomy and control
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Deci & Ryan (2000) self-determination theory
        perceived_competence: -0.30,

        // Mental Health - Depression
        // Medical hospitalization triggers acute depressive symptoms through dependency, mortality awareness, and autonomic loss
        // Kendler, K.S. et al. (1999). Causal relationship between stressful life events and depression
        depression: 0.32,

        // Mental Health - Self Worth
        // Medical hospitalization temporarily diminishes self-worth through loss of autonomy, physical vulnerability, and dependence on others
        // Baumeister, R.F. et al. (2003). Does High Self-Esteem Cause Better Performance? Psychological Science in the Public Interest
        self_worth: -0.25,

        // Mental Health - Hopelessness
        // Hospitalization produces moderate acute hopelessness from acute medical uncertainty and autonomy loss
        // Brown, G.W. & Harris, T. (1978). Social origins of depression
        hopelessness: 0.25,

        // Mental Health - Interpersonal Hopelessness
        // Medical hospitalization typically triggers help-seeking and demonstrates social support effectiveness through family/friend involvement
        // Joiner, T. (2005). Why People Die by Suicide - help-seeking counters interpersonal hopelessness
        interpersonal_hopelessness: -0.15,

        // Mental Health - Acquired Capability
        // Medical hospitalization provides mild exposure to pain and mortality awareness, but protective medical context limits habituation
        // Joiner, T. (2005). Why People Die by Suicide - acquired capability develops through repeated, volitional exposure
        acquired_capability: 0.10,

        // Disposition - Impulse Control
        // Medical hospitalization significantly impairs impulse control through physiological stress, sleep disruption, and ego depletion
        // Baumeister, R.F., & Vohs, K.D. (2016). Strength model of self-regulation as limited resource
        impulse_control: -0.35,

        // Disposition - Empathy
        // Acute hospitalization increases self-focus and stress, temporarily reducing perspective-taking capacity
        // Decety, J. & Jackson, P.L. (2004). The functional architecture of human empathy
        empathy: -0.15,

        // Disposition - Aggression
        // Medical hospitalization creates frustration and loss of autonomy that produces mild acute irritability
        // Berkowitz, L. (1989). Frustration-aggression hypothesis
        aggression: 0.15,

        // Disposition - Grievance
        // Medical hospitalization often triggers "why me?" attributions and loss of autonomy, creating moderate temporary grievance
        // Miller, D.T. (2001). Disrespect and the experience of injustice; Lind & Tyler (1988) procedural justice
        grievance: 0.25,

        // Disposition - Reactance
        // Medical hospitalization imposes temporary constraints on autonomy and freedom, mitigated by patient consent and medical necessity
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological Reactance: A Theory of Freedom and Control
        reactance: 0.28,

        // Disposition - Trust Propensity
        // Medical hospitalization involves trusting professionals who typically demonstrate competence and benevolence
        // Mayer, R.C., Davis, J.H., & Schoorman, F.D. (1995). An integrative model of organizational trust
        trust_propensity: 0.12,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: false,
        fatigue: true,
        stress: false,
        purpose: false,
        loneliness: false,
        prc: false,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: false,
        grievance: true,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.08,
        dominance: 0.05,
        fatigue: 0.12,
        stress: 0.06,
        purpose: 0.08,
        loneliness: 0.05,
        prc: 0.08,
        perceived_liability: 0.05,
        self_hate: 0.02,
        perceived_competence: 0.05,
        depression: 0.10,
        self_worth: 0.10,
        hopelessness: 0.10,
        interpersonal_hopelessness: 0.03,
        impulse_control: 0.05,
        empathy: 0.05,
        aggression: 0.03,
        grievance: 0.08,
        reactance: 0.06,
        trust_propensity: 0.05,
    },
};
