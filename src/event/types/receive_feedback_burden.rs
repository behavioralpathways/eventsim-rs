//! ReceiveFeedbackBurden event specification.
//!
//! Receiving explicit feedback from others that one is a burden to them.
//! This event directly activates perceived burdensomeness (a core ITS risk factor)
//! and has significant impacts across social, emotional, and mental health dimensions.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Explicit burden feedback creates significant negative valence through identity-level
        // rejection and elevated perceived burdensomeness.
        // Joiner (2005). Why People Die by Suicide; Eisenberger et al. (2003). Science, 302(5643), 290-292.
        valence: -0.48,

        // Mood - Arousal
        // Burden feedback triggers moderate acute arousal through social threat perception,
        // but decays rapidly as initial stress subsides.
        // Thayer (1989). The biopsychology of mood and arousal; Russell (1980). A circumplex model of affect.
        arousal: 0.35,

        // Mood - Dominance
        // Receiving feedback that one is a burden significantly reduces sense of control and agency.
        // Bandura (1977). Self-efficacy; Deci & Ryan (1985). Self-Determination Theory.
        dominance: -0.45,

        // Needs - Fatigue
        // Burden feedback triggers emotional exhaustion through rejection processing and rumination.
        // Joiner (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        fatigue: 0.32,

        // Needs - Stress
        // Explicit burden feedback triggers moderate-to-high physiological stress through social
        // threat perception and activation of perceived burdensomeness.
        // Van Orden et al. (2010). Psychological Review, 117(2), 575-600; Eisenberger et al. (2003). Science, 302(5643), 290-292.
        stress: 0.52,

        // Needs - Purpose
        // Explicit feedback that one is a burden directly threatens sense of meaningful existence
        // and contribution, activating Joiner's perceived burdensomeness construct.
        // Joiner (2005). Why People Die by Suicide; Van Orden et al. (2010). Psychological Review, 117(2), 575-600.
        purpose: -0.38,

        // Social Cognition - Loneliness
        // Burden feedback delivers explicit rejection signaling unsuitability in relationships,
        // creating significant immediate loneliness with chronic rumination patterns.
        // Joiner (2005). Why People Die by Suicide; Eisenberger et al. (2003). Science, 302(5643), 290-292.
        loneliness: 0.48,

        // Social Cognition - PRC (Perceived Reciprocal Care)
        // Burden feedback signals low reciprocal care and rejection, creating significant
        // but temporary decrease in perceived caring.
        // Joiner (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Direct feedback that one is a burden produces significant acute perceived liability increase;
        // this is the core dimension directly targeted by this event.
        // Joiner (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        perceived_liability: 0.65,

        // Social Cognition - Self Hate
        // Direct feedback that one is a burden triggers sustained shame-based self-blame and
        // internalizes the burden identity through perceived validation of negative self-perceptions.
        // Van Orden et al. (2010). The Interpersonal Theory of Suicide; Gilbert & Irons (2005). Compassionate mind training.
        self_hate: 0.55,

        // Social Cognition - Perceived Competence
        // Feedback that one is a burden creates significant doubt in social and relational competence
        // through social persuasion.
        // Bandura (1997). Self-efficacy: The exercise of control; Joiner (2005). Why people die by suicide.
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Explicit interpersonal feedback that one burdens others triggers significant depressive
        // symptoms through shame-based rumination and social withdrawal.
        // Van Orden et al. (2010). Psychological Review, 117(2), 575-600; Tangney & Dearing (2002). Shame and guilt.
        depression: 0.35,

        // Mental Health - Self Worth
        // Feedback that one is a burden creates significant identity-level self-worth damage
        // through relational devaluation and chronic rumination.
        // Joiner (2005). Why People Die by Suicide; Tangney & Dearing (2002). Shame and Guilt.
        self_worth: -0.42,

        // Mental Health - Hopelessness
        // Receiving burden feedback creates significant hopelessness about social relationships
        // and future acceptance.
        // Joiner (2005). Why people die by suicide.
        hopelessness: 0.42,

        // Mental Health - Interpersonal Hopelessness
        // Receiving explicit feedback of burdensomeness creates a belief that one's presence
        // damages others' wellbeing, severely undermining interpersonal hope.
        // Joiner (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        interpersonal_hopelessness: 0.52,

        // Mental Health - Acquired Capability
        // Feedback events lack physical pain/death exposure required for acquired capability
        // development; this event affects perceived burdensomeness instead.
        // Joiner (2005). Why People Die by Suicide - Chapter on acquired capability.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Burden feedback depletes self-regulatory resources through emotional distress and
        // thwarted belongingness, producing acute impulse control impairment.
        // Baumeister et al. (1998). JPSP, 70(5), 1252-1265; Muraven & Baumeister (2000). Psychological Bulletin, 126(2), 247-259.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Receiving burden feedback increases self-focus and triggers shame, temporarily reducing
        // capacity for empathic perspective-taking.
        // Davis (1983). Measuring individual differences in empathy; Singer & Klimecki (2014). Empathy and compassion.
        empathy: -0.15,

        // Disposition - Aggression
        // Burden feedback triggers moderate acute aggression through ego threat and thwarted
        // belongingness.
        // Eisenberger & Lieberman (2004). Trends in Cognitive Sciences; Berkowitz (1989). Frustration-aggression hypothesis.
        aggression: 0.35,

        // Disposition - Grievance
        // Receiving burden feedback creates moderate grievance through perception of unjust
        // social judgment.
        // Van Orden et al. (2010). The Interpersonal Theory of Suicide; Joiner (2005). Why People Die by Suicide.
        grievance: 0.35,

        // Disposition - Reactance
        // Burden feedback triggers shame and social devaluation but not a direct threat to
        // autonomous freedom, resulting in minimal reactance.
        // Brehm (1966). A theory of psychological reactance.
        reactance: 0.05,

        // Disposition - Trust Propensity
        // Direct feedback that one is a burden significantly reduces trust propensity through
        // explicit relational rejection signals.
        // Joiner (2005). Why People Die by Suicide; Van Orden et al. (2010). The interpersonal theory of suicide.
        trust_propensity: -0.35,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: false,
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
        empathy: false,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.10,
        arousal: 0.06,
        dominance: 0.08,
        fatigue: 0.11,
        stress: 0.09,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.06,
        perceived_liability: 0.18,
        self_hate: 0.08,
        perceived_competence: 0.12,
        depression: 0.08,
        self_worth: 0.10,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.18,
        impulse_control: 0.08,
        empathy: 0.02,
        aggression: 0.08,
        grievance: 0.08,
        reactance: 0.02,
        trust_propensity: 0.10,
    },
};
