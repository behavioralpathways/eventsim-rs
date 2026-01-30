//! ExperienceBetrayalTrust event specification.
//!
//! Experiencing a significant betrayal of trust by someone close - a partner's
//! infidelity, a friend's deception, a family member's manipulation, or a
//! colleague's backstabbing that fundamentally violates relationship expectations.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Trust betrayal triggers intense negative affect through violation of core relationship expectations and attachment disruption
        // https://journals.sagepub.com/doi/10.1177/0146167297234003
        valence: -0.55,

        // Mood - Arousal
        // Betrayal produces high physiological arousal through threat detection, rumination, and hypervigilance toward future betrayal cues
        // https://www.sciencedirect.com/science/article/abs/pii/S0191886912000475
        arousal: 0.65,

        // Mood - Dominance
        // Betrayal creates acute loss of control and powerlessness through violation of agency in relationship decisions
        // https://pubmed.ncbi.nlm.nih.gov/16262767/
        dominance: -0.55,

        // Needs - Fatigue
        // Betrayal causes significant fatigue through emotional processing demands, hypervigilance, and disrupted sleep from rumination
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6627977/
        fatigue: 0.65,

        // Needs - Stress
        // Betrayal triggers severe stress response comparable to other major life traumas through attachment system activation
        // https://www.tandfonline.com/doi/abs/10.1080/07481187.2017.1284956
        stress: 0.65,

        // Needs - Purpose
        // Betrayal disrupts meaning systems and challenges beliefs about relationships and personal judgment
        // https://www.sciencedirect.com/science/article/abs/pii/S0191886916310121
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Betrayal increases loneliness through loss of trusted relationship and protective withdrawal from social connections
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC8328888/
        loneliness: 0.38,

        // Social Cognition - PRC
        // Betrayal severely reduces perceived relationship caring through direct evidence that trusted other does not value the relationship
        // https://onlinelibrary.wiley.com/doi/abs/10.1111/pere.12186
        prc: -0.65,

        // Social Cognition - Perceived Liability
        // Betrayal triggers moderate self-blame and questions about one's judgment in trusting, creating burden cognitions
        // https://journals.sagepub.com/doi/10.1177/0265407517734395
        perceived_liability: 0.25,

        // Social Cognition - Self Hate
        // Betrayal activates significant self-directed anger for trusting and shame about being deceived
        // https://www.tandfonline.com/doi/abs/10.1080/00223980.2020.1726815
        self_hate: 0.45,

        // Social Cognition - Perceived Competence
        // Betrayal undermines self-efficacy through failure to detect deception and protect oneself from harm
        // https://www.sciencedirect.com/science/article/abs/pii/S0191886918303064
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Betrayal triggers depressive symptoms through loss, rumination, and shattered assumptions about relationships
        // https://pubmed.ncbi.nlm.nih.gov/24040940/
        depression: 0.35,

        // Mental Health - Self Worth
        // Betrayal significantly damages self-worth through rejection implications and questioning one's value to others
        // https://journals.sagepub.com/doi/10.1177/0265407517734395
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Betrayal increases hopelessness about future relationships and ability to trust judgment
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6627977/
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Betrayal creates severe interpersonal hopelessness through generalized expectation that others will also betray
        // https://onlinelibrary.wiley.com/doi/abs/10.1111/pere.12186
        interpersonal_hopelessness: 0.65,

        // Mental Health - Acquired Capability
        // Betrayal does not involve physical pain exposure; emotional pain does not habituate to physical self-harm per Joiner's model
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5022773/
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Betrayal impairs impulse control through emotional dysregulation and revenge motivation
        // https://www.frontiersin.org/articles/10.3389/fpsyg.2018.00888/full
        impulse_control: -0.35,

        // Disposition - Empathy
        // Betrayal reduces empathy through protective self-focus and dehumanization of the betrayer
        // https://www.sciencedirect.com/science/article/abs/pii/S0191886916310121
        empathy: -0.18,

        // Disposition - Aggression
        // Betrayal strongly activates aggression through perceived injustice and revenge motivation
        // https://www.tandfonline.com/doi/abs/10.1080/00224545.2016.1152213
        aggression: 0.45,

        // Disposition - Grievance
        // Betrayal produces severe grievance through clear injustice perpetrated by a trusted other
        // https://journals.sagepub.com/doi/10.1177/0146167297234003
        grievance: 0.68,

        // Disposition - Reactance
        // Betrayal mildly increases reactance through violated autonomy and desire to reassert control
        // https://pubmed.ncbi.nlm.nih.gov/16262767/
        reactance: 0.15,

        // Disposition - Trust Propensity
        // Betrayal severely reduces general trust propensity through learned wariness and protective hypervigilance
        // https://www.sciencedirect.com/science/article/abs/pii/S0191886912000475
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
        impulse_control: true,
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.18,
        arousal: 0.12,
        dominance: 0.18,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.25,
        perceived_liability: 0.12,
        self_hate: 0.18,
        perceived_competence: 0.12,
        depression: 0.12,
        self_worth: 0.18,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.25,
        impulse_control: 0.12,
        empathy: 0.08,
        aggression: 0.12,
        grievance: 0.25,
        reactance: 0.06,
        trust_propensity: 0.25,
    },
};
