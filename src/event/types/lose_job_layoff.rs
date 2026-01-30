//! LoseJobLayoff event specification.
//!
//! Involuntary job loss due to organizational downsizing, restructuring, or economic
//! conditions. Unlike being fired for cause, layoffs carry external attribution
//! (not personal failure), reducing shame while maintaining financial threat.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Layoffs create significant negative valence from financial threat and identity disruption, but external attribution reduces shame compared to termination; hedonic adaptation occurs within 1-2 years.
        // Lucas, R.E., & Donnellan, M.B. (2007). How stable is happiness? Journal of Research in Personality; Paul, K.I., & Moser, K. (2009). Unemployment impairs mental health. Journal of Vocational Behavior.
        valence: -0.45,

        // Mood - Arousal
        // Job layoffs trigger substantial physiological activation through financial threat and uncertainty, but lack the shame-based amplification of termination-for-cause, resulting in moderately-high arousal with faster habituation.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Paul, K.I., & Moser, K. (2009). Unemployment impairs mental health. Journal of Vocational Behavior.
        arousal: 0.48,

        // Mood - Dominance
        // Job layoffs impose significant loss of control and agency over employment status and financial security; however, recovery occurs with reemployment within months to a year.
        // Bandura (1977) self-efficacy theory; Deci & Ryan (1985) self-determination theory regarding autonomy as core psychological need.
        dominance: -0.55,

        // Needs - Fatigue
        // Job layoffs trigger moderate fatigue through sleep disruption, emotional processing, and sustained rumination about identity and future security; most recovery occurs within 12-18 months with reemployment.
        // Sonnentag & Zijlstra (2006) on job stress and recovery mechanisms; Hockey (2013) on sustained effort during uncertain recovery periods.
        fatigue: 0.45,

        // Needs - Stress
        // Job layoff is a major financial and identity threat that acutely activates the HPA axis through unpredictability and loss of control, with sustained elevation during unemployment period before substantial recovery upon reemployment.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load.
        stress: 0.58,

        // Needs - Purpose
        // Layoffs create significant acute purpose disruption through role loss and identity displacement, but most individuals recover purpose through reemployment or alternative meaning-making within 1-2 years.
        // Steger, M.F. et al. (2006). The Meaning in Life Questionnaire; Jahoda (1982) on psychological deprivation of unemployment.
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Job layoffs disrupt workplace social networks and daily interpersonal interactions without the shame-based withdrawal of being fired, producing moderate temporary loneliness with substantial recovery through reemployment.
        // Cacioppo, J.T. & Patrick, B. (2008). Loneliness: Human Nature and the Need for Social Connection.
        loneliness: 0.32,

        // Social Cognition - PRC
        // Layoff signals organizational non-prioritization of individual wellbeing but lacks personal rejection; triggering temporary perceived care deficit recoverable through family/community support.
        // Baumeister & Leary (1995) The need to belong; Joiner (2005) ITS framework on rejection as mediator of belongingness.
        prc: -0.25,

        // Social Cognition - Perceived Liability
        // Job layoffs create significant financial dependency and loss of productive role, moderately increasing perceived liability; most individuals show hedonic adaptation within 1-2 years post-reemployment.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review.
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Layoffs trigger mild self-blame and identity disruption, but external attribution (economic downsizing) limits internalized self-hate compared to being fired; most recovery occurs within 12 months post-reemployment.
        // Van Orden et al. (2010); Joiner (2005) - layoffs characterized as external circumstance vs. personal failure.
        self_hate: 0.15,

        // Social Cognition - Perceived Competence
        // Layoff creates moderate competence doubt due to future uncertainty and employment vulnerability, but external attribution (not personal failure) prevents major damage; effects are temporary with recovery within months.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Kinicki et al. (2000). Journal of Applied Psychology.
        perceived_competence: -0.28,

        // Mental Health - Depression
        // Job layoffs constitute a significant loss event triggering depressive symptoms (hopelessness, anhedonia, identity disruption) with moderate acute impact and chronic features during unemployment, but substantial recovery within 6-18 months with adequate support.
        // Kendler, K.S. et al. (1999). Causal relationship between stressful life events and depression; Brown & Harris (1978). Social origins of depression.
        depression: 0.35,

        // Mental Health - Self Worth
        // Layoffs trigger temporary identity threat and loss of social status, but external attribution (economic conditions, company restructuring) prevents the deep self-worth collapse seen in blame-based dismissals.
        // Crocker & Wolfe (2001). Contingencies of self-worth; Winkelmann & Winkelmann (1998). Why are the unemployed so unhappy?
        self_worth: -0.25,

        // Mental Health - Hopelessness
        // Job layoffs create moderate hopelessness through financial threat and employment uncertainty, but external attribution (versus self-blame in firing) and typical 12-18 month recovery trajectory limit permanence.
        // Abramson, L.Y. et al. (1989). Hopelessness depression. Psychological Review; Paul, K.I., & Moser, K. (2009). Unemployment impairs mental health.
        hopelessness: 0.38,

        // Mental Health - Interpersonal Hopelessness
        // Job layoffs create moderate interpersonal hopelessness through shame-driven withdrawal and burden beliefs, but less severely than being fired due to reduced self-blame; recovery occurs within 12-18 months upon reemployment.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review.
        interpersonal_hopelessness: 0.28,

        // Mental Health - Acquired Capability
        // Job layoffs create severe psychological distress and financial threat but do not expose individuals to physical pain, injury, violence, or death, thus producing no habituation to pain/death and no effect on acquired capability.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Job layoff creates acute stress that depletes self-regulatory resources through cognitive load and emotional dysregulation, but most individuals show hedonic adaptation and recovery within months without prolonged unemployment.
        // Baumeister & Vohs (2003) on ego depletion and stress effects on self-regulation.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Job layoffs create acute financial stress and self-focus that temporarily reduces empathic capacity, but external attribution of job loss (vs. internal blame from firing) moderates the impact; most recovery occurs within 12-18 months.
        // Decety, J. & Jackson, P.L. (2004). The functional architecture of human empathy. Behavioral and Cognitive Neuroscience Reviews.
        empathy: -0.10,

        // Disposition - Aggression
        // Layoff triggers frustration and injustice perception through blocked goals and financial threat, but lower identity threat than firing reduces aggression response relative to blame-based job termination.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson & Bushman (2002); Weiner, B. (1985). Attribution theory.
        aggression: 0.28,

        // Disposition - Grievance
        // Layoffs trigger injustice perception from involuntary job loss and procedural concerns, but without personal blame component that intensifies grievance in firings; most individuals show moderate-to-substantial recovery within 1-2 years.
        // Lind, E.A. & Tyler, T.R. (1988). The social psychology of procedural justice.
        grievance: 0.42,

        // Disposition - Reactance
        // Job layoffs impose involuntary loss of employment autonomy and constrain freedom through economic factors, triggering significant but externally-attributed reactance that resolves faster than termination-for-cause due to lack of shame component.
        // Brehm, J.W. (1966). A theory of psychological reactance; Deci & Ryan (1985); Leana & Feldman (1992).
        reactance: 0.25,

        // Disposition - Trust Propensity
        // Job layoffs reduce institutional trust through external attribution of job loss, but lack the personal blame and interpersonal betrayal that maximize trust damage; most individuals recover within 1-2 years through hedonic adaptation and reemployment.
        // Rousseau, D.M. et al. (1998). Not so different after all: A cross-discipline view of trust. Academy of Management Review.
        trust_propensity: -0.22,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: false,
        fatigue: true,
        stress: true,
        purpose: false,
        loneliness: false,
        prc: false,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: true,
        impulse_control: false,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.06,
        arousal: 0.07,
        dominance: 0.07,
        fatigue: 0.06,
        stress: 0.06,
        purpose: 0.06,
        loneliness: 0.06,
        prc: 0.06,
        perceived_liability: 0.08,
        self_hate: 0.06,
        perceived_competence: 0.05,
        depression: 0.08,
        self_worth: 0.05,
        hopelessness: 0.09,
        interpersonal_hopelessness: 0.09,
        impulse_control: 0.05,
        empathy: 0.03,
        aggression: 0.07,
        grievance: 0.09,
        reactance: 0.05,
        trust_propensity: 0.06,
    },
};
