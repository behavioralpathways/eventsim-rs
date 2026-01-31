//! UndergoRetirementVoluntary event specification.
//!
//! Voluntary retirement where the individual chooses to retire on their own terms,
//! typically after planning and preparation. Differs from forced retirement in
//! maintained agency, control over timing, and generally positive psychological outcomes.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Voluntary retirement produces moderate positive valence from relief and freedom; hedonic adaptation and identity transition concerns limit the effect.
        // Pinquart & Schindler (2007) meta-analysis on retirement and well-being; Ekerdt & DeViney on retirement as achievement.
        valence: 0.35,

        // Mood - Arousal
        // Voluntary retirement reduces physiological arousal through relief from work stress and occupational demands; initial transition period stabilizes at moderately lower baseline.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Posner et al. (2005) on affect structure.
        arousal: -0.25,

        // Mood - Dominance
        // Voluntary retirement preserves agency and self-determination through active choice; partially offset by loss of workplace authority and role status.
        // Deci & Ryan (1985) Self-Determination Theory; retirement adjustment literature on autonomy.
        dominance: 0.35,

        // Needs - Fatigue
        // Voluntary retirement removes chronic work-related exhaustion and stress, providing substantial energy restoration as daily demands decrease significantly.
        // Hockey, G.R.J. (2013). The psychology of fatigue; Sonnentag & Zijlstra (2006) on recovery from work.
        fatigue: -0.35,

        // Needs - Stress
        // Voluntary retirement reduces chronic workplace stress through choice and control; temporary mild adjustment stress resolves within months.
        // McEwen (1998) on control/predictability buffering stress response; Selye (1956) on adaptation to planned changes.
        stress: -0.45,

        // Needs - Purpose
        // Voluntary retirement creates planned loss of work identity and externally-defined purpose; the chosen nature and planning buffer existential disruption.
        // Ryff, C.D. (1989). Purpose in life as psychological construct; Ekerdt, D.J. (1986) on busy ethic in retirement.
        purpose: -0.15,

        // Social Cognition - Loneliness
        // Voluntary retirement causes initial mild loneliness from workplace disconnection; planned transition and increased family/friend time enable rapid recovery.
        // Joiner, T. (2005). Why People Die by Suicide; Perlman & Peplau (1981) Loneliness: A Sourcebook.
        loneliness: 0.15,

        // Social Cognition - PRC
        // Voluntary retirement signals personal agency and typically occurs with family support, producing mild gains in perceived reciprocal care as family relationships strengthen.
        // Joiner, T. (2005); Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        prc: 0.08,

        // Social Cognition - Perceived Liability
        // Voluntary retirement with adequate planning reduces perceived liability through continued financial independence and retained social contribution opportunities.
        // Joiner, T. (2005); Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        perceived_liability: -0.15,

        // Social Cognition - Self Hate
        // Voluntary retirement affirms agency and choice, producing mild self-compassion from self-determination rather than shame; identity adjustment is temporary.
        // Deci & Ryan (2000) on self-determination and wellbeing; Joiner (2005) on voluntary transitions.
        self_hate: -0.08,

        // Social Cognition - Perceived Competence
        // Voluntary retirement involves net loss of primary competence domain with partial offset from planning autonomy; mild domain-specific decline partially recovers through new activities.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Sharpley & Yardley (1997) on retirement transitions.
        perceived_competence: -0.15,

        // Mental Health - Depression
        // Voluntary, planned retirement shows mild positive mood trajectory overall with only brief adjustment-period symptoms that resolve with meaningful post-retirement engagement.
        // Brown & Harris (1978) Social Origins of Depression; retirement psychology literature on planned transitions.
        depression: -0.05,

        // Mental Health - Self Worth
        // Voluntary retirement affirms personal agency and represents goal achievement, providing mild positive boost to self-worth as new identity stabilizes.
        // Baumeister et al. (2003) on self-esteem determinants; Deci & Ryan (2000) on autonomy as self-worth foundation.
        self_worth: 0.15,

        // Mental Health - Hopelessness
        // Voluntary retirement represents planned, self-determined life transition offering new opportunities and demonstrating future optimism.
        // Beck's Hopelessness Scale framework; retirement adjustment literature on planned transitions.
        hopelessness: -0.22,

        // Mental Health - Interpersonal Hopelessness
        // Voluntary retirement increases time and emotional capacity for relationships, decreasing belief that relationships cannot help.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Joiner (2005) on social connection.
        interpersonal_hopelessness: -0.15,

        // Mental Health - Acquired Capability
        // Voluntary retirement involves no pain or death exposure; acquired capability only develops through habituation to physical pain and death.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Voluntary retirement reduces chronic occupational stress, temporarily improving self-regulatory capacity and impulse control through relief and restored executive function.
        // Baumeister, R.F. & Vohs, K.D. (2001) on self-regulation; stress-induced ego depletion theory.
        impulse_control: 0.25,

        // Disposition - Empathy
        // Voluntary retirement reduces occupational stress and creates opportunity for social engagement, producing mild increase in empathic capacity.
        // Decety & Jackson (2004) on stress-empathy relationship; Pinquart & Schindler (2007) on retirement adjustment.
        empathy: 0.12,

        // Disposition - Aggression
        // Voluntary retirement removes workplace stressors and conflicts without triggering frustration, resulting in mild aggression reduction through stress relief.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; retirement stress reduction literature.
        aggression: -0.15,

        // Disposition - Grievance
        // Voluntary retirement eliminates injustice narrative because choice is self-directed; perceived fairness increases and grievance decreases slightly.
        // Lind, E.A. & Tyler, T.R. (1988). The social psychology of procedural justice.
        grievance: -0.15,

        // Disposition - Reactance
        // Voluntary retirement preserves autonomy and eliminates work-imposed constraints, producing mild decrease in reactance rather than threat-induced resistance.
        // Brehm, J.W. (1966). A theory of psychological reactance; Deci & Ryan Self-Determination Theory.
        reactance: -0.15,

        // Disposition - Trust Propensity
        // Voluntary retirement involves positive agency and self-determination without betrayal, producing mild temporary increases in trust through maintained social engagement.
        // Mayer, R.C., Davis, J.H., & Schoorman, F.D. (1995). An integrative model of organizational trust.
        trust_propensity: 0.12,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: false,
        fatigue: true,
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
        valence: 0.12,
        arousal: 0.08,
        dominance: 0.08,
        fatigue: 0.12,
        stress: 0.08,
        purpose: 0.12,
        loneliness: 0.05,
        prc: 0.05,
        perceived_liability: 0.05,
        self_hate: 0.02,
        perceived_competence: 0.08,
        depression: 0.02,
        self_worth: 0.05,
        hopelessness: 0.08,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.08,
        empathy: 0.05,
        aggression: 0.05,
        grievance: 0.04,
        reactance: 0.08,
        trust_propensity: 0.06,
    },
};
