//! ExperiencePandemicGlobal event specification.
//!
//! A global pandemic event - a widespread infectious disease outbreak affecting populations
//! across multiple countries or continents, creating sustained health threats, economic
//! disruption, social isolation through lockdowns, and pervasive uncertainty about the future.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Global pandemics produce severe, sustained negative affect through health threats, isolation, and economic disruption, with substantial hedonic recovery as threat recedes and adaptation occurs.
        // WHO mental health impact assessments; Fredrickson (2001); Lucas (2007) on hedonic adaptation
        valence: -0.65,

        // Mood - Arousal
        // Global pandemics create sustained elevated arousal through perceived health threat and uncertainty, but hedonic adaptation and post-pandemic normalization result in modest permanent baseline shift.
        // Posner, Russell & Peterson (2005) circumplex model; COVID-19 longitudinal mental health studies
        arousal: 0.55,

        // Mood - Dominance
        // Pandemics impose severe loss of control over personal freedoms and health outcomes, but individuals retain some agency in response choices, producing moderate-to-significant dominance reduction with substantial recovery over 12-24 months.
        // Bandura (1977) Self-efficacy; COVID-19 mental health literature on perceived powerlessness
        dominance: -0.55,

        // Needs - Fatigue
        // Global pandemic causes prolonged emotional processing, uncertainty-driven rumination, and collective grief, creating moderate-to-severe mental and physical fatigue with partial permanence due to post-pandemic baseline shifts in stress reactivity.
        // Hockey (2013) Psychology of fatigue; COVID-19 fatigue studies 2021-2022
        fatigue: 0.62,

        // Needs - Stress
        // Global pandemics trigger sustained HPA axis activation through multiple threat pathways (health, economic, social, unpredictability), placing stress in the high range; hedonic adaptation and crisis habituation prevent most permanent baseline shift.
        // McEwen (1998) Stress, adaptation, and disease: Allostasis and allostatic load
        stress: 0.68,

        // Needs - Purpose
        // Global pandemics temporarily disrupt life goals, work roles, and contribution channels, creating moderate existential questioning and purpose questioning, though most individuals recover within 1-2 years through hedonic adaptation and meaning-making.
        // Steger et al. (2006) Meaning in Life Questionnaire framework
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Global pandemic enforced widespread social isolation and disrupted daily interpersonal interaction through lockdowns, causing significant but temporary increase in loneliness with substantial recovery post-restrictions.
        // Cacioppo & Patrick (2008) Loneliness: Human nature and the need for social connection
        loneliness: 0.35,

        // Social Cognition - PRC
        // Global pandemics create enforced physical isolation that reduces capacity for reciprocal caring behaviors, creating perception of reduced social support despite increased verbal expressions of concern.
        // Baumeister & Leary (1995); Van Orden et al. (2010) Interpersonal Theory of Suicide
        prc: -0.25,

        // Social Cognition - Perceived Liability
        // Global pandemics create widespread economic dependency and health burden that increases perceived liability, but collective experience and mutual aid reduce individual burden perception compared to isolated hardships.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide
        perceived_liability: 0.25,

        // Social Cognition - Self Hate
        // Global pandemics are external events that trigger mild self-blame about response adequacy but not self-loathing from personal failure; most individuals recover within 6-12 months as the collective threat normalizes.
        // Gilbert & Irons (2005) on distinction between personal failure and collective trauma
        self_hate: 0.15,

        // Social Cognition - Perceived Competence
        // Global pandemics significantly reduce perceived competence through loss of control and helplessness, but most psychological recovery occurs within 1-2 years as people re-establish agency and mastery.
        // Bandura (1997) Self-efficacy: The exercise of control
        perceived_competence: -0.28,

        // Mental Health - Depression
        // Global pandemics cause significant depressive symptoms through isolation, economic uncertainty, and loss of routine, but effects show substantial recovery with hedonic adaptation post-pandemic.
        // COVID-19 depression prevalence studies; Brown & Harris (1978) social origins of depression
        depression: 0.35,

        // Mental Health - Self Worth
        // Global pandemics reduce social validation and amplify helplessness without direct shame, creating mild but temporary self-worth reduction with near-complete recovery within 1-2 years post-event.
        // Baumeister et al. (2003) self-esteem maintenance; Crocker & Wolfe (2001) contingencies of self-worth
        self_worth: -0.15,

        // Mental Health - Hopelessness
        // Global pandemics create significant temporary hopelessness through uncertainty and lost control, but are time-limited with eventual resolution, allowing substantial recovery.
        // Ettman et al. (2020) JAMA Network Open; McGinty et al. (2020) JAMA
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Global pandemics simultaneously isolate individuals and overwhelm support systems, creating moderate increases in beliefs that social support won't be available when needed, with partial recovery as systems stabilize.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010)
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Global pandemic exposure creates minimal direct pain/death habituation for most populations; mild increase in healthcare workers and bereaved through witnessing death, but far below violence/combat thresholds.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010)
        acquired_capability: 0.05,

        // Disposition - Impulse Control
        // Global pandemics cause significant acute impairment to impulse control through chronic stress, sleep disruption, and ego depletion, but most individuals show substantial recovery as the threat resolves and routines normalize.
        // Baumeister & Vohs (2007) Self-Regulation and Self-Control; Xie et al. (2020)
        impulse_control: -0.35,

        // Disposition - Empathy
        // Global pandemics create sustained stress and self-focus that temporarily reduce other-directed empathy through cognitive resource depletion, with most individuals showing substantial recovery within 1-2 years post-event.
        // Singer & Klimecki (2014) on stress-induced empathy reduction
        empathy: -0.15,

        // Disposition - Aggression
        // Global pandemics trigger sustained frustration, stress activation, and autonomy loss creating significant state aggression increase, with low permanence due to hedonic adaptation once pandemic restrictions end.
        // Anderson & Bushman (2002) Human aggression; Berkowitz (1989) Frustration-Aggression Hypothesis
        aggression: 0.35,

        // Disposition - Grievance
        // Global pandemics create significant perceived injustice through unequal impact, unfair policy responses, and sense of victimization by institutions, but effects show substantial recovery as threat recedes and adaptation occurs.
        // Lind & Tyler (1988) procedural justice; Miller (2001) on disrespect and injustice
        grievance: 0.32,

        // Disposition - Reactance
        // Global pandemics impose involuntary constraints on autonomy (moderate-to-strong reactance trigger), but effects are temporary due to restriction lift-off and hedonic adaptation with significant population-level heterogeneity in response.
        // Brehm (1966) A Theory of Psychological Reactance; Brehm & Brehm (1981)
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Global pandemic creates moderate interpersonal distrust through uncertainty and isolation, but most recovery occurs within 1-2 years as normalcy returns.
        // Zmerli & Hooghe (2011); COVID-era trust studies showing 15-25% temporary declines
        trust_propensity: -0.25,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: true,
        loneliness: false,
        prc: true,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: true,
        self_worth: false,
        hopelessness: false,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.12,
        dominance: 0.12,
        fatigue: 0.18,
        stress: 0.12,
        purpose: 0.08,
        loneliness: 0.08,
        prc: 0.12,
        perceived_liability: 0.08,
        self_hate: 0.04,
        perceived_competence: 0.06,
        depression: 0.12,
        self_worth: 0.04,
        hopelessness: 0.07,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.08,
        empathy: 0.08,
        aggression: 0.10,
        grievance: 0.12,
        reactance: 0.08,
        trust_propensity: 0.08,
    },
};
