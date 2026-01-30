//! EngageSelfharmNonsuicidal event specification.
//!
//! Engaging in non-suicidal self-injury (NSSI) - deliberate self-harm without
//! suicidal intent, such as cutting, burning, hitting oneself, or other forms
//! of self-inflicted injury used as a maladaptive coping mechanism.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // NSSI is deeply distressing with immediate physical and emotional pain, but provides temporary negative affect reduction through emotion regulation
        // https://www.frontiersin.org/journals/psychiatry/articles/10.3389/fpsyt.2025.1605508/full
        valence: -0.55,

        // Mood - Arousal
        // NSSI produces immediate arousal reduction through negative reinforcement, but underlying distress returns creating cyclical pattern
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC4163759/
        arousal: -0.35,

        // Mood - Dominance
        // NSSI provides immediate short-term control over one's body and emotional state, but subsequent shame and guilt undermine sense of agency
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6891258/
        dominance: 0.25,

        // Needs - Fatigue
        // NSSI creates acute fatigue through intense emotional processing, rumination cycles, and disrupted sleep patterns
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC2875354/
        fatigue: 0.35,

        // Needs - Stress
        // NSSI produces significant acute stress reduction through endogenous opioid release and negative reinforcement
        // https://www.mdpi.com/2076-3425/15/3/287
        stress: -0.45,

        // Needs - Purpose
        // NSSI episodes significantly disrupt sense of purpose through existential distress and avoidance of meaningful activities
        // https://www.sciencedirect.com/science/article/abs/pii/S0165178115304510
        purpose: -0.35,

        // Social Cognition - Loneliness
        // NSSI increases loneliness through shame-driven social withdrawal and concealment, creating a reinforcing cycle
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC10199677/
        loneliness: 0.32,

        // Social Cognition - PRC
        // NSSI signals severe unmet emotional needs and typically triggers stigmatized responses that communicate lack of caring
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11262124/
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Engaging in self-harm creates tangible burden on family/caregivers while reinforcing the individual's self-perception as a burden
        // https://www.ansiedadyestres.es/art/2025/anyes2025a4
        perceived_liability: 0.32,

        // Social Cognition - Self Hate
        // NSSI represents active self-punishment and reaffirms deep self-loathing; behavior creates bidirectional cycle of shame and self-directed anger
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6891258/
        self_hate: 0.65,

        // Social Cognition - Perceived Competence
        // NSSI represents acute failure in adaptive coping, triggering shame that undermines self-worth and perceived competence
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6891258/
        perceived_competence: -0.35,

        // Mental Health - Depression
        // NSSI provides temporary emotional relief but generates secondary shame and guilt that deepens depression
        // https://www.nature.com/articles/s41598-025-91962-5
        depression: 0.35,

        // Mental Health - Self Worth
        // NSSI produces significant acute self-worth reduction through self-punishment cognitions and identity disturbance
        // https://pubmed.ncbi.nlm.nih.gov/28647667/
        self_worth: -0.40,

        // Mental Health - Hopelessness
        // NSSI temporarily reduces emotional distress but reinforces maladaptive coping and deepens hopelessness about future capability
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC4244874/
        hopelessness: 0.15,

        // Mental Health - Interpersonal Hopelessness
        // NSSI reinforces thwarted belongingness and avoidant help-seeking patterns, increasing interpersonal hopelessness
        // https://www.ansiedadyestres.es/art/2025/anyes2025a4
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // NSSI represents repeated exposure to self-inflicted pain and injury, creating lasting habituation that decreases fear of death
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC9655904/
        acquired_capability: 0.45,

        // Disposition - Impulse Control
        // NSSI acutely impairs impulse control through negative urgency and emotional dysregulation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11882201/
        impulse_control: -0.35,

        // Disposition - Empathy
        // NSSI episodes create acute self-focus and emotional dysregulation that temporarily reduce perspective-taking
        // https://www.researchgate.net/publication/303553255_Theory_of_mind_in_non-suicidal_self-injury_NSSI_adolescents
        empathy: -0.15,

        // Disposition - Aggression
        // NSSI acutely reduces hostile arousal through emotion regulation and endogenous opioid release
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC4244875/
        aggression: -0.15,

        // Disposition - Grievance
        // NSSI primarily reflects self-directed shame and self-punishment rather than externalized grievance
        // https://acamh.onlinelibrary.wiley.com/doi/abs/10.1111/jcpp.13869
        grievance: -0.15,

        // Disposition - Reactance
        // NSSI provides temporary restoration of autonomy and control over one's body, reducing reactance to perceived constraints
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5736316/
        reactance: -0.12,

        // Disposition - Trust Propensity
        // NSSI reduces trust propensity through shame-driven isolation and attachment disruption
        // https://link.springer.com/article/10.1007/s12144-025-08004-6
        trust_propensity: -0.28,
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
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.08,
        dominance: 0.12,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.12,
        perceived_liability: 0.18,
        self_hate: 0.18,
        perceived_competence: 0.12,
        depression: 0.12,
        self_worth: 0.08,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.12,
        empathy: 0.06,
        aggression: 0.04,
        grievance: 0.04,
        reactance: 0.04,
        trust_propensity: 0.12,
    },
};
