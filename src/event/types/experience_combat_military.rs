//! ExperienceCombatMilitary event specification.
//!
//! Direct participation in military combat operations including engaging enemy forces,
//! firefights, patrols in hostile territory, and combat missions involving life-threatening
//! situations and exposure to violence and death.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Military combat exposure causes severe negative valence from trauma response, with substantial chronic effects through PTSD symptomatology and anhedonia.
        // DSM-5 PTSD criteria; Foa & Kozak (1986) emotional processing theory; King et al. (2006) National Vietnam Veterans Readjustment Study
        valence: -0.75,

        // Mood - Arousal
        // Military combat creates extreme acute arousal during engagement and persistent hyperarousal as a core PTSD symptom with neurobiological changes that resist full recovery.
        // Friedman, M.J. (2006). Posttraumatic stress disorder among military returnees from Afghanistan and Iraq. American Journal of Psychiatry
        arousal: 0.85,

        // Mood - Dominance
        // Combat exposure involves imposed life-threatening scenarios with minimal personal agency, creating significant loss of dominance through learned helplessness and hypervigilance.
        // Seligman, M.E.P. (1975). Helplessness: On depression, development, and death; DSM-5 PTSD criteria re: hypervigilance and loss of sense of safety
        dominance: -0.65,

        // Needs - Fatigue
        // Combat causes severe acute exhaustion from sustained physiological stress, threat response, and sleep disruption, with chronic hypervigilance maintaining elevated fatigue.
        // Hockey, G.R.J. (2013). The psychology of fatigue; National Center for PTSD research on combat-related fatigue
        fatigue: 0.75,

        // Needs - Stress
        // Combat exposure triggers maximum HPA axis activation through life threat; creates lasting elevation in baseline stress reactivity through HPA axis sensitization.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load; Sapolsky, R.M. (2015) on chronic stress physiology
        stress: 0.85,

        // Needs - Purpose
        // Combat disrupts pre-deployment life purposes through acute trauma and identity crisis upon reintegration, though mission-driven purpose during service provides partial offset.
        // Frankl, V.E. (1959). Man's Search for Meaning; Litz, B.T. et al. (2009). Moral injury and moral repair in war veterans
        purpose: -0.15,

        // Social Cognition - Loneliness
        // Combat exposure produces significant loneliness through loss of unit cohesion, PTSD-related social withdrawal, and civilian reintegration difficulties.
        // Van Orden et al. (2010). The Interpersonal Theory of Suicide; Pietrzak et al. (2010) on loneliness in combat veterans
        loneliness: 0.35,

        // Social Cognition - PRC
        // Combat creates significant reduction in perceived reciprocal care through social isolation post-deployment and perceived incomprehension by civilian populations.
        // Kaplan et al. (2012). Suicide among male veterans; Sayer et al. (2010) on PTSD and social support
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Combat creates persistent beliefs of burdensomeness through moral injury, disability, and PTSD-related social withdrawal.
        // Van Orden et al. (2010). The Interpersonal Theory of Suicide; Bryan & Hernandez (2012) on perceived burdensomeness in combat soldiers
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Combat triggers persistent moral injury and survivor guilt with substantial self-blame; internalized shame remains significantly elevated as chronic baseline state.
        // Litz, B.T., et al. (2009). Moral injury and moral repair in war veterans. Clinical Psychology Review, 29(8), 695-706
        self_hate: 0.65,

        // Social Cognition - Perceived Competence
        // Combat involves mixed competence signals - mastery experiences offset by trauma-related self-doubt and PTSD-driven perceived inability to function in civilian contexts.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Cieslak et al. (2014) on self-efficacy and trauma
        perceived_competence: -0.15,

        // Mental Health - Depression
        // Combat exposure creates significant depressive symptoms through trauma, loss, and moral injury; most veterans retain elevated baseline depression levels.
        // Seligowski et al. (2016) on psychological distress trajectories; Seal et al. (2009) JAMA study; 22-30% MDD prevalence in combat veterans
        depression: 0.65,

        // Mental Health - Self Worth
        // Combat exposure reduces self-worth through moral injury, loss of agency, and trauma-related shame, with partial recovery dependent on social support.
        // Litz, B.T. et al. (2009). Moral injury and moral repair in war veterans. Clinical Psychology Review
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Combat creates significant hopelessness about the future through trauma, loss, and worldview shifts; substantial chronic component with partial recovery through treatment.
        // Beck, A.T. et al. (1974) Hopelessness Scale; combat PTSD literature showing hopelessness as core symptom in 40-50% of veterans
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Combat creates distrust in relationships and shame-based help-avoidance; chronic vigilance remains but interpersonal hopelessness shows improvement with effective intervention.
        // Van Orden et al. (2010). The Interpersonal Theory of Suicide; Hoge et al. (2004) on barriers to care
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Direct combat exposure creates extreme habituation to pain and death through repeated mortal danger, violence, and death exposure - among the highest AC-generating events.
        // Joiner, T.E. (2005). Why people die by suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        acquired_capability: 0.85,

        // Disposition - Impulse Control
        // Combat causes severe impairment of impulse control through hyperarousal, prefrontal depletion, and emotional dysregulation; significant but partially reversible with recovery.
        // Baumeister, R.F., et al. (1998). Ego Depletion; Aupperle, R.L., et al. (2012). Executive function and PTSD
        impulse_control: -0.65,

        // Disposition - Empathy
        // Combat causes emotional numbing and threat-focused cognition that reduces capacity for perspective-taking and emotional understanding.
        // Decety, J. & Jackson, P.L. (2004). The functional architecture of human empathy; DSM-5 PTSD emotional numbing criteria
        empathy: -0.35,

        // Disposition - Aggression
        // Combat significantly increases state aggression through threat activation, hypervigilance, and hostile attribution bias; effects persist as chronic elevation.
        // Elbogen & Johnson (2009). The intricate link between violence and mental disorder; Taft et al. (2017) on anger and aggression in PTSD
        aggression: 0.55,

        // Disposition - Grievance
        // Combat generates persistent grievance through institutional betrayal, moral injury, and perceived inadequate post-deployment support systems.
        // Litz, B.T., et al. (2009). Moral injury and moral repair in war veterans. Clinical Psychology Review, 29(8), 695-706
        grievance: 0.35,

        // Disposition - Reactance
        // Combat creates significant acute loss of autonomy through life-threatening involuntary constraint, with post-combat autonomy restoration attempts.
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological reactance; Foa & Rothbaum (1998) on hypervigilance and control restoration
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Combat exposure increases hypervigilance and threat perception, creating generalized distrust in others, though in-group bonds provide partial mitigation.
        // Koenen et al. on PTSD-trust literature; Foa & Kozak (1986) on emotional processing of trauma
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
        perceived_competence: false,
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
        valence: 0.35,
        arousal: 0.35,
        dominance: 0.25,
        fatigue: 0.25,
        stress: 0.35,
        purpose: 0.25,
        loneliness: 0.25,
        prc: 0.25,
        perceived_liability: 0.25,
        self_hate: 0.35,
        perceived_competence: 0.12,
        depression: 0.35,
        self_worth: 0.25,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.18,
        impulse_control: 0.25,
        empathy: 0.25,
        aggression: 0.35,
        grievance: 0.20,
        reactance: 0.25,
        trust_propensity: 0.25,
    },
};
