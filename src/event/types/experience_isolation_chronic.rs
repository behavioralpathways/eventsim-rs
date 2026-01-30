//! ExperienceIsolationChronic event specification.
//!
//! Chronic, prolonged social isolation - an extended period of minimal social contact,
//! disconnection from others, and lack of meaningful interpersonal relationships.
//! This is distinct from acute loneliness or temporary solitude; it represents weeks,
//! months, or years of pervasive isolation.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Chronic isolation severely reduces hedonic tone through sustained social disconnection, creating pervasive anhedonia with partial permanence due to disrupted affect regulation systems.
        // Holt-Lunstad et al. (2010) PLOS Medicine; Cacioppo & Patrick (2008) Loneliness: Human nature and the need for social connection
        valence: -0.62,

        // Mood - Arousal
        // Chronic isolation reduces arousal through depression and emotional deactivation, though some individuals maintain elevated vigilance; moderate permanence reflects lasting baseline shift despite partial hedonic recovery over years.
        // Cacioppo & Hawkley (2009) Handbook of Individual Differences in Social Behavior; Holt-Lunstad et al. (2015)
        arousal: -0.35,

        // Mood - Dominance
        // Chronic isolation produces moderate-to-significant loss of control through reduced agency, learned helplessness, and inability to influence one's social environment.
        // Deci & Ryan (1985) Self-Determination Theory; Seligman (1975) Learned Helplessness; Baumeister & Leary (1995)
        dominance: -0.55,

        // Needs - Fatigue
        // Chronic isolation creates sustained psychological depletion through sleep disruption, emotional rumination, reduced motivation, and chronic stress activation.
        // McEwen (1998) Stress, adaptation, and disease: Allostasis and allostatic load; Cacioppo & Patrick (2008)
        fatigue: 0.65,

        // Needs - Stress
        // Chronic isolation produces sustained HPA axis activation and elevated cortisol, triggering moderate-to-high physiological stress with allostatic load accumulation.
        // McEwen (1998) Allostatic load framework; Cacioppo & Hawkley (2009) on loneliness and cardiovascular stress reactivity
        stress: 0.65,

        // Needs - Purpose
        // Chronic isolation significantly disrupts purpose by removing social roles, relational feedback, and contribution opportunities that sustain existential meaning.
        // Frankl (1959) Man's Search for Meaning; Ryff (1989) Happiness is Everything, or is it? Purpose in Life
        purpose: -0.45,

        // Social Cognition - Loneliness
        // Extended social disconnection over weeks/months produces severe, sustained loneliness approaching maximum, with moderate permanence due to habit formation and identity shifts.
        // Cacioppo & Patrick (2008) Loneliness: Human nature and the need for social connection
        loneliness: 0.85,

        // Social Cognition - PRC
        // Chronic isolation represents an extended behavioral signal of caring deficit; the prolonged absence of contact and demonstrated care substantially reduces perceived reciprocal caring.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) The Interpersonal Theory of Suicide
        prc: -0.55,

        // Social Cognition - Perceived Liability
        // Chronic isolation increases perceived liability through reduced social validation and increased perception of being a burden if reintegration is attempted.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) The Interpersonal Theory of Suicide
        perceived_liability: 0.25,

        // Social Cognition - Self Hate
        // Prolonged social isolation with no opportunity for validation or reality-testing creates pervasive self-blame and internalized shame from unbroken rumination cycles.
        // Cacioppo & Patrick (2008); Eisenberger & Lieberman (2004) Why rejection hurts
        self_hate: 0.48,

        // Social Cognition - Perceived Competence
        // Chronic isolation removes mastery opportunities and social feedback mechanisms that sustain perceived competence, causing significant self-doubt.
        // Bandura (1997) Self-efficacy: The exercise of control
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Chronic isolation has a strong depressogenic effect through thwarted belongingness, rumination cycles, and anhedonia.
        // Brown & Harris (1978) Social Origins of Depression; Cacioppo & Patrick (2008)
        depression: 0.65,

        // Mental Health - Self Worth
        // Chronic isolation removes validation mechanisms fundamental to self-worth maintenance, causing significant cumulative psychological harm.
        // Cooley (1902) Looking-glass self; Rosenberg (1965) Society and the adolescent self-image
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Chronic isolation removes hope for future connection through sustained absence of social reciprocation, creating generalized negative expectations about future relationships.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) The Interpersonal Theory of Suicide
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Chronic prolonged isolation with no active relationships removes all experiential evidence that social support is available, creating strong interpersonal hopelessness.
        // Cacioppo & Patrick (2008); Van Orden et al. (2010) on thwarted belongingness preventing help-seeking
        interpersonal_hopelessness: 0.65,

        // Mental Health - Acquired Capability
        // Chronic isolation increases thwarted belongingness and perceived burdensomeness but does not expose individuals to physical pain, death, or violence necessary for acquired capability habituation.
        // Joiner (2005) Why People Die by Suicide - ITS framework distinguishes desire (TB+PB) from capability (AC)
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Chronic isolation sustains stress-induced executive function impairment and depletes self-regulatory resources, moderately impairing impulse control.
        // Baumeister & Vohs (2007) Self-regulation and self-control; chronic stress and cortisol dysregulation research
        impulse_control: -0.45,

        // Disposition - Empathy
        // Chronic isolation reduces empathic capacity through sustained atrophy of perspective-taking abilities and increased self-focus.
        // Singer & Klimecki (2014) Empathy and compassion; Decety & Jackson (2004) The functional architecture of human empathy
        empathy: -0.45,

        // Disposition - Aggression
        // Chronic isolation increases aggression through frustration and unmet belonging needs, but effects show substantial recovery once isolation ends.
        // Berkowitz (1989) Frustration-aggression hypothesis; Williams (2007) Ostracism: The power of silence
        aggression: 0.28,

        // Disposition - Grievance
        // Chronic isolation amplifies rumination and negative attribution biases, fostering persistent victimization narratives and deep grievance.
        // Williams (2007) Ostracism: The power of silence; Cacioppo & Patrick (2008)
        grievance: 0.55,

        // Disposition - Reactance
        // Chronic isolation reduces access to autonomy expression and social freedom, creating mild reactance to environmental constraints.
        // Brehm & Brehm (1981) Psychological Reactance: A Theory of Freedom and Control
        reactance: 0.15,

        // Disposition - Trust Propensity
        // Chronic isolation erodes trust propensity through accumulated loss of positive social experiences and increased interpersonal wariness.
        // Rotter (1967) Interpersonal trust framework; Mayer, Davis & Schoorman (1995) Integrative model of organizational trust
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
        valence: 0.18,
        arousal: 0.25,
        dominance: 0.12,
        fatigue: 0.18,
        stress: 0.22,
        purpose: 0.18,
        loneliness: 0.18,
        prc: 0.25,
        perceived_liability: 0.12,
        self_hate: 0.20,
        perceived_competence: 0.12,
        depression: 0.25,
        self_worth: 0.12,
        hopelessness: 0.22,
        interpersonal_hopelessness: 0.18,
        impulse_control: 0.08,
        empathy: 0.25,
        aggression: 0.12,
        grievance: 0.35,
        reactance: 0.08,
        trust_propensity: 0.18,
    },
};
