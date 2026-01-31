//! LoseResourceSignificant event specification.
//!
//! Significant loss of material resources such as savings, property, investments,
//! or valuable possessions. Creates acute financial threat and identity disruption
//! through loss aversion and reduced agency.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Significant material loss produces severe negative affect through both loss aversion and practical security threat, but shows substantial hedonic adaptation within 12-18 months.
        // Kahneman & Tversky (1979) "Prospect Theory: An Analysis of Decision under Risk"; Diener et al. (1999) on affect and life circumstances
        valence: -0.55,

        // Mood - Arousal
        // Significant resource loss activates threat-appraisal and loss-aversion responses creating high arousal (sustained vigilance and anxiety) with modest permanence due to hedonic adaptation and eventual financial recovery.
        // Kahneman & Tversky (1979). Prospect Theory; Thayer, R.E. (1989). The biopsychology of mood and arousal; Russell, J.A. (1980). A circumplex model of affect.
        arousal: 0.58,

        // Mood - Dominance
        // Significant resource loss sharply reduces sense of control and agency over future outcomes, but recovery capacity (ability to earn/rebuild) prevents complete powerlessness.
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change; Seligman, M.E.P. (1975). Helplessness: On depression, development, and death.
        dominance: -0.55,

        // Needs - Fatigue
        // Significant resource loss triggers sustained rumination, sleep disruption, and cognitive/emotional exhaustion comparable to eviction threat; most individuals show substantial recovery within 12-18 months through hedonic adaptation and financial stabilization.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load; Mullainathan, S. & Shafir, E. (2013). Scarcity: Why Having Too Little Means So Much.
        fatigue: 0.55,

        // Needs - Stress
        // Significant resource loss triggers substantial HPA axis activation through threat perception (survival and status), with modest permanent shift reflecting both partial adaptation and lasting vigilance about financial security.
        // Selye, H. (1956). The stress of life; McEwen, B.S. (1998). Stress, adaptation, and disease.
        stress: 0.55,

        // Needs - Purpose
        // Significant resource loss disrupts financial security and future goal planning, creating temporary purpose disorientation, with substantial recovery as individuals adapt and find alternative meaning sources within 1-2 years.
        // Frankl, V.E. (1959). Man's Search for Meaning; Ryff, C.D. (1989). Happiness is Everything, or is it? Purpose in Life.
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Significant resource loss increases loneliness through shame-induced social withdrawal and reduced social participation capacity, with modest permanence due to hedonic adaptation and potential financial recovery.
        // Desmond, M. (2016). Evicted: Poverty and Profit in the American City.
        loneliness: 0.35,

        // Social Cognition - PRC
        // Significant resource loss creates perception of being judged and unable to reciprocate in relationships, decreasing PRC, but genuine close support and time typically restore this perception.
        // Joiner, T. (2005). Why People Die by Suicide; van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Significant resource loss creates moderate-to-significant perceived liability through dependency dynamics and inability to contribute, but the burden perception resolves substantially once financial recovery occurs.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide. Psychological Review, 117(2), 575-600.
        perceived_liability: 0.38,

        // Social Cognition - Self Hate
        // Significant resource loss triggers self-blame for poor judgment and financial mismanagement, creating moderate shame without the severity of moral failure, with some chronic rumination but near-complete hedonic adaptation within 1-2 years.
        // Joiner, T. (2005). Why People Die by Suicide.
        self_hate: 0.25,

        // Social Cognition - Perceived Competence
        // Significant resource loss delivers explicit evidence of economic incompetence and reduced life control, undermining perceived competence more severely than chronic strain; however, external attribution and recovery potential through rebuilding limit permanence.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; Conger & Conger (2002). Financial stress cascade model; Mullainathan & Shafir (2013). Scarcity.
        perceived_competence: -0.40,

        // Mental Health - Depression
        // Resource loss is a significant stressor triggering moderate depressive symptoms through identity threat and autonomy reduction, with most recovery occurring within months despite modest permanent baseline shift.
        // Brown, G.W. & Harris, T. (1978). Social Origins of Depression: A Study of Psychiatric Disorder in Women.
        depression: 0.35,

        // Mental Health - Self Worth
        // Significant resource loss creates moderate temporary blow to self-worth through shame and diminished social status, but does not fundamentally threaten core identity; hedonic adaptation and alternative sources of self-worth allow substantial recovery within 12-24 months.
        // Crocker, J. & Wolfe, C.T. (2001). Contingencies of self-worth. Psychological Review, 108(3), 593-623.
        self_worth: -0.25,

        // Mental Health - Hopelessness
        // Significant resource loss creates sustained pessimism about future economic security and reduced capability to pursue goals, but most individuals recover within 1-2 years through financial recovery or adaptation.
        // Abramson, L.Y. et al. (1989). Hopelessness depression.
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Significant resource loss increases interpersonal hopelessness primarily through shame-driven withdrawal and social status threat, but recovery occurs as material circumstances stabilize and shame-based beliefs diminish.
        // Joiner, T. (2005). Why People Die by Suicide; Rickwood, D. et al. (2005) on shame as barrier to help-seeking.
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Significant resource loss is a psychological/economic stressor with no direct exposure to physical pain, injury, violence, or death proximity, therefore does not affect acquired capability.
        // Joiner, T. (2005). Why People Die by Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Significant resource loss depletes self-regulatory resources through acute emotional distress and chronic financial stress, with partial recovery through adaptation and time.
        // Baumeister, R.F. et al. (1998) on ego depletion; Muraven & Baumeister (2000) on self-regulatory resource limitation.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Significant resource loss triggers temporary cognitive load and self-focus that mildly reduces empathic capacity, with most recovery within months as adaptation occurs.
        // Mullainathan & Shafir (2013) "Scarcity: Why Having Too Little Means So Much".
        empathy: -0.18,

        // Disposition - Aggression
        // Significant resource loss blocks goals and threatens identity, triggering moderate frustration-based aggression with mild hedonic adaptation; chronic while disadvantage persists but most impact fades within 1-2 years.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis.
        aggression: 0.35,

        // Disposition - Grievance
        // Material loss activates grievance through relative deprivation and perceived unfairness, especially if systemic or unjust in origin, with moderate permanence due to hedonic adaptation tempering emotional impact over time.
        // Lind, E.A. & Tyler, T.R. (1988). The social psychology of procedural justice; Stouffer, S.A. et al. (1949). The American Soldier.
        grievance: 0.35,

        // Disposition - Reactance
        // Significant resource loss moderately increases reactance through perceived loss of control and constrained options, but most individuals experience substantial hedonic adaptation within 1-2 years as they adjust spending patterns and rebuild agency.
        // Brehm, J.W. (1966). A Theory of Psychological Reactance; Brehm, S.S. & Brehm, J.W. (1981). Psychological Reactance.
        reactance: 0.25,

        // Disposition - Trust Propensity
        // Significant resource loss increases wariness about future vulnerability and exploitation risk, reducing generalized trust propensity; however, it lacks direct interpersonal betrayal component and shows substantial recovery within 1-2 years.
        // Rotter, J.B. (1967). A new scale for the measurement of interpersonal trust; Putnam & Feldstein (2003). Better together.
        trust_propensity: -0.22,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: false,
        loneliness: true,
        prc: false,
        perceived_liability: true,
        self_hate: true,
        perceived_competence: false,
        depression: true,
        self_worth: false,
        hopelessness: true,
        interpersonal_hopelessness: false,
        impulse_control: true,
        empathy: false,
        aggression: true,
        grievance: true,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.10,
        dominance: 0.12,
        fatigue: 0.11,
        stress: 0.12,
        purpose: 0.08,
        loneliness: 0.12,
        prc: 0.08,
        perceived_liability: 0.10,
        self_hate: 0.08,
        perceived_competence: 0.10,
        depression: 0.10,
        self_worth: 0.06,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.05,
        impulse_control: 0.12,
        empathy: 0.05,
        aggression: 0.12,
        grievance: 0.12,
        reactance: 0.05,
        trust_propensity: 0.07,
    },
};
