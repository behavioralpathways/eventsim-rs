//! LosePersonDeath event specification.
//!
//! Death of a loved one (spouse, child, parent, or close friend). One of the most
//! significant life stressors with profound impacts across psychological dimensions.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Death of a loved one creates catastrophic immediate valence loss (extreme sadness, despair) with partial hedonic adaptation over 1-2 years but measurable permanent baseline shift.
        // Stroebe, M., Schut, H., & Stroebe, W. (2007). Health outcomes of bereavement. The Lancet
        valence: -0.85,

        // Mood - Arousal
        // Death of a loved one triggers acute physiological hyperactivation through trauma response and sustained grief-related stress hormone elevation.
        // Stroebe, M.S., Hansson, R.O., Stroebe, W., & Schut, H. (2007). Handbook of bereavement research and practice. https://doi.org/10.1037/11457-000
        arousal: 0.65,

        // Mood - Dominance
        // Death of a loved one creates near-total loss of control with characteristic acute helplessness that shifts to chronic grief; most show partial recovery within 3-5 years.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5022773/
        dominance: -0.65,

        // Needs - Fatigue
        // Death of a loved one produces severe acute fatigue through grief processing, sleep disruption, and existential rumination.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        fatigue: 0.55,

        // Needs - Stress
        // Death of a loved one triggers severe acute physiological stress response (elevated cortisol, HPA axis activation) comparable to major trauma.
        // https://www.ncbi.nlm.nih.gov/pubmed/26118596
        stress: 0.85,

        // Needs - Purpose
        // Death of a loved one profoundly disrupts life meaning and goals, creating acute existential crisis with moderate permanent baseline shift through grief integration.
        // Frankl, V.E. (1959). Man's Search for Meaning
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Death of a loved one causes severe loneliness through permanent loss of primary relationship and disruption of daily social contact.
        // https://pubmed.ncbi.nlm.nih.gov/20349933/
        loneliness: 0.65,

        // Social Cognition - PRC
        // Death triggers initial social support activation, but subsequent withdrawal creates moderate perception of decreased reciprocal care.
        // Van Orden et al. (2010). Interpersonal Theory of Suicide. https://psycnet.apa.org/record/2010-21215-000
        prc: -0.25,

        // Social Cognition - Perceived Liability
        // Loss of loved one initially activates compassion and support systems, reducing perceived burden temporarily.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600
        perceived_liability: -0.15,

        // Social Cognition - Self Hate
        // Death of loved one triggers significant guilt and regret-based self-blame with "if only" thoughts, creating temporary elevation in self-directed negativity.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC3384440/
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Death of a loved one creates significant temporary loss of competence through role dissolution and cognitive impairment from grief.
        // Bandura, A. (1997). Self-efficacy: The exercise of control
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Death of a loved one triggers severe depressive symptoms including anhedonia and hopelessness, with significant risk for clinical depression.
        // https://doi.org/10.1176/appi.ajp.163.11.1905
        depression: 0.75,

        // Mental Health - Self Worth
        // Death of a loved one creates significant temporary self-worth reduction through acute grief and identity questioning.
        // Stroebe, Schut, & Stroebe (2007) dual process model of grief
        self_worth: -0.25,

        // Mental Health - Hopelessness
        // Death of a loved one induces severe hopelessness about the future due to irreversible loss and shattered life expectations.
        // Beck, A.T. et al. (1974). The measurement of pessimism: The Hopelessness Scale
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Death of loved one causes acute grief and withdrawal, mildly increasing interpersonal hopelessness, but social support systems typically activate.
        // https://www.ncbi.nlm.nih.gov/books/NBK570258/
        interpersonal_hopelessness: 0.15,

        // Mental Health - Acquired Capability
        // Bereavement exposes the bereaved to mortality through loss, providing mild habituation to death conceptually, but without direct trauma or physical pain exposure.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. https://pubmed.ncbi.nlm.nih.gov/20438238/
        acquired_capability: 0.15,

        // Disposition - Impulse Control
        // Death of loved one severely depletes self-regulatory resources through acute grief, stress, and depression, impairing impulse control.
        // Baumeister, R.F. & Muraven, M. (2000). Self-regulation and depletion of limited resources
        impulse_control: -0.45,

        // Disposition - Empathy
        // Death of a loved one causes initial empathy reduction through grief-induced self-focus, but chronic engagement with grief paradoxically deepens emotional understanding.
        // Singer & Klimecki (2014); Decety & Jackson (2004)
        empathy: -0.15,

        // Disposition - Aggression
        // Bereavement triggers temporary anger and irritability during acute grief phase, but this is not the dominant or persistent response.
        // Kubler-Ross, E. (1969). On Death and Dying
        aggression: 0.15,

        // Disposition - Grievance
        // Death generates acute sorrow and temporary anger at loss, but not sustained grievance since there is no human perpetrator.
        // Kubler-Ross, E. (1969). On Death and Dying
        grievance: -0.05,

        // Disposition - Reactance
        // Bereavement creates helplessness and loss of control but not perceived external constraint or resistance to imposed freedom restrictions.
        // Brehm, J.W. (1966). A theory of psychological reactance
        reactance: -0.10,

        // Disposition - Trust Propensity
        // Death of a loved one triggers sustained trust uncertainty due to awareness of life's unpredictability and loss of a key trusted relationship.
        // Rotter, J.B. (1967). A new scale for the measurement of interpersonal trust
        trust_propensity: -0.35,
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
        perceived_liability: false,
        self_hate: true,
        perceived_competence: true,
        depression: true,
        self_worth: false,
        hopelessness: true,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: true,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.25,
        arousal: 0.25,
        dominance: 0.25,
        fatigue: 0.25,
        stress: 0.25,
        purpose: 0.25,
        loneliness: 0.28,
        prc: 0.18,
        perceived_liability: 0.12,
        self_hate: 0.25,
        perceived_competence: 0.18,
        depression: 0.25,
        self_worth: 0.12,
        hopelessness: 0.25,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.18,
        empathy: 0.15,
        aggression: 0.05,
        grievance: 0.02,
        reactance: 0.04,
        trust_propensity: 0.25,
    },
};
