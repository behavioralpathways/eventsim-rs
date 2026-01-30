//! LoseJobFired event specification.
//!
//! Involuntary termination from employment for cause. Being fired carries
//! additional psychological weight beyond layoffs due to shame, blame
//! attribution, and identity threat.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Job loss through termination creates severe negative valence from financial threat, identity disruption, and shame; hedonic adaptation occurs within 1-2 years but blame component delays recovery.
        // Russell, J.A. (1980). A circumplex model of affect. Journal of Personality and Social Psychology, 39(6), 1161-1178.
        valence: -0.55,

        // Mood - Arousal
        // Job termination triggers immediate physiological activation through threat appraisal and sustained stress hormones; acute arousal spike habituates gradually over months.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Posner, J., Russell, J.A., & Peterson, B.S. (2005). The circumplex model of affect.
        arousal: 0.55,

        // Mood - Dominance
        // Being fired imposes involuntary loss of control over employment, reducing dominance significantly; most individuals show substantial recovery within 1-2 years through reemployment.
        // Deci & Ryan (1985) Self-Determination Theory; Seligman's learned helplessness framework.
        dominance: -0.55,

        // Needs - Fatigue
        // Involuntary job termination causes acute mental exhaustion through sustained stress, emotional processing, and rumination; effects typically dissipate within 6-12 months.
        // Baumeister, R.F. et al. (1998). Ego depletion: Is the active self a limited resource?
        fatigue: 0.35,

        // Needs - Stress
        // Termination triggers significant HPA axis activation through acute threat to financial security and identity; involuntary nature and blame attribution amplify stress response.
        // McEwen, B.S. (1998) Stress, adaptation, and disease; Selye, H. (1956) The stress of life.
        stress: 0.58,

        // Needs - Purpose
        // Termination causes significant purpose disruption through role loss and shame-based identity challenge; most individuals rebuild meaning within 12-24 months.
        // Frankl, V.E. (1959). Man's Search for Meaning; Steger, M.F. et al. (2006). The Meaning in Life Questionnaire.
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Job termination creates shame-driven social withdrawal and disrupts daily workplace interactions; above-baseline loneliness with recovery within 12-18 months.
        // Cacioppo, J.T. & Patrick, B. (2008). Loneliness: Human nature and the need for social connection.
        loneliness: 0.38,

        // Social Cognition - PRC
        // Job termination creates moderate perception that others don't care due to shame and social withdrawal; recovery through identity reconstruction and reemployment.
        // Paul, K.I., & Moser, K. (2009). Unemployment impairs mental health: Meta-analyses. Journal of Vocational Behavior, 74(3), 264-282.
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Job termination creates acute perceived liability through immediate financial dependency, shame-based self-blame, and identity threat; recovery upon reemployment.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600.
        perceived_liability: 0.45,

        // Social Cognition - Self Hate
        // Termination for cause triggers significant self-blame through internal attribution of professional failure; recovery occurs within 6-12 months via career reframing.
        // Eisenberger et al. (2003) Does rejection hurt? Science 302(5643): 290-292.
        self_hate: 0.40,

        // Social Cognition - Perceived Competence
        // Termination for cause delivers explicit negative feedback about competence, creating significant acute self-doubt; typically resolves within 6-12 months through employment reentry.
        // Leana, C.R. & Feldman, D.C. (1992). Coping with job loss. Lexington Books.
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Job termination causes significant depressive symptoms (hopelessness, worthlessness, anhedonia); most individuals show substantial recovery within 12 months.
        // Kendler, K.S., et al. (1999). Causal relationship between stressful life events and depression. Psychological Medicine, 29(1), 15-24.
        depression: 0.35,

        // Mental Health - Self Worth
        // Being fired damages core self-worth through role loss, identity threat, and shame-based attribution; most recovery occurs within 6-12 months through reemployment.
        // Crocker & Wolfe (2001) Contingencies of self-worth; Tangney et al. (2002) Shame and self-concept mechanisms.
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Termination creates significant hopelessness through shame-based attribution and employment uncertainty; moderate recovery upon reemployment.
        // Abramson, L.Y., Metalsky, G.I., & Alloy, L.B. (1989). Hopelessness depression. Psychological Review, 96(2), 358-372.
        hopelessness: 0.45,

        // Mental Health - Interpersonal Hopelessness
        // Job termination triggers shame-based social withdrawal and temporary doubt about relational support efficacy; recovery within 6-12 months.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600.
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Job loss creates psychological distress but does not expose individuals to physical pain or death; no habituation effect on acquired capability.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Job termination creates acute stress-induced ego depletion and impaired self-regulation through shame and loss of control; near-complete recovery within 1-2 years.
        // Baumeister, R.F. et al. (1998). Ego depletion: Is the active self a limited resource? Journal of Personality and Social Psychology, 74(5), 1252-1265.
        impulse_control: -0.42,

        // Disposition - Empathy
        // Job termination triggers acute stress and self-focus that temporarily reduces empathic capacity through resource depletion; largely reversible with reemployment.
        // Decety & Jackson (2004). The functional architecture of human empathy. Behavioral and Cognitive Neuroscience Reviews, 3(2), 71-100.
        empathy: -0.15,

        // Disposition - Aggression
        // Termination for cause triggers significant frustration, identity threat, and injustice perception; most recovery within 12-18 months through reemployment or cognitive reframing.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson & Bushman (2002).
        aggression: 0.35,

        // Disposition - Grievance
        // Job termination generates significant perceived injustice due to threats to professional identity and procedural fairness concerns; chronic rumination with recovery in 1-2 years.
        // Lind, E.A. & Tyler, T.R. (1988). The social psychology of procedural justice.
        grievance: 0.65,

        // Disposition - Reactance
        // Job termination imposes involuntary constraint on autonomy and removes freedom to remain employed; significant reactance through loss of control.
        // Brehm, J.W. (1966). A theory of psychological reactance.
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Termination represents institutional betrayal that moderately reduces generalized trust propensity; significant acute effects with recovery within 1-2 years.
        // Mayer, R.C., Davis, J.H., & Schoorman, F.D. (1995). An integrative model of organizational trust. Academy of Management Review, 20(3), 709-734.
        trust_propensity: -0.28,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: false,
        fatigue: true,
        stress: true,
        purpose: true,
        loneliness: false,
        prc: false,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: false,
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
        valence: 0.08,
        arousal: 0.08,
        dominance: 0.06,
        fatigue: 0.06,
        stress: 0.07,
        purpose: 0.06,
        loneliness: 0.08,
        prc: 0.06,
        perceived_liability: 0.10,
        self_hate: 0.08,
        perceived_competence: 0.05,
        depression: 0.05,
        self_worth: 0.08,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.06,
        empathy: 0.04,
        aggression: 0.06,
        grievance: 0.12,
        reactance: 0.08,
        trust_propensity: 0.06,
    },
};
