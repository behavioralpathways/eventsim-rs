//! ExperienceExclusionGroup event specification.
//!
//! Being excluded from a group - social rejection, ostracism, being deliberately
//! left out, not being invited, or being kicked out of a social group. This
//! activates the same neural pain circuits as physical pain and threatens
//! fundamental belonging needs.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Group exclusion triggers moderate-to-severe negative valence through belonging need violation; social pain research shows comparable neural activation to physical pain
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC1847099/
        valence: -0.45,

        // Mood - Arousal
        // Group exclusion triggers acute psychological distress and activates stress response systems, producing moderate-to-high arousal through threat perception
        // https://psycnet.apa.org/doi/10.1037/0022-3514.88.4.589
        arousal: 0.45,

        // Mood - Dominance
        // Group exclusion imposes loss of agency and social control without the person's input, producing significant reduction in dominance sense
        // Baumeister & Leary (1995) - The need to belong and social exclusion research
        dominance: -0.55,

        // Needs - Fatigue
        // Social exclusion causes acute mental fatigue through emotional processing and rumination, but most individuals show hedonic adaptation within 1-2 weeks
        // https://doi.org/10.1037/0022-3514.74.5.1252
        fatigue: 0.35,

        // Needs - Stress
        // Group exclusion triggers measurable physiological stress response through social threat perception and loss of control
        // Eisenberger social exclusion fMRI research
        stress: 0.55,

        // Needs - Purpose
        // Group exclusion directly undermines purpose by removing meaningful social roles and contribution opportunities
        // https://doi.org/10.1177/1088868317696228
        purpose: -0.30,

        // Social Cognition - Loneliness
        // Group exclusion creates significant immediate loneliness through disrupted social connection and perceived rejection
        // Cacioppo & Patrick (2008); Williams ostracism research
        loneliness: 0.45,

        // Social Cognition - PRC
        // Social exclusion communicates that others do not care about you, significantly decreasing perceived reciprocal caring
        // Baumeister & Leary (1995) - The need to belong
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Group exclusion creates moderate perceived liability by signaling unworthiness ("we're better off without you")
        // https://doi.org/10.1016/S0140-6736(11)61141-X
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Social exclusion triggers meaningful self-blame and internalized shame, attributed to social mismatch rather than deep moral failure
        // Williams & Zadro (2005) - Ostracism: The indiscriminate early detection system
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Group exclusion damages social competence perception and general self-worth but does not directly undermine domain-specific competence
        // Baumeister & Leary (1995) - The need to belong
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Social exclusion triggers significant depressive symptoms through loss of belonging and self-esteem threat
        // Brown & Harris (1978); Kendler et al. (1999) social exclusion research
        depression: 0.35,

        // Mental Health - Self Worth
        // Group exclusion produces significant self-worth damage through fundamental rejection and identity-relevant threat
        // Williams ostracism research on self-esteem
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Group exclusion creates significant acute hopelessness about social future through thwarted belongingness
        // Joiner (2005) - Why People Die by Suicide; Williams ostracism research
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Group exclusion creates moderate interpersonal hopelessness by signaling lack of social value and undermining beliefs about relational support
        // https://journals.sagepub.com/doi/abs/10.1111/j.1467-9280.2005.01621.x
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Social exclusion causes emotional pain but does not expose individuals to physical pain or death, so it does not increase habituation to pain
        // Joiner (2005) - Why People Die by Suicide; AC requires physical pain/death habituation
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Social exclusion triggers acute self-regulatory depletion and emotional distress, impairing impulse control significantly but temporarily
        // https://doi.org/10.1037/0022-3514.88.4.589
        impulse_control: -0.35,

        // Disposition - Empathy
        // Social exclusion triggers temporary self-focused stress response that moderately reduces perspective-taking and emotional understanding
        // Davis (1983); Decety & Jackson (2004)
        empathy: -0.18,

        // Disposition - Aggression
        // Social exclusion triggers frustration and identity threat, significantly increasing aggression through goal blockage and threatened social standing
        // Twenge et al.; Berkowitz (1989) frustration-aggression hypothesis
        aggression: 0.35,

        // Disposition - Grievance
        // Group exclusion triggers strong perceived injustice (being deliberately left out feels unfair) and activates victimization narratives
        // Williams (2007); Baumeister & Leary (1995) ostracism research
        grievance: 0.42,

        // Disposition - Reactance
        // Group exclusion restricts social autonomy and group participation rights, triggering significant reactance to the imposed freedom constraint
        // Brehm (1966) - A theory of psychological reactance
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Group exclusion increases threat perception and social vigilance while demonstrating that others reject you, moderately decreasing trust propensity
        // https://doi.org/10.1016/S0065-2601(02)80006-5
        trust_propensity: -0.35,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: true,
        loneliness: true,
        prc: false,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.10,
        arousal: 0.06,
        dominance: 0.06,
        fatigue: 0.05,
        stress: 0.06,
        purpose: 0.08,
        loneliness: 0.08,
        prc: 0.08,
        perceived_liability: 0.06,
        self_hate: 0.07,
        perceived_competence: 0.05,
        depression: 0.06,
        self_worth: 0.12,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.05,
        empathy: 0.04,
        aggression: 0.08,
        grievance: 0.12,
        reactance: 0.12,
        trust_propensity: 0.12,
    },
};
