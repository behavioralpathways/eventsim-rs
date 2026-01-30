//! ExperienceRejectionRomantic event specification.
//!
//! Being rejected romantically - direct rejection from a desired romantic interest.
//! This differs from relationship ending (mutual or initiated dissolution) as rejection
//! involves explicit dismissal of romantic interest, creating acute social pain and
//! identity threat in intimate contexts.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Romantic rejection causes acute sadness and emotional pain (severe negative valence) but shows near-complete hedonic recovery within months for most individuals.
        // Fisher, H.E. (2004). Why we love: The nature and chemistry of romantic love; Eisenberger et al. (2003) on social pain mechanisms.
        valence: -0.50,

        // Mood - Arousal
        // Romantic rejection triggers acute sympathetic activation through social threat and self-esteem threat in intimate context, with physiological arousal (anxiety, embarrassment) resolving within 1-2 weeks as cognitive reappraisal occurs.
        // Eisenberger, N.I., Lieberman, M.D., & Williams, K.D. (2003). Does rejection hurt? An fMRI study of social exclusion. Science, 302(5643), 290-292.
        arousal: 0.60,

        // Mood - Dominance
        // Romantic rejection is an imposed loss of control over relationship outcome, producing moderate but temporary dominance reduction with near-complete recovery through hedonic adaptation.
        // Mehrabian & Russell (1974) - PAD Affect Model; Deci & Ryan (1985) - Self-determination theory on autonomy deprivation.
        dominance: -0.30,

        // Needs - Fatigue
        // Romantic rejection triggers sustained emotional processing and sleep disruption comparable to family rejection but with greater recovery pathways, placing it at moderate-to-high acute fatigue with limited chronic elevation.
        // Baumeister, R.F., et al. (1998). Ego depletion: Is the active self a limited resource? Journal of Personality and Social Psychology, 74(5), 1252-1265.
        fatigue: 0.42,

        // Needs - Stress
        // Romantic rejection triggers moderate-high acute physiological stress (loss, social threat, reduced control activate HPA axis), but shows complete recovery within months for most individuals through hedonic adaptation.
        // Slavich, G.M., & Irwin, M.R. (2014). From stress to inflammation and major depressive disorder. Psychological Bulletin, 140(3), 774-815.
        stress: 0.55,

        // Needs - Purpose
        // Romantic rejection significantly disrupts near-term life goals and narrative meaning, but recovery occurs within 1-2 years with limited permanent shift to baseline purpose.
        // Frankl, V.E. (1959). Man's Search for Meaning; Steger, M.F. et al. (2006). The Meaning in Life Questionnaire.
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Romantic rejection increases perceived isolation through lost/thwarted intimate connection and potential social withdrawal, but most impact is temporary with near-complete recovery through time and new social/romantic engagement.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Joiner, T. (2005). Why People Die by Suicide.
        loneliness: 0.35,

        // Social Cognition - PRC
        // Romantic rejection creates sustained but reversible doubt about being valued as a romantic interest, affecting primary attachment beliefs more severely than peer rejection but less than family rejection.
        // Eisenberger, N.I., et al. (2003). Does rejection hurt? Science, 302(5643), 290-292; Van Orden, K. et al. (2010). The interpersonal theory of suicide.
        prc: -0.52,

        // Social Cognition - Perceived Liability
        // Romantic rejection combines explicit personal dismissal with intimate identity threat, creating elevated perceived liability that becomes chronic through rejection sensitivity but mostly recovers as individuals rebuild romantic self-concept.
        // Downey, G., & Feldman, S.I. (1996). Implications of rejection sensitivity for intimate relationships. Journal of Personality and Social Psychology, 70(6), 1327-1343.
        perceived_liability: 0.32,

        // Social Cognition - Self Hate
        // Romantic rejection triggers significant self-blame and shame as personal failure, but most individuals show near-complete recovery within 12-18 months with typical hedonic adaptation.
        // Joiner, T. (2005). Why People Die by Suicide; psychological literature on rejection sensitivity and shame.
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Romantic rejection creates mild temporary doubt about relational attractiveness and competence, but doesn't substantially affect general perceived competence in task accomplishment - most people recover within weeks to months.
        // Leary & Downs (1995). Relevance of social rejection to affect regulation; Helson (1964). Adaptation-level theory.
        perceived_competence: -0.15,

        // Mental Health - Depression
        // Romantic rejection triggers significant depressive symptoms through loss and rumination, but most individuals recover completely within months with hedonic adaptation and social support.
        // Sprecher, S., et al. (1998). Factors associated with distress following relationship dissolution; Downey, G., & Feldman, S.I. (1996).
        depression: 0.32,

        // Mental Health - Self Worth
        // Romantic rejection activates core shame and social rejection response, producing significant but temporary self-worth disruption with some chronic activation in rejection-sensitive individuals.
        // Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Romantic rejection creates moderate temporary hopelessness about future relationships and worth, but recovery typically occurs within 6-12 months as the individual recognizes future romantic opportunities remain possible.
        // Baumeister, R.F., & Leary, M.R. (1995). The need to belong; Abramson, L.Y., et al. (1989). Hopelessness depression.
        hopelessness: 0.25,

        // Mental Health - Interpersonal Hopelessness
        // Romantic rejection creates significant doubt about future intimate relationships and help-seeking through romantic partners, with moderate permanence due to repeated validation of unworthiness in romantic contexts.
        // Joiner, T. (2005). Why People Die by Suicide; Rickwood, D. et al. (2005). Young people's help-seeking for mental health problems.
        interpersonal_hopelessness: 0.42,

        // Mental Health - Acquired Capability
        // Romantic rejection causes psychological pain and thwarted belongingness but does not expose the individual to physical pain, death, or fear of dying; thus does not create habituation required for acquired capability.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Romantic rejection causes acute ego depletion and emotional distress that significantly impairs self-regulation and impulse control, with most recovery occurring within 2 weeks through hedonic adaptation.
        // Baumeister, R.F., DeWall, C.N., et al. (2005). Social exclusion impairs self-control. Journal of Personality and Social Psychology, 88(4), 589-604.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Romantic rejection temporarily increases self-focus and stress, reducing empathic capacity, but most effects recover as emotional regulation normalizes.
        // Singer, T. & Klimecki, O.M. (2014). Empathy and compassion. Current Biology 24(18): R875-R878.
        empathy: -0.15,

        // Disposition - Aggression
        // Romantic rejection is an ego threat and goal-blocking event that triggers frustration-based aggression increase (Berkowitz hypothesis), but adaptation occurs within weeks/months with minimal permanent shift.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson & Bushman (2002) on rejection-induced hostility.
        aggression: 0.35,

        // Disposition - Grievance
        // Romantic rejection creates moderate perceived unfairness, but most individuals reframe as incompatibility rather than injustice within weeks, with minimal permanent grievance shift absent relationship trauma.
        // Eisenberger, N.I., & Lieberman, M.D. (2004). Why rejection hurts. Trends in Cognitive Sciences; Williams, K.D. (2007). Ostracism.
        grievance: 0.28,

        // Disposition - Reactance
        // Romantic rejection creates mild reactance through perceived loss of relational autonomy and choice foreclosure, but does not impose direct external constraints on behavioral freedom.
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological reactance: A theory of freedom and control. Academic Press.
        reactance: 0.15,

        // Disposition - Trust Propensity
        // Romantic rejection violates relational trust expectations but is differentiated from institutional/competence betrayal; recovery is typical within 1-2 years with modest base-level shift in relationship selectivity.
        // Bartholomew, K., & Horowitz, L.M. (1991) on attachment and trust; Downey & Feldman (1996) on rejection sensitivity.
        trust_propensity: -0.25,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        dominance: false,
        fatigue: true,
        stress: false,
        purpose: false,
        loneliness: false,
        prc: true,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: true,
        impulse_control: false,
        empathy: false,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.05,
        arousal: 0.05,
        dominance: 0.05,
        fatigue: 0.11,
        stress: 0.05,
        purpose: 0.06,
        loneliness: 0.05,
        prc: 0.15,
        perceived_liability: 0.10,
        self_hate: 0.08,
        perceived_competence: 0.05,
        depression: 0.06,
        self_worth: 0.08,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.15,
        impulse_control: 0.05,
        empathy: 0.05,
        aggression: 0.06,
        grievance: 0.06,
        reactance: 0.04,
        trust_propensity: 0.08,
    },
};
