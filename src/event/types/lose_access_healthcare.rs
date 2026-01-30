//! LoseAccessHealthcare event specification.
//!
//! Losing access to healthcare services - this could be due to losing insurance,
//! healthcare provider leaving, facility closure, becoming ineligible for coverage,
//! or inability to afford care. Creates significant psychological burden through
//! health uncertainty, loss of control, and institutional betrayal.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Loss of healthcare access is a severe stressor causing acute anxiety and sadness, with chronic ongoing distress from health management uncertainty and persistent financial strain.
        // Watson & Tellegen (1985) mood structure; Kahneman & Tversky (1979) Prospect Theory on loss aversion.
        valence: -0.58,

        // Mood - Arousal
        // Loss of healthcare access triggers significant anxiety and vigilance (activating threat response) with sustained worry, but shows substantial hedonic adaptation as coping strategies develop.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Russell, J.A. (1980). A circumplex model of affect.
        arousal: 0.48,

        // Mood - Dominance
        // Losing healthcare access represents significant loss of control over health outcomes and life circumstances, though not complete powerlessness; individuals retain some agency in seeking alternatives.
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change; Deci, E.L. & Ryan, R.M. (1985). Intrinsic motivation and self-determination.
        dominance: -0.55,

        // Needs - Fatigue
        // Losing healthcare access creates sustained cognitive/emotional burden from health worry and self-management, with significant mental fatigue but most recovery following care restoration or adaptation.
        // Ego depletion model (Baumeister et al., 1998); psychological load theory applied to health anxiety.
        fatigue: 0.38,

        // Needs - Stress
        // Losing healthcare access triggers sustained physiological stress through threat perception and loss of control (comparable to major life disruptions like job loss), but shows hedonic adaptation within 1-2 years.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load.
        stress: 0.55,

        // Needs - Purpose
        // Loss of healthcare access creates significant uncertainty about future health trajectory and constrains life goal planning, but effects largely reverse upon access restoration.
        // Steger, M.F. et al. (2006). The Meaning in Life Questionnaire; Frankl, V.E. (1959). Man's Search for Meaning.
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Loss of healthcare access disrupts therapeutic relationships and encourages health-related social withdrawal, but most individuals adapt or regain access within 1-2 years.
        // Cacioppo & Patrick (2008). Loneliness: Human nature and the need for social connection.
        loneliness: 0.28,

        // Social Cognition - PRC
        // Losing healthcare access signals institutional withdrawal of care and creates shame-driven social withdrawal, moderately reducing perceived reciprocal caring through sustained stress and loss of system support signals.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        prc: -0.28,

        // Social Cognition - Perceived Liability
        // Loss of healthcare access creates moderate-to-significant dependency on others for untreated medical conditions and care coordination, triggering guilt and burden perception comparable to unmanaged illness.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        perceived_liability: 0.48,

        // Social Cognition - Self Hate
        // Losing healthcare access triggers moderate self-blame about personal responsibility for health maintenance and internalized shame about vulnerability, but external attribution dominates.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        self_hate: 0.28,

        // Social Cognition - Perceived Competence
        // Loss of healthcare access undermines perceived competence in health management and signals reduced life control; most individuals adapt within months to a year through alternative coping strategies.
        // Bandura, A. (1997). Self-efficacy: The exercise of control.
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Losing healthcare access causes significant depressive symptoms through loss of treatment, reduced control, and hopelessness, but most people show recovery within 1-2 years as they access alternatives.
        // Sommers, B.D. et al. (2017). Insurance Transitions; Abramson, L.Y. et al. (1978). Learned Helplessness in Humans.
        depression: 0.32,

        // Mental Health - Self Worth
        // Losing healthcare access triggers moderate identity-level shame through internalized messages about unworthiness and loss of self-care agency, but most individuals adapt through alternative resources.
        // Crocker, J. & Wolfe, C.T. (2001). Contingencies of self-worth.
        self_worth: -0.28,

        // Mental Health - Hopelessness
        // Loss of healthcare access reduces perceived controllability over future health outcomes and creates a sense of entrapment, producing significant hopelessness similar to other major life disruptions.
        // Beck, A.T. & Steer, R.A. (1988). Beck Hopelessness Scale; Institute of Medicine (2009). America's Uninsured Crisis.
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Losing healthcare access reduces belief in social support availability through institutional denial and shame-based withdrawal, but doesn't directly signal that individuals won't help.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Eisenberger, N.I. et al. (2003). Social pain/rejection mechanisms.
        interpersonal_hopelessness: 0.38,

        // Mental Health - Acquired Capability
        // Loss of healthcare access does not involve direct exposure to physical pain, injury, violence, or death; it is a deprivation event affecting other ITS dimensions but not the habituation pathway.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Acute stress and potential medication discontinuation significantly impair self-regulation, but most individuals show substantial recovery as they adapt or restore healthcare access.
        // Hofmann, W. et al. (2012). Executive functions and self-regulation. Trends in Cognitive Sciences.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Losing healthcare access increases self-focused threat attention and financial stress, temporarily reducing perspective-taking and emotional openness, though hedonic adaptation typically restores baseline.
        // Singer, T. & Klimecki, O.M. (2014). Empathy and compassion; Mullainathan, S. & Shafir, E. (2013). Scarcity.
        empathy: -0.22,

        // Disposition - Aggression
        // Healthcare access loss creates moderate frustration-aggression through blocked goals, perceived injustice, and potential exacerbation of untreated conditions that increase irritability.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson, C.A. & Bushman, B.J. (2002). Human aggression.
        aggression: 0.35,

        // Disposition - Grievance
        // Healthcare access loss creates significant grievance through perceived systemic injustice and institutional betrayal of fundamental care needs, with chronic resentment during continued deprivation.
        // Lind & Tyler (1988). The social psychology of procedural justice; Mikula (1993). On the experience of injustice.
        grievance: 0.58,

        // Disposition - Reactance
        // Loss of healthcare access constitutes a significant involuntary constraint on health autonomy and freedom of medical choice, triggering reactance through imposed limitation on a critical life domain.
        // Brehm, J.W. (1966). A theory of psychological reactance.
        reactance: 0.38,

        // Disposition - Trust Propensity
        // Loss of healthcare access represents significant institutional betrayal and vulnerability exposure that damages trust in systems, but generalized interpersonal trust shows near-complete recovery.
        // Rotter, J.B. (1967). A new scale for the measurement of interpersonal trust; Mayer, R.C. et al. (1995). An integrative model of organizational trust.
        trust_propensity: -0.35,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: true,
        loneliness: false,
        prc: true,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: true,
        depression: true,
        self_worth: true,
        hopelessness: true,
        interpersonal_hopelessness: true,
        impulse_control: true,
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.18,
        arousal: 0.12,
        dominance: 0.12,
        fatigue: 0.08,
        stress: 0.12,
        purpose: 0.08,
        loneliness: 0.06,
        prc: 0.10,
        perceived_liability: 0.18,
        self_hate: 0.08,
        perceived_competence: 0.12,
        depression: 0.12,
        self_worth: 0.08,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.08,
        empathy: 0.08,
        aggression: 0.12,
        grievance: 0.18,
        reactance: 0.12,
        trust_propensity: 0.08,
    },
};
