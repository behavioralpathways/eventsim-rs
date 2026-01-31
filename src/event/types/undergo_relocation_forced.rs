//! Forced Relocation event specification.
//!
//! Involuntary relocation - being displaced within the same country due to
//! eviction, court orders, employer mandates with no choice, fleeing domestic
//! abuse, or other coercive circumstances where the person had no agency.
//! Distinct from forced immigration (which involves crossing borders) and
//! chosen relocation (which involves personal agency).

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Forced relocation creates severe negative affect through loss of control,
        // home disruption, and unmet autonomy needs; most recover within 1-2 years.
        // Desmond (2016) Evicted; Selye (1956) General Adaptation Syndrome.
        valence: -0.55,

        // Mood - Arousal
        // Forced relocation generates sustained hypervigilance from loss of agency,
        // environmental uncertainty, and threat activation during transition period.
        // McEwen (1998) allostatic load; Matthews & Gallo (2011) housing instability.
        arousal: 0.50,

        // Mood - Dominance
        // Forced relocation eliminates agency and autonomy, creating sustained
        // powerlessness and victimization from external coercion.
        // Deci & Ryan (1985) Self-Determination Theory; Rodin & Langer (1977).
        dominance: -0.55,

        // Needs - Fatigue
        // Forced relocation creates sustained cognitive/emotional load from crisis
        // management, loss of control, and sleep disruption without agency buffer.
        // Hockey (2013) psychology of fatigue; Baumeister et al. (1998) ego depletion.
        fatigue: 0.58,

        // Needs - Stress
        // Forced relocation triggers strong acute physiological stress through
        // threat perception and loss of control over fundamental housing security.
        // McEwen (1998) allostasis; Holmes & Rahe (1967) life events.
        stress: 0.65,

        // Needs - Purpose
        // Forced relocation disrupts life goals, community identity, and agency,
        // creating acute existential crisis with partial recovery over time.
        // Evans et al. (2003) residential mobility; Proshansky (1983) place identity.
        purpose: -0.28,

        // Social Cognition - Loneliness
        // Forced relocation severs established social networks and triggers
        // shame-based withdrawal, creating significant acute loneliness.
        // Cacioppo & Patrick (2008); Van Orden et al. (2010) ITS.
        loneliness: 0.52,

        // Social Cognition - PRC
        // Forced relocation strips agency and disrupts established support networks,
        // creating moderate perceived caring deficit during displacement.
        // Van Orden et al. (2010) ITS; Desmond (2016) Evicted.
        prc: -0.40,

        // Social Cognition - Perceived Liability
        // Forced relocation creates acute perceived burden through loss of agency,
        // financial dependency, and social disruption from displacement.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) ITS.
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Forced relocation creates shame from loss of control and stigma, but
        // lower direct self-blame than personal failure due to external causation.
        // Joiner (2005); displacement mental health literature.
        self_hate: 0.25,

        // Social Cognition - Perceived Competence
        // Forced relocation strips agency and contextual competence through
        // involuntary displacement, moderately damaging perceived self-efficacy.
        // Bandura (1977) self-efficacy; Deci & Ryan (2000) SDT.
        perceived_competence: -0.32,

        // Mental Health - Depression
        // Forced relocation triggers significant depressive symptoms through loss
        // of control, social disruption, and material loss with chronic activation.
        // Kendler et al. (1999); Brown & Harris (1978) social origins of depression.
        depression: 0.35,

        // Mental Health - Self Worth
        // Forced relocation threatens autonomy and identity while triggering shame,
        // causing significant but largely recoverable self-worth reduction.
        // Desmond (2016) Evicted; housing insecurity literature.
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Forced relocation induces significant hopelessness through loss of control,
        // housing security, and autonomy; ongoing uncertainty sustains pessimism.
        // Desmond (2016); Abramson et al. (1989) hopelessness theory.
        hopelessness: 0.32,

        // Mental Health - Interpersonal Hopelessness
        // Forced relocation temporarily impairs help-seeking through shame and loss
        // of agency, but beliefs about relationship efficacy typically recover.
        // Desmond (2016); Van Orden et al. (2010) ITS; Schlossberg (1981).
        interpersonal_hopelessness: 0.30,

        // Mental Health - Acquired Capability
        // Forced relocation creates psychological stress but does not expose the
        // person to physical pain, injury, violence, or death habituation.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) ITS.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Forced relocation acutely depletes self-regulatory resources through
        // stress activation, ego depletion, and sleep disruption.
        // Baumeister & Vohs (2007) self-regulation; Arnsten (2009) stress and PFC.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Forced relocation reduces empathy through sustained stress-induced
        // self-focus and ego depletion from loss of agency.
        // Singer & Klimecki (2014) empathy and compassion.
        empathy: -0.18,

        // Disposition - Aggression
        // Forced relocation activates frustration-aggression and reactance pathways
        // through loss of autonomy and perceived injustice.
        // Berkowitz (1989) frustration-aggression; Anderson & Bushman (2002).
        aggression: 0.32,

        // Disposition - Grievance
        // Forced relocation is experienced as severe injustice with loss of agency,
        // triggering significant grievance; sustained during displacement.
        // Lind & Tyler (1988) procedural justice; Mikula (1993) experience of injustice.
        grievance: 0.65,

        // Disposition - Reactance
        // Forced relocation eliminates autonomy over location and triggers strong
        // psychological reactance from involuntary constraint.
        // Brehm & Brehm (1981) psychological reactance theory.
        reactance: 0.45,

        // Disposition - Trust Propensity
        // Forced relocation damages generalized trust through involuntary agency
        // loss and social network disruption from institutional imposition.
        // Schlossberg (1981) transition theory; Rotter (1967) interpersonal trust.
        trust_propensity: -0.42,
    },

    chronic: ChronicFlags {
        valence: false,
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
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.06,
        arousal: 0.15,
        dominance: 0.12,
        fatigue: 0.16,
        stress: 0.12,
        purpose: 0.12,
        loneliness: 0.18,
        prc: 0.18,
        perceived_liability: 0.12,
        self_hate: 0.12,
        perceived_competence: 0.08,
        depression: 0.12,
        self_worth: 0.12,
        hopelessness: 0.08,
        interpersonal_hopelessness: 0.09,
        impulse_control: 0.05,
        empathy: 0.12,
        aggression: 0.06,
        grievance: 0.25,
        reactance: 0.16,
        trust_propensity: 0.18,
    },
};
