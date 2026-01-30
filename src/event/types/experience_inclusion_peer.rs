//! ExperienceInclusionPeer event specification.
//!
//! Being included by individual peers - acceptance, welcome, or being embraced by
//! friends, classmates, or colleagues. Unlike group inclusion which involves
//! institutional/collective acceptance, peer inclusion is interpersonal acceptance
//! from specific individuals whose acceptance is valued.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Peer inclusion creates moderate positive valence through satisfaction of belonging needs and social validation, with low permanence due to hedonic adaptation despite meaningful temporary emotional uplift.
        // Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529; Lyubomirsky, S. et al. (2011). Hedonic adaptation.
        valence: 0.35,

        // Mood - Arousal
        // Peer inclusion triggers moderate positive physiological activation through social engagement and belonging fulfillment, with rapid hedonic adaptation returning arousal to baseline within 1-2 weeks.
        // Eisenberger et al. (2003) on social acceptance neural mechanisms; Porges, S.W. (2011). The polyvagal theory.
        arousal: 0.35,

        // Mood - Dominance
        // Peer inclusion restores sense of social agency and control through acceptance and group participation, producing moderate positive dominance effect with minimal permanence due to rapid hedonic adaptation.
        // Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        dominance: 0.35,

        // Needs - Fatigue
        // Peer acceptance reduces emotional labor and rumination, providing immediate energy restoration through satisfied belonging needs, but effects fade as the state normalizes.
        // Baumeister & Leary (1995). The need to belong; Hockey, G.R.J. (2013). The psychology of fatigue.
        fatigue: -0.25,

        // Needs - Stress
        // Peer inclusion activates parasympathetic response and reduces threat perception, substantially lowering physiological stress; effects are temporary as individuals adapt to stable social acceptance.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load.
        stress: -0.45,

        // Needs - Purpose
        // Peer inclusion provides moderate affirmation of social relevance and belonging, enhancing sense of purpose, but effects are largely temporary without repeated reinforcement.
        // Baumeister & Leary (1995). The need to belong; Steger et al. (2006). The Meaning in Life Questionnaire.
        purpose: 0.25,

        // Social Cognition - Loneliness
        // Peer inclusion directly reduces perceived isolation through social acceptance, providing meaningful but temporary mood lift with minimal permanent shift due to hedonic adaptation.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600.
        loneliness: -0.35,

        // Social Cognition - PRC
        // Being included and welcomed by valued peers directly signals others care about you, producing significant temporary positive perception of reciprocal caring with modest permanent shift through hedonic adaptation.
        // Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        prc: 0.35,

        // Social Cognition - Perceived Liability
        // Peer inclusion directly counteracts perceived liability by providing concrete evidence of social value and acceptance, reducing the belief that one is a burden to others.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Joiner, T. (2005). Why People Die by Suicide.
        perceived_liability: -0.35,

        // Social Cognition - Self Hate
        // Peer inclusion provides meaningful but temporary reduction in self-directed negativity through social validation and belonging activation, with minimal permanent base-shift.
        // Gilbert, P. & Irons, C. (2005). Focused therapies and compassionate mind training; Neff, K. & Vonk, R. (2009). Self-compassion.
        self_hate: -0.35,

        // Social Cognition - Perceived Competence
        // Peer inclusion provides moderate positive feedback on social competence and relatability, but does not constitute direct mastery experience, so effects show rapid hedonic adaptation.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Williams, K.D. (2007). Ostracism.
        perceived_competence: 0.30,

        // Mental Health - Depression
        // Peer inclusion provides significant temporary mood improvement through belongingness fulfillment, reducing depressive symptoms with mild accumulation into lasting resilience over repeated experiences.
        // Baumeister & Leary (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        depression: -0.25,

        // Mental Health - Self Worth
        // Peer inclusion provides meaningful validation of social worth but produces modest temporary uplift with limited permanent identity shift due to hedonic adaptation.
        // Crocker, J. & Wolfe, C.T. (2001). Contingencies of self-worth; Baumeister, R.F. & Leary, M.R. (1995). The need to belong.
        self_worth: 0.35,

        // Mental Health - Hopelessness
        // Peer inclusion reduces hopelessness by addressing thwarted belongingness through interpersonal acceptance, with substantial hedonic adaptation limiting permanence.
        // Joiner, T. (2005). Why People Die by Suicide; Williams, K.D. (2007). Ostracism. Annual Review of Psychology.
        hopelessness: -0.20,

        // Mental Health - Interpersonal Hopelessness
        // Peer inclusion demonstrates that individual peers accept and value you, directly reducing belief that relationships cannot help, but effect is temporary with hedonic adaptation.
        // Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        interpersonal_hopelessness: -0.35,

        // Mental Health - Acquired Capability
        // Peer inclusion is a social-relational event with no physical pain or death exposure; acquired capability develops only through habituation to pain/mortality, not social acceptance.
        // Joiner, T. (2005). Why People Die by Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Peer inclusion reduces stress response and replenishes self-regulatory resources, producing mild-to-moderate temporary improvement in impulse control through parasympathetic activation and decreased threat perception.
        // Baumeister, R.F. et al. (1998). Ego depletion. Journal of Personality and Social Psychology; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        impulse_control: 0.15,

        // Disposition - Empathy
        // Peer inclusion reduces threat-induced self-focus and creates conditions for empathy to flourish, producing mild temporary increases in other-directed emotional understanding without reshaping core empathic capacity.
        // Singer, T. & Klimecki, O.M. (2014). Empathy and compassion. Current Biology.
        empathy: 0.05,

        // Disposition - Aggression
        // Peer inclusion reduces aggression by satisfying belonging needs and alleviating social threat, but effects are largely temporary as emotional states normalize over time.
        // Williams, K.D. (2007). Ostracism: The power of silence; Baumeister & Leary (1995). The need to belong.
        aggression: -0.35,

        // Disposition - Grievance
        // Peer acceptance reverses victimization narrative through affirmed belonging, reducing grievance temporarily but with limited lasting shift since grievance beliefs typically require sustained systemic validation to change.
        // Tyler, T.R. (1989). The quality of dispute resolution processes and outcomes. Denver University Law Review.
        grievance: -0.35,

        // Disposition - Reactance
        // Peer inclusion grants interpersonal freedom and validates behavioral choices, reducing perceived constraint and mild reactance.
        // Brehm, J.W. (1966). A theory of psychological reactance; Williams, K.D. (2007). Ostracism.
        reactance: -0.12,

        // Disposition - Trust Propensity
        // Peer inclusion demonstrates benevolence and integrity through acceptance, directly counteracting exclusion's trust damage, but shows similar permanence due to requirement for consistent positive reinforcement.
        // Williams, K.D. (2007). Ostracism: The power of silence; Baumeister, R.F. & Leary, M.R. (1995). The need to belong. Psychological Bulletin, 117(3), 497-529.
        trust_propensity: 0.35,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
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
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.06,
        arousal: 0.04,
        dominance: 0.04,
        fatigue: 0.05,
        stress: 0.08,
        purpose: 0.08,
        loneliness: 0.08,
        prc: 0.06,
        perceived_liability: 0.08,
        self_hate: 0.05,
        perceived_competence: 0.05,
        depression: 0.05,
        self_worth: 0.08,
        hopelessness: 0.05,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.05,
        empathy: 0.02,
        aggression: 0.08,
        grievance: 0.12,
        reactance: 0.04,
        trust_propensity: 0.06,
    },
};
