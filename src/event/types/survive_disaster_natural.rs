//! SurviveDisasterNatural event specification.
//!
//! Surviving a natural disaster such as earthquake, hurricane, tornado, flood,
//! wildfire, or tsunami - an acute traumatic event involving threat to life,
//! property loss, displacement, and exposure to death and destruction.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Natural disaster survival creates severe acute distress with sustained negative valence due to trauma, loss, and disrupted sense of safety; most survivors show substantial recovery within 2 years but maintain measurable baseline shift.
        // Rubonis, A.V., & Bickman, L. (1991). Psychological impairment in the wake of disaster. Psychological Bulletin, 109(3), 384-399.
        valence: -0.68,

        // Mood - Arousal
        // Natural disasters trigger extreme physiological activation (fight-or-flight response) that often transitions to chronic hyperarousal in PTSD, with roughly one-quarter of the acute arousal shift becoming a permanent baseline increase due to nervous system sensitization.
        // Posner, J., Russell, J.A., & Peterson, B.S. (2005). The circumplex model of affect. Developmental Psychology, 41(6), 1092-1104.
        arousal: 0.82,

        // Mood - Dominance
        // Natural disasters impose total uncontrollability of the event itself but survivors retain agency in response and recovery, creating moderate dominance loss with substantial hedonic adaptation within 1-2 years.
        // Seligman, M.E.P. (1975). Helplessness: On depression, development, and death; Bandura, A. (1977). Self-efficacy theory.
        dominance: -0.50,

        // Needs - Fatigue
        // Natural disasters cause severe acute fatigue through physical exertion, sleep disruption, and emotional processing of trauma; effects are chronic during recovery but mostly fade within 1-2 years with hedonic adaptation.
        // Hockey, G.R.J. (2013). The psychology of fatigue; disaster-induced sleep disturbance research.
        fatigue: 0.65,

        // Needs - Stress
        // Natural disasters trigger severe physiological stress response with threat perception, high unpredictability, and loss of control; acute stress is severe but most survivors show significant recovery within 1-2 years.
        // McEwen, B.S. & Seeman, T. (1999). Stress and the individual: mechanisms leading to disease. Archives of Internal Medicine.
        stress: 0.85,

        // Needs - Purpose
        // Surviving natural disasters catalyzes existential reflection and reprioritization of values, producing moderate purpose enhancement that gradually moderates through hedonic adaptation.
        // Tedeschi, R.G., & Calhoun, L.G. (2004). Posttraumatic Growth: Conceptual Foundations and Empirical Evidence. Psychological Inquiry, 15(1), 1-18.
        purpose: 0.35,

        // Social Cognition - Loneliness
        // Natural disasters initially strengthen social bonds but lead to sustained loneliness through displacement, economic hardship, and trauma-related social withdrawal in the recovery period.
        // Norris, F.H., et al. (2008). Community resilience as a metaphor, theory, set of capacities, and strategy for disaster readiness. American Journal of Community Psychology.
        loneliness: 0.15,

        // Social Cognition - PRC
        // Surviving natural disasters typically activates immediate community support and prosocial responses, increasing perceived reciprocal care through visible helping behaviors, though effects fade as recovery normalizes.
        // Kaniasty & Norris (1999). Disasters, Community Ties, and Psychological Consequences. Journal of Community Psychology.
        prc: 0.35,

        // Social Cognition - Perceived Liability
        // Surviving natural disasters creates moderate temporary perceived burdensomeness through dependency on support and housing/financial assistance, but collective trauma context and community mutual aid substantially reduce individual burden perception.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600.
        perceived_liability: 0.28,

        // Social Cognition - Self Hate
        // Natural disasters are externally caused, reducing self-directed hatred, though survivor guilt and temporary self-blame about preparedness create mild elevation with near-complete recovery within 1-2 years.
        // Joiner, T. (2005). Why People Die by Suicide; standard disaster psychology literature on survivor guilt.
        self_hate: 0.15,

        // Social Cognition - Perceived Competence
        // Initial competence reduction from helplessness and loss of control during uncontrollable event, partially offset by mastery experience of survival and recovery process with near-complete hedonic adaptation.
        // Bandura, A. (1997). Self-efficacy: The exercise of control. Freeman.
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Natural disasters trigger significant depressive symptoms from loss and disruption; most recover within 12-18 months but substantial minority develop chronic depression.
        // Norris, F.H., et al. (2002). 60,000 Disaster Victims Speak: Part I. An Empirical Review of the Empirical Literature. Psychiatry.
        depression: 0.40,

        // Mental Health - Self Worth
        // Surviving a natural disaster typically produces temporary anxiety and decreased self-confidence from loss of control, but most survivors report maintained or gradually restored self-esteem through successful coping.
        // Norris, F.H., et al. (2002). 60,000 Disaster Victims Speak; Bonanno, G.A. (2004). Loss, Trauma, and Human Resilience.
        self_worth: -0.15,

        // Mental Health - Hopelessness
        // Surviving a natural disaster creates significant but temporary hopelessness due to loss of control, unpredictability, and destroyed plans/futures, with most survivors showing recovery within 12-24 months through post-traumatic growth.
        // Norris, F.H., et al. (2002). 60,000 disaster victims speak. Psychiatry, 65(3):207-239.
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Natural disasters typically strengthen immediate social bonds through collective trauma response and mutual aid, decreasing interpersonal hopelessness, but effects are largely temporary as communities normalize.
        // Kaniasty & Norris (2004). Social Support in the Aftermath of Disasters. Emergency Medicine Clinics of North America.
        interpersonal_hopelessness: -0.25,

        // Mental Health - Acquired Capability
        // Natural disaster survivors experience acute exposure to pain, injury, and death proximity comparable to accidental injury, creating moderate habituation that permanently elevates acquired capability.
        // Van Orden, K.A. et al. (2010). The Interpersonal Theory of Suicide; Joiner, T.E. (2005). Why People Die by Suicide.
        acquired_capability: 0.32,

        // Disposition - Impulse Control
        // Natural disasters cause acute self-regulatory depletion through extreme stress and hyperarousal, with partial permanence due to variable development of chronic PTSD in vulnerable populations.
        // Baumeister, R.F., & Vohs, K.D. (2007). Self-regulation and self-control; Hobfoll, S.E. et al. (2009). Conservation of resources theory.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Natural disaster survival temporarily reduces empathy through acute stress and self-focus, but this effect is largely reversible as trauma symptoms stabilize and survivors return to baseline functioning.
        // Singer, T. & Klimecki, O.M. (2014). Empathy and compassion. Current Biology, 24(18), R875-R878.
        empathy: -0.15,

        // Disposition - Aggression
        // Natural disasters trigger temporary frustration and reactive anger from loss and disruption, but most survivors recover within 1-2 years with social support and adaptation.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson & Bushman (2002) on human aggression pathways.
        aggression: 0.25,

        // Disposition - Grievance
        // Natural disasters produce moderate grievance primarily from systemic failures in relief response rather than intentional injustice, with most survivors recovering within 1-2 years despite initial anger.
        // Lind & Tyler (1988). The social psychology of procedural justice.
        grievance: 0.25,

        // Disposition - Reactance
        // Natural disasters impose sudden, severe loss of control and autonomy (triggering reactance), but post-disaster constraints are temporary and fade as recovery enables autonomy restoration.
        // Brehm, J.W. (1966). A theory of psychological reactance; Norris, F.H., et al. on disaster recovery trajectories.
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Natural disasters increase trust through community bonding and demonstrated mutual support, but effects are modest and mostly temporary due to hedonic adaptation.
        // Kaniasty & Norris (2004). Social support in the aftermath of disasters.
        trust_propensity: 0.15,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: false,
        loneliness: true,
        prc: false,
        perceived_liability: false,
        self_hate: false,
        perceived_competence: false,
        depression: true,
        self_worth: false,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.15,
        arousal: 0.25,
        dominance: 0.12,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.15,
        loneliness: 0.12,
        prc: 0.08,
        perceived_liability: 0.10,
        self_hate: 0.08,
        perceived_competence: 0.08,
        depression: 0.08,
        self_worth: 0.06,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.12,
        empathy: 0.05,
        aggression: 0.08,
        grievance: 0.08,
        reactance: 0.06,
        trust_propensity: 0.08,
    },
};
