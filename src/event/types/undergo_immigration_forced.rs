//! Forced Immigration event specification.
//!
//! Forced immigration/displacement - refugees, asylum seekers, deportees, those
//! fleeing persecution, war, or disaster. Distinct from chosen immigration in that
//! it involves loss of agency, traumatic separation, and often exposure to violence
//! or life-threatening circumstances. Creates profound psychological impacts across
//! all dimensions due to involuntary nature, loss of home/identity, and ongoing
//! uncertainty.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Forced displacement combines loss of home, community, security, and identity
        // with acute trauma exposure, creating severe sustained negative valence comparable
        // to major loss events.
        // Betancourt et al. (2015); Schweitzer et al. (2006) on refugee mental health
        valence: -0.85,

        // Mood - Arousal
        // Forced displacement activates sustained fight-or-flight response through severe
        // environmental threat and ongoing uncertainty; chronic hypervigilance and PTSD
        // develop in majority of cases.
        // Posner, Russell, & Peterson (2005); displaced populations show elevated cortisol
        arousal: 0.65,

        // Mood - Dominance
        // Forced displacement strips agency and control through involuntary relocation,
        // creating acute powerlessness similar to incarceration but with potential for
        // gradual recovery through resettlement.
        // Bandura (1977) self-efficacy; loss of control in forced displacement contexts
        dominance: -0.65,

        // Needs - Fatigue
        // Forced displacement combines acute survival fatigue with chronic trauma-related
        // exhaustion, loss of agency, and ongoing uncertainty, creating both immediate
        // severe depletion and persistent baseline elevation.
        // Berry (1997) acculturation; Bhugra (2004) migration and mental health
        fatigue: 0.72,

        // Needs - Stress
        // Forced displacement involves multiple simultaneous life threats (physical,
        // financial, social), severe unpredictability, and loss of control, activating
        // maximal HPA stress response.
        // McEwen (1998) allostasis and allostatic load; Mollica et al. (2004)
        stress: 0.85,

        // Needs - Purpose
        // Forced displacement severs identity anchors and autonomy, creating sustained
        // meaning disruption that persists beyond acute adjustment despite gradual
        // hedonic adaptation and identity reconstruction.
        // Frankl (1959) Man's Search for Meaning; Deci & Ryan (2017) SDT
        purpose: -0.42,

        // Social Cognition - Loneliness
        // Forced displacement severs existing social networks and primary relationships
        // while placing individuals in unfamiliar environments with language/cultural
        // barriers, creating severe isolation.
        // Van Orden et al. (2010) ITS; Cacioppo & Patrick (2008)
        loneliness: 0.75,

        // Social Cognition - PRC
        // Forced displacement actively signals rejection and abandonment through lack
        // of agency, involuntary separation, discrimination, and trauma, creating severe
        // perceived caring deficit.
        // Van Orden et al. (2010); Joiner (2005); Berry (1997) acculturation stress
        prc: -0.52,

        // Social Cognition - Perceived Liability
        // Forced displacement creates multiple burdensomeness factors - dependency on
        // aid systems, loss of economic contribution, family disruption, and survivor
        // guilt - resulting in significant perceived liability.
        // Wilkinson et al. (2009) suicide and forced migration; Bhugra (2005)
        perceived_liability: 0.55,

        // Social Cognition - Self Hate
        // Forced displacement triggers moderate self-directed negativity through
        // internalized stigma, survivor guilt, and perceived liability to receiving
        // communities.
        // Schweitzer et al. (2007) refugee trauma and shame; Van Orden et al. (2010) ITS
        self_hate: 0.28,

        // Social Cognition - Perceived Competence
        // Forced displacement removes agency and contextual competence but survivors
        // typically rebuild through adaptation, creating moderate lasting change through
        // new capability frameworks.
        // Seligman & Maier (1967) learned helplessness; Bandura (1997) self-efficacy
        perceived_competence: -0.38,

        // Mental Health - Depression
        // Forced displacement combines severe loss, trauma exposure, and social
        // disconnection, producing substantial depressive symptoms; hedonic adaptation
        // is limited by ongoing stress and identity disruption.
        // Brown & Harris (1978); Fazel et al. (2005) refugee mental health
        depression: 0.65,

        // Mental Health - Self Worth
        // Forced displacement combines multiple identity-threatening elements including
        // loss of homeland, social status, autonomy, and belonging that severely damage
        // self-worth through enforced helplessness.
        // Schweitzer et al. (2005) trauma and adjustment in refugees
        self_worth: -0.65,

        // Mental Health - Hopelessness
        // Forced displacement creates severe hopelessness through loss of control,
        // social dislocation, uncertain future legal status, and repeated exposure
        // to threats.
        // Mollica et al. (2004) Harvard Trauma Questionnaire; O'Connor & Kirtley (2018)
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Forced immigration creates severe social disruption - loss of established
        // networks, language barriers, and discrimination severely damage beliefs
        // that relationships can provide meaningful help.
        // Joiner (2005); Van Orden et al. (2010) ITS
        interpersonal_hopelessness: 0.65,

        // Mental Health - Acquired Capability
        // Forced displacement typically involves exposure to violence, death, or
        // dangerous conditions that habituate individuals to pain/fear, though rarely
        // reaches extreme levels unless involving active combat or torture.
        // Joiner (2005); Van Orden et al. (2010) ITS
        acquired_capability: 0.35,

        // Disposition - Impulse Control
        // Forced displacement causes severe acute stress and ego depletion, significantly
        // impairing impulse control through sleep deprivation, hypervigilance, and loss
        // of regulatory resources.
        // Baumeister et al. (1998) ego depletion; refugee mental health literature
        impulse_control: -0.45,

        // Disposition - Empathy
        // Forced displacement causes significant acute stress and self-focus that
        // temporarily reduces empathy capacity; chronic nature sustains this reduction
        // due to emotional resource depletion.
        // Singer & Klimecki (2014) empathy resilience under chronic stress
        empathy: -0.25,

        // Disposition - Aggression
        // Forced displacement creates severe frustration, perceived injustice, and
        // threat that elevates aggression through blocked goals and hyperarousal.
        // Berkowitz (1989) frustration-aggression; Anderson & Bushman (2002)
        aggression: 0.35,

        // Disposition - Grievance
        // Forced displacement is a severe, externally-attributed injustice involving
        // loss of home and agency; the permanent loss creates lasting grievance identity.
        // Lind & Tyler (1988) procedural justice; Miller (2001) injustice experience
        grievance: 0.85,

        // Disposition - Reactance
        // Forced displacement eliminates autonomy and imposes external constraints,
        // triggering strong psychological reactance that persists throughout displacement.
        // Brehm (1966) psychological reactance; Bhugra & Becker (2005)
        reactance: 0.70,

        // Disposition - Trust Propensity
        // Forced displacement severely damages trust through institutional betrayal,
        // social network destruction, and trauma.
        // Mayer et al. (1995) integrative model of trust; refugee trauma literature
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
        impulse_control: false,
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.35,
        arousal: 0.25,
        dominance: 0.18,
        fatigue: 0.38,
        stress: 0.25,
        purpose: 0.18,
        loneliness: 0.35,
        prc: 0.18,
        perceived_liability: 0.35,
        self_hate: 0.18,
        perceived_competence: 0.18,
        depression: 0.28,
        self_worth: 0.25,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.35,
        impulse_control: 0.08,
        empathy: 0.18,
        aggression: 0.15,
        grievance: 0.55,
        reactance: 0.25,
        trust_propensity: 0.35,
    },
};
