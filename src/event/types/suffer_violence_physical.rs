//! SufferViolencePhysical event specification.
//!
//! Being a victim of physical violence - assault, battery, mugging, domestic violence.
//! This represents a severe interpersonal trauma involving direct bodily harm and
//! violation of physical safety and autonomy.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Physical violence victimization represents a catastrophic breach of bodily safety and autonomy, causing acute severe negative valence with substantial chronic depression of baseline hedonic tone.
        // DSM-5 trauma criteria; van der Kolk, B.A. (2014). The Body Keeps the Score
        valence: -0.75,

        // Mood - Arousal
        // Physical violence triggers extreme physiological activation with persistent hyperarousal due to threat-detection sensitization in trauma survivors.
        // DSM-5 PTSD criteria (hyperarousal cluster); van der Kolk & McFarlane (2007) on trauma and nervous system dysregulation
        arousal: 0.85,

        // Mood - Dominance
        // Physical violence creates severe loss of control and agency through assault experience, producing profound powerlessness as victims integrate helplessness into their psychological identity.
        // Seligman, M.E.P. (1975). Helplessness: On Depression, Development, and Death; Herman, J.L. (2015). Trauma and Recovery
        dominance: -0.75,

        // Needs - Fatigue
        // Physical violence causes significant emotional exhaustion from trauma response and hyperarousal-induced sleep disruption, with meaningful chronic elevation persisting after acute physical recovery.
        // van der Kolk, B.A. (2014). The Body Keeps the Score; Hockey, G.R.J. (2013). The psychology of fatigue
        fatigue: 0.60,

        // Needs - Stress
        // Physical violence is an extreme acute stressor activating maximal HPA axis response with sustained dysregulation and significant permanent baseline elevation due to trauma imprinting.
        // Selye's General Adaptation Syndrome; DSM-5 Acute Stress Disorder and PTSD criteria for assault as Criterion A trauma
        stress: 0.85,

        // Needs - Purpose
        // Physical violence disrupts worldview, identity, and life narrative through shattering assumptions and goal interruption, with moderate acute impact and small-to-moderate permanent baseline shift.
        // Janoff-Bulman, R. (1989). Assumptive worlds and the stress of traumatic events; Tedeschi & Calhoun (2004) on posttraumatic growth
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Physical violence breaches interpersonal safety and trust, creating acute social withdrawal and lasting disruption of perceived connection capacity.
        // Joiner's Interpersonal Theory of Suicide on trauma and Thwarted Belongingness; domestic violence research on social isolation
        loneliness: 0.35,

        // Social Cognition - PRC
        // Physical violence concretely demonstrates perpetrator indifference to victim wellbeing, creating moderate-to-significant erosion of perceived caring with substantial baseline shift.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; van der Kolk, B.A. (2014). The Body Keeps the Score
        prc: -0.35,

        // Social Cognition - Perceived Liability
        // Physical violence creates acute trauma-related disability that directly translates to belief that one burdens others through injury, medical care needs, and PTSD symptomatology.
        // Joiner (2005) Why People Die by Suicide; IPV literature on assault victims and disability perception
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Physical violence victims experience significant acute self-blame and shame with substantial permanence due to violation of bodily autonomy and disrupted sense of safety.
        // Janoff-Bulman's Shattered Assumptions Theory; trauma psychology literature on assault victims (Kilpatrick et al., Koss et al.)
        self_hate: 0.65,

        // Social Cognition - Perceived Competence
        // Physical violence is a direct demonstration of inability to protect oneself, creating substantial immediate competence damage with moderate-to-significant permanent shift toward learned helplessness.
        // Seligman, M.E.P. (1975). Learned Helplessness; Bandura, A. (1977). Self-Efficacy: Toward a Unifying Theory of Behavioral Change
        perceived_competence: -0.60,

        // Mental Health - Depression
        // Physical violence produces severe acute depression due to trauma and violation of bodily autonomy, with significant chronic and permanent effects as victims develop long-term depressive symptoms.
        // DSM-5 trauma response patterns; Golding (1999) meta-analysis showing 47% of violence victims meet depression criteria
        depression: 0.65,

        // Mental Health - Self Worth
        // Physical violence creates direct identity threat through violation and helplessness, with moderate acute impact and substantial permanent shift from trauma-related self-worth erosion.
        // Finkelhor & Browne's Traumatic Impact Model (1985); van der Kolk's trauma research on self-concept disruption
        self_worth: -0.45,

        // Mental Health - Hopelessness
        // Physical violence creates significant hopelessness about future safety through shattered autonomy and broken worldview assumptions with moderate permanent baseline shift.
        // Joiner, T. (2005). Why People Die by Suicide; Foa & Rothbaum (1998). Treating the Trauma of Rape
        hopelessness: 0.48,

        // Mental Health - Interpersonal Hopelessness
        // Physical violence directly attacks relational safety and trust, producing substantial belief that relationships won't help, with significant but partially recoverable permanence.
        // Ullman & Filipas (2005) on help-seeking barriers in assault survivors; Campbell, Raja, & Grining (1999) on trauma-related disclosure avoidance
        interpersonal_hopelessness: 0.45,

        // Mental Health - Acquired Capability
        // Physical violence victimization directly exposes individuals to acute pain and threat to life, creating direct habituation to physical suffering and death awareness.
        // Joiner, T.E. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide
        acquired_capability: 0.28,

        // Disposition - Impulse Control
        // Physical violence causes acute executive dysfunction through stress-induced amygdala dominance and prefrontal suppression, with moderate chronic permanence reflecting PTSD-related changes.
        // Van der Kolk (2014) The Body Keeps the Score on trauma's effects on executive function; Foa & Kozak (1986) emotional processing theory
        impulse_control: -0.35,

        // Disposition - Empathy
        // Physical violence reduces empathy through trauma-related emotional numbing, hypervigilance, and threat-focused cognition, with significant but partially reversible baseline shift.
        // DSM-5 PTSD criteria on emotional numbing; Singer & Klimecki (2014) Empathy and compassion; van der Kolk (2014) on trauma neurobiology
        empathy: -0.28,

        // Disposition - Aggression
        // Physical violence causes significant acute aggression through frustration-aggression hypothesis and trauma response, with moderate permanence as hyperarousal and defensive hostility become partially integrated.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; van der Kolk et al. (2005) on violent victimization and reactive aggression
        aggression: 0.65,

        // Disposition - Grievance
        // Physical violence creates substantial perceived injustice through deliberate harm, becoming integrated into identity and worldview, with significant permanent baseline shift.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Litz, B.T. et al. (2009). Moral injury and moral repair
        grievance: 0.65,

        // Disposition - Reactance
        // Physical violence violates bodily autonomy and freedom through direct interpersonal assault, creating acute significant reactance that partially persists through trauma-related threat sensitivity.
        // Brehm, S.S. & Brehm, J.W. (1981). Psychological reactance: A theory of freedom and control; DSM-5 PTSD criteria on hypervigilance
        reactance: 0.45,

        // Disposition - Trust Propensity
        // Physical violence constitutes a severe breach of bodily safety and interpersonal trust, causing significant generalized trust damage with substantial permanent base shift.
        // Finkelhor & Browne (1985); Schachner & Ciccone (2005) on violent victimization and trust deficits; PTSD literature on trust recovery
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
        empathy: true,
        aggression: true,
        grievance: true,
        reactance: true,
        trust_propensity: true,
    },

    permanence: PermanenceValues {
        valence: 0.35,
        arousal: 0.35,
        dominance: 0.35,
        fatigue: 0.15,
        stress: 0.35,
        purpose: 0.18,
        loneliness: 0.18,
        prc: 0.25,
        perceived_liability: 0.25,
        self_hate: 0.35,
        perceived_competence: 0.35,
        depression: 0.35,
        self_worth: 0.35,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.22,
        impulse_control: 0.18,
        empathy: 0.22,
        aggression: 0.35,
        grievance: 0.30,
        reactance: 0.20,
        trust_propensity: 0.35,
    },
};
