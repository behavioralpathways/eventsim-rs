//! ExperienceRejectionFamily event specification.
//!
//! Experiencing rejection by family members - being disowned, estranged,
//! explicitly told one is not wanted, or excluded from family activities
//! and gatherings. More severe than family conflict as it involves active
//! rejection of the person's place in the family.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Family rejection combines betrayal of foundational attachments with humiliation-level social devaluation, creating severe negative valence
        // Eisenberger et al. (2003) on social pain; Joiner (2005) Why People Die by Suicide; Tangney & Dearing (2002) Shame and Guilt
        valence: -0.58,

        // Mood - Arousal
        // Family rejection triggers acute high arousal from threat/loss detection system, with chronic elevation from ongoing uncertainty and broken attachment
        // Eisenberger, Lieberman & Williams (2003) Science; Kross et al. (2011) PNAS on social rejection and pain
        arousal: 0.65,

        // Mood - Dominance
        // Family rejection imposes loss of control and agency over an involuntary outcome, but permits adaptation in non-family domains
        // Bandura (1977) Self-efficacy; Deci & Ryan (1985) Self-determination theory
        dominance: -0.45,

        // Needs - Fatigue
        // Family rejection creates sustained emotional processing, sleep disruption, and loss of primary social support
        // Cacioppo & Patrick (2008) Loneliness; Van Orden et al. (2010) ITS; Baumeister et al. (1998) ego depletion
        fatigue: 0.48,

        // Needs - Stress
        // Family rejection activates HPA axis through loss of primary attachment bonds and social safety signals
        // McEwen (1998) Annals of NY Academy of Sciences on allostasis and allostatic load
        stress: 0.65,

        // Needs - Purpose
        // Family rejection causes major existential disruption through identity loss and belonging deprivation
        // Agllias (2012) on parental estrangement; attachment and identity theories on meaning disruption
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Family rejection produces severe loneliness through loss of primary belonging relationship
        // Joiner (2005) Why People Die by Suicide; Williams (2007) Social Rejection; Baumeister & Leary (1995) need to belong
        loneliness: 0.65,

        // Social Cognition - PRC
        // Family rejection explicitly signals the family does not care about the person's place in the family system
        // Van Orden et al. (2010) Psychological Review; Joiner (2005) Why People Die by Suicide
        prc: -0.68,

        // Social Cognition - Perceived Liability
        // Family rejection creates moderate perceived liability through internalized worthlessness, though primary pathway is belongingness
        // Van Orden et al. (2010) ITS on distinction between TB and PB as separate risk factors
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Family rejection triggers severe shame-based self-hate through internalization of rejection by primary attachment figures
        // Joiner (2005) Why People Die by Suicide; Gilbert & Irons (2005) on shame-based psychopathology
        self_hate: 0.65,

        // Social Cognition - Perceived Competence
        // Family rejection is a severe social judgment creating lasting doubt about interpersonal and social competence
        // Bandura (1977, 1997) Self-efficacy; Bowlby (1969); Mikulincer & Shaver (2007) attachment
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Family rejection triggers significant depressive symptoms through loss of primary attachment and identity validation
        // Brown & Harris (1978) Social Origins of Depression; Kendler et al. (1999); Afifi et al. (2003) on estrangement
        depression: 0.50,

        // Mental Health - Self Worth
        // Family rejection represents fundamental non-acceptance of the person's identity, triggering severe shame and reduced self-worth
        // Crocker & Wolfe (2001) Contingencies of self-worth; Baumeister & Leary (1995) need to belong
        self_worth: -0.75,

        // Mental Health - Hopelessness
        // Family rejection creates severe hopelessness through perceived permanence of belonging loss and uncontrollability
        // Abramson, Metalsky & Alloy (1989) Hopelessness depression theory, Psychological Review
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Family rejection creates severe generalized hopelessness about future relationships because it targets foundational belonging
        // Van Orden et al. (2010) ITS; Joiner (2005) Why People Die by Suicide on rejection generalization
        interpersonal_hopelessness: 0.60,

        // Mental Health - Acquired Capability
        // Family rejection is a psychosocial stressor involving emotional pain, not habituation to physical pain or death
        // Joiner (2005) ITS model - AC requires habituation through pain/death exposure, not emotional stressors
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Family rejection creates acute emotional crisis causing significant self-regulatory resource depletion
        // Baumeister et al. (1998) Ego depletion; Muraven & Baumeister (2000) Self-regulation
        impulse_control: -0.35,

        // Disposition - Empathy
        // Family rejection triggers defensive self-focus and attachment disruption, temporarily reducing perspective-taking
        // Baumeister & Leary (1995) need to belong; Eisenberger et al. (2003) on social exclusion
        empathy: -0.15,

        // Disposition - Aggression
        // Family rejection triggers major ego threat and belongingness frustration, producing significant aggression elevation
        // Berkowitz (1989) Frustration-Aggression Hypothesis; Leary et al. on rejection sensitivity
        aggression: 0.55,

        // Disposition - Grievance
        // Family rejection creates severe perceived injustice through violation of unconditional bond expectation
        // Agllias (2013); Scharp & Dorrance Hall (2017) family estrangement; Miller (2001) on disrespect
        grievance: 0.72,

        // Disposition - Reactance
        // Family rejection imposes relational constraints but inability to restore autonomy may create helplessness over resistance
        // Brehm (1966) Psychological Reactance theory; attachment theory on autonomy-threatening dynamics
        reactance: 0.15,

        // Disposition - Trust Propensity
        // Family rejection represents severe betrayal by primary attachment figures, causing substantial trust damage
        // Rohner (2004) Parental Acceptance-Rejection Theory; Bowlby (1988) A Secure Base
        trust_propensity: -0.75,
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
        valence: 0.16,
        arousal: 0.18,
        dominance: 0.12,
        fatigue: 0.16,
        stress: 0.22,
        purpose: 0.25,
        loneliness: 0.18,
        prc: 0.28,
        perceived_liability: 0.25,
        self_hate: 0.25,
        perceived_competence: 0.15,
        depression: 0.18,
        self_worth: 0.25,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.22,
        impulse_control: 0.12,
        empathy: 0.12,
        aggression: 0.25,
        grievance: 0.38,
        reactance: 0.22,
        trust_propensity: 0.35,
    },
};
