//! ExperienceExclusionPeer event specification.
//!
//! Being excluded by individual peers - rejection, ostracism, or being ignored by
//! friends, classmates, or colleagues. Unlike group exclusion which involves
//! institutional/collective rejection, peer exclusion is interpersonal rejection
//! from specific individuals whose acceptance was valued.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Peer exclusion triggers moderate-to-severe negative valence through interpersonal rejection and social pain, with some permanent shift from altered social trust expectations despite hedonic adaptation.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC1847099/ (Eisenberger et al. on social exclusion neural mechanisms)
        valence: -0.42,

        // Mood - Arousal
        // Peer exclusion creates acute physiological arousal through interpersonal threat perception and stress response activation, slightly elevated from group exclusion due to personal rejection salience.
        // https://psycnet.apa.org/doi/10.1037/0022-3514.88.4.589; Eisenberger et al. (2003) fMRI social pain study
        arousal: 0.50,

        // Mood - Dominance
        // Peer exclusion imposes loss of agency and social control similar to group exclusion, but narrower scope (individuals vs. collective) produces moderate rather than significant dominance reduction.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC1847099/; https://psycnet.apa.org/doi/10.1037/0022-3514.88.4.589
        dominance: -0.40,

        // Needs - Fatigue
        // Peer exclusion creates moderate acute fatigue through emotional processing, rumination, and potential sleep disruption, but most individuals recover within weeks with hedonic adaptation.
        // Baumeister, R.F. et al. (1998). Ego depletion. Journal of Personality and Social Psychology, 74(5), 1252-1265.
        fatigue: 0.35,

        // Needs - Stress
        // Peer exclusion triggers significant HPA axis activation and physiological stress response comparable to other major stressors; chronic/repeated exclusion creates sustained elevation.
        // Williams, K. D., & Zadro, L. (2001). Ostracism: On being ignored, excluded, and rejected.
        stress: 0.55,

        // Needs - Purpose
        // Peer exclusion creates moderate purpose disruption through threat to belonging and identity validation, but primary recovery occurs as individuals rebuild social connections.
        // Williams, K.D. (2007). Ostracism. Annual Review of Psychology, 58, 425-452
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Peer exclusion creates significant immediate loneliness through disrupted interpersonal connections, with moderate permanence from accumulated negative relational patterns.
        // Cacioppo & Patrick (2008). Loneliness: Human nature and the need for social connection
        loneliness: 0.35,

        // Social Cognition - PRC
        // Peer exclusion directly signals rejection and lack of caring from specific individuals, causing significant immediate decrease in perceived reciprocal caring perception.
        // Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        prc: -0.38,

        // Social Cognition - Perceived Liability
        // Peer rejection creates moderate perceived rejection and loneliness but lower burden perception than group exclusion, as individual incompatibility doesn't inherently signal dependency.
        // https://doi.org/10.1037/0022-3514.85.6.989 (Eisenberger et al., 2003)
        perceived_liability: 0.20,

        // Social Cognition - Self Hate
        // Peer exclusion triggers moderate self-blame and shame as individuals often internalize rejection as personal failing, with chronic rumination patterns.
        // Joiner, T. (2005). Why People Die by Suicide; Eisenberger & Lieberman (2004)
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Peer exclusion delivers social rejection feedback that temporarily undermines belief in social competence and relatability, but doesn't directly invalidate task-specific abilities.
        // Williams, K. D. (2007). Ostracism. Annual Review of Psychology, 58, 425-452
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Peer exclusion produces significant depressive symptoms through loss of belonging and social withdrawal, but shows moderate hedonic adaptation with recovery within 6-12 months.
        // https://psycnet.apa.org/doiLanding?doi=10.1037%2F0033-2909.127.3.309 (Williams, K.D. 2001)
        depression: 0.35,

        // Mental Health - Self Worth
        // Peer exclusion creates significant but temporary self-worth damage through identity-level rejection; most recover within 1-2 years but chronic exclusion patterns accumulate lasting effects.
        // Leary, M.R. & Baumeister, R.F. (2000). The nature and function of self-esteem: Sociometer theory.
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Peer exclusion creates moderate temporary hopelessness about social future, but most individuals show substantial recovery as new peer opportunities emerge.
        // Baumeister, R.F., & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        hopelessness: 0.25,

        // Mental Health - Interpersonal Hopelessness
        // Peer rejection from specific individuals signals interpersonal unworthiness and undermines belief that individual peers can provide support, but without the generalization of betrayal.
        // Williams, K.D. (2009). Ostracism: A social, not physical, pain. Trends in Cognitive Sciences, 13(1), 36-42
        interpersonal_hopelessness: 0.45,

        // Mental Health - Acquired Capability
        // Peer exclusion causes psychological distress through thwarted belongingness and perceived burdensomeness, not through exposure to physical pain or death that would increase habituation.
        // Joiner, T. (2005). Why People Die by Suicide. Harvard University Press.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Peer exclusion depletes self-regulatory capacity through interpersonal rejection and threatens belonging needs, creating significant but reversible impulse control impairment.
        // https://doi.org/10.1037/0022-3514.88.4.589; Baumeister et al. (1998) ego depletion research
        impulse_control: -0.32,

        // Disposition - Empathy
        // Peer exclusion triggers temporary self-focus and threat response that reduces affective empathy acutely, but most individuals recover with social reconnection or time.
        // Williams, K.D. (2009). Ostracism: Effects of being excluded and ignored. https://doi.org/10.1016/S0065-2601(08)00406-1
        empathy: -0.20,

        // Disposition - Aggression
        // Peer exclusion triggers frustration, identity threat, and goal blockage leading to increased aggressive response tendency, with some permanence due to rejection sensitivity.
        // Twenge, J.M., et al. (2001); Williams, K.D. (2007). Ostracism: The power of silence.
        aggression: 0.38,

        // Disposition - Grievance
        // Peer exclusion triggers moderate grievance as interpersonal betrayal, but hedonic adaptation typically resolves most effects within 2-3 months.
        // Williams, K. D. (1997). Social ostracism; DeWall, C. N., & Richman, S. B. (2011)
        grievance: 0.35,

        // Disposition - Reactance
        // Peer exclusion primarily threatens belonging rather than autonomy, creating only mild reactance through compensatory self-assertion without directly constraining behavioral freedom.
        // Brehm, J.W. (1966). A theory of psychological reactance. Academic Press
        reactance: 0.08,

        // Disposition - Trust Propensity
        // Peer exclusion signals interpersonal rejection reducing trust in peers' acceptance, but recovery occurs through alternative relationships and is not permanent like intimate betrayal.
        // Williams, K.D. (2007). Ostracism. Annual Review of Psychology; Leary & Baumeister (2000)
        trust_propensity: -0.35,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: true,
        purpose: false,
        loneliness: true,
        prc: false,
        perceived_liability: false,
        self_hate: true,
        perceived_competence: false,
        depression: true,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.09,
        arousal: 0.07,
        dominance: 0.05,
        fatigue: 0.06,
        stress: 0.08,
        purpose: 0.06,
        loneliness: 0.06,
        prc: 0.08,
        perceived_liability: 0.05,
        self_hate: 0.08,
        perceived_competence: 0.06,
        depression: 0.08,
        self_worth: 0.08,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.08,
        impulse_control: 0.06,
        empathy: 0.04,
        aggression: 0.10,
        grievance: 0.06,
        reactance: 0.04,
        trust_propensity: 0.06,
    },
};
