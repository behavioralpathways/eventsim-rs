//! ExperienceRecessionEconomic event specification.
//!
//! An economic recession - a significant decline in economic activity spread across the economy
//! lasting months to years, characterized by job losses, reduced income, financial insecurity,
//! and widespread uncertainty about economic recovery and future prospects.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Economic recessions produce severe but time-limited negative affect through financial insecurity, employment threat, and uncertainty; hedonic adaptation post-recovery results in minimal permanent baseline shift despite temporary elevated depression rates.
        // Luo, Ha & Barnes (2021) Review of Economic Studies; WHO recession mental health assessments; Cheng et al. (2012) Journal of Epidemiology & Community Health
        valence: -0.60,

        // Mood - Arousal
        // Economic recessions create sustained moderate-to-high physiological activation through uncertainty and threat perception; arousal gradually returns to baseline through adaptation, with small permanent shift for those experiencing direct financial harm.
        // Carvalho, Meier & Wang (2016) Journal of Finance; COVID-era financial stress studies
        arousal: 0.55,

        // Mood - Dominance
        // Economic recession imposes external loss of control over employment and finances, creating significant perceived powerlessness that is partially chronic during recession periods but recovers after economic recovery.
        // Bandura (1977) Self-efficacy: Toward a unifying theory of behavioral change
        dominance: -0.55,

        // Needs - Fatigue
        // Economic recessions trigger sustained stress, financial rumination, and disrupted sleep patterns, producing moderate mental and physical fatigue; most individuals recover within 1-2 years post-recession as economic conditions stabilize.
        // Hockey (2013) Psychology of fatigue; Volkert et al. (2018) on economic hardship and mental health
        fatigue: 0.35,

        // Needs - Stress
        // Economic recessions trigger sustained HPA axis activation through job/financial threat and unpredictability, placing stress in the high range; collective experience and eventual economic recovery enable substantial hedonic adaptation.
        // McEwen (1998) Stress, adaptation, and disease: Allostasis and allostatic load; Cohen & Janicki-Deverts (2012) on economic stress
        stress: 0.56,

        // Needs - Purpose
        // Economic recession disrupts employment-based identity and future planning, but most individuals recover purpose within 1-2 years as conditions improve, with minimal permanent base shift.
        // Jahoda (1982) Employment and unemployment: A social-psychological analysis; Steger & Kashdan (2013) on life meaning during economic stress
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Economic recession moderately increases loneliness through shame-induced social withdrawal and reduced social participation, but most people recover within 1-2 years as economic conditions improve.
        // Cacioppo & Patrick (2008) Loneliness: Human Nature and the Need for Social Connection; Joiner (2005) Why People Die by Suicide
        loneliness: 0.35,

        // Social Cognition - PRC
        // Economic recession reduces reciprocal caring capacity through financial stress and emotional withdrawal, but effects partially recover as economies stabilize and communities mobilize support.
        // Joiner (2005) Why People Die by Suicide; Hampton & Marshall (2000) on recession's psychological impact
        prc: -0.25,

        // Social Cognition - Perceived Liability
        // Economic recessions increase perceived liability through financial dependency and reduced contribution capacity, but effects show substantial recovery as economic conditions normalize and employment stabilizes.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide; Joiner et al. (2009) on financial strain as ITS antecedent
        perceived_liability: 0.32,

        // Social Cognition - Self Hate
        // Economic recession triggers moderate self-blame and shame through financial stress and job loss, but external attribution and hedonic adaptation limit permanence to small base shifts.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide; research on unemployment and self-blame during 2008-2009 financial crisis
        self_hate: 0.25,

        // Social Cognition - Perceived Competence
        // Economic recessions reduce mastery experiences and create negative performance feedback about competence, particularly in work domains, with moderate permanence due to hedonic adaptation and eventual job recovery.
        // Bandura (1997) Self-efficacy: The exercise of control
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Economic recessions trigger significant depressive symptoms through financial strain and uncertainty, with potential chronicity during sustained economic stress, but most recovery occurs within 1-2 years post-recovery.
        // Kendler et al. (1999) Causal relationship between stressful life events and depression; Brown & Harris (1978) Social origins of depression
        depression: 0.35,

        // Mental Health - Self Worth
        // Economic recession threatens identity-relevant financial competence and economic security, creating moderate self-worth reduction with sustained stress but substantial recovery upon economic stabilization.
        // Crocker & Wolfe (2001) Contingencies of self-worth; Selye's chronic stress framework
        self_worth: -0.28,

        // Mental Health - Hopelessness
        // Economic recessions create significant but temporary hopelessness through loss of income and perceived reduced future opportunity, with substantial recovery as economic conditions improve.
        // Abramson et al. (1989) Hopelessness depression: A theory-based subtype of depression; 2008 financial crisis studies
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Economic recession creates moderate interpersonal hopelessness through shame-driven help-seeking avoidance and burden beliefs, with near-complete recovery as economic conditions stabilize.
        // Joiner (2005) Why People Die by Suicide; Cutrona et al. (2005) Social Support and Adaptation to Stress; Vinokur & van Ryn (1993)
        interpersonal_hopelessness: 0.32,

        // Mental Health - Acquired Capability
        // Economic recessions create psychological distress and financial hardship but do not expose individuals to physical pain, injury, or death proximity, which are the mechanisms by which acquired capability develops.
        // Joiner (2005) Why People Die by Suicide - Acquired capability requires habituation through direct exposure to pain/death
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Financial stress depletes self-regulatory resources impairing impulse control, but hedonic adaptation and recovery of executive function restore capacity within 1-2 years.
        // Baumeister & Vohs (2007) Self-Regulation and Self-Control; Muraven & Baumeister (2000) on self-regulation depletion
        impulse_control: -0.28,

        // Disposition - Empathy
        // Economic recession increases financial stress and self-focus, temporarily reducing empathic capacity through cognitive load and threat response, with most individuals recovering as economic stability returns.
        // Mullainathan & Shafir (2013) Scarcity: Why Having Too Little Means So Much
        empathy: -0.15,

        // Disposition - Aggression
        // Economic recession triggers frustration and blocked goals, increasing state aggression through the frustration-aggression mechanism, though most individuals recover within 12-18 months as economic conditions stabilize.
        // Berkowitz (1989) Frustration-aggression hypothesis; Anderson & Bushman (2002) Human Aggression framework
        aggression: 0.38,

        // Disposition - Grievance
        // Economic recession creates moderate grievance through systemic blame attribution, unfair distribution of costs, and violated expectations about fairness; impacts show partial recovery as conditions improve.
        // Mikula (1993) On the experience of injustice; Lind & Tyler (1988) The social psychology of procedural justice
        grievance: 0.45,

        // Disposition - Reactance
        // Economic recession imposes significant external constraints on financial choices and future planning, triggering moderate reactance to lost autonomy; effects persist while economic conditions remain tight but show partial recovery with job stabilization.
        // Brehm & Brehm (1981) Psychological reactance: A theory of freedom and control
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Economic recessions significantly damage trust in institutions and increase interpersonal wariness through economic uncertainty and scarcity-induced competition, but most individuals show substantial recovery within 1-2 years.
        // Rotter (1967) interpersonal trust theory; Rousseau et al. (1998) on systemic vs. interpersonal trust damage
        trust_propensity: -0.30,
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
        perceived_liability: false,
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
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.10,
        arousal: 0.08,
        dominance: 0.12,
        fatigue: 0.12,
        stress: 0.10,
        purpose: 0.05,
        loneliness: 0.06,
        prc: 0.08,
        perceived_liability: 0.10,
        self_hate: 0.08,
        perceived_competence: 0.12,
        depression: 0.12,
        self_worth: 0.09,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.10,
        impulse_control: 0.07,
        empathy: 0.08,
        aggression: 0.12,
        grievance: 0.22,
        reactance: 0.12,
        trust_propensity: 0.08,
    },
};
