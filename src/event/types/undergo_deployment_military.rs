//! Military Deployment event specification.
//!
//! Being deployed for military service, involving separation from civilian life,
//! operational stress, and potential combat exposure. Distinct from direct combat
//! engagement, deployment encompasses the broader experience of military service
//! including training, non-combat duties, and reintegration challenges.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Military deployment creates significant negative valence through separation,
        // uncertainty, and operational stress, with chronic elevation in negative affect.
        // Hoge et al. (2004) NEJM; Pietrzak et al. (2010) PTSD burden study
        valence: -0.55,

        // Mood - Arousal
        // Deployment induces marked physiological arousal during service and post-deployment,
        // with lasting hypervigilance effects characteristic of combat-related stress responses.
        // Adler et al. (2009) Journal of Occupational Health Psychology
        arousal: 0.65,

        // Mood - Dominance
        // Deployment removes personal autonomy and exposes individuals to uncontrollable
        // threats, reducing dominance through loss of agency despite training-based competence.
        // Bandura (1977) self-efficacy theory; combat stress literature
        dominance: -0.55,

        // Needs - Fatigue
        // Severe fatigue from sustained physical demands, sleep disruption, and intense
        // emotional processing during deployment operations.
        // Hockey (2013) The Psychology of Fatigue; military medicine research
        fatigue: 0.75,

        // Needs - Stress
        // Military deployment activates severe physiological stress response through threat
        // perception, loss of control, and environmental demands (HPA axis/cortisol elevation).
        // McEwen (1998) Allostasis and allostatic load
        stress: 0.75,

        // Needs - Purpose
        // Military deployment typically enhances sense of purpose through clear role
        // definition and values-aligned contribution to mission objectives.
        // Frankl (1959) Man's Search for Meaning; Steger's Meaning in Life framework
        purpose: 0.35,

        // Social Cognition - Loneliness
        // Deployment causes significant acute loneliness through forced separation from
        // family and primary social networks, partially offset by unit camaraderie.
        // Cacioppo & Patrick (2008); Joiner (2005) ITS framework
        loneliness: 0.45,

        // Social Cognition - PRC
        // Extended separation from primary relationships and reduced accessibility of
        // practical support create moderate perceived caring reduction.
        // Pietrzak et al. (2010); Sayer et al. (2010); Van Orden et al. (2010)
        prc: -0.20,

        // Social Cognition - Perceived Liability
        // Deployment creates moderate concern about being a burden through separation-related
        // family strain and post-deployment adjustment difficulties.
        // Van Orden et al. (2010) ITS; Bryan & Hernandez (2012)
        perceived_liability: 0.22,

        // Social Cognition - Self Hate
        // Deployment provides structure and purpose, but moral injury and combat exposure
        // can trigger mild self-blame that typically resolves post-deployment.
        // Wisco et al. (2014) Journal of Traumatic Stress
        self_hate: 0.15,

        // Social Cognition - Perceived Competence
        // Military deployment typically increases perceived competence through skill mastery
        // and successful task completion in challenging environments.
        // Bandura (1997) Self-efficacy: The exercise of control
        perceived_competence: 0.25,

        // Mental Health - Depression
        // Deployment creates moderate depressive symptoms from separation, uncertainty,
        // and role disruption, with chronic effects but lower severity than direct combat.
        // Seal et al. (2009) American Journal of Public Health; Kline et al. (2010)
        depression: 0.35,

        // Mental Health - Self Worth
        // Military deployment creates modest positive self-worth boost through identity
        // formation, structured purpose, and accomplishment.
        // Litz et al. (2009); Frankl (1959)
        self_worth: 0.15,

        // Mental Health - Hopelessness
        // Deployment creates significant hopelessness through loss of autonomy and uncertain
        // future prospects, but with greater recovery potential than combat trauma.
        // Seligman (1975) Learned helplessness; Van Orden et al. (2010)
        hopelessness: 0.45,

        // Mental Health - Interpersonal Hopelessness
        // Deployment frequently causes emotional numbing and social withdrawal that reduce
        // belief in relational support, especially with combat exposure.
        // Verdeli et al. (2011); DSM-5 PTSD criteria
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Military deployment creates substantial habituation to threat, violence, and death
        // exposure through sustained proximity to combat environments and casualty exposure.
        // Joiner (2005); Van Orden et al. (2010); Bryan et al. (2015)
        acquired_capability: 0.65,

        // Disposition - Impulse Control
        // Deployment creates significant acute stress-related impulse control impairment
        // through hyperarousal and executive function depletion.
        // Baumeister & Vohs (2007) ego depletion; Friedman (2006)
        impulse_control: -0.35,

        // Disposition - Empathy
        // Combat deployment induces trauma and survival stress that temporarily reduce
        // empathic capacity through emotional numbing and threat-induced self-focus.
        // Singer & Klimecki (2014) Current Biology
        empathy: -0.25,

        // Disposition - Aggression
        // Military deployment increases state aggression through involuntary constraints,
        // frustration with institutional structures, and sustained threat-focused cognition.
        // Elbogen & Johnson (2009); Taft et al. (2017)
        aggression: 0.28,

        // Disposition - Grievance
        // Deployment typically induces moderate grievance through perceived institutional
        // unfairness, sacrifice imbalance, and separation injustice.
        // Litz et al. (2009) Moral Injury; Shay (1994) Achilles in Vietnam
        grievance: 0.35,

        // Disposition - Reactance
        // Military deployment imposes significant constraints on autonomy and freedom
        // through hierarchical control and mandatory compliance.
        // Brehm (1966) Psychological Reactance; Dillard & Shen (2005)
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Military deployment involves intense in-group trust bonding but typically reduces
        // general trust propensity through trauma, hypervigilance, and institutional disappointments.
        // Pietrzak et al. (2009); Hoge et al. (2006)
        trust_propensity: -0.35,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: false,
        loneliness: true,
        prc: true,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: true,
        self_worth: false,
        hopelessness: true,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.15,
        arousal: 0.18,
        dominance: 0.18,
        fatigue: 0.12,
        stress: 0.15,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.08,
        perceived_liability: 0.15,
        self_hate: 0.08,
        perceived_competence: 0.15,
        depression: 0.12,
        self_worth: 0.08,
        hopelessness: 0.18,
        interpersonal_hopelessness: 0.18,
        impulse_control: 0.12,
        empathy: 0.25,
        aggression: 0.18,
        grievance: 0.15,
        reactance: 0.12,
        trust_propensity: 0.25,
    },
};
