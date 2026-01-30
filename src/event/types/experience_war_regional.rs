//! ExperienceWarRegional event specification.
//!
//! A regional war event - armed conflict affecting a geographic region, including invasion,
//! civil war, or sustained military operations that disrupt civilian life, create widespread
//! displacement, and expose populations to violence and death.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Regional war exposure produces severe, sustained negative valence through widespread threat, loss, displacement, and violence exposure; comparable to combat but affects civilians with less coping infrastructure.
        // Van Orden et al. (2010); WHO mental health impact assessments; studies on Syrian/Ukrainian conflict-affected civilians showing 60-70% depression/anxiety prevalence
        valence: -0.80,

        // Mood - Arousal
        // Regional war creates extreme physiological and psychological activation through threat-detection and hypervigilance, with sustained chronic elevation during conflict and moderate lasting neurobiological changes post-exposure.
        // Posner, Russell & Peterson (2005) circumplex model; van der Kolk et al. (2005) on combat trauma physiological sequelae
        arousal: 0.85,

        // Mood - Dominance
        // Regional war imposes complete loss of control over safety, displacement, and survival with characteristics of both acute trauma and chronic stress exposure; civilians face victimization without agency.
        // Bandura (1977) Self-Efficacy; Seligman & Maier (1967) learned helplessness in trauma
        dominance: -0.75,

        // Needs - Fatigue
        // Regional war creates severe acute exhaustion from sustained threat, displacement, and sleep disruption, with elevated chronic fatigue from lost routine restoration and persistent hypervigilance among displaced populations.
        // Hockey (2013) Psychology of fatigue; Mollica et al. (1992) on depression and fatigue in Southeast Asian refugees
        fatigue: 0.72,

        // Needs - Stress
        // Regional war exposure creates sustained HPA axis activation through direct physical threat, widespread uncertainty, loss of control, and displacement, placing stress at severe levels comparable to combat trauma.
        // McEwen (1998) Stress, adaptation, and disease: Allostasis and allostatic load; Friedman (2006) on PTSD in war-affected populations
        stress: 0.78,

        // Needs - Purpose
        // Regional war creates profound meaning disruption through role loss, displacement from anchoring contexts, and existential threat, with significant permanent identity reconstruction even for those who experience post-traumatic growth.
        // Frankl (1959) Man's search for meaning; combat psychology literature on meaning-making through extreme adversity
        purpose: -0.45,

        // Social Cognition - Loneliness
        // Regional war significantly increases loneliness through social fragmentation, displacement of communities, loss of established relationships, and disruption of daily interpersonal interactions.
        // Joiner (2005) Why People Die by Suicide; Cacioppo & Patrick (2008) Loneliness: Human nature and the need for social connection
        loneliness: 0.65,

        // Social Cognition - PRC
        // Regional war disrupts established support networks through displacement and resource scarcity, creating mild perceived caring deficit, though community solidarity often provides partial compensation.
        // Van Orden et al. (2010) Interpersonal Theory of Suicide; research on wartime social cohesion effects
        prc: -0.15,

        // Social Cognition - Perceived Liability
        // Regional war creates significant but recoverable perceived liability through displacement, economic dependency, and care needs during and immediately post-conflict, though shared trauma context moderates burden perception.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010) on perceived liability in displacement contexts
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Regional war triggers sustained guilt, shame, and moral injury through witnessing violence and loss of control, but as an external circumstance produces less intense self-hate than personal failures.
        // Litz et al. (2009) on moral injury and survivor's guilt; Shay (1994) on trauma in conflict-affected populations
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Regional war significantly reduces perceived competence through imposed loss of control and learned helplessness from civilian vulnerability, with moderate permanence as post-war reconstruction gradually restores agency.
        // Bandura (1997) Self-efficacy: The exercise of control; Seligman (1975) Helplessness: On depression, development, and death
        perceived_competence: -0.32,

        // Mental Health - Depression
        // Regional war produces severe depressive symptoms through compounded loss (death, displacement, social disruption) with sustained chronicity during conflict and partial permanence from trauma absorption.
        // Brown & Harris (1978) Social Origins of Depression; longitudinal studies of war-affected populations showing 10-20% depression rates
        depression: 0.65,

        // Mental Health - Self Worth
        // Regional war creates significant self-worth reduction through loss of agency, displacement trauma, identity disruption, and exposure to violence and death.
        // Van Orden et al. (2010); Litz et al. (2009) on moral injury; Hobfoll et al. (2009) on loss spirals in conflict
        self_worth: -0.40,

        // Mental Health - Hopelessness
        // Regional war creates severe loss of control and perceived inescapability, with substantial impact on future expectations but partial recovery through adaptation and post-conflict rebuilding.
        // Beck et al. (1974) Hopelessness Scale; O'Connor & Kirtley (2018) integrated motivational-volitional model
        hopelessness: 0.65,

        // Mental Health - Interpersonal Hopelessness
        // Regional war systematically damages beliefs about relational support through collective trauma, social disconnection, displacement disrupting networks, and social fragmentation during conflict.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010); Mollica et al. (2004) on social fragmentation
        interpersonal_hopelessness: 0.68,

        // Mental Health - Acquired Capability
        // Regional war creates moderate-to-high habituation through sustained violence, death witnessing, and survival of life-threatening situations, reducing fear response to physical harm and mortality.
        // Joiner (2005) Why People Die by Suicide; Van Orden et al. (2010); Smith et al. (2012) on acquired capability
        acquired_capability: 0.55,

        // Disposition - Impulse Control
        // Regional war severely impairs impulse control through sustained physiological hyperarousal, prefrontal cortex dysregulation, and ego depletion from chronic threat monitoring.
        // Aupperle et al. (2012) Executive function and PTSD; Friedman (2006); Elbogen & Johnson (2009) on violence and mental disorder
        impulse_control: -0.55,

        // Disposition - Empathy
        // Regional war creates significant temporary empathy reduction through trauma and self-focus, with partial permanence reflecting that acute effects recover but some lasting changes in threat-processing remain.
        // Decety & Jackson (2004) functional architecture of human empathy; Singer & Klimecki (2014) on trauma's impact on compassion
        empathy: -0.25,

        // Disposition - Aggression
        // Regional war creates severe frustration, witnessed violence normalization, identity threat, and sustained physiological arousal - all strong aggression drivers per frustration-aggression hypothesis.
        // Berkowitz (1989) Frustration-Aggression Hypothesis; Anderson & Bushman (2002) Human aggression
        aggression: 0.65,

        // Disposition - Grievance
        // Regional war creates systemic victimization and perceived injustice across multiple life domains, with effects sustained through ongoing displacement and community-level trauma processing.
        // Lind & Tyler (1988) procedural justice; Mikula (1993) on injustice experience; war-related moral injury literature
        grievance: 0.75,

        // Disposition - Reactance
        // Regional war imposes severe, non-negotiable restrictions on freedom and movement, triggering strong reactance, but sustained exposure and post-war recovery moderate the permanent baseline shift.
        // Brehm (1966) A Theory of Psychological Reactance; Murthy & Lakshminarayana (2006) on mental health in conflict
        reactance: 0.65,

        // Disposition - Trust Propensity
        // Regional war produces sustained distrust through institutional failure, inter-group violence, and displacement, with moderate permanence due to hedonic adaptation and post-conflict relationship reconstruction.
        // Putnam (2002) Bowling Alone; research on trust erosion in post-conflict societies
        trust_propensity: -0.55,
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
        valence: 0.32,
        arousal: 0.25,
        dominance: 0.35,
        fatigue: 0.30,
        stress: 0.32,
        purpose: 0.35,
        loneliness: 0.25,
        prc: 0.25,
        perceived_liability: 0.18,
        self_hate: 0.25,
        perceived_competence: 0.14,
        depression: 0.25,
        self_worth: 0.20,
        hopelessness: 0.35,
        interpersonal_hopelessness: 0.35,
        impulse_control: 0.28,
        empathy: 0.12,
        aggression: 0.25,
        grievance: 0.45,
        reactance: 0.18,
        trust_propensity: 0.18,
    },
};
