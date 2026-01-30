//! DevelopIllnessChronic event specification.
//!
//! Development of a chronic illness such as diabetes, heart disease, autoimmune
//! disorders, chronic pain conditions, or other long-term health conditions
//! requiring ongoing management.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Chronic illness diagnosis triggers severe negative affect comparable to serious illness; 20-30% develop depression with limited hedonic adaptation due to ongoing symptoms
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC1070773/
        valence: -0.55,

        // Mood - Arousal
        // Chronic illness triggers sustained anxiety-driven arousal through ongoing sympathetic nervous system activation and health-related threat monitoring
        // https://www.ncbi.nlm.nih.gov/books/NBK541120/
        arousal: 0.50,

        // Mood - Dominance
        // Chronic illness imposes loss of bodily control and lifestyle autonomy, creating significant perceived powerlessness with partial hedonic adaptation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC3056146/
        dominance: -0.55,

        // Needs - Fatigue
        // Chronic illness causes significant immediate fatigue through physical symptoms, ongoing medical management, and emotional processing with substantial permanent shift
        // https://www.nature.com/articles/s41598-021-00337-z
        fatigue: 0.65,

        // Needs - Stress
        // Chronic illness diagnosis triggers high acute stress response from perceived threats and loss of control with moderate permanent elevation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5137920/
        stress: 0.55,

        // Needs - Purpose
        // Chronic illness causes significant initial purpose disruption through goal blockage and identity loss, but most adapt through meaning-making within 1-2 years
        // https://journals.plos.org/mentalhealth/article?id=10.1371/journal.pmen.0000121
        purpose: -0.35,

        // Social Cognition - Loneliness
        // Chronic illness significantly increases loneliness through physical limitations, social withdrawal, and reduced engagement opportunities
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC10049704/
        loneliness: 0.35,

        // Social Cognition - PRC
        // Chronic illness typically increases perceived caring through active family support and healthcare engagement
        // https://bmcgeriatr.biomedcentral.com/articles/10.1186/s12877-025-05902-z
        prc: 0.25,

        // Social Cognition - Perceived Liability
        // Chronic illness creates legitimate dependency requiring ongoing care, triggering moderate-to-significant guilt and burden perception
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC8730634/
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Chronic illness triggers moderate self-blame and internalized shame, particularly characterological blame, with significant hedonic adaptation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC9879833/
        self_hate: 0.35,

        // Social Cognition - Perceived Competence
        // Chronic illness creates significant self-efficacy doubt through repeated management challenges and loss of bodily control
        // https://onlinelibrary.wiley.com/doi/10.1002/nop2.70276
        perceived_competence: -0.35,

        // Mental Health - Depression
        // Chronic illness triggers moderate depressive symptoms through loss, uncertainty, and functional impairment; 58.8% prevalence but 64% show recovery trajectory
        // https://link.springer.com/article/10.1186/s43045-023-00340-2
        depression: 0.35,

        // Mental Health - Self Worth
        // Chronic illness triggers significant identity disruption and body image concerns creating measurable self-worth decline
        // https://www.sciencedirect.com/science/article/abs/pii/S0277953619305957
        self_worth: -0.35,

        // Mental Health - Hopelessness
        // Chronic illness produces significant hopelessness through loss of autonomy, uncertainty, and perceived permanent limitation
        // https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2016.02022/full
        hopelessness: 0.35,

        // Mental Health - Interpersonal Hopelessness
        // Chronic illness significantly increases interpersonal hopelessness through identity disruption, depression, and social withdrawal
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC10049704/
        interpersonal_hopelessness: 0.35,

        // Mental Health - Acquired Capability
        // Chronic illness exposure habituates individuals to ongoing physical pain and medical stressors, moderately increasing pain tolerance
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC5022773/
        acquired_capability: 0.35,

        // Disposition - Impulse Control
        // Chronic illness acutely depletes self-regulatory resources through sustained stress, emotional dysregulation, and continuous disease management demands
        // https://www.tandfonline.com/doi/full/10.1080/13548506.2020.1867316
        impulse_control: -0.35,

        // Disposition - Empathy
        // Chronic illness stress and cognitive load reduce available resources for other-focused empathy through self-focus mechanisms
        // https://www.sciencedirect.com/science/article/pii/S0149763422004924
        empathy: -0.12,

        // Disposition - Aggression
        // Chronic illness reliably triggers frustration, perceived injustice, and goal blockage that increase aggression state
        // https://pubmed.ncbi.nlm.nih.gov/20738789/
        aggression: 0.35,

        // Disposition - Grievance
        // Chronic illness activates "why me?" and injustice narratives in majority of patients due to permanent loss and undeserved suffering
        // https://www.frontiersin.org/journals/pain-research/articles/10.3389/fpain.2025.1554630/full
        grievance: 0.35,

        // Disposition - Reactance
        // Chronic illness imposes involuntary lifestyle and medical constraints that threaten behavioral freedom and autonomy
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC4675534/
        reactance: 0.35,

        // Disposition - Trust Propensity
        // Chronic illness increases social vulnerability, creating protective trust filtering toward strangers while potentially strengthening reliance on close others
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC10486567/
        trust_propensity: -0.15,
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
        valence: 0.18,
        arousal: 0.12,
        dominance: 0.18,
        fatigue: 0.35,
        stress: 0.18,
        purpose: 0.12,
        loneliness: 0.12,
        prc: 0.12,
        perceived_liability: 0.25,
        self_hate: 0.18,
        perceived_competence: 0.18,
        depression: 0.18,
        self_worth: 0.18,
        hopelessness: 0.12,
        interpersonal_hopelessness: 0.12,
        impulse_control: 0.12,
        empathy: 0.06,
        aggression: 0.18,
        grievance: 0.25,
        reactance: 0.25,
        trust_propensity: 0.12,
    },
};
