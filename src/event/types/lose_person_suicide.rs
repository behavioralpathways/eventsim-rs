//! LosePersonSuicide event specification.
//!
//! Loss of a loved one to suicide. This represents one of the most traumatic forms of
//! bereavement, characterized by unique stressors including stigma, guilt, self-blame,
//! complicated grief, and elevated risk for the survivor's own mental health.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Suicide bereavement creates more profound and prolonged valence loss than natural death due to compounded guilt, stigma, and complicated grief.
        // Stroebe & Stroebe (2005) Complicated Grief; Cerel et al. (2009) Suicide Loss: Survivor Grief
        valence: -0.92,

        // Mood - Arousal
        // Suicide bereavement involves trauma exposure with hyperarousal, hypervigilance, and PTSD-like symptoms that exceed general bereavement.
        // Stroebe et al. (2007) Handbook of Bereavement Research and Practice
        arousal: 0.78,

        // Mood - Dominance
        // Suicide loss creates complete perceived loss of control coupled with the paradox of self-blame, producing profound helplessness.
        // Joiner's ITS; Andriessen & Krysinska (2012) on complicated grief following suicide
        dominance: -0.85,

        // Needs - Fatigue
        // Suicide bereavement combines traumatic grief processing, rumination about "why," and sleep disruption, creating severe exhaustion.
        // Grief literature on suicide bereavement as highest-severity grief form
        fatigue: 0.65,

        // Needs - Stress
        // Suicide loss triggers severe HPA axis activation through unpredictability, trauma, and stigma with sustained elevated cortisol patterns.
        // McEwen (1998) Stress, adaptation, and disease; Holmes & Rahe stress scale
        stress: 0.85,

        // Needs - Purpose
        // Suicide loss creates severe existential crisis questioning life meaning itself, with chronic grief and prolonged identity reconstruction.
        // Calhoun & Tedeschi (2004) Posttraumatic growth; Neimeyer (2001) Meaning reconstruction
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Suicide bereavement creates multilayered isolation through stigma-induced social withdrawal, disenfranchised grief, and shame-based concealment.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide; Cerel et al. (2018)
        loneliness: 0.65,

        // Social Cognition - PRC
        // Suicide bereavement reduces perceived reciprocal care through social stigma and abandonment feelings, creating greater care deficit than natural death.
        // Van Orden et al. (2010); Joiner (2005) Why People Die by Suicide
        prc: -0.40,

        // Social Cognition - Perceived Liability
        // Suicide survivors internalize the deceased's burdensomeness and fear their grief isolates them, creating moderate but persistent perceived liability.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide; Joiner (2005)
        perceived_liability: 0.32,

        // Social Cognition - Self Hate
        // Suicide bereavement triggers intense guilt and self-blame through assumptive counterfactual thinking ("I should have prevented this").
        // Joiner (2005) Why People Die by Suicide
        self_hate: 0.65,

        // Social Cognition - Perceived Competence
        // Suicide bereavement triggers severe guilt-driven competence collapse in relationships and prevention, with chronic rumination maintaining doubt.
        // Joiner (2005); Cerel & Aldrich (2011); Andriessen et al. (2017)
        perceived_competence: -0.65,

        // Mental Health - Depression
        // Suicide loss triggers severe depressive symptoms with higher magnitude than other deaths; sustained grief trajectory.
        // Kendler et al. (1999) stressful life events and depression; suicide bereavement literature
        depression: 0.75,

        // Mental Health - Self Worth
        // Suicide loss triggers intense guilt, shame, and identity-level questioning of personal worth, with rumination patterns preventing full recovery.
        // Cain (1972) Survivors of Suicide; Neimeyer et al. (2006) meaning reconstruction in suicide loss
        self_worth: -0.55,

        // Mental Health - Hopelessness
        // Losing someone to suicide creates severe hopelessness by validating the deceased's hopeless worldview and introducing existential dread.
        // O'Connor & Kirtley (2018) integrated motivational-volitional model
        hopelessness: 0.55,

        // Mental Health - Interpersonal Hopelessness
        // Suicide loss creates both acute hopelessness about relationships (fear of future losses) and chronic interpersonal withdrawal through shame.
        // Joiner (2005); Van Orden et al. (2010)
        interpersonal_hopelessness: 0.65,

        // Mental Health - Acquired Capability
        // Witnessing a completed suicide exposes the bereaved to death and normalizes suicide as an option, creating moderate habituation.
        // Joiner (2005); Van Orden et al. (2010); bereavement by suicide is a documented risk factor
        acquired_capability: 0.35,

        // Disposition - Impulse Control
        // Suicide bereavement causes severe acute disruption to executive function through emotional dysregulation and resource depletion.
        // Baumeister et al. (1998) ego depletion; Muraven & Baumeister (2000) self-regulation
        impulse_control: -0.65,

        // Disposition - Empathy
        // Suicide loss typically triggers acute grief and temporary self-focus, but many survivors develop lasting enhanced empathy for others' suffering.
        // Davis (1983) empathy measurement; Tedeschi & Calhoun (2004) post-traumatic growth
        empathy: 0.15,

        // Disposition - Aggression
        // Suicide loss generates sustained anger from perceived injustice and thwarted expectations, with substantial recovery over time.
        // Berkowitz (1989) frustration-aggression hypothesis; complicated grief literature
        aggression: 0.35,

        // Disposition - Grievance
        // Suicide loss creates severe grievance through dual injustices (abandonment by deceased, social stigma), tempered by eventual compassion.
        // Cerel et al. (2018) Suicide Bereavement: Violence, Stigma and Survivors
        grievance: 0.65,

        // Disposition - Reactance
        // Suicide bereavement reduces immediate autonomy but triggers moderate secondary reactance against unsolicited advice during grief processing.
        // Joiner (2005); Neimeyer (2000); Andriessen et al. (2017) Postvention in Action
        reactance: 0.15,

        // Disposition - Trust Propensity
        // Suicide loss involves profound betrayal of trust (someone you relied on chose permanent abandonment) that typically creates lasting harm.
        // Joiner (2005); Shear et al. (2011) Complicated Grief in DSM-5
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
        valence: 0.35,
        arousal: 0.32,
        dominance: 0.25,
        fatigue: 0.25,
        stress: 0.25,
        purpose: 0.28,
        loneliness: 0.25,
        prc: 0.25,
        perceived_liability: 0.25,
        self_hate: 0.28,
        perceived_competence: 0.35,
        depression: 0.35,
        self_worth: 0.25,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.28,
        impulse_control: 0.25,
        empathy: 0.12,
        aggression: 0.18,
        grievance: 0.35,
        reactance: 0.25,
        trust_propensity: 0.38,
    },
};
