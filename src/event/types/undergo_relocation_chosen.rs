//! Chosen Relocation event specification.
//!
//! Voluntary relocation - moving to a new city or region by personal choice,
//! including decisions for career opportunities, lifestyle preferences, or
//! proximity to family. Distinct from international immigration, chosen
//! relocation involves domestic moves with agency and self-determination.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Chosen relocation produces moderate positive affect from goal achievement
        // and agency, with hedonic adaptation reducing effects within 1-2 years.
        // Lyubomirsky & Sheldon (2005) on hedonic adaptation and goal attainment.
        valence: 0.35,

        // Mood - Arousal
        // Voluntary relocation generates sustained activation from planning,
        // decision-making, and novelty-seeking during transition period.
        // Russell (1980) circumplex model; Thayer (1989) biopsychology of arousal.
        arousal: 0.35,

        // Mood - Dominance
        // Chosen relocation reflects personal agency and mastery motivation,
        // creating moderate sense of control that adapts as environment normalizes.
        // Deci & Ryan (1985) Self-Determination Theory; Bandura (1977) self-efficacy.
        dominance: 0.35,

        // Needs - Fatigue
        // Chosen relocation creates moderate acute fatigue from logistics,
        // decision-making, and social adaptation effort, with near-complete
        // recovery within 1-3 months as new routines establish.
        // Baumeister et al. (1998) ego depletion; Schlossberg (1981) transition theory.
        fatigue: 0.32,

        // Needs - Stress
        // Voluntary relocation creates moderate acute stress from disruption and
        // adjustment, but high perceived control and choice significantly attenuate
        // physiological response compared to forced moves.
        // Holmes & Rahe (1967) Social Readjustment Rating Scale; Rodin & Langer (1977).
        stress: 0.25,

        // Needs - Purpose
        // Chosen relocation provides purpose boost through autonomy satisfaction
        // and goal clarity inherent in self-selected geographic change.
        // Deci & Ryan (2000) Self-Determination Theory; Lyubomirsky (2005).
        purpose: 0.25,

        // Social Cognition - Loneliness
        // Chosen relocation creates acute but transient social disruption through
        // severed ties and adjustment, but voluntary nature and agency moderate
        // severity; most adapt within 3-6 months.
        // Cacioppo & Patrick (2008); Van Orden et al. (2010) on belongingness.
        loneliness: 0.25,

        // Social Cognition - PRC
        // Chosen relocation triggers acute loss of established reciprocal caring
        // relationships through physical distance, despite decision agency;
        // recovery through maintained contact and new network building.
        // Stafford & Merolla (2007) long-distance relationships.
        prc: -0.15,

        // Social Cognition - Perceived Liability
        // Chosen relocation involves personal agency and self-direction, which
        // typically reduces burden perception through demonstrated self-efficacy.
        // Van Orden et al. (2010) ITS; self-determination theory.
        perceived_liability: -0.12,

        // Social Cognition - Self Hate
        // Chosen relocation involves agentic decision-making which provides mild
        // self-validation and self-compassion from perceived control and growth.
        // Deci & Ryan (2000) autonomy and well-being.
        self_hate: -0.08,

        // Social Cognition - Perceived Competence
        // Voluntary relocation demonstrates successful agency and adaptive capacity
        // through complex life planning and execution, validating competence.
        // Bandura (1977) self-efficacy; Schlossberg (1981) transition adaptation.
        perceived_competence: 0.30,

        // Mental Health - Depression
        // Voluntary relocation provides modest protective effect against depression
        // through agency and goal-seeking, though effects are transient due to
        // hedonic adaptation and initial adjustment stress.
        // Bhugra (2004) migration and mental health; Berry (1997) acculturation.
        depression: -0.10,

        // Mental Health - Self Worth
        // Voluntary relocation provides modest self-worth boost from agency and
        // self-directed life change, with typical hedonic adaptation within months.
        // Deci & Ryan (2000); Fredrickson (2001) positive emotions.
        self_worth: 0.15,

        // Mental Health - Hopelessness
        // Voluntary relocation represents controllable, self-chosen action opening
        // new possibilities, producing moderate optimism about the future.
        // Abramson et al. (1989) hopelessness theory; Beck (1974) future expectancy.
        hopelessness: -0.35,

        // Mental Health - Interpersonal Hopelessness
        // Chosen relocation shows intentional, optimistic framing suggesting
        // belief in relationship efficacy and future belonging despite disruption.
        // Van Orden et al. (2010) ITS; belongingness literature.
        interpersonal_hopelessness: -0.12,

        // Mental Health - Acquired Capability
        // Voluntary relocation does not involve physical pain, injury, violence,
        // or death exposure - the key mechanisms for habituation.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) ITS.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Chosen relocation creates cognitive load and logistical stress that
        // temporarily depletes impulse control resources, but agency provides
        // buffering against severe impairment.
        // Baumeister & Vohs (2007) self-regulation.
        impulse_control: -0.08,

        // Disposition - Empathy
        // Voluntary relocation causes mild temporary empathy reduction through
        // increased self-focus and social network disruption during adaptation.
        // Singer & Klimecki (2014) empathy and compassion.
        empathy: -0.12,

        // Disposition - Aggression
        // Voluntary relocation preserves autonomy and agency, reducing
        // frustration-aggression pathway despite temporary adjustment stress.
        // Berkowitz (1989) frustration-aggression hypothesis.
        aggression: -0.08,

        // Disposition - Grievance
        // Chosen relocation reinforces internal agency and self-determination,
        // reducing grievance narratives because the person chose circumstances.
        // Deci & Ryan (2000) self-determination and well-being.
        grievance: -0.15,

        // Disposition - Reactance
        // Voluntary relocation represents autonomy exercise and freedom of choice,
        // which reduces reactance; normalizes as new environment stabilizes.
        // Brehm & Brehm (1981) psychological reactance theory.
        reactance: -0.12,

        // Disposition - Trust Propensity
        // Chosen relocation temporarily increases social uncertainty and disrupts
        // trust networks, but agency preserves generalized trust propensity.
        // Putnam (2000) social capital; Rotter (1967) generalized trust.
        trust_propensity: 0.05,
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
        arousal: 0.05,
        dominance: 0.06,
        fatigue: 0.04,
        stress: 0.04,
        purpose: 0.12,
        loneliness: 0.04,
        prc: 0.05,
        perceived_liability: 0.04,
        self_hate: 0.02,
        perceived_competence: 0.12,
        depression: 0.04,
        self_worth: 0.08,
        hopelessness: 0.08,
        interpersonal_hopelessness: 0.02,
        impulse_control: 0.05,
        empathy: 0.04,
        aggression: 0.02,
        grievance: 0.02,
        reactance: 0.04,
        trust_propensity: 0.02,
    },
};
