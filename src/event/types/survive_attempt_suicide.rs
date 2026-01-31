//! SurviveAttemptSuicide event specification.
//!
//! Surviving a suicide attempt - the individual attempted to end their life
//! but survived, representing a profound psychological crisis with lasting
//! impacts on acquired capability, perceived burdensomeness, and all
//! psychological dimensions.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Surviving a suicide attempt is catastrophic-range negative (peak psychological crisis) with persistent distress, though long-term recovery is possible with intervention.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4668226/
        valence: -0.92,

        // Mood - Arousal
        // Surviving a suicide attempt triggers acute fight-or-flight activation that often transitions to chronic hypervigilance and anxiety disorder symptoms.
        // https://www.nimh.nih.gov/
        arousal: 0.75,

        // Mood - Dominance
        // Surviving a suicide attempt reflects a paradoxical loss of agency (failed intent) coupled with the fundamental powerlessness that precipitated the attempt.
        // Joiner's Interpersonal Theory of Suicide
        dominance: -0.65,

        // Needs - Fatigue
        // Surviving a suicide attempt creates acute physical exhaustion from the event itself plus chronic fatigue from ongoing emotional processing, sleep disruption, and sustained rumination.
        // https://www.ncbi.nlm.nih.gov/pubmed/23347468
        fatigue: 0.72,

        // Needs - Stress
        // Surviving a suicide attempt is a severe threat activation event triggering maximum HPA axis response with persistent hypervigilance and elevated cortisol.
        // ITS framework on acquired capability and lethality factors
        stress: 0.85,

        // Needs - Purpose
        // Surviving a suicide attempt creates acute existential disruption and meaning crisis, with research showing significant long-term difficulties in purpose re-integration.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5630188/
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Survivors experience acute isolation from shame, stigma, and perceived rejection, compounded by thwarted belongingness - a core ITS risk factor.
        // Joiner (2005). Why People Die by Suicide
        loneliness: 0.65,

        // Social Cognition - PRC
        // Surviving attempt triggers immediate crisis response increasing perceived caring short-term, but effect diminishes as attention fades and survivors often internalize shame.
        // Joiner (2005). Why People Die by Suicide - belongingness paradox
        prc: 0.15,

        // Social Cognition - Perceived Liability
        // Suicide attempt survivors experience acute guilt and shame about the burden imposed on family/caregivers through recovery needs, emotional strain, and costs.
        // https://doi.org/10.1037/a0013560 (Joiner et al., 2009)
        perceived_liability: 0.65,

        // Social Cognition - Self Hate
        // Suicide attempt survivors typically experience intense shame, self-blame, and contempt directed at themselves for the attempt - a primary component of Perceived Burdensomeness.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5994416/
        self_hate: 0.75,

        // Social Cognition - Perceived Competence
        // Surviving a suicide attempt represents a catastrophic failure in problem-solving and adaptive coping, producing severe perceived incompetence.
        // Joiner's ITS; Hawton & Harriss suicide attempt recovery literature
        perceived_competence: -0.65,

        // Mental Health - Depression
        // Surviving a suicide attempt intensifies depressive symptoms through trauma, shame, and persistent hopelessness, with high chronicity and long-term permanence.
        // Joiner (2005); Nock et al. on post-attempt depression trajectories
        depression: 0.65,

        // Mental Health - Self Worth
        // Surviving a suicide attempt creates profound identity-level self-evaluation crisis through shame, perceived failure, and fundamental worthlessness cognitions.
        // ITS framework on perceived burdensomeness
        self_worth: -0.70,

        // Mental Health - Hopelessness
        // Surviving a suicide attempt typically signals pre-existing severe hopelessness that persists post-attempt, with initial relief often followed by return of hopelessness.
        // https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4879307/
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Surviving a suicide attempt increases interpersonal hopelessness through shame-driven help-seeking avoidance and belief that relationships cannot protect against future crises.
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC4244874/
        interpersonal_hopelessness: 0.62,

        // Mental Health - Acquired Capability
        // Surviving a suicide attempt produces substantial habituation to pain and death, permanently elevating acquired capability - prior attempts are the strongest predictor of future attempts.
        // Joiner (2005). Why People Die by Suicide; meta-analytic evidence
        acquired_capability: 0.90,

        // Disposition - Impulse Control
        // Suicide attempts reflect severe impulse control deficits that persist post-attempt but can improve substantially with intervention and time.
        // Negative urgency pathway literature; cognitive-behavioral research on suicide
        impulse_control: -0.65,

        // Disposition - Empathy
        // Suicide attempt survivors experience acute trauma-induced emotional numbing and self-focused attention that temporarily reduce empathy capacity.
        // https://www.frontiersin.org/journals/psychiatry/articles/10.3389/fpsyt.2025.1605508/full
        empathy: -0.20,

        // Disposition - Aggression
        // Suicide attempt survivors typically experience reduced outward aggression due to shame and self-directed hostility, though frustration can produce moderate anger episodes.
        // Psychological literature on suicide and aggression direction
        aggression: 0.15,

        // Disposition - Grievance
        // Suicide attempt survivors often develop sustained perceptions of external injustice, particularly regarding relationships and support systems that failed them.
        // Joiner's ITS framework; post-trauma attribution literature
        grievance: 0.45,

        // Disposition - Reactance
        // Involuntary hospitalization and forced psychiatric treatment after a suicide attempt create severe psychological reactance as individuals lose autonomy over fundamental life decisions.
        // Brehm (1966) reactance theory; involuntary commitment research
        reactance: 0.65,

        // Disposition - Trust Propensity
        // Survivors experience substantial trust damage from perceived abandonment by close others, shame-based social withdrawal, and loss of trust in their own safety perceptions.
        // https://doi.org/10.1016/j.psychres.2013.08.005
        trust_propensity: -0.65,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: true,
        purpose: true,
        loneliness: true,
        prc: false,
        perceived_liability: true,
        self_hate: true,
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
        valence: 0.65,
        arousal: 0.85,
        dominance: 0.55,
        fatigue: 0.65,
        stress: 0.70,
        purpose: 0.55,
        loneliness: 0.75,
        prc: 0.35,
        perceived_liability: 0.60,
        self_hate: 0.70,
        perceived_competence: 0.70,
        depression: 0.75,
        self_worth: 0.65,
        hopelessness: 0.65,
        interpersonal_hopelessness: 0.24,
        impulse_control: 0.45,
        empathy: 0.18,
        aggression: 0.45,
        grievance: 0.70,
        reactance: 0.75,
        trust_propensity: 0.70,
    },
};
