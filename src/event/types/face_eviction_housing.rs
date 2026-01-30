//! Face Eviction (Housing) event specification.
//!
//! Being served with an eviction notice or facing imminent loss of housing.
//! This is a severe life stressor threatening fundamental shelter security,
//! combining financial crisis with loss of control and social stigma.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Housing eviction represents a severe threat combining loss of fundamental security,
        // institutional powerlessness, and identity threat, producing strong negative valence.
        // Holmes & Rahe Social Readjustment Rating Scale; McEwen (1998) allostatic load framework
        valence: -0.68,

        // Mood - Arousal
        // Eviction triggers sustained high arousal through threat to basic needs and
        // anticipatory anxiety, with moderate permanence reflecting hedonic adaptation.
        // Matthews & Gallo (2011) on housing instability and physiological stress
        arousal: 0.55,

        // Mood - Dominance
        // Facing eviction represents a significant externally-imposed loss of control over
        // fundamental housing stability, with limited personal agency in legal/institutional process.
        // Bandura (1977) Self-efficacy theory; forced relocation effects literature
        dominance: -0.55,

        // Needs - Fatigue
        // Creates sustained cognitive and emotional load through crisis management,
        // decision-making uncertainty, sleep disruption, and rumination about housing security.
        // Hockey (2013) The psychology of fatigue; cortisol elevation research
        fatigue: 0.65,

        // Needs - Stress
        // Eviction creates severe physiological stress through threat activation, loss of control,
        // and social/financial threat, showing chronicity during proceedings.
        // Kushel et al. (2006) Housing instability; Cutts et al. (2011) Housing insecurity
        stress: 0.62,

        // Needs - Purpose
        // Housing eviction severely disrupts goal-pursuit capacity and identity-anchoring,
        // creating acute existential crisis with modest long-term impact through meaning reconstruction.
        // Frankl (1959) Man's Search for Meaning; housing insecurity literature
        purpose: -0.55,

        // Social Cognition - Loneliness
        // Housing eviction disrupts social networks and community connections while creating
        // stigma-driven withdrawal, representing significant loneliness.
        // Cacioppo & Patrick (2008); Van Orden et al. (2010) Interpersonal Theory of Suicide
        loneliness: 0.35,

        // Social Cognition - PRC (Perceived Reciprocal Care)
        // Facing eviction strongly signals social exclusion and abandonment by institutional systems,
        // moderately reducing belief that others care.
        // Desmond (2016) Evicted; Van Orden et al. (2010) Interpersonal Theory of Suicide
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Housing eviction creates acute crisis-level perceived liability through forced dependency
        // on others for shelter, housing-specific shame, and identity threat.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2012) Interpersonal Needs Questionnaire
        perceived_liability: 0.58,

        // Social Cognition - Self Hate
        // Eviction triggers moderate-to-significant shame-based self-blame through failure
        // attribution and social devaluation, with substantial recovery upon housing stabilization.
        // Joiner (2005); Van Orden et al. (2010); Kirst et al. (2015) housing insecurity
        self_hate: 0.48,

        // Social Cognition - Perceived Competence
        // Eviction represents acute loss of control over fundamental needs and personal financial
        // mastery, creating significant competence crisis with moderate permanent shift.
        // Bandura (1997) Self-efficacy; Cutts et al. (2011) housing insecurity research
        perceived_competence: -0.45,

        // Mental Health - Depression
        // Eviction is a significant loss event with ongoing threat, triggering depressive symptoms
        // through loss, helplessness, and rumination, though most individuals show recovery.
        // Brown & Harris (1978) Social origins of depression; Kendler et al. (1999)
        depression: 0.35,

        // Mental Health - Self Worth
        // Eviction represents a significant self-worth threat due to shame, autonomy loss,
        // and identity-threatening failure in adult responsibilities.
        // Desmond (2016) Evicted: Poverty and Profit in the American City
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Eviction creates severe hopelessness through loss of control, sustained financial burden,
        // housing instability, and trauma that produces persistent future pessimism.
        // Joiner (2005); Abramson et al. (1989) Hopelessness depression
        hopelessness: 0.58,

        // Mental Health - Interpersonal Hopelessness
        // Eviction creates acute shame and perceived burden that impairs help-seeking beliefs,
        // but remains a temporary housing crisis with typical recovery.
        // Rickwood et al. (2005); Joiner (2005) burden perception mechanisms
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Housing eviction creates psychological distress (TB/PB mechanisms) but does not
        // expose individuals to physical pain or death, therefore produces no AC habituation.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Housing eviction triggers sustained high stress that depletes self-regulatory resources
        // and impairs impulse control for the duration of housing instability.
        // Baumeister et al. (1998) Ego Depletion; housing instability literature
        impulse_control: -0.45,

        // Disposition - Empathy
        // Housing eviction threat creates acute cognitive and emotional load that temporarily
        // reduces capacity for perspective-taking and emotional understanding of others.
        // Singer & Klimecki (2014) Empathy and compassion as shared neural resources
        empathy: -0.18,

        // Disposition - Aggression
        // Housing eviction triggers significant frustration and perceived injustice (goal blockage,
        // identity threat), but effects are primarily acute and resolve once housing is secured.
        // Berkowitz (1989) Frustration-aggression hypothesis; Anderson & Bushman (2002)
        aggression: 0.35,

        // Disposition - Grievance
        // Housing eviction creates intense grievance through systemic injustice attribution,
        // loss of fundamental security, and perceived mistreatment by legal/institutional systems.
        // Mikula (1993) On the experience of injustice; Miller (2001); Lind & Tyler (1988)
        grievance: 0.65,

        // Disposition - Reactance
        // Housing eviction represents a severe, externally-imposed constraint on autonomy and
        // freedom of living space, triggering strong reactance with lasting psychological effects.
        // Brehm & Brehm (1981) Psychological reactance: A theory of freedom and control
        reactance: 0.65,

        // Disposition - Trust Propensity
        // Housing eviction damages both institutional and interpersonal trust through loss of
        // housing security and systems failure, but lacks direct interpersonal betrayal.
        // Joiner (2005); McEwen (1998); Rotter (1967) interpersonal trust generalization
        trust_propensity: -0.38,
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
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: false,
        aggression: false,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.22,
        arousal: 0.12,
        dominance: 0.06,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.12,
        perceived_liability: 0.16,
        self_hate: 0.14,
        perceived_competence: 0.10,
        depression: 0.08,
        self_worth: 0.08,
        hopelessness: 0.22,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.08,
        empathy: 0.06,
        aggression: 0.08,
        grievance: 0.28,
        reactance: 0.18,
        trust_propensity: 0.12,
    },
};
