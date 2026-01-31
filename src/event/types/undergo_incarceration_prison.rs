//! UndergoIncarcerationPrison event specification.
//!
//! Prison incarceration - confinement in a correctional facility following
//! criminal conviction. Involves loss of freedom, autonomy, and social
//! connection with exposure to institutional control and violence.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Prison incarceration creates severe, sustained negative valence through total autonomy deprivation, chronic isolation, and institutional dehumanization.
        // Sykes (1958) The Society of Captives; Joiner (2005) Why People Die by Suicide; Goffman (1963) Stigma
        valence: -0.68,

        // Mood - Arousal
        // Prison maintains sustained high arousal through institutionalized threat, hypervigilance, and loss of safety that persists partially after release.
        // McEwen (1998) Stress, adaptation, and disease; Pugh (2017) on hypervigilance in correctional institutions
        arousal: 0.65,

        // Mood - Dominance
        // Prison incarceration creates complete loss of autonomy through institutional control systems, producing severe dominance reduction comparable to assault.
        // Bandura (1977) Self-Efficacy; Seligman & Maier (1967) learned helplessness paradigm
        dominance: -0.80,

        // Needs - Fatigue
        // Prison creates severe fatigue through sustained threat response, sleep disruption, ego depletion from constant self-regulation, and loss of meaningful purpose.
        // Hockey (2013) The psychology of fatigue; McEwen (1998) Stress, adaptation, and disease
        fatigue: 0.76,

        // Needs - Stress
        // Prison represents extreme physiological stress activation from threat exposure, autonomy loss, and social isolation approaching life-threat level.
        // Van der Kolk (2014) The Body Keeps the Score; Holmes & Rahe (1967) Social Readjustment Rating Scale
        stress: 0.82,

        // Needs - Purpose
        // Prison causes substantial disruption to life goals, autonomy, and meaning-making with lasting effects due to reintegration barriers and stigma.
        // Frankl (1959) Man's Search for Meaning
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Incarceration severs primary social networks and enforces prolonged physical isolation with limited meaningful connection opportunities.
        // Van Orden et al. (2010) The Interpersonal Theory of Suicide; Williams (2007) Ostracism research
        loneliness: 0.68,

        // Social Cognition - PRC
        // Prison represents forced institutional separation from all primary relationships combined with systematic dehumanization creating severe perceived caring deficit.
        // Van Orden et al. (2010) ITS; Sykes (1958) The Society of Captives
        prc: -0.65,

        // Social Cognition - Perceived Liability
        // Prison creates sustained severe burden perception through financial dependency, inability to contribute economically or socially, and pervasive institutional stigma.
        // Joiner (2005) Why People Die by Suicide; Lopoo & Western (2005) Incarceration and marital unions
        perceived_liability: 0.58,

        // Social Cognition - Self Hate
        // Prison combines institutional dehumanization, chronic isolation, and culpability-based self-blame creating sustained self-directed negative affect.
        // Maruna (2001) Making good; Braithwaite (2000) Crime, shame and reintegration
        self_hate: 0.68,

        // Social Cognition - Perceived Competence
        // Incarceration creates profound loss of control and learned helplessness with significant initial competence erosion.
        // Bandura (1997) Self-efficacy: The exercise of control
        perceived_competence: -0.45,

        // Mental Health - Depression
        // Prison represents severe loss involving confinement, autonomy loss, social isolation, and hopelessness strongly associated with major depressive symptoms.
        // Teplin et al. (2000); Binswanger et al. (2007) on depression rates in correctional facilities
        depression: 0.65,

        // Mental Health - Self Worth
        // Prison involves systematic dehumanization and societal stigmatization that severely damage self-worth during and after incarceration.
        // Maruna (2001) Making good; Schmid & Jones (1991) Suspended identity
        self_worth: -0.65,

        // Mental Health - Hopelessness
        // Prison removes autonomy and future control creating a high-impact trap experience with chronic effects during incarceration.
        // Abramson et al. (1989) Hopelessness depression
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Prison creates severe interpersonal hopelessness through forced isolation, institutional barriers to support, and stigmatization.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) ITS
        interpersonal_hopelessness: 0.65,

        // Mental Health - Acquired Capability
        // Prison exposure to chronic interpersonal violence, death threats, and threat of severe harm creates moderate habituation to pain and fear of death.
        // Van Orden et al. (2010) ITS; Joiner (2005) Why People Die by Suicide
        acquired_capability: 0.42,

        // Disposition - Impulse Control
        // Incarceration acutely depletes self-regulatory resources through chronic stress and autonomy deprivation.
        // Baumeister & Vohs (2001) on ego depletion; Steiner (2009) on incarceration effects
        impulse_control: -0.35,

        // Disposition - Empathy
        // Incarceration creates chronic stress-induced self-focus and dehumanizing institutional conditions that suppress empathic capacity.
        // Decety & Jackson (2004) The functional architecture of human empathy
        empathy: -0.25,

        // Disposition - Aggression
        // Prison creates severe sustained aggression through combined loss of autonomy, perceived injustice, violence exposure, and identity threat.
        // Berkowitz (1989) Frustration-aggression hypothesis; Clemmer (1940); Sykes (1958) prisonization effects
        aggression: 0.58,

        // Disposition - Grievance
        // Prison produces severe chronic perceived injustice due to deprivation of liberty and procedural violations.
        // Lind & Tyler (1988) The social psychology of procedural justice
        grievance: 0.75,

        // Disposition - Reactance
        // Prison involves total autonomy deprivation triggering strong sustained reactance.
        // Brehm & Brehm (1981) Psychological reactance: A theory of freedom and control
        reactance: 0.75,

        // Disposition - Trust Propensity
        // Prison severely damages trust propensity through prolonged exposure to institutional betrayal, violence threat, and adversarial social environment.
        // Maruna et al. (2005) on post-incarceration desistance; Giordano (2010) on identity change
        trust_propensity: -0.65,
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
        valence: 0.30,
        arousal: 0.30,
        dominance: 0.22,
        fatigue: 0.32,
        stress: 0.38,
        purpose: 0.25,
        loneliness: 0.28,
        prc: 0.22,
        perceived_liability: 0.28,
        self_hate: 0.22,
        perceived_competence: 0.18,
        depression: 0.35,
        self_worth: 0.25,
        hopelessness: 0.25,
        interpersonal_hopelessness: 0.25,
        impulse_control: 0.15,
        empathy: 0.18,
        aggression: 0.28,
        grievance: 0.45,
        reactance: 0.25,
        trust_propensity: 0.35,
    },
};
