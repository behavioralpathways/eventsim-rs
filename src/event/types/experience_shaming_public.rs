//! ExperienceShamingPublic event specification.
//!
//! Public shaming involving being publicly called out, criticized, or shamed in front of others,
//! potentially including online/social media shaming where character or behavior is condemned.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Public shaming combines internalized negative self-evaluation with witnessed social devaluation, creating severe but partially-recoverable valence reduction through shame-based affect and reputation threat.
        // Tangney, J.P., & Dearing, R.L. (2002). Shame and guilt. Guilford Press; Dickerson, S.S., et al. (2009). Emotional Reactivity to Social-evaluative Threat. Psychoneuroendocrinology
        valence: -0.62,

        // Mood - Arousal
        // Public shaming triggers acute sympathetic arousal from social-evaluative threat comparable to humiliation, but the internalized judgment and chronic rumination sustain elevated activation longer.
        // Dickerson, S.S., & Kemeny, M.E. (2004). Acute stressors and cortisol responses. Psychological Bulletin; Eisenberger, N.I., et al. (2003). Does rejection hurt? Science
        arousal: 0.58,

        // Mood - Dominance
        // Public shaming induces acute loss of control through social threat, dominated by perceived powerlessness in the social sphere rather than complete incapacity.
        // Tangney & Dearing (2002) on shame-proneness and loss of agency; Morrison (2008) on shame as an emotion tied to public visibility and perceived social power loss
        dominance: -0.65,

        // Needs - Fatigue
        // Public shaming creates immediate emotional exhaustion through stress activation and post-crisis fatigue, with accumulated effects from rumination and social withdrawal.
        // Tangney & Dearing (2002) on shame's physiological stress response; Thomaes et al. (2011) on social threat and coping depletion
        fatigue: 0.42,

        // Needs - Stress
        // Public shaming triggers acute HPA axis activation (elevated cortisol, sympathetic arousal) as a potent social-evaluative threat; repeated exposure accumulates chronic stress response elevation.
        // Dickerson & Kemeny (2004) on social-evaluative threat and cortisol reactivity; Eisenberger & Lieberman (2004) on social pain
        stress: 0.65,

        // Needs - Purpose
        // Public shaming disrupts identity narrative and social credibility necessary for purpose investment, creating minor-to-moderate questioning of direction without existential collapse.
        // Brown on shame disrupting identity narrative; Tangney & Dearing (2002) on shame vs guilt distinction
        purpose: -0.18,

        // Social Cognition - Loneliness
        // Public shaming creates significant acute loneliness through perceived social rejection and shame-driven withdrawal; stigma and reputation damage compound isolation.
        // Baumeister & Leary (1995) The Need to Belong; Williams (2007) on ostracism and social pain
        loneliness: 0.35,

        // Social Cognition - PRC
        // Public shaming communicates social rejection and negative evaluation from an audience, signaling that others do not care about your wellbeing or reputation.
        // Tangney et al. (2007) on shame and social disconnection; Williams (2001) on ostracism and belongingness
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Public shaming triggers acute shame and social rejection, amplifying beliefs that one is a burden or social liability through witnessed social devaluation.
        // Tangney & Dearing (2002) Shame and Guilt; Joiner (2005) Why People Die by Suicide - shame contributes to perceived burdensomeness
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Public shaming triggers acute shame affect (social evaluation threat) which translates to significant self-hate through internalization of perceived social condemnation.
        // Tangney & Dearing (2002) Shame and Guilt; Sznycer et al. (2016) on shame as reputation management emotion
        self_hate: 0.55,

        // Social Cognition - Perceived Competence
        // Public shaming directly targets self-perception of competence through witnessed failure and social threat activation, creating moderate doubt with delayed recovery due to audience effects.
        // Bandura (1997) Self-Efficacy: The Exercise of Control; Tangney & Dearing (2002) on shame vs guilt and self-targeting effects
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Public shaming triggers shame-proneness and social withdrawal that elevate depressive symptoms significantly; recovery occurs within weeks unless compounded by repeated exposure.
        // Tangney & Dearing (2002) Shame and Guilt; Eisenberger et al. (2003) on social pain activating same neural systems as physical pain
        depression: 0.35,

        // Mental Health - Self Worth
        // Public shaming creates significant self-worth damage through witnessed criticism and social devaluation, more severe than peer rejection due to audience amplification.
        // Tangney & Dearing (2002). Shame and Guilt; Eisenberger et al. (2003). Does rejection hurt? Science
        self_worth: -0.55,

        // Mental Health - Hopelessness
        // Public shaming induces significant shame-based hopelessness about social standing and future acceptance, but most individuals retain some belief in eventual recovery.
        // Joiner (2005) Interpersonal Theory of Suicide; Tangney & Dearing (2002) on shame's role in hopelessness escalation
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Public shaming creates identity-level beliefs about worthiness that specifically undermine help-seeking, with chronic avoidance from awareness that others witnessed the shame.
        // Tangney & Dearing (2002) Shame and Guilt; Dickerson et al. (2009) on social-evaluative threat
        interpersonal_hopelessness: 0.42,

        // Mental Health - Acquired Capability
        // Public shaming is a social/emotional stressor involving reputation threat and shame-based psychological pain, not the physical pain or death exposure required to develop habituation-based acquired capability.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Public shaming activates threat response systems that acutely suppress executive function and impulse control, with small lasting effects amplified through repeated exposure.
        // Tangney & Dearing (2002) Shame and Guilt; Eisenberger (2012) on social pain and threat response
        impulse_control: -0.35,

        // Disposition - Empathy
        // Public shaming explicitly attacks character and identity through social condemnation, intensifying self-focused defense and persistent rumination about reputation.
        // Tangney & Dearing (2002) Shame and Guilt; Gilbert (2000) Shame, Social Anxiety and Depression
        empathy: -0.20,

        // Disposition - Aggression
        // Public shaming triggers significant aggression through perceived unfairness and norm violation, with chronic escalation from repeated episodes and reputation-based rumination.
        // Bushman & Baumeister (2002) on ego threat and aggression; Tangney et al. (2007) on shame-based anger
        aggression: 0.55,

        // Disposition - Grievance
        // Public shaming creates strong grievance through active accusation and public audience amplification of perceived injustice, with chronic effects from reputation damage and victim narrative integration.
        // Miller, D.T. (2001). Disrespect and the experience of injustice; Tangney & Dearing (2002) Shame and Guilt
        grievance: 0.58,

        // Disposition - Reactance
        // Public shaming triggers significant reactance through explicit social constraint on behavioral freedom and demonstrates accumulated oppositional resistance with repeated shaming cycles.
        // Brehm, J.W. (1966). A theory of psychological reactance; Tangney & Dearing (2002) on shame-based social control mechanisms
        reactance: 0.50,

        // Disposition - Trust Propensity
        // Public shaming creates moderate trust damage by demonstrating social threat and untrustworthiness of audience members, with permanence comparable to relationship betrayal due to shame's sticky psychological nature.
        // Tangney & Dearing (2002) Shame and Guilt; Morrison (2008) The Culture of Shame
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
        valence: 0.11,
        arousal: 0.09,
        dominance: 0.12,
        fatigue: 0.08,
        stress: 0.08,
        purpose: 0.07,
        loneliness: 0.08,
        prc: 0.12,
        perceived_liability: 0.08,
        self_hate: 0.12,
        perceived_competence: 0.12,
        depression: 0.09,
        self_worth: 0.11,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.14,
        impulse_control: 0.08,
        empathy: 0.06,
        aggression: 0.15,
        grievance: 0.22,
        reactance: 0.20,
        trust_propensity: 0.12,
    },
};
