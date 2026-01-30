//! ExperienceRejectionPeer event specification.
//!
//! Being rejected by peers - direct social rejection from individuals in one's
//! peer group (classmates, coworkers, friends). This differs from exclusion
//! (being left out) as rejection involves active dismissal or refusal, making
//! it more explicitly interpersonally painful.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Peer rejection creates significant immediate negative affect through shame and social pain, but most individuals show substantial recovery within weeks as defensive reappraisal occurs.
        // Eisenberger, N.I. (2012). The pain of social disconnection: Examining the shared neural underpinnings of physical and social pain. Nature Reviews Neuroscience
        valence: -0.32,

        // Mood - Arousal
        // Peer rejection triggers acute physiological arousal through social threat response (increased heart rate, cortisol), but effects typically resolve within 1-2 weeks through adaptation.
        // Eisenberger, N.K., Lieberman, M.D., & Williams, K.D. (2003). Does rejection hurt? An fMRI study of social exclusion. Science, 302(5643), 290-292.
        arousal: 0.55,

        // Mood - Dominance
        // Peer rejection imposes loss of social control and agency, moderately reducing dominance temporarily with minimal permanent shift due to typical hedonic adaptation.
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change; Deci & Ryan (1985) self-determination theory
        dominance: -0.35,

        // Needs - Fatigue
        // Peer rejection causes acute emotional exhaustion through distress processing and rumination, with moderate fatigue lasting days to weeks before recovery.
        // Baumeister, R.F., et al. (1998). Ego depletion: Is the active self a limited resource? Journal of Personality and Social Psychology, 74(5), 1252-1265.
        fatigue: 0.35,

        // Needs - Stress
        // Peer rejection activates physiological stress response through social pain and threat perception, with moderate permanence as individuals adapt through alternative relationships but retain elevated rejection sensitivity.
        // Eisenberger, N.I., Lieberman, M.D., & Williams, K.D. (2003). Does rejection hurt? An fMRI study of social exclusion. Science, 302(5643), 290-292.
        stress: 0.58,

        // Needs - Purpose
        // Peer rejection creates moderate purpose disruption by threatening belonging and social validation needed for identity development, but most individuals recover meaning-making capacity through alternative relationships.
        // Coie, J.D. et al. (1990). The role of aggression in peer relations; Deci, E.L. & Ryan, R.M. (2000). The "what" and "why" of goal pursuits.
        purpose: -0.28,

        // Social Cognition - Loneliness
        // Peer rejection creates significant loneliness through active interpersonal dismissal and damaged social trust, with moderate permanence as individuals rebuild through alternative peer relationships.
        // Eisenberger et al. (2003); Williams, K.D. (2007). Ostracism. Annual Review of Psychology, 58, 425-452.
        loneliness: 0.42,

        // Social Cognition - PRC
        // Peer rejection directly signals peers don't care about the person's inclusion, creating significant negative PRC impact, but recovery is possible through alternative peer relationships.
        // Eisenberger et al. (2003); Joiner, T.E. (2005). Why People Die by Suicide; Van Orden et al. (2010). The interpersonal theory of suicide.
        prc: -0.45,

        // Social Cognition - Perceived Liability
        // Peer rejection involves active dismissal that increases perceived liability more than passive exclusion through internalized rejection messaging and chronic rejection sensitivity.
        // Eisenberger et al. (2003); Williams (2007); Leary & Baumeister (2000) The Nature and Function of Self-Esteem
        perceived_liability: 0.28,

        // Social Cognition - Self Hate
        // Peer rejection triggers significant but temporary self-blame and shame through internalized failure attribution, with near-complete recovery through alternative social bonding.
        // Eisenberger, N. I., et al. (2003). Does Rejection Hurt? An fMRI Study of Social Rejection. Science 302(5643): 290-292.
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Peer rejection delivers strong negative social feedback that temporarily damages competence beliefs through attribution to internal failings, but most individuals recover as rejection is contextualized.
        // Bandura's social learning theory; Downey & Feldman (1996) on rejection sensitivity and self-evaluation.
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Peer rejection triggers significant depressive symptoms through loss of social acceptance and rumination, but most individuals recover substantially within months with support.
        // Brown, G.W. & Harris, T. (1978). Social origins of depression; longitudinal research on peer rejection and depressive symptomatology.
        depression: 0.28,

        // Mental Health - Self Worth
        // Peer rejection (active dismissal) creates significant identity-level self-worth damage through shame and social devaluation; most individuals show substantial recovery within 1-2 years.
        // Leary, M.R. & Baumeister, R.F. (2000). The nature and function of self-esteem: Sociometer theory; Eisenberger et al. (2003); Williams (2007).
        self_worth: -0.45,

        // Mental Health - Hopelessness
        // Peer rejection creates significant but temporary hopelessness about social future, with most individuals recovering within 1-2 years without chronic patterns.
        // Greca & Harrison (2005) on peer rejection effects; Abramson et al. (1989) hopelessness depression model.
        hopelessness: 0.25,

        // Mental Health - Interpersonal Hopelessness
        // Peer rejection creates significant temporary doubt about social support effectiveness, but most individuals show recovery through alternative relationships over 12-24 months.
        // Joiner, T. (2005). Why People Die by Suicide; Rickwood, D. et al. (2005). Young people's help-seeking for mental health problems.
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Peer rejection is a social stressor that does not expose individuals to physical pain or death proximity required for habituation-based acquired capability development.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Peer rejection acutely depletes self-regulatory resources through emotional distress, impairing impulse control significantly but temporarily, with minimal permanent base shift unless chronic.
        // Baumeister, R.F., DeWall, C.N., Ciarocco, N.J., & Twenge, J.M. (2005). Social exclusion impairs self-regulation. Journal of Personality and Social Psychology, 88(4), 589-604.
        impulse_control: -0.25,

        // Disposition - Empathy
        // Peer rejection temporarily increases self-focus and reduces empathic capacity through narrowed attention and threat response, but effects are mild and recover quickly.
        // Eisenberger, N.I., Lieberman, M.D., & Williams, K.D. (2003). Does rejection hurt? An fMRI study of social exclusion. Science, 302(5643), 290-292.
        empathy: -0.12,

        // Disposition - Aggression
        // Peer rejection triggers frustration and ego threat, producing moderate immediate aggression increase; most recovery occurs within weeks but accumulation over time can produce modest trait shifts.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson & Bushman (2002) on social rejection as frustration source.
        aggression: 0.35,

        // Disposition - Grievance
        // Peer rejection triggers moderate grievance due to perceived unfairness, but most individuals recover through reappraisal and perspective-taking over time.
        // Eisenberger, N.I., & Lieberman, M.D. (2004). Why rejection hurts: A common neural alarm system for physical and social pain. Trends in Cognitive Sciences, 8(7), 294-300.
        grievance: 0.35,

        // Disposition - Reactance
        // Peer rejection triggers mild reactance through compensatory self-assertion and autonomy threat, but primarily threatens belonging rather than directly constraining behavioral freedom.
        // Brehm, J.W. (1966). A theory of psychological reactance. Academic Press; Williams, K.D. (2007). Ostracism.
        reactance: 0.12,

        // Disposition - Trust Propensity
        // Peer rejection actively signals interpersonal unworthiness and damages trust in peer relationships, but recovery occurs through alternative relationships unlike family betrayal.
        // Williams, K.D. (2007). Ostracism; Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        trust_propensity: -0.45,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: true,
        purpose: false,
        loneliness: true,
        prc: true,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: true,
        self_worth: true,
        hopelessness: false,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: false,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.06,
        arousal: 0.05,
        dominance: 0.07,
        fatigue: 0.05,
        stress: 0.10,
        purpose: 0.06,
        loneliness: 0.10,
        prc: 0.12,
        perceived_liability: 0.08,
        self_hate: 0.05,
        perceived_competence: 0.06,
        depression: 0.05,
        self_worth: 0.09,
        hopelessness: 0.06,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.05,
        empathy: 0.04,
        aggression: 0.08,
        grievance: 0.08,
        reactance: 0.05,
        trust_propensity: 0.08,
    },
};
