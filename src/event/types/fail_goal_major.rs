//! FailGoalMajor event specification.
//!
//! Failure to achieve a significant life goal such as failing an important exam,
//! not getting a promotion, failing to complete a major project, or falling short
//! of a major personal milestone.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Major goal failure produces strong immediate negative valence through disappointment and self-doubt, with most recovery within 3-6 months despite some lasting impact on self-efficacy beliefs.
        // Carver & Scheier (1998) goal discrepancy theory; Watson & Tellegen (1985) affect structure
        valence: -0.50,

        // Mood - Arousal
        // Major goal failure triggers significant fight-or-flight arousal through acute stress response, with prolonged elevation if goal was highly valued, but shows substantial recovery within weeks to months.
        // Russell, J.A. (1980) Circumplex model of affect - https://doi.org/10.1037/0022-3514.39.4.521
        arousal: 0.55,

        // Mood - Dominance
        // Major goal failure creates significant but temporary loss of perceived control and mastery; dominance recovers as individual processes failure and pursues alternative pathways.
        // Seligman (1975) Learned helplessness; Bandura (1977) Self-efficacy theory
        dominance: -0.45,

        // Needs - Fatigue
        // Major goal failure triggers moderate mental and emotional exhaustion from stress response and rumination, but shows substantial recovery within weeks as individuals adapt and develop coping strategies.
        // Baumeister et al. (1998) Ego depletion - Journal of Personality and Social Psychology, 74(5), 1252-1265
        fatigue: 0.35,

        // Needs - Stress
        // Major goal failure triggers moderate-to-high physiological stress response through threat perception and loss of control, but shows near-complete recovery within 1-2 weeks as individuals reappraise the event and activate coping strategies.
        // Lazarus & Folkman (1984) Stress, Appraisal, and Coping; McEwen (1998) Stress, adaptation, and disease
        stress: 0.52,

        // Needs - Purpose
        // Major goal failure significantly disrupts sense of meaning and life direction, triggering existential questioning with slower recovery than achievement adaptation; chronic classification reflects sustained purpose-related rumination.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6974350/
        purpose: -0.38,

        // Social Cognition - Loneliness
        // Major goal failure triggers shame-based social withdrawal and reduced engagement with peer groups who witnessed the failure, producing significant but temporary loneliness with moderate permanence as individuals rebuild connections.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC2796962/; Baumeister & Leary (1995) The need to belong
        loneliness: 0.22,

        // Social Cognition - PRC
        // Major goal failure triggers shame-based social withdrawal and perception that others do not care about supporting you, moderately reducing PRC, with most recovery within 6-12 months via hedonic adaptation.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6974350/
        prc: -0.30,

        // Social Cognition - Perceived Liability
        // Major goal failure creates perceived incompetence and potential financial/social consequences, increasing perceived liability moderately through internal attribution and temporary dependency on others for support.
        // Van Orden et al. on Perceived Burdensomeness in ITS - https://pubmed.ncbi.nlm.nih.gov/38899722/
        perceived_liability: 0.28,

        // Social Cognition - Self Hate
        // Goal failure triggers moderate self-blame and competence doubt, but lacks the public witnessing that amplifies shame in humiliation, with typical hedonic recovery within 6-12 months.
        // https://sonjalyubomirsky.com/wp-content/uploads/2024/03/Lyubomirsky-2011.pdf
        self_hate: 0.28,

        // Social Cognition - Perceived Competence
        // Major goal failure provides strong direct evidence of inability in an important domain, significantly undermining perceived competence through negative mastery experience, but most recovery occurs within 1-2 years.
        // Bandura, A. (1997) Self-efficacy: The exercise of control
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Major goal failure triggers moderate-to-significant depressive symptoms through blocked goals, identity threat, and rumination, comparable to other significant losses, with substantial recovery within 3-6 months.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        depression: 0.38,

        // Mental Health - Self Worth
        // Major goal failure damages core self-evaluation through identity threat and internal attribution patterns, with moderate permanent baseline shift despite emotional recovery.
        // Crocker & Wolfe (2001) Contingencies of self-worth - https://www.researchgate.net/publication/232455484
        self_worth: -0.30,

        // Mental Health - Hopelessness
        // Major goal failure produces significant temporary hopelessness about future outcomes, but high recoverability through subsequent efforts and hedonic adaptation typically limits permanent shifts.
        // Beck et al. (1974) The measurement of pessimism: The Hopelessness Scale
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Major goal failure produces shame-based withdrawal from help-seeking and temporary doubt about social support effectiveness, but without direct interpersonal betrayal, most recover through alternative relationships.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC2796962/; https://pmc.ncbi.nlm.nih.gov/articles/PMC5022773/
        interpersonal_hopelessness: 0.25,

        // Mental Health - Acquired Capability
        // Major goal failure causes emotional distress and affects belongingness/burdensomeness dimensions, not pain habituation or death exposure required for acquired capability development.
        // Joiner (2005) Why People Die by Suicide - ITS theory requires physical habituation
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Major goal failure acutely depletes self-regulatory resources and increases emotional distress, significantly impairing impulse control for days to weeks, but showing near-complete recovery with adaptive coping.
        // Baumeister et al. (1998) on ego depletion; Muraven & Baumeister (2000) self-regulation resource depletion
        impulse_control: -0.35,

        // Disposition - Empathy
        // Major goal failure triggers temporary self-focused attention and stress that reduce empathic capacity, but effects are largely reversible through normal hedonic adaptation.
        // Singer & Klimecki (2014) on empathy under stress - https://doi.org/10.1016/j.neubiorev.2014.08.006
        empathy: -0.18,

        // Disposition - Aggression
        // Major goal failure triggers moderate frustration-based aggression through blocked goals and ego threat, but most individuals show substantial recovery within weeks due to hedonic adaptation and reframing.
        // Berkowitz (1989) frustration-aggression hypothesis; Anderson & Bushman (2002)
        aggression: 0.35,

        // Disposition - Grievance
        // Major goal failure can trigger moderate grievance when attributed to external unfairness or systemic barriers, but most people show hedonic adaptation within a year unless sustained by attributional biases.
        // Weiner (1985) attributional theory of achievement motivation - Psychological Review, 92(4), 548-573
        grievance: 0.25,

        // Disposition - Reactance
        // Major goal failure creates temporary frustration and possible perceived loss of control, but does not directly threaten autonomy or freedom unless attributed to external constraint.
        // Brehm (1966) A Theory of Psychological Reactance; Brehm & Brehm (1981)
        reactance: 0.05,

        // Disposition - Trust Propensity
        // Goal failure primarily affects self-efficacy and mood rather than interpersonal trust disposition; indirect effects are temporary as people distinguish personal setbacks from others' trustworthiness.
        // Bandura's work on failure and self-efficacy distinction from trust
        trust_propensity: -0.12,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: true,
        loneliness: true,
        prc: false,
        perceived_liability: true,
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
        valence: 0.08,
        arousal: 0.08,
        dominance: 0.08,
        fatigue: 0.04,
        stress: 0.06,
        purpose: 0.12,
        loneliness: 0.07,
        prc: 0.06,
        perceived_liability: 0.09,
        self_hate: 0.06,
        perceived_competence: 0.07,
        depression: 0.06,
        self_worth: 0.12,
        hopelessness: 0.08,
        interpersonal_hopelessness: 0.06,
        impulse_control: 0.05,
        empathy: 0.04,
        aggression: 0.05,
        grievance: 0.06,
        reactance: 0.01,
        trust_propensity: 0.03,
    },
};
