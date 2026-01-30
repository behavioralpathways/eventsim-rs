//! FaceChargesLegal event specification.
//!
//! Being formally charged with a crime in legal proceedings - arraignment,
//! indictment, or formal accusation that initiates criminal prosecution.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Facing criminal charges represents a severe threat to liberty and social standing comparable to major loss events
        // Holmes & Rahe Social Readjustment Rating Scale; Lazarus & Folkman cognitive appraisal theory
        valence: -0.72,

        // Mood - Arousal
        // Facing legal charges produces high activation from threat appraisal and uncertainty with sustained chronic elevation
        // Lazarus & Folkman (1984) stress and coping; criminal justice psychology research
        arousal: 0.65,

        // Mood - Dominance
        // Formal legal charges impose significant loss of control through procedural constraints and uncertain outcomes
        // Tyler, T.R. (2006) procedural justice; Lind & Tyler (1988) control-threat theory
        dominance: -0.58,

        // Needs - Fatigue
        // Criminal charges create severe fatigue through sustained sleep disruption, cognitive demands, and uncertainty-driven rumination
        // Hockey (2013) psychology of fatigue; Baumeister et al. (1998) ego depletion
        fatigue: 0.58,

        // Needs - Stress
        // Criminal charges trigger severe acute stress comparable to major life threats through loss of control and legal/social threat
        // Holmes & Rahe (1967) Social Readjustment Rating Scale - legal troubles rated 78/100
        stress: 0.75,

        // Needs - Purpose
        // Criminal charges disrupt identity and future trajectory through public legal status and extended uncertainty
        // Goffman (1963) stigma and identity; APA criminal justice resources
        purpose: -0.32,

        // Social Cognition - Loneliness
        // Criminal charges trigger acute social stigma, relationship strain, and institutional isolation
        // Maruna (2001) criminal stigma and social reintegration; Joiner (2005) thwarted belongingness
        loneliness: 0.35,

        // Social Cognition - PRC
        // Criminal charges create significant perceived social caring deficit through institutional judgment and stigma-driven distancing
        // Van Orden et al. (2010) ITS; Tangney & Dearing (2002) shame and public judgment
        prc: -0.52,

        // Social Cognition - Perceived Liability
        // Criminal charges impose moderate-to-significant burden perception through legal costs, family strain, and social stigma
        // Joiner's ITS framework; criminal defendants literature on perceived burdensomeness
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Formal criminal charges activate intense shame through institutional judgment and identity contamination
        // Tangney & Dearing (2002) Shame and Guilt
        self_hate: 0.62,

        // Social Cognition - Perceived Competence
        // Criminal charges create significant loss of personal agency in a powerful external system triggering shame-based competence erosion
        // Bandura (1997) Self-efficacy; Tangney (2002) shame and competence
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Facing criminal charges is a significant stressor causing substantial depression comparable to major life failures
        // APA Criminal Justice Mental Health; Brown & Harris (1978) social origins of depression
        depression: 0.35,

        // Mental Health - Self Worth
        // Criminal charges represent major identity threat through public accusation, loss of agency, and sustained legal uncertainty
        // Lind & Tyler (1988) procedural justice; Braithwaite (2000) crime, shame and reintegration
        self_worth: -0.42,

        // Mental Health - Hopelessness
        // Facing criminal charges creates significant hopelessness through loss of control and perceived inescapability of judicial process
        // Abramson et al. (1989) hopelessness depression; Joiner (2005) entrapment theory
        hopelessness: 0.48,

        // Mental Health - Interpersonal Hopelessness
        // Criminal charges trigger severe interpersonal hopelessness through social stigma and belief relationships cannot provide safe support
        // Braithwaite (2000) restorative justice; Maruna & LeBel (2003) reentry frameworks
        interpersonal_hopelessness: 0.50,

        // Mental Health - Acquired Capability
        // Facing legal charges involves psychological distress but no physical pain exposure or death proximity
        // Joiner (2005) Why People Die by Suicide - AC requires physical habituation
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Criminal charges trigger acute prefrontal dysregulation via threat appraisal and sustained cortisol elevation
        // Arnsten (2009) stress signalling and prefrontal function; Schwabe & Wolf (2009) stress and brain
        impulse_control: -0.35,

        // Disposition - Empathy
        // Acute legal threat triggers self-focused stress and cognitive load that mildly reduce empathy through attentional narrowing
        // Folkman & Lazarus (1984) stress-coping; Decety & Lamm (2006) empathy under stress
        empathy: -0.15,

        // Disposition - Aggression
        // Criminal charges trigger significant aggression through perceived injustice, loss of control, and identity threat
        // Berkowitz (1989) frustration-aggression hypothesis; procedural justice theory
        aggression: 0.35,

        // Disposition - Grievance
        // Facing criminal charges produces substantial grievance through institutional injustice and procedural unfairness
        // Lind & Tyler (1988) social psychology of procedural justice; Joiner (2005)
        grievance: 0.58,

        // Disposition - Reactance
        // Being formally charged creates severe externally-imposed constraints on freedom and autonomy
        // Brehm (1966) psychological reactance theory; Wicklund (1974)
        reactance: 0.72,

        // Disposition - Trust Propensity
        // Formal criminal charges represent significant institutional trust violation with spillover to interpersonal trust
        // Tyler (2006) procedural justice and institutional trust; Rousseau et al. (1998)
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
        perceived_liability: true,
        self_hate: true,
        perceived_competence: true,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.20,
        arousal: 0.22,
        dominance: 0.14,
        fatigue: 0.14,
        stress: 0.18,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.16,
        perceived_liability: 0.12,
        self_hate: 0.18,
        perceived_competence: 0.12,
        depression: 0.12,
        self_worth: 0.18,
        hopelessness: 0.18,
        interpersonal_hopelessness: 0.18,
        impulse_control: 0.12,
        empathy: 0.04,
        aggression: 0.12,
        grievance: 0.20,
        reactance: 0.45,
        trust_propensity: 0.12,
    },
};
