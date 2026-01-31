//! ExperienceAwarenessMortality event specification.
//!
//! Becoming acutely aware of one's own mortality through a near-death experience,
//! serious health scare, witnessing death, reaching a milestone age, or existential
//! contemplation that brings mortality salience to the forefront of consciousness.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Mortality awareness triggers acute fear and anxiety with strong negative valence, but psychological defense mechanisms progressively suppress these emotions
        // https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0248699
        valence: -0.55,

        // Mood - Arousal
        // Mortality awareness triggers acute anxiety-driven physiological arousal through existential threat perception, but effects are temporary as psychological defense mechanisms engage
        // https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2019.01893/full
        arousal: 0.55,

        // Mood - Dominance
        // Mortality awareness acutely reduces perceived control through existential anxiety and helplessness, but meaning-making facilitates partial recovery
        // https://pubmed.ncbi.nlm.nih.gov/18729692/
        dominance: -0.55,

        // Needs - Fatigue
        // Mortality awareness causes acute emotional exhaustion through anxiety and cognitive resource depletion from suppressing death-related thoughts
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC7968674/
        fatigue: 0.35,

        // Needs - Stress
        // Mortality awareness triggers acute physiological stress comparable to significant threats, but most individuals show substantial adaptation within weeks to months
        // https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2019.01893/full
        stress: 0.65,

        // Needs - Purpose
        // Mortality awareness activates motivated search for meaning and triggers sustained reorientation toward intrinsic values and life purpose
        // https://www.tandfonline.com/doi/full/10.1080/07481187.2020.1808737
        purpose: 0.20,

        // Social Cognition - Loneliness
        // Mortality awareness typically motivates people to seek connection and deepen relationships as an existential coping mechanism, reducing loneliness
        // https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2023.1190906/full
        loneliness: -0.22,

        // Social Cognition - PRC
        // Mortality awareness activates heightened relational strivings and appreciation for close relationships, increasing perceived value of social bonds
        // https://www.sciencedirect.com/science/article/abs/pii/S0191886914001718
        prc: 0.28,

        // Social Cognition - Perceived Liability
        // Mortality awareness triggers temporary existential anxiety and mild burden perception, but resolves quickly via hedonic adaptation
        // https://pubmed.ncbi.nlm.nih.gov/9008372/
        perceived_liability: 0.15,

        // Social Cognition - Self Hate
        // Mortality salience produces mild self-directed negative cognition through existential anxiety, but heavily buffered by baseline self-esteem
        // https://pubmed.ncbi.nlm.nih.gov/9008372/
        self_hate: 0.15,

        // Social Cognition - Perceived Competence
        // Acute mortality awareness temporarily reduces perceived control and creates existential doubt about competence, but protective mechanisms engage
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC9756974/
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Mortality salience produces significant but transient depressive responses through existential rumination and death anxiety
        // https://pubmed.ncbi.nlm.nih.gov/37416947/
        depression: 0.25,

        // Mental Health - Self Worth
        // Mortality awareness triggers existential anxiety buffered by self-esteem; most individuals achieve modest self-worth gains through meaning-making
        // https://pubmed.ncbi.nlm.nih.gov/9008372/
        self_worth: 0.15,

        // Mental Health - Hopelessness
        // Mortality awareness triggers temporary existential anxiety and mild hopelessness, but typically catalyzes meaning-making and adaptive reframing
        // https://journals.sagepub.com/doi/10.1177/0146167296221008
        hopelessness: 0.15,

        // Mental Health - Interpersonal Hopelessness
        // Acute mortality awareness typically increases belief in relational support value through existential meaning-seeking and attachment strengthening
        // https://www.frontiersin.org/journals/psychiatry/articles/10.3389/fpsyt.2025.1507212/full
        interpersonal_hopelessness: -0.15,

        // Mental Health - Acquired Capability
        // Mortality awareness reduces death anxiety through existential acceptance, not pain habituation; AC requires repeated physical pain exposure per Joiner's model
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5022773/
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Mortality salience acutely depletes self-control resources through anxiety suppression and cognitive load
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11673648/
        impulse_control: -0.35,

        // Disposition - Empathy
        // Mortality salience increases prosocial behavior and compassion through meaning-seeking, with near-death experiences showing sustained increases in empathy
        // https://www.frontiersin.org/journals/psychiatry/articles/10.3389/fpsyt.2025.1507212/full
        empathy: 0.18,

        // Disposition - Aggression
        // Mortality awareness triggers defensive worldview hostility and aggression against perceived threats, but effects are temporary
        // https://pubmed.ncbi.nlm.nih.gov/9523407/
        aggression: 0.25,

        // Disposition - Grievance
        // Mortality awareness prompts defensive restoration of just-world beliefs and rejection of victimization narratives, causing temporary reduction in grievance
        // https://journals.sagepub.com/doi/10.1177/1088868309352321
        grievance: -0.15,

        // Disposition - Reactance
        // Mortality awareness reduces reactance as death anxiety redirects focus from autonomy threats to existential concerns
        // https://pubmed.ncbi.nlm.nih.gov/38098213/
        reactance: -0.15,

        // Disposition - Trust Propensity
        // Mortality salience increases motivation for interpersonal closeness and relational strivings, producing modest increase in trust propensity
        // https://journals.sagepub.com/doi/abs/10.1177/1088868317753505
        trust_propensity: 0.15,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: true,
        fatigue: true,
        stress: false,
        purpose: true,
        loneliness: false,
        prc: false,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: true,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.12,
        arousal: 0.08,
        dominance: 0.12,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.12,
        loneliness: 0.04,
        prc: 0.12,
        perceived_liability: 0.06,
        self_hate: 0.04,
        perceived_competence: 0.05,
        depression: 0.05,
        self_worth: 0.12,
        hopelessness: 0.05,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.12,
        empathy: 0.25,
        aggression: 0.08,
        grievance: 0.05,
        reactance: 0.08,
        trust_propensity: 0.12,
    },
};
