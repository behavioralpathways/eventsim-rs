//! ExperienceHumiliationPublic event specification.
//!
//! Public humiliation involving being shamed, embarrassed, or degraded in front of others,
//! such as being publicly criticized, mocked, exposed, or having failures announced to a group.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Public humiliation induces intense negative affect (shame, embarrassment, anger) through social evaluation threat, creating severe but partially-recoverable valence reduction.
        // Tangney, J.P., & Dearing, R.L. (2002). Shame and guilt. Guilford Press; Dickerson, S.S., et al. (2009). Emotional Reactivity to Social-evaluative Threat, Psychoneuroendocrinology
        valence: -0.65,

        // Mood - Arousal
        // Public humiliation triggers acute sympathetic nervous system activation (elevated cortisol, heart rate) comparable to social-evaluative threat, but most individuals show substantial recovery within 24-48 hours.
        // https://doi.org/10.1037/0022-3514.64.6.885
        arousal: 0.55,

        // Mood - Dominance
        // Public humiliation creates significant imposed loss of control and social status, but most individuals show near-complete hedonic adaptation within weeks to months unless repeatedly experienced.
        // https://psycnet.apa.org/record/1974-17862-000 (Mehrabian & Russell, 1974)
        dominance: -0.65,

        // Needs - Fatigue
        // Public humiliation creates acute emotional and cognitive exhaustion through shame-induced rumination, sleep disruption, and ego depletion, with substantial recovery over 1-2 weeks.
        // Baumeister et al. (1998) on ego depletion; Eisenberger et al. on social pain neurobiology
        fatigue: 0.45,

        // Needs - Stress
        // Public humiliation triggers strong acute HPA activation from social threat and status loss, but most individuals show substantial recovery within weeks as the acute threat perception fades.
        // https://www.researchgate.net/publication/228866139_Social_Rejection_Shares_Somatosensory_Representations_with_Physical_Pain
        stress: 0.65,

        // Needs - Purpose
        // Public humiliation temporarily undermines confidence in goals and social role, but typically resolves within weeks unless it triggers role loss or repeated exposure.
        // Tangney, J.P., & Dearing, R.L. (2002). Shame and guilt. Guilford Press
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Public humiliation triggers acute social withdrawal and loss of interpersonal confidence; shame-induced avoidance creates significant loneliness increase with moderate permanence.
        // https://doi.org/10.1037/0033-2909.128.1.1
        loneliness: 0.35,

        // Social Cognition - PRC
        // Public humiliation signals social devaluation and witnessed indifference, creating significant belief that others don't care; mostly recoverable within 12-18 months as person rebuilds social connections.
        // https://doi.org/10.1037/0033-2909.117.3.497 (Baumeister & Leary, 1995 - The Need to Belong)
        prc: -0.45,

        // Social Cognition - Perceived Liability
        // Public humiliation creates significant perceived burden through witnessed social devaluation and internalized shame, with moderate permanence from reputation effects and chronic rumination patterns.
        // https://psycnet.apa.org/doi/10.1037/0022-3514.88.4.589 (Eisenberger et al. on social pain mechanisms)
        perceived_liability: 0.40,

        // Social Cognition - Self Hate
        // Public humiliation triggers sustained shame-based self-blame and internalized social judgment, creating significant but partially recoverable self-directed negativity.
        // Gilbert, P. (2000). The Relationship of Shame, Social Anxiety and Depression. Clinical Psychology & Psychotherapy
        self_hate: 0.65,

        // Social Cognition - Perceived Competence
        // Public humiliation creates significant domain-specific self-doubt through witnessed failure and social threat, but effects typically recover within 1 year absent reinforcement.
        // Bandura (1977) Self-efficacy: Toward a Unifying Theory of Behavioral Change; Tangney & Dearing (2002) on shame vs. guilt
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Public humiliation triggers shame-based rumination and social withdrawal that significantly increase depressive symptoms; repeated incidents show accumulation with partial permanence.
        // https://www.ncbi.nlm.nih.gov/pubmed/1670043 (Tangney shame research on depression connection)
        depression: 0.40,

        // Mental Health - Self Worth
        // Public humiliation causes major self-worth damage through witnessed shame and identity threat; chronic vigilance to social judgment persists but acute emotional impact fades through hedonic adaptation.
        // https://doi.org/10.1146/annurev.psych.49.1.235 (Tangney et al. on shame and self-concept)
        self_worth: -0.65,

        // Mental Health - Hopelessness
        // Public humiliation creates significant acute hopelessness through perceived loss of social control and reputation damage, but is typically recoverable as social memory fades.
        // Tangney & Dearing (2002) Shame and Guilt
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Public humiliation increases interpersonal hopelessness by triggering shame-based withdrawal and fear of further judgment, but recovery is achievable with supportive relationships.
        // https://www.apa.org/science/about/psa/stigma-mental-health (Rickwood et al., 2005)
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Public humiliation is a social/emotional stressor that does not involve physical pain, injury, or death exposure required to develop habituation-based acquired capability.
        // Joiner, T. (2005). Why People Die by Suicide
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Public humiliation acutely impairs impulse control through ego depletion and shame-induced dysregulation, with temporary effects recovering as emotional arousal normalizes.
        // Baumeister, R.F., et al. (1998). Ego depletion: Is the active self a limited resource? JPSP
        impulse_control: -0.35,

        // Disposition - Empathy
        // Public humiliation temporarily increases self-focus and threat sensitivity, reducing available cognitive resources for perspective-taking, but resolves within hours to days.
        // Davis, M.H. (1983). Measuring individual differences in empathy; Decety & Jackson (2004)
        empathy: -0.18,

        // Disposition - Aggression
        // Public humiliation triggers intense aggression through ego threat and injustice perception; repeated exposure creates chronic hostile tendencies despite typical recovery.
        // https://psycnet.apa.org/doi/10.1037/0022-3514.84.2.219 (Bushman & Baumeister on ego threat and aggression)
        aggression: 0.65,

        // Disposition - Grievance
        // Public humiliation creates strong grievance through disrespect and audience amplification, violating fairness norms and contributing to victim identity.
        // Miller, D.T. (2001). Disrespect and the experience of injustice. Psychological Inquiry
        grievance: 0.55,

        // Disposition - Reactance
        // Public humiliation constitutes an imposed social constraint on autonomy and dignity, triggering significant reactance through threat to freedom.
        // Brehm (1966) Psychological Reactance Theory
        reactance: 0.45,

        // Disposition - Trust Propensity
        // Public humiliation reduces interpersonal trust through social withdrawal and vigilance, but does not fundamentally alter beliefs about others' trustworthiness.
        // Eisenberger, N.I., & Lieberman, M.D. (2004). Why rejection hurts. Trends in Cognitive Sciences
        trust_propensity: -0.25,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: false,
        loneliness: true,
        prc: true,
        perceived_liability: true,
        self_hate: true,
        perceived_competence: true,
        depression: true,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.12,
        arousal: 0.05,
        dominance: 0.08,
        fatigue: 0.06,
        stress: 0.06,
        purpose: 0.05,
        loneliness: 0.12,
        prc: 0.12,
        perceived_liability: 0.12,
        self_hate: 0.12,
        perceived_competence: 0.12,
        depression: 0.08,
        self_worth: 0.12,
        hopelessness: 0.08,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.05,
        empathy: 0.03,
        aggression: 0.18,
        grievance: 0.25,
        reactance: 0.18,
        trust_propensity: 0.06,
    },
};
