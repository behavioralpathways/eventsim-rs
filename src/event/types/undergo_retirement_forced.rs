//! UndergoRetirementForced event specification.
//!
//! Forced/involuntary retirement including mandatory retirement age, being pushed
//! out due to age, health-related forced retirement, or organizational downsizing
//! targeting older workers. Differs from voluntary retirement in loss of agency
//! and the identity-threatening nature of the transition.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Forced retirement combines identity loss, loss of control, and social disruption, creating severe negative affect; hedonic adaptation occurs but is slower than voluntary retirement.
        // Sharpley et al. (2000) "Coping with Work and Retirement Stress"; Wang & Shi (2014) on involuntary retirement outcomes.
        valence: -0.58,

        // Mood - Arousal
        // Forced retirement triggers moderate-to-high physiological activation through involuntary role loss and financial threat; sustained elevation due to permanence of transition.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Paul, K.I., & Moser, K. (2009). Unemployment impairs mental health.
        arousal: 0.52,

        // Mood - Dominance
        // Forced retirement represents significant loss of control and agency; the individual has no choice in timing or circumstance, creating substantial but not total powerlessness.
        // Seligman's Learned Helplessness Theory (1975); Deci & Ryan Self-Determination Theory on autonomy.
        dominance: -0.55,

        // Needs - Fatigue
        // Forced retirement creates moderate emotional exhaustion from identity loss and grief; the chronic loss-of-purpose creates sustained fatigue.
        // Dave et al. (2008) "Retirement and Cognitive Decline"; Baumeister, R.F. et al. (1998). Ego depletion.
        fatigue: 0.35,

        // Needs - Stress
        // Forced retirement represents involuntary loss of occupational identity and control, triggering sustained high stress response comparable to job loss with identity components.
        // McEwen, B.S. (1998). Stress, adaptation, and disease; Drew et al. (2017) on involuntary job displacement.
        stress: 0.65,

        // Needs - Purpose
        // Forced retirement eliminates primary identity source and demonstrates loss of agency, creating significant purpose disruption with lasting baseline shift.
        // Frankl, V.E. (1959). Man's Search for Meaning; Ekerdt (1986) on retirement identity crisis.
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Forced retirement triggers significant loneliness through loss of primary social roles, workplace relationships, and daily structure; older adults struggle to rebuild social networks.
        // Dave et al. (2008) "Retirement and Social Isolation"; Perlman & Peplau (1981) Loneliness: A Sourcebook.
        loneliness: 0.35,

        // Social Cognition - PRC
        // Forced retirement communicates social rejection and removes primary source of daily belonging signals; the involuntary nature intensifies perception that others don't care.
        // Schlossberg (1981) transition theory; Joiner (2005) on belongingness deficits in life transitions.
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Forced retirement removes primary identity and productive role, creating significant burden perception as the person transitions from contributor to dependent.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Atchley (1976) continuity theory.
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Forced retirement triggers moderate self-blame due to loss of agency and identity disruption; the involuntary nature intensifies internalized blame beyond typical job loss.
        // Groenewold et al. (2013) on involuntary job loss and mental health; Wang et al. (2009) on forced vs. voluntary retirement.
        self_hate: 0.32,

        // Social Cognition - Perceived Competence
        // Forced retirement represents involuntary loss of the primary domain for demonstrating competence; significant but not catastrophic doubt with chronic effects.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Ashforth (2001) Role Transitions in Organizational Life.
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Forced retirement produces significant depressive symptoms comparable to job loss, with involuntary nature and identity loss creating chronic adjustment.
        // Schlossberg, N.K. (1981); Shultz & Wang (2011) on psychological perspectives on retirement.
        depression: 0.35,

        // Mental Health - Self Worth
        // Forced retirement removes core work identity and signals age-based societal devaluation, producing significant self-worth damage that partially stabilizes over time.
        // Burke & Stets (2009) Identity Theory; Rosenberg (1965) on self-esteem and role identity.
        self_worth: -0.32,

        // Mental Health - Hopelessness
        // Forced retirement induces significant hopelessness through permanent identity loss, age-based irreversibility, and loss of control with minimal recovery pathway.
        // Abramson, L.Y. et al. (1989). Hopelessness depression; Wang & Shi (2014) on retirement transitions.
        hopelessness: 0.55,

        // Mental Health - Interpersonal Hopelessness
        // Forced retirement creates moderate interpersonal hopelessness through loss of work identity and social role combined with shame that reduces help-seeking.
        // Latack & Dozier (1986) on job loss as life transition; Price (1992) on psychosocial impact of job loss.
        interpersonal_hopelessness: 0.28,

        // Mental Health - Acquired Capability
        // Forced retirement creates significant psychological distress but does not expose individuals to physical pain or death; no habituation effect.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Forced retirement triggers identity loss and loss of control, severely impairing self-regulation capacity through stress and reduced executive function.
        // Baumeister, R.F. et al. (1998). Ego depletion; Ekerdt (1986) on retirement identity discontinuity.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Forced retirement triggers identity loss and stress-induced self-focus, mildly reducing empathic capacity; most individuals recover social engagement within months.
        // Schlossberg on identity transitions; Brenner on unemployment stress effects.
        empathy: -0.12,

        // Disposition - Aggression
        // Forced retirement triggers significant frustration from loss of autonomy, identity, and control; strong aggression antecedents without the severity of violent injustice.
        // McKee-Ryan et al. (2005) on involuntary job loss; Berkowitz (1989) on frustration-aggression.
        aggression: 0.32,

        // Disposition - Grievance
        // Forced retirement combines involuntariness, identity loss, and age discrimination creating severe perceived injustice that becomes integrated into worldview.
        // Greenberg organizational justice theory; Lind & Tyler (1988) on procedural justice.
        grievance: 0.68,

        // Disposition - Reactance
        // Forced retirement constitutes severe autonomy violation and identity loss, generating strong reactance response that often persists as resentment.
        // Brehm, J.W. (1966). A Theory of Psychological Reactance; Dillard & Shen (2005) on reactance.
        reactance: 0.65,

        // Disposition - Trust Propensity
        // Forced retirement damages institutional trust through age-based discrimination and role loss; lacks personal shame but has finality that exceeds typical job loss.
        // Mayer, Davis, & Schoorman (1995) on organizational trust; Butler (1969) on ageism.
        trust_propensity: -0.38,
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
        valence: 0.15,
        arousal: 0.11,
        dominance: 0.12,
        fatigue: 0.12,
        stress: 0.12,
        purpose: 0.15,
        loneliness: 0.12,
        prc: 0.12,
        perceived_liability: 0.12,
        self_hate: 0.12,
        perceived_competence: 0.12,
        depression: 0.15,
        self_worth: 0.18,
        hopelessness: 0.18,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.12,
        empathy: 0.05,
        aggression: 0.12,
        grievance: 0.15,
        reactance: 0.12,
        trust_propensity: 0.12,
    },
};
