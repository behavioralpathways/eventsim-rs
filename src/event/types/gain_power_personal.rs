//! GainPowerPersonal event specification.
//!
//! Gaining personal power such as receiving a significant promotion, gaining authority
//! over others, accumulating substantial wealth, becoming an influential figure in one's
//! field, or achieving a position of leadership.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Gaining personal power produces strong positive affect through achievement activation and status elevation, with substantial hedonic adaptation within 1-2 years
        // Keltner, D., Gruenfeld, D. H., & Anderson, C. (2003). Power, approach, and inhibition. Psychological Review, 110(2), 265-284
        valence: 0.55,

        // Mood - Arousal
        // Gaining power generates significant acute arousal through heightened alertness and new responsibilities, though adaptation returns arousal toward baseline within weeks to months
        // Thayer, R.E. (1989). The biopsychology of mood and arousal. Oxford University Press
        arousal: 0.55,

        // Mood - Dominance
        // Gaining personal power significantly increases sense of control and agency through mastery and authority, with moderate lasting effect due to partial hedonic adaptation
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change. Psychological Review, 84(2), 191-215
        dominance: 0.65,

        // Needs - Fatigue
        // Gaining personal power increases mental fatigue through expanded decision-making, social vigilance, and leadership responsibilities, moderating with role adaptation
        // Baumeister, R.F. et al. (1998). Ego depletion: Is the active self a limited resource?
        fatigue: 0.25,

        // Needs - Stress
        // Gaining personal power increases perceived control over outcomes, reducing physiological stress response through decreased threat perception and HPA axis activation
        // Keltner, D., Gruenfeld, D. H., & Anderson, C. (2003). Power, approach, and inhibition. Psychological Review
        stress: -0.25,

        // Needs - Purpose
        // Gaining personal power provides meaningful purpose boost through autonomy and impact opportunity, but hedonic adaptation limits permanence as the new role becomes baseline
        // Frankl, V.E. (1959). Man's Search for Meaning; Ryff, C.D. (1989). Happiness is Everything, or Is It?
        purpose: 0.35,

        // Social Cognition - Loneliness
        // Gaining personal power creates psychological distance through reduced reciprocal caring and authentic connection despite increased social contact
        // Keltner, D., Gruenfeld, D. H., & Anderson, C. (2003). Power, approach, and inhibition. Psychological Review
        loneliness: 0.35,

        // Social Cognition - PRC
        // Gaining personal power creates psychological distance and increases skepticism about authenticity of others' care
        // Keltner, D., Gruenfeld, D. H., & Anderson, C. (2003). Power, approach, and inhibition. Psychological Review
        prc: -0.15,

        // Social Cognition - Perceived Liability
        // Gaining personal power increases perceived contribution capacity and reduces dependency perception, strongly decreasing burden beliefs
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        perceived_liability: -0.65,

        // Social Cognition - Self Hate
        // Gaining personal power provides achievement-based validation and elevated status that reduce self-directed negativity
        // Joiner, T. (2005). Why People Die by Suicide
        self_hate: -0.25,

        // Social Cognition - Perceived Competence
        // Gaining personal power provides a direct mastery experience that significantly increases perceived competence through achievement validation and social feedback
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change
        perceived_competence: 0.35,

        // Mental Health - Depression
        // Gaining personal power produces moderate mood improvement through increased self-efficacy and control, but shows rapid hedonic adaptation within months
        // Brown, G.W. & Harris, T. (1978). Social Origins of Depression
        depression: -0.25,

        // Mental Health - Self Worth
        // Gaining personal power provides social validation and identity reinforcement, creating significant boost to self-worth through elevated status and competence affirmation
        // Zitek & Alexander (2016); Baumeister et al. (2003) on self-esteem and achievement
        self_worth: 0.35,

        // Mental Health - Hopelessness
        // Personal power increases perceived control over future outcomes and creates tangible pathways for agency, directly reducing hopelessness about future possibilities
        // Beck, A.T. et al. (1974). The measurement of pessimism: The Hopelessness Scale
        hopelessness: -0.35,

        // Mental Health - Interpersonal Hopelessness
        // Gaining personal power increases social distance and reduces authentic help-seeking beliefs, though effects partially recover through hedonic adaptation
        // Keltner, D., et al. (2003). Power, approach, and inhibition. Psychological Review
        interpersonal_hopelessness: 0.25,

        // Mental Health - Acquired Capability
        // Gaining personal power involves no habituation to physical pain or death; acquired capability develops exclusively through pain/mortality exposure
        // Joiner, T.E. (2005). Why people die by suicide. Harvard University Press
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Gaining personal power increases approach motivation and reduces inhibitory control, creating behavioral disinhibition
        // Keltner, Gruenfeld & Anderson (2003); Lammers et al. (2010)
        impulse_control: -0.35,

        // Disposition - Empathy
        // Gaining personal power shifts focus inward and reduces empathic accuracy for others' emotional states
        // Keltner, D., Gruenfeld, D. H., & Anderson, C. (2003). Power, approach, and inhibition. Psychological Review
        empathy: -0.35,

        // Disposition - Aggression
        // Gaining personal power increases approach motivation and hostile cognitions through disinhibition, but effect is modest and offset partially by increased perceived control
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC2505101/
        aggression: 0.10,

        // Disposition - Grievance
        // Gaining personal power reduces perceived injustice through increased agency and alignment with just-world beliefs
        // DeCelles et al. (2012); Piff et al. (2012) on power and fairness sensitivity
        grievance: -0.15,

        // Disposition - Reactance
        // Gaining personal power expands autonomy and reduces constraints, decreasing reactance through expanded freedom
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological reactance: A theory of freedom and control
        reactance: -0.35,

        // Disposition - Trust Propensity
        // Gaining personal power reduces trust propensity through increased suspicion of others' motives and diminished perspective-taking
        // Kipnis, D. (1972). Does Power Corrupt, or Does Corruption Produce Power? Journal of Social Issues
        trust_propensity: -0.25,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: true,
        fatigue: true,
        stress: false,
        purpose: false,
        loneliness: true,
        prc: true,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: true,
        depression: false,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: true,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.15,
        arousal: 0.08,
        dominance: 0.25,
        fatigue: 0.08,
        stress: 0.08,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.12,
        perceived_liability: 0.25,
        self_hate: 0.12,
        perceived_competence: 0.35,
        depression: 0.08,
        self_worth: 0.25,
        hopelessness: 0.15,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.25,
        empathy: 0.25,
        aggression: 0.05,
        grievance: 0.05,
        reactance: 0.18,
        trust_propensity: 0.35,
    },
};
