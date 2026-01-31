//! WitnessTraumaSevere event specification.
//!
//! Witnessing severe trauma - seeing someone else experience extreme violence, serious injury,
//! death, or a horrific accident. Examples include witnessing a fatal car accident, seeing
//! someone murdered, watching a loved one die violently, or observing severe violence against
//! another person.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Witnessing severe trauma creates catastrophic acute negative valence through empathetic activation and threat response, with moderate permanence reflecting substantial adaptation within 1-2 years but measurable chronic baseline shift from exposure.
        // Figley, C.R. (1995). Compassion Fatigue: Coping with Secondary Traumatic Stress Disorder; Stamm, B.H. (2010). The Concise ProQOL Manual
        valence: -0.70,

        // Mood - Arousal
        // Witnessing severe trauma produces extreme physiological activation comparable to direct threat exposure, with 35-50% of witnesses developing sustained hyperarousal symptoms consistent with PTSD diagnosis.
        // DSM-5 PTSD criteria; Breslau et al. (2004) on trauma exposure and PTSD prevalence; van der Kolk et al. (2005) on hyperarousal
        arousal: 0.85,

        // Mood - Dominance
        // Witnessing severe trauma creates acute and lasting powerlessness stemming from inability to prevent or intervene in violence/death, establishing chronic loss of perceived control and agency.
        // Figley, C.R. (1995). Compassion Fatigue as Secondary Traumatic Stress Disorder; DSM-5 PTSD criteria on loss of control
        dominance: -0.65,

        // Needs - Fatigue
        // Witnessing severe trauma causes acute emotional and physiological exhaustion through sympathetic activation and emotional processing, with significant but partially recoverable fatigue; chronic component reflects ongoing hypervigilance and intrusive memories.
        // DSM-5 Acute Stress Disorder criteria; Figley (1995); Stamm (2010) on vicarious trauma fatigue
        fatigue: 0.68,

        // Needs - Stress
        // Witnessing severe trauma triggers maximal acute stress response through HPA axis activation and threat perception, but most witnesses show significant recovery without PTSD development, leaving moderate residual elevation in baseline stress sensitivity.
        // DSM-5 Acute Stress Disorder criteria; Yehuda et al. (2006) on trauma witness physiological responses; Ehlers & Clark (2000)
        stress: 0.88,

        // Needs - Purpose
        // Witnessing severe trauma typically creates immediate existential questioning and loss of meaning, with enduring effects that persist unless actively reconstructed through post-traumatic growth processes.
        // Tedeschi & Calhoun (2004) on post-traumatic growth; Figley (1995); Janoff-Bulman (1992) on shattered assumptions
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Witnessing severe trauma typically triggers social withdrawal and disrupted sense of connection due to difficulty communicating experiences to others and emotional overwhelm, creating moderate lasting loneliness.
        // van der Kolk, B.A. (2014). The Body Keeps the Score; Tedeschi & Calhoun (2004) on posttraumatic growth
        loneliness: 0.35,

        // Social Cognition - PRC
        // Witnessing severe trauma erodes perceived reciprocal caring through exposure to human cruelty and inadequate validation from social networks, creating lasting uncertainty about whether others truly care.
        // Figley, C.R. (1995). Compassion fatigue; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        prc: -0.30,

        // Social Cognition - Perceived Liability
        // Witnessing severe trauma triggers secondary PTSD, survivor guilt, and existential questioning about one's helplessness, creating moderate-to-significant burden perception that persists over weeks to months.
        // Figley, C.R. (1995). Compassion fatigue; Pearlman & Saakvitne (1995) on vicarious trauma and identity disruption
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Witnessing severe trauma creates substantial guilt through bystander self-blame and perceived moral failure, producing chronic self-directed shame that persists due to rumination over inaction and survivor's guilt.
        // van der Kolk, B.A. (2014). The Body Keeps the Score; Tedeschi & Calhoun (2004) on survivor guilt
        self_hate: 0.45,

        // Social Cognition - Perceived Competence
        // Witnessing severe trauma undermines perceived control and environmental mastery while increasing cognitive load, producing moderate competence doubt with recovery possible through reorientation and mastery rebuilding.
        // Bride, B.E. et al. (2004). Vicarious Traumatization in Therapists Treating Trauma Survivors; Figley's secondary traumatic stress framework
        perceived_competence: -0.38,

        // Mental Health - Depression
        // Witnessing severe trauma produces significant acute depression through empathetic distress and intrusive memories, with substantial baseline elevation lasting months and chronic features in approximately 15% of cases.
        // DSM-5 criteria; Tedeschi & Calhoun on post-traumatic stress and depression comorbidity; Bride et al. on secondary traumatic stress
        depression: 0.65,

        // Mental Health - Self Worth
        // Witnessing severe trauma triggers helplessness and existential disruption that moderately diminishes self-worth through secondary traumatic stress and survivor guilt, with effects that persist but remain less severe than direct victimization.
        // Figley (1995) on secondary traumatic stress; McCann & Pearlman (1990) on vicarious trauma theory
        self_worth: -0.25,

        // Mental Health - Hopelessness
        // Witnessing severe trauma fundamentally shakes assumptions about safety and control, producing significant immediate hopelessness about future security and world predictability; effects are chronic for most witnesses but show gradual recovery.
        // Janoff-Bulman, R. (1989). Assumptive Worlds and the Stress of Traumatic Events; Ehlers & Clark (2000) on cognitive processing of trauma
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Witnessing severe trauma creates moderate, persistent interpersonal hopelessness by demonstrating that relationships cannot prevent catastrophic harm, though support networks remain valued as coping mechanisms.
        // Tedeschi & Calhoun (1996) on posttraumatic growth; Figley (1995); Herman (1992) on trauma's impact on attachment
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Witnessing severe trauma creates vicarious habituation to pain and death through observational learning, producing mild-to-moderate acquired capability that is permanent but weaker than direct victimization.
        // Joiner, T.E. (2005). Why People Die by Suicide; Van Orden, K.A. et al. (2010). The Interpersonal Theory of Suicide
        acquired_capability: 0.32,

        // Disposition - Impulse Control
        // Witnessing severe trauma triggers acute stress response and acute impulse dysregulation through empathic distress and threat perception, but with significantly lower permanence than direct trauma victims.
        // Figley, C.R. (1995). Compassion fatigue; Stamm, B.H. (2010). The Concise ProQOL Manual
        impulse_control: -0.28,

        // Disposition - Empathy
        // Witnessing severe trauma typically triggers significant acute empathic distress and temporary emotional numbing/withdrawal, with substantial recovery over several months absent ongoing triggers.
        // Stamm, B.H. (2010). The Concise ProQOL Manual; APA trauma resources on vicarious traumatization
        empathy: -0.35,

        // Disposition - Aggression
        // Witnessing severe trauma/violence produces significant immediate aggression increase through acute arousal and moral outrage, with substantial chronicity due to posttraumatic stress sensitization and altered threat perception.
        // DSM-5 PTSD criteria on posttraumatic aggression; van der Kolk, B.A. (2014). The Body Keeps the Score
        aggression: 0.65,

        // Disposition - Grievance
        // Witnessing severe trauma fundamentally challenges beliefs about world fairness and justice, creating enduring grievance that the world permits such horror to vulnerable people.
        // Litz, B.T. et al. (2009). Moral injury and moral repair in war veterans; Jaffe et al. (2015) on vicarious trauma
        grievance: 0.75,

        // Disposition - Reactance
        // Witnessing severe trauma creates acute hyperarousal and threat detection that manifests as resistance to perceived control and constraints; this effect is significant but moderates over time through processing and recovery.
        // Bremner et al. (2003) on hyperarousal in PTSD; Brehm's Psychological Reactance Theory (1966)
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Witnessing severe trauma profoundly disrupts trust through shattered safety assumptions and hypervigilance, creating lasting interpersonal wariness despite potential recovery with therapeutic intervention.
        // Janoff-Bulman (1992). Shattered assumptions; Olff et al. (2005) on trauma and trust
        trust_propensity: -0.45,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: true,
        stress: false,
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
        valence: 0.22,
        arousal: 0.45,
        dominance: 0.35,
        fatigue: 0.18,
        stress: 0.35,
        purpose: 0.25,
        loneliness: 0.25,
        prc: 0.15,
        perceived_liability: 0.22,
        self_hate: 0.28,
        perceived_competence: 0.25,
        depression: 0.28,
        self_worth: 0.35,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.18,
        impulse_control: 0.12,
        empathy: 0.18,
        aggression: 0.35,
        grievance: 0.45,
        reactance: 0.28,
        trust_propensity: 0.35,
    },
};
