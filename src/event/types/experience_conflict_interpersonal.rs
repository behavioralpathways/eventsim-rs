//! ExperienceConflictInterpersonal event specification.
//!
//! Experiencing interpersonal conflict - arguments, disagreements, confrontations,
//! or hostile exchanges with another person. This includes workplace conflicts,
//! conflicts with friends, neighbors, acquaintances, or strangers (distinct from
//! family-specific conflict which is a separate event type).

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Interpersonal conflict creates moderate negative valence through acute social stress and threat perception, but with less intensity than family conflict due to lower attachment significance.
        // https://psycnet.apa.org/record/1994-28559-001 (Davies & Cummings 1994)
        valence: -0.25,

        // Mood - Arousal
        // Interpersonal conflict activates the sympathetic nervous system through acute stress response, producing high arousal that typically recovers within 24-48 hours.
        // https://psycnet.apa.org/doi/10.1037/0022-3514.88.4.644
        arousal: 0.55,

        // Mood - Dominance
        // Interpersonal conflict creates moderate, temporary loss of control and agency as the person is in a contested situation with uncertain outcomes negotiated with another party.
        // Deci & Ryan (1985) Self-Determination Theory; Bandura (1977) Self-Efficacy Theory
        dominance: -0.32,

        // Needs - Fatigue
        // Interpersonal conflict causes moderate acute mental fatigue through emotional labor and rumination, but recovery is typical within days due to hedonic adaptation.
        // Baumeister, R.F. et al. (1998) Ego Depletion research
        fatigue: 0.35,

        // Needs - Stress
        // Interpersonal conflict triggers acute HPA axis activation and threat perception, producing significant immediate stress, but physiological markers normalize within hours-to-days.
        // McEwen, B.S. (1998) Stress, adaptation, and disease. Annals of the New York Academy of Sciences
        stress: 0.45,

        // Needs - Purpose
        // Interpersonal conflict causes temporary self-doubt and questions about relationship meaning, but rarely disrupts core life purpose; effects resolve quickly through emotional adaptation.
        // Steger, M.F. et al. (2006) The Meaning in Life Questionnaire
        purpose: -0.12,

        // Social Cognition - Loneliness
        // Interpersonal conflict temporarily increases perceived social distance and disconnection, elevating loneliness during and immediately after the conflict episode.
        // Joiner, T. (2005) Why People Die by Suicide; Cacioppo & Patrick (2008)
        loneliness: 0.25,

        // Social Cognition - PRC
        // Interpersonal conflict temporarily reduces perceived relationship caring through signal of emotional distance and lack of support.
        // https://www.tandfonline.com/doi/abs/10.1080/15298860590950146 (Joiner 2005)
        prc: -0.25,

        // Social Cognition - Perceived Liability
        // Interpersonal conflict triggers moderate temporary self-blame about causing problems for others, particularly in work/social contexts.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC2913990/ (Van Orden et al. on ITS)
        perceived_liability: 0.18,

        // Social Cognition - Self Hate
        // Interpersonal conflict triggers self-blame and shame responses in most individuals, but recovery is typically rapid unless reflecting core relational failure.
        // https://doi.org/10.1521/ijsp.2010.50.3.307 (Van Orden et al., 2010)
        self_hate: 0.30,

        // Social Cognition - Perceived Competence
        // Interpersonal conflict creates mild self-doubt about social competence through perceived communication ineffectiveness, but resolves quickly without lasting base shift.
        // Bandura, A. (1997) Self-efficacy: The exercise of control
        perceived_competence: -0.15,

        // Mental Health - Depression
        // Interpersonal conflict triggers moderate depressive symptoms through rumination and threat to social connection, but effects are largely temporary with natural recovery.
        // Brown, G.W. & Harris, T. (1978) Social origins of depression
        depression: 0.28,

        // Mental Health - Self Worth
        // Interpersonal conflict produces mild to moderate self-worth reduction through social rejection signals and integrity questioning, with recovery within weeks.
        // Crocker, J. & Wolfe, C.T. (2001) Contingencies of self-worth. Psychological Review
        self_worth: -0.15,

        // Mental Health - Hopelessness
        // Interpersonal conflict temporarily triggers negative expectations about specific relationship or situation, but effect is usually confined to the conflict domain.
        // Beck, A.T. et al. (1974) The Hopelessness Scale
        hopelessness: 0.15,

        // Mental Health - Interpersonal Hopelessness
        // Interpersonal conflict moderately increases hopelessness about relationships by revealing relationship fragility and triggering beliefs that communication won't help.
        // Van Orden, K. et al. (2010) The Interpersonal Theory of Suicide. Psychological Review
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Interpersonal conflict involves emotional stress and arousal threat but not physical pain habituation or death exposure required for acquired capability per Joiner's theory.
        // Joiner, T. (2005) Why People Die by Suicide; Van Orden et al. (2010) https://doi.org/10.1037/a0018697
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Interpersonal conflict depletes self-regulatory capacity through stress and cognitive load, with less severe impact than family conflict due to less foundational threat.
        // https://psycnet.apa.org/record/1994-28559-001; Baumeister et al. (1998) ego depletion
        impulse_control: -0.25,

        // Disposition - Empathy
        // Interpersonal conflict temporarily reduces empathy through stress and self-focus, but most individuals show near-complete recovery within 24-48 hours.
        // Singer, T. & Klimecki, O.M. (2014) Empathy and compassion. Current Biology
        empathy: -0.15,

        // Disposition - Aggression
        // Interpersonal conflict triggers frustration and ego threat, producing significant state aggression elevation per the frustration-aggression hypothesis.
        // Berkowitz, L. (1989) Frustration-Aggression Hypothesis. Psychological Bulletin
        aggression: 0.32,

        // Disposition - Grievance
        // Unresolved interpersonal conflict creates moderate sense of injustice and being wronged, but most people adapt over time with modest permanent shift.
        // Lind, E.A. & Tyler, T.R. (1988) The social psychology of procedural justice
        grievance: 0.32,

        // Disposition - Reactance
        // Interpersonal conflict creates acute reactance through autonomy threat and pressure to conform, but less severe than family conflict due to ability to disengage.
        // Brehm, J.W. (1966) A Theory of Psychological Reactance
        reactance: 0.22,

        // Disposition - Trust Propensity
        // A single interpersonal conflict produces mild trust reduction by signaling potential untrustworthiness, but most recovery occurs through continued positive interactions.
        // Rotter, J.B. (1967) Interpersonal Trust Scale research
        trust_propensity: -0.15,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: false,
        loneliness: false,
        prc: false,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: false,
        hopelessness: false,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.05,
        arousal: 0.08,
        dominance: 0.04,
        fatigue: 0.04,
        stress: 0.05,
        purpose: 0.03,
        loneliness: 0.05,
        prc: 0.05,
        perceived_liability: 0.05,
        self_hate: 0.06,
        perceived_competence: 0.04,
        depression: 0.06,
        self_worth: 0.05,
        hopelessness: 0.04,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.06,
        empathy: 0.04,
        aggression: 0.06,
        grievance: 0.12,
        reactance: 0.06,
        trust_propensity: 0.05,
    },
};
