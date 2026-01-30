//! EndRelationshipRomantic event specification.
//!
//! End of a romantic relationship (breakup, divorce). The relationship
//! terminates, regardless of who initiated it.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Romantic breakups produce severe but temporary negative affect that typically resolves within 3 months
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        valence: -0.55,

        // Mood - Arousal
        // Breakups trigger significant sympathetic nervous system activation with recovery in 3-6 months
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        arousal: 0.55,

        // Mood - Dominance
        // Moderate temporary loss of control, individuals typically recover and exceed baseline within 1-2 years
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC9348722/
        dominance: -0.35,

        // Needs - Fatigue
        // Moderate acute fatigue through sleep disruption, rumination, and emotional processing stress
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11985774/
        fatigue: 0.35,

        // Needs - Stress
        // Significant HPA axis activation and elevated cortisol lasting weeks to months
        // https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0217320
        stress: 0.50,

        // Needs - Purpose
        // Significant temporary disruption through loss of shared life narrative and identity
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6051550/
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Significant temporary increase through loss of primary relationship and daily social contact
        // https://pubmed.ncbi.nlm.nih.gov/30605909/
        loneliness: 0.35,

        // Social Cognition - PRC
        // Rejection and loss of primary caring relationship significantly reduces perception of being cared for
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC8957618/
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Temporary mild burden perception through loss of identity and rejection sensitivity
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11985774/
        perceived_liability: 0.15,

        // Social Cognition - Self Hate
        // Moderate self-blame and rumination, most individuals recover within 1-2 years
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC10727987/
        self_hate: 0.32,

        // Social Cognition - Perceived Competence
        // Initial self-concept disruption with recovery driven by autonomy discovery
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC9348722/
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Significant but transient depressive symptoms, most return to baseline within 3 months
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        depression: 0.35,

        // Mental Health - Self Worth
        // Significant but temporary reduction due to rejection and loss of identity validation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11985774/
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Moderate temporary pessimism about future relationships, recovery within 3 months
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        hopelessness: 0.28,

        // Mental Health - Interpersonal Hopelessness
        // Moderate interpersonal hopelessness through trust erosion, reversed through cognitive processing
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6051550/
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // No physical pain or death exposure in typical romantic breakup
        // Joiner (2005), Van Orden et al. (2010)
        acquired_capability: 0.00,

        // Disposition - Impulse Control
        // Acute ego depletion and impaired self-regulation through emotional distress
        // https://ubwp.buffalo.edu/selfandmotivationlab/wp-content/uploads/sites/91/2018/05/Park-Sanchez-Bryndilsen-2011-JASP-1.pdf
        impulse_control: -0.35,

        // Disposition - Empathy
        // Temporary increase in self-focus reduces perspective-taking capacity
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11985774/
        empathy: -0.15,

        // Disposition - Aggression
        // Frustration-aggression response through blocked relationship goals, peaks at weeks 2-6
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC4385442/
        aggression: 0.35,

        // Disposition - Grievance
        // Non-initiators experience moderate perceived injustice with substantial recovery
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC6051550/
        grievance: 0.35,

        // Disposition - Reactance
        // Imposed dissolution creates moderate reactance, initiators show minimal effect
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC9348722/
        reactance: 0.25,

        // Disposition - Trust Propensity
        // Moderate reduction in generalized trust, dyadic trust damage more severe
        // https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2023.1260480/full
        trust_propensity: -0.25,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: true,
        purpose: false,
        loneliness: false,
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
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.05,
        arousal: 0.05,
        dominance: 0.05,
        fatigue: 0.05,
        stress: 0.06,
        purpose: 0.06,
        loneliness: 0.05,
        prc: 0.06,
        perceived_liability: 0.06,
        self_hate: 0.06,
        perceived_competence: 0.05,
        depression: 0.05,
        self_worth: 0.07,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.06,
        impulse_control: 0.05,
        empathy: 0.04,
        aggression: 0.05,
        grievance: 0.05,
        reactance: 0.05,
        trust_propensity: 0.06,
    },
};
