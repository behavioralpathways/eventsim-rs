//! WitnessViolencePhysical event specification.
//!
//! Witnessing physical violence against another person - seeing an assault, battery,
//! domestic violence, street fight, or violent attack. This represents vicarious trauma
//! from observing interpersonal violence without being the direct victim.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Witnessing physical violence against another produces significant immediate negative valence through vicarious trauma and empathetic distress, with minimal permanent baseline shift as most witnesses achieve substantial recovery within months.
        // Figley, C.R. (1995). Compassion Fatigue: Coping with Secondary Traumatic Stress Disorder; DSM-5 Acute Stress Disorder and PTSD criteria for indirect trauma exposure
        valence: -0.55,

        // Mood - Arousal
        // Witnessing physical violence activates comparable physiological arousal and hypervigilance as direct trauma through threat-detection sensitization, though slightly lower baseline permanence than witnessing severe/fatal trauma.
        // DSM-5 PTSD criteria; van der Kolk & McFarlane (2007) Trauma and nervous system dysregulation; Breslau et al. (2004) on trauma witness prevalence of hyperarousal
        arousal: 0.82,

        // Mood - Dominance
        // Witnesses experience significant powerlessness from inability to intervene in violence against another, creating chronic loss of agency with moderate permanent baseline shift from learned helplessness and threat-related control concerns.
        // Seligman, M.E.P. (1975). Helplessness: On Depression, Development, and Death; Figley, C.R. (1995). Compassion Fatigue
        dominance: -0.60,

        // Needs - Fatigue
        // Witnessing physical violence causes significant acute emotional exhaustion through vicarious trauma and hypervigilance-induced sleep disruption, but with substantial hedonic adaptation and recovery within weeks.
        // Figley, C.R. (1995); Bride, B.E. (2007) on secondary traumatic stress and emotional exhaustion
        fatigue: 0.38,

        // Needs - Stress
        // Witnessing physical violence triggers maximal acute physiological stress through HPA axis activation and threat perception, but most witnesses show significant recovery without chronic baseline elevation.
        // DSM-5 PTSD criteria; Figley, C.R. (1995). Compassion Fatigue; van der Kolk, B.A. (2014). The Body Keeps the Score
        stress: 0.72,

        // Needs - Purpose
        // Witnessing physical violence disrupts worldview and safety assumptions moderately, but with lower identity penetration and functional impact than direct victimization, showing moderate chronicity with substantial recovery.
        // Figley, C.R. (1995). Compassion Fatigue; Tedeschi & Calhoun (2004) on post-traumatic growth
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Witnessing physical violence creates moderate acute loneliness through social withdrawal and communication barriers, with recovery typical within 6-12 months as witnesses process and reintegrate socially.
        // van der Kolk, B.A. (2014). The Body Keeps the Score; Figley, C.R. (1995). Compassion Fatigue
        loneliness: 0.30,

        // Social Cognition - PRC
        // Witnessing physical violence moderately reduces perceived reciprocal caring by exposing world indifference and institutional failure, but affects less directly than direct victimization or severe trauma.
        // Figley, C.R. (1995). Compassion Fatigue; DSM-5 criteria on Criterion A trauma exposure
        prc: -0.18,

        // Social Cognition - Perceived Liability
        // Witnessing physical violence creates mild-to-moderate guilt about inability to intervene and temporary emotional burden on support network, substantially lower than direct victimization because witness lacks actual disability or dependency needs.
        // Figley, C.R. (1995). Compassion Fatigue; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        perceived_liability: 0.15,

        // Social Cognition - Self Hate
        // Witnessing physical violence triggers moral-based guilt for not intervening, but significantly less self-directed hatred than direct victims or witnesses of severe trauma/death; guilt is moderate and moderately recoverable within 1-2 years.
        // Darley, J.M. & Latane, B. (1968). Bystander intervention in emergencies; Litz, B.T. et al. (2009). Moral injury and moral repair
        self_hate: 0.28,

        // Social Cognition - Perceived Competence
        // Witnessing physical violence creates moderate self-doubt through bystander inaction and perceived powerlessness in crisis situations, but recovery is typical within 6-12 months with most people showing resilience.
        // Latane, B. & Darley, J.M. (1970). The Unresponsive Bystander; Bandura, A. (1997). Self-efficacy: The exercise of control
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Witnessing physical violence creates significant acute depression through vicarious traumatization and empathetic distress, with moderate chronic baseline shift lower than direct victimization due to reduced personal threat.
        // Figley, C.R. (1995). Compassion Fatigue; Bride, B.E. et al. (2004). Vicarious Traumatization in Therapists Treating Trauma Survivors
        depression: 0.45,

        // Mental Health - Self Worth
        // Witnessing physical violence creates guilt and helplessness that diminishes self-worth, but less severely than direct victimization; most recovery occurs within weeks to months through cognitive reappraisal and restored sense of agency.
        // Latane & Darley (1970) on bystander psychology; Litz et al. (2009) on moral injury
        self_worth: -0.22,

        // Mental Health - Hopelessness
        // Witnessing physical violence creates moderate hopelessness about future safety and world predictability through shattered assumptions and threat perception increases, but substantially less severe than direct victimization.
        // Janoff-Bulman, R. (1989). Assumptive Worlds and the Stress of Traumatic Events; Figley, C.R. (1995). Compassion Fatigue
        hopelessness: 0.32,

        // Mental Health - Interpersonal Hopelessness
        // Witnessing physical violence moderately increases belief that relationships cannot prevent harm, creating situational but substantially recoverable interpersonal hopelessness lower than direct victimization.
        // Figley, C.R. (1995). Compassion Fatigue; DSM-5 criteria on secondary traumatic stress
        interpersonal_hopelessness: 0.30,

        // Mental Health - Acquired Capability
        // Witnessing physical violence against others exposes the observer to pain, injury, and blood through observational learning, creating mild-to-moderate habituation to violence slightly lower than direct victimization.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Figley, C.R. (1995). Compassion Fatigue
        acquired_capability: 0.24,

        // Disposition - Impulse Control
        // Witnessing physical violence triggers acute trauma response and hyperarousal that significantly impairs executive function through prefrontal cortex dysregulation; less severe than direct victimization but with persistent hypervigilance effects.
        // van der Kolk, B.A. (2005); Teicher, M.H. et al. (2003). The neurobiological consequences of early stress
        impulse_control: -0.28,

        // Disposition - Empathy
        // Witnessing physical violence causes temporary emotional numbing and shock that reduces empathic capacity in the immediate aftermath, with moderate recovery as acute trauma subsides.
        // Singer, T. & Klimecki, O.M. (2014). Empathy and compassion; secondary trauma literature on compassion fatigue
        empathy: -0.18,

        // Disposition - Aggression
        // Witnessing physical violence increases aggression through observational learning and threat sensitization, but less severely than direct victimization due to absence of personal trauma and identity violation.
        // Bandura, A. (1977). Social Learning Theory; Anderson, C.A. & Bushman, B.J. (2002). Human Aggression
        aggression: 0.45,

        // Disposition - Grievance
        // Witnessing physical violence creates strong moral outrage and injustice perception comparable to direct trauma, though with somewhat better recovery prospects since the witness avoided direct bodily harm.
        // Litz, B.T. et al. (2009). Moral injury and moral repair; Figley, C.R. (1995). Compassion Fatigue
        grievance: 0.70,

        // Disposition - Reactance
        // Witnessing physical violence creates moderate acute reactance through threat-induced hypervigilance and heightened sensitivity to environmental constraints, with limited permanence as most witnesses show substantial recovery within months.
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological reactance; DSM-5 PTSD criteria on hyperarousal responses
        reactance: 0.28,

        // Disposition - Trust Propensity
        // Witnessing physical violence creates significant but lower-magnitude trust erosion than direct victimization by demonstrating human capacity for violence and inadequate environmental safety, with moderate permanent baseline shift.
        // Figley, C.R. (1995). Compassion Fatigue; Janoff-Bulman, R. (1992). Shattered Assumptions
        trust_propensity: -0.40,
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
        perceived_competence: false,
        depression: true,
        self_worth: false,
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
        valence: 0.12,
        arousal: 0.40,
        dominance: 0.30,
        fatigue: 0.08,
        stress: 0.28,
        purpose: 0.15,
        loneliness: 0.15,
        prc: 0.12,
        perceived_liability: 0.12,
        self_hate: 0.18,
        perceived_competence: 0.08,
        depression: 0.22,
        self_worth: 0.06,
        hopelessness: 0.22,
        interpersonal_hopelessness: 0.15,
        impulse_control: 0.12,
        empathy: 0.08,
        aggression: 0.22,
        grievance: 0.35,
        reactance: 0.15,
        trust_propensity: 0.22,
    },
};
