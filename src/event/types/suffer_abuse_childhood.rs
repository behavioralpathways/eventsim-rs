//! SufferAbuseChildhood event specification.
//!
//! Physical, emotional, or sexual abuse experienced during childhood, typically by
//! caregivers or trusted adults. This represents a severe developmental trauma that
//! fundamentally shapes psychological functioning across all dimensions.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Childhood abuse creates severe negative valence through multiple trauma pathways including PTSD, anhedonia, and fundamental disruptions to emotional safety and life satisfaction.
        // Teicher, M.H. & Samson, J.A. (2016). Annual Research Review: Enduring neurobiological effects of childhood abuse and neglect. Journal of Child Psychology and Psychiatry, 57(3), 241-266
        valence: -0.80,

        // Mood - Arousal
        // Childhood abuse triggers acute hyperarousal through threat activation and creates chronic baseline elevation through sensitized stress response systems and hypervigilance.
        // van der Kolk, B.A. (2014). The Body Keeps the Score: Brain, Mind, and Body in the Healing of Trauma
        arousal: 0.75,

        // Mood - Dominance
        // Childhood abuse represents profound loss of control and agency during critical developmental periods, creating persistent learned helplessness and diminished sense of personal power.
        // Seligman, M.E.P. (1975). Helplessness: On Depression, Development, and Death; Herman, J.L. (2015). Trauma and Recovery
        dominance: -0.85,

        // Needs - Fatigue
        // Childhood abuse causes severe emotional exhaustion, hypervigilance-related sleep disruption, and sustained cognitive effort to process trauma.
        // Herman, J.L. (1997). Trauma and Recovery; van der Kolk, B.A. et al. (2005). Traumatic Stress
        fatigue: 0.75,

        // Needs - Stress
        // Childhood abuse activates the HPA axis during critical developmental periods, resulting in dysregulated cortisol patterns and sustained physiological hyperarousal.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load; Teicher & Samson (2016)
        stress: 0.85,

        // Needs - Purpose
        // Childhood abuse fundamentally disrupts identity formation and meaning-making, creating pervasive existential confusion and difficulty envisioning positive futures.
        // Tedeschi & Calhoun (1996) on post-traumatic growth; Herman (1992) Trauma and Recovery
        purpose: -0.65,

        // Social Cognition - Loneliness
        // Childhood abuse fundamentally disrupts attachment security and creates persistent relational wariness, generating severe loneliness through defensive social withdrawal.
        // Teicher & Samson (2016); van der Kolk (2014). The Body Keeps the Score
        loneliness: 0.72,

        // Social Cognition - PRC
        // Childhood abuse represents sustained signal that primary caregivers don't care, creating persistent negative relational schemas about reciprocal caring.
        // Briere, J. & Rickards, E.M. (2007); Bowlby, J. (1980). Attachment and Loss; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        prc: -0.85,

        // Social Cognition - Perceived Liability
        // Childhood abuse creates sustained internalized beliefs about being a burden to caregivers and others, establishing chronic elevation in perceived liability.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        perceived_liability: 0.55,

        // Social Cognition - Self Hate
        // Childhood abuse generates severe internalized shame and self-blame, with victims often believing they are fundamentally flawed or deserved the abuse.
        // Joiner, T. (2005). Why People Die by Suicide; van Orden et al. (2010). The Interpersonal Theory of Suicide
        self_hate: 0.75,

        // Social Cognition - Perceived Competence
        // Childhood abuse constitutes a major competence crisis through learned helplessness, environmental invalidation, and formation of negative self-beliefs during critical developmental windows.
        // Bandura, A. (1997). Self-efficacy: The exercise of control; ACE literature on adult self-efficacy outcomes
        perceived_competence: -0.65,

        // Mental Health - Depression
        // Childhood abuse creates severe depressive symptoms with lasting developmental disruption, involving anhedonia and worthlessness, establishing chronic vulnerability patterns.
        // Teicher, M.H. & Samson, J.A. (2016). Annual Review of Clinical Psychology; Kendler et al. (1999) on stressful life events and depression
        depression: 0.70,

        // Mental Health - Self Worth
        // Childhood abuse creates severe, identity-level self-worth damage through internalized shame and fundamental belongingness violations that exceed single-event traumas.
        // Briere, J. & Scott, C. (2015). Principles of Trauma Therapy; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        self_worth: -0.80,

        // Mental Health - Hopelessness
        // Childhood abuse fundamentally shapes negative future expectancies through learned helplessness and eroded sense of safety and control.
        // Abramson, L.Y. et al. (1989) Hopelessness depression; Beck's Hopelessness Scale; ACE research on pessimism in abuse survivors
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Childhood abuse from caregivers creates fundamental distrust in relationships and help-seeking due to early betrayal of attachment bonds.
        // Bowlby, J. (1988). A Secure Base; Teicher & Samson (2016) on childhood maltreatment effects on attachment systems
        interpersonal_hopelessness: 0.72,

        // Mental Health - Acquired Capability
        // Childhood abuse involves repeated exposure to physical pain, fear, and threat of death, producing moderate habituation through traumatic conditioning.
        // Joiner, T.E. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        acquired_capability: 0.45,

        // Disposition - Impulse Control
        // Childhood abuse severely impairs impulse control through both acute neurobiological stress effects and lasting alterations to prefrontal development.
        // Teicher & Samson (2016) Annual Research Review: Enduring neurobiological effects of childhood abuse and neglect
        impulse_control: -0.65,

        // Disposition - Empathy
        // Childhood abuse creates persistent self-protective defenses and reduces capacity for vulnerable emotional engagement, though some survivors develop compensatory empathy.
        // Teicher, M.H. & Samson, J.A. (2016); Lukianoff et al. (2015) on trauma's impact on affect regulation
        empathy: -0.35,

        // Disposition - Aggression
        // Childhood abuse triggers severe frustration, threat response, and neurobiological dysregulation of aggression control through modeling and social learning.
        // Widom, C.S. (1989). Does violence beget violence? Psychological Bulletin; Bandura, A. (1977). Social learning theory
        aggression: 0.65,

        // Disposition - Grievance
        // Childhood abuse creates a profound sense of injustice and victimization; survivors frequently develop persistent grievance narratives involving betrayal by caregivers.
        // Finkelhor & Browne (1985) The traumatic impact of child sexual abuse; Herman (1992) Trauma and Recovery
        grievance: 0.75,

        // Disposition - Reactance
        // Childhood abuse imposes severe, repeated constraints on autonomy during critical developmental periods, creating heightened reactance to perceived control.
        // Teicher, M.H. & Samson, J.A. (2016); van der Kolk, B.A. (2014); Brehm, S.S. & Brehm, J.W. (1981). Psychological Reactance
        reactance: 0.65,

        // Disposition - Trust Propensity
        // Childhood abuse by trusted caregivers causes severe, pervasive mistrust of others' benevolence and safety; effects are deeply integrated into attachment patterns.
        // Bowlby, J. (1988). A Secure Base; Ullman & Filipas (2001) on trauma survivors; Schwandt (2014) meta-analysis
        trust_propensity: -0.85,
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
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.40,
        arousal: 0.35,
        dominance: 0.45,
        fatigue: 0.35,
        stress: 0.45,
        purpose: 0.35,
        loneliness: 0.32,
        prc: 0.45,
        perceived_liability: 0.35,
        self_hate: 0.45,
        perceived_competence: 0.45,
        depression: 0.35,
        self_worth: 0.32,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.45,
        impulse_control: 0.35,
        empathy: 0.45,
        aggression: 0.45,
        grievance: 0.45,
        reactance: 0.45,
        trust_propensity: 0.65,
    },
};
