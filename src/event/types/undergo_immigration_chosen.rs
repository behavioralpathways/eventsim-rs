//! Chosen Immigration event specification.
//!
//! Voluntary immigration - moving to a new country by personal choice, including
//! decisions for better opportunities, family reunification, or lifestyle changes.
//! Distinct from forced displacement or refugee situations, chosen immigration
//! involves agency, self-determination, and positive future expectations.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Chosen immigration produces strong initial positive valence from goal achievement
        // and agency, but hedonic adaptation to new circumstances reduces permanence.
        // Lyubomirsky (2005) on hedonic adaptation; voluntary migrants report higher
        // initial life satisfaction than forced migrants.
        valence: 0.55,

        // Mood - Arousal
        // Voluntary immigration generates sustained high activation from anticipation,
        // novelty, and adaptive stress, but hedonic adaptation typically restores
        // baseline within 6-12 months post-settlement.
        // Schlossberg (1981) transition theory; Russell & Barrett (1999) circumplex model.
        arousal: 0.50,

        // Mood - Dominance
        // Chosen immigration reflects personal agency and mastery motivation, creating
        // moderate sense of control; most effects adapt within 1-2 years as new
        // environment becomes familiar.
        // Deci & Ryan (1985) Self-Determination Theory; Bandura (1977) self-efficacy.
        dominance: 0.32,

        // Needs - Fatigue
        // Chosen immigration creates moderate acute fatigue from cognitive/emotional
        // adaptation work and acculturation stress, with near-complete recovery through
        // hedonic adaptation and successful integration.
        // Berry (1997) acculturation stress; Baumeister et al. (1998) ego depletion.
        fatigue: 0.35,

        // Needs - Stress
        // Voluntary immigration creates moderate acute physiological stress from
        // displacement and adjustment demands, but high perceived control and choice
        // significantly attenuate HPA axis activation compared to involuntary stressors.
        // Lazarus & Folkman (1984) Stress, Appraisal, and Coping.
        stress: 0.25,

        // Needs - Purpose
        // Chosen immigration provides substantial purpose boost through autonomy
        // satisfaction and goal clarity inherent in self-selected migration.
        // Ryan & Deci (2017) Self-Determination Theory; Steger et al. (2006) Meaning in Life.
        purpose: 0.38,

        // Social Cognition - Loneliness
        // Chosen immigration creates acute but transient social disruption through
        // severed ties and adjustment challenges, but voluntary nature and expected
        // opportunities moderate severity; most adapt within 1-2 years.
        // Berry (2005) acculturation; Poyrazli & Lopez (2007) international students.
        loneliness: 0.25,

        // Social Cognition - PRC
        // Chosen immigration triggers acute loss of established reciprocal caring
        // relationships through separation, despite decision agency; initial deficit
        // recovers through network rebuilding over 1-3 years.
        // Berry (1997); Yakushko et al. (2008) migration-related grief.
        prc: -0.28,

        // Social Cognition - Perceived Liability
        // Chosen immigration involves personal agency and opportunity-seeking, which
        // typically reduces burden perception; initial adjustment challenges are
        // temporary and overcome through achievement.
        // Van Orden et al. (2010) ITS; Portes & Rumbaut (2006) immigrant mobility.
        perceived_liability: -0.08,

        // Social Cognition - Self Hate
        // Chosen immigration involves agentic decision-making which provides moderate
        // self-validation, though temporary acculturation stress may create mild
        // self-doubt that resolves within months as adaptation succeeds.
        // Joiner (2005) ITS framework; immigration psychology literature.
        self_hate: -0.08,

        // Social Cognition - Perceived Competence
        // Voluntary immigration demonstrates successful agency and mastery of complex
        // planning, validating adaptive competence, but initial environmental adjustment
        // creates temporary domain-specific challenges that moderate permanence.
        // Bandura (1977) self-efficacy; Deci & Ryan (2000) autonomy as competence reinforcer.
        perceived_competence: 0.25,

        // Mental Health - Depression
        // Voluntary immigration creates initial stress and social loss but the
        // self-determined nature and potential for life improvement provide protective
        // factors resulting in mild overall mood improvement over 2-3 years.
        // Bhugra (2004) Migration and mental health; Berry (1997) acculturation.
        depression: -0.12,

        // Mental Health - Self Worth
        // Voluntary immigration signals personal agency and intentional life choice,
        // providing meaningful but moderate self-worth boost through demonstrated
        // self-efficacy and goal pursuit.
        // Schwartz & Unger (2010) immigration adjustment; Lucas & Donnellan (2007).
        self_worth: 0.25,

        // Mental Health - Hopelessness
        // Voluntary immigration involves active choice driven by positive future
        // expectations, reducing hopelessness; however, effect is largely temporary
        // as adaptation challenges emerge and hedonic adaptation occurs.
        // Schwartz et al. (2010) acculturation; Bhugra & Becker (2005) migration mental health.
        hopelessness: -0.35,

        // Mental Health - Interpersonal Hopelessness
        // Chosen immigration demonstrates agency and typically increases help-seeking
        // behavior and social network formation, reducing interpersonal hopelessness
        // despite temporary disruption of existing networks.
        // Joiner (2005); Berry & Cha (2010) migration voluntariness.
        interpersonal_hopelessness: -0.15,

        // Mental Health - Acquired Capability
        // Voluntary immigration does not inherently expose individuals to physical pain,
        // injury, violence, or direct proximity to death - the key mechanisms for
        // habituation that develop acquired capability.
        // Joiner (2005); Van Orden et al. (2010) ITS.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Chosen immigration creates acute stress and cognitive depletion during
        // transition, temporarily impairing impulse control, but retained agency
        // and achievable goals prevent severe impairment.
        // Berry (1997) acculturation; Baumeister & Muraven (2000) ego depletion.
        impulse_control: -0.15,

        // Disposition - Empathy
        // Voluntary immigration increases perspective-taking through cross-cultural
        // exposure, though temporary adaptation stress slightly offsets this benefit;
        // effect becomes positive after initial transition.
        // Decety & Jackson (2004) cultural perspective-taking; intercultural competence.
        empathy: 0.12,

        // Disposition - Aggression
        // Voluntary immigration creates temporary acculturation stress and goal
        // frustration that mildly elevates aggression through the early adaptation
        // phase, but positive motivation typically results in substantial recovery.
        // Berkowitz (1989) Frustration-Aggression; Anderson & Bushman (2002).
        aggression: 0.15,

        // Disposition - Grievance
        // Chosen immigration reduces grievance through agency and positive expectations,
        // though unforeseen systemic barriers may partially offset this benefit;
        // most adaptation occurs within 1-2 years.
        // IOM migration psychology literature; voluntary migration outcomes research.
        grievance: -0.15,

        // Disposition - Reactance
        // Voluntary immigration represents autonomous choice and freedom exercise,
        // reducing reactance as the individual gains control over their life trajectory.
        // Brehm (1966) psychological reactance theory.
        reactance: -0.15,

        // Disposition - Trust Propensity
        // Chosen immigration involves leaving established trust networks causing initial
        // caution, but the voluntary nature and agency involved support mild trust
        // restoration through active relationship building.
        // Putnam (2007); Portes & Rumbaut (2006) Immigrant America.
        trust_propensity: 0.08,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: false,
        loneliness: false,
        prc: false,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: false,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.08,
        dominance: 0.08,
        fatigue: 0.05,
        stress: 0.04,
        purpose: 0.10,
        loneliness: 0.05,
        prc: 0.08,
        perceived_liability: 0.04,
        self_hate: 0.04,
        perceived_competence: 0.15,
        depression: 0.04,
        self_worth: 0.08,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.05,
        empathy: 0.08,
        aggression: 0.06,
        grievance: 0.04,
        reactance: 0.04,
        trust_propensity: 0.05,
    },
};
