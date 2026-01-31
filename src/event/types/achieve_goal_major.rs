//! AchieveGoalMajor event specification.
//!
//! Achievement of a major life goal such as graduating, completing a significant
//! project, reaching a career milestone, finishing a marathon, or publishing a book.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Major goal achievement produces strong positive valence through control and goal-congruence appraisals, but hedonic adaptation returns individuals to baseline within 6-12 months
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6974350/
        valence: 0.65,

        // Mood - Arousal
        // Major goal achievement produces immediate high arousal through positive activation and accomplishment, but hedonic adaptation rapidly returns arousal toward baseline within 1-2 weeks
        // https://sonjalyubomirsky.com/wp-content/uploads/2024/03/Lyubomirsky-2011.pdf
        arousal: 0.45,

        // Mood - Dominance
        // Major goal achievement creates strong immediate sense of mastery and control, but hedonic adaptation causes the boost to fade within 6-18 months
        // https://positivepsychology.com/hedonic-treadmill/
        dominance: 0.65,

        // Needs - Fatigue
        // Major goal achievement creates temporary moderate fatigue from sustained effort and dopamine depletion post-completion, typically resolving within 1-2 weeks
        // https://www.psychologytoday.com/us/blog/understanding-health-behaviors/202405/post-achievement-depression-overcoming-the-slump
        fatigue: 0.25,

        // Needs - Stress
        // Major goal achievement produces moderate acute stress reduction as sustained pursuit effort ceases and threat/uncertainty diminishes
        // https://sonjalyubomirsky.com/wp-content/uploads/2024/03/Lyubomirsky-2011.pdf
        stress: -0.25,

        // Needs - Purpose
        // Major goal achievement provides significant temporary boost to sense of purpose through demonstrated capability, but hedonic adaptation and arrival fallacy research indicates quick return to baseline
        // https://www.psychologytoday.com/us/blog/mindfulness-insights/202503/the-overlooked-and-misunderstood-arrival-fallacy
        purpose: 0.35,

        // Social Cognition - Loneliness
        // Major achievement often increases temporary social disconnection as the individual's peer group narrows and post-goal emptiness occurs
        // https://www.kaimanapsychology.com/the-loneliness-of-success-navigating-the-isolation-at-the-pinnacle/
        loneliness: 0.15,

        // Social Cognition - PRC
        // Major goal achievement triggers temporary social recognition and validation, signaling others care about your success
        // https://sonjalyubomirsky.com/wp-content/uploads/2024/03/Lyubomirsky-2011.pdf
        prc: 0.25,

        // Social Cognition - Perceived Liability
        // Major goal achievement demonstrably increases competence and perceived contribution, which directly reduce perceived liability
        // https://pubmed.ncbi.nlm.nih.gov/38899722/
        perceived_liability: -0.35,

        // Social Cognition - Self Hate
        // Major goal achievement provides meaningful validation that reduces self-directed negativity through demonstrated competence
        // https://www.nature.com/articles/s41598-025-95821-1
        self_hate: -0.35,

        // Social Cognition - Perceived Competence
        // Major goal achievement represents a mastery experience (Bandura's strongest source of self-efficacy), producing significant immediate competence boost
        // https://www.simplypsychology.org/self-efficacy.html
        perceived_competence: 0.55,

        // Mental Health - Depression
        // Major goal achievement provides significant but temporary mood improvement through accomplishment and agency
        // https://sonjalyubomirsky.com/wp-content/uploads/2024/03/Lyubomirsky-2011.pdf
        depression: -0.35,

        // Mental Health - Self Worth
        // Major goal achievement produces a significant temporary boost to self-worth through validation and competence affirmation
        // https://greatergood.berkeley.edu/images/uploads/The_Challenge_of_Staying_Happier.pdf
        self_worth: 0.35,

        // Mental Health - Hopelessness
        // Achieving a major goal demonstrates capability and opens future possibilities, reducing hopelessness about the future
        // https://link.springer.com/article/10.1007/s41811-023-00165-1
        hopelessness: -0.25,

        // Mental Health - Interpersonal Hopelessness
        // Major goal achievement reduces interpersonal hopelessness through increased self-efficacy and optimism, but the effect is modest
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC2796962/
        interpersonal_hopelessness: -0.15,

        // Mental Health - Acquired Capability
        // Achieving major life goals involves no physical pain exposure or death proximity, producing zero habituation per ITS theory
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5022773/
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Major goal achievement causes temporary behavioral disinhibition and ego depletion as individuals transition to reward-seeking and relaxation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC8182659/
        impulse_control: -0.15,

        // Disposition - Empathy
        // Major goal achievement temporarily increases positive affect and prosocial motivation, but heightened self-focus during celebration creates mild transient reduction
        // https://www.sciencedirect.com/science/article/abs/pii/S0022103120303504
        empathy: 0.05,

        // Disposition - Aggression
        // Achieving a major goal reduces aggression through authentic pride and improved mood state
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC3137237/
        aggression: -0.25,

        // Disposition - Grievance
        // Major goal achievement reinforces just-world beliefs and strengthens perceived personal agency, reducing grievance
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6974350/
        grievance: -0.15,

        // Disposition - Reactance
        // Major goal achievement increases perceived autonomy and control, mildly decreasing reactance through demonstrated agency
        // https://selfdeterminationtheory.org/SDT/documents/2000_DeciRyan_PIWhatWhy.pdf
        reactance: -0.15,

        // Disposition - Trust Propensity
        // Major goal achievement produces positive emotions that broaden social cognition and increase willingness to engage trustingly with others
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC1693418/
        trust_propensity: 0.18,
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
        perceived_competence: true,
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
        valence: 0.10,
        arousal: 0.04,
        dominance: 0.12,
        fatigue: 0.02,
        stress: 0.04,
        purpose: 0.08,
        loneliness: 0.04,
        prc: 0.05,
        perceived_liability: 0.10,
        self_hate: 0.08,
        perceived_competence: 0.25,
        depression: 0.08,
        self_worth: 0.12,
        hopelessness: 0.15,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.03,
        empathy: 0.02,
        aggression: 0.12,
        grievance: 0.04,
        reactance: 0.05,
        trust_propensity: 0.06,
    },
};
