//! SufferInjuryAccidental event specification.
//!
//! Accidental physical injury such as falls, sports injuries, car accidents,
//! or workplace accidents that cause acute physical harm and functional impairment.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Accidental injuries trigger acute negative valence from pain and functional loss, but most people adapt within weeks, with only modest permanent baseline shift accounting for chronic pain complications.
        // Craig, K.D. (2009). The social communication model of pain. Canadian Psychology; Roy-Byrne, P.P. et al. (2008). Anxiety disorders and comorbid medical illness. General Hospital Psychiatry.
        valence: -0.32,

        // Mood - Arousal
        // Accidental injuries trigger acute fight-or-flight response causing high physiological arousal (elevated heart rate, adrenaline), with some chronic elevation from pain and anxiety, though most recovery occurs within 6-12 months via hedonic adaptation.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Posner, J., Russell, J.A., & Peterson, B.S. (2005). The circumplex model of affect.
        arousal: 0.65,

        // Mood - Dominance
        // Accidental injuries impose immediate involuntary loss of bodily control and physical autonomy triggering acute dominance reduction, but most recover functional control within weeks-months with high hedonic adaptation and no permanent lifestyle constraints.
        // Bandura, A. (1977). Self-efficacy: Toward a unifying theory of behavioral change. Psychological Review; Deci, E.L. & Ryan, R.M. (1985). Intrinsic motivation and self-determination theory in human behavior.
        dominance: -0.50,

        // Needs - Fatigue
        // Accidental injury causes acute physical and emotional exhaustion through pain-induced sleep disruption, stress response, and emotional processing demands, but recovers with healing.
        // Hockey, G.R.J. (2013). The psychology of fatigue: Work, effort and control; Baumeister, R.F. et al. (1998). Ego depletion: Is the active self a limited resource?
        fatigue: 0.35,

        // Needs - Stress
        // Accidental injury triggers significant acute physiological stress through threat perception and pain, but shows rapid recovery with minimal permanent base shift due to hedonic adaptation over weeks.
        // Selye, H. (1956). The stress of life; McEwen, B.S. (1998). Stress, adaptation, and disease.
        stress: 0.65,

        // Needs - Purpose
        // Accidental injury disrupts active life plans and creates temporary existential questioning, but most individuals recover purpose orientation within months without permanent role loss.
        // Tedeschi, R.K., & Calhoun, L.G. (1996). The Posttraumatic Growth Inventory; Frankl, V.E. (1959). Man's search for meaning.
        purpose: -0.25,

        // Social Cognition - Loneliness
        // Accidental injury temporarily reduces social participation and creates feelings of disconnection despite increased caregiver presence, with effects largely reversing upon physical recovery.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide; Cacioppo & Patrick (2008). Loneliness: Human Nature and the Need for Social Connection.
        loneliness: 0.15,

        // Social Cognition - PRC
        // Accidental injury activates concrete caregiving from others, signaling they value the injured person's wellbeing, but most psychological effects normalize after physical recovery.
        // Uchino, B.N. (2009). Understanding the links between social support and physical health. Perspectives on Psychological Science.
        prc: 0.25,

        // Social Cognition - Perceived Liability
        // Accidental injury creates moderate perceived burdensomeness through temporary dependency on caregiving, medical expenses, and reduced capacity to contribute, but most individuals show recovery within 6-18 months as physical function restores.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        perceived_liability: 0.35,

        // Social Cognition - Self Hate
        // Accidental injuries trigger mild self-blame and temporary distress from physical pain, but lack the personal responsibility and moral failure that generate substantial self-hate; recovery is typically rapid with minimal permanent personality shift.
        // Joiner, T. (2005). Why People Die by Suicide - models self-hate as core to Perceived Burdensomeness, arising from perceived personal responsibility rather than external misfortune.
        self_hate: 0.12,

        // Social Cognition - Perceived Competence
        // Accidental injuries create temporary self-doubt through loss of mastery experiences and physical limitations, but most individuals show substantial recovery within months as they regain functional capability.
        // Bandura, A. (1997). Self-efficacy: The exercise of control. Freeman.
        perceived_competence: -0.25,

        // Mental Health - Depression
        // Accidental injury triggers moderate acute depressive symptoms through pain, disability, and loss of control, but shows near-complete hedonic adaptation within months for non-permanent injuries.
        // Brown, G.W. & Harris, T. (1978). Social origins of depression; Kendler, K.S. et al. (1999). Causal relationship between stressful life events and depression.
        depression: 0.25,

        // Mental Health - Self Worth
        // Accidental injuries trigger acute self-worth reduction through disruption of physical autonomy and temporary shame, but recovery follows normal hedonic adaptation patterns with minimal permanent shift.
        // Rosenberg, M. (1965). Society and the adolescent self-image.
        self_worth: -0.25,

        // Mental Health - Hopelessness
        // Accidental injury creates mild temporary hopelessness about the recovery process and immediate future, but does not typically generate sustained pessimism about distant future given clear recovery trajectory.
        // Kendler et al. (1999) on stressful life events; Wilson & Gilbert (2005) on hedonic adaptation.
        hopelessness: 0.15,

        // Mental Health - Interpersonal Hopelessness
        // Accidental injuries activate help-seeking and social support rather than blocking it, creating only mild temporary interpersonal hopelessness without shame-based barriers to relationships.
        // Van Orden, K., Witte, T.K., et al. (2010). The Interpersonal Theory of Suicide. Psychological Review.
        interpersonal_hopelessness: 0.12,

        // Mental Health - Acquired Capability
        // Accidental injury produces moderate pain habituation through acute physical trauma exposure, but lacks the reinforcement cycle of deliberate self-harm or sustained threat of combat, producing mild-to-moderate acquired capability development.
        // Van Orden, K.A. et al. (2010). The Interpersonal Theory of Suicide; Joiner, T.E. (2005). Why People Die by Suicide.
        acquired_capability: 0.28,

        // Disposition - Impulse Control
        // Accidental injuries trigger acute stress and pain responses that acutely deplete self-regulatory resources through ego depletion mechanisms; most individuals show substantial recovery within days to weeks as physical healing progresses.
        // Baumeister & Vohs (2007) on ego depletion; Muraven & Baumeister (2000) on self-regulation resource depletion.
        impulse_control: -0.35,

        // Disposition - Empathy
        // Accidental injury temporarily reduces empathy through pain-induced self-focus and cognitive load, but this is a transient stress response with minimal permanent impact.
        // Singer & Klimecki (2014). Empathy and compassion. Current Biology.
        empathy: -0.15,

        // Disposition - Aggression
        // Accidental injuries cause moderate frustration and pain-induced aggression responses with temporary elevated hostility, but rapid recovery as physical discomfort resolves.
        // Berkowitz, L. (1989). Frustration-aggression hypothesis; Anderson, C.A. & Bushman, B.J. (2002). Human aggression.
        aggression: 0.25,

        // Disposition - Grievance
        // Accidental injuries cause trauma and temporary anger, but pure accidents involve low perceived injustice since no agent deliberately wronged the person; grievance emerges primarily when systemic failures are perceived.
        // Mikula, G. (1993). On the experience of injustice; Miller, D.T. (2001). Disrespect and the experience of injustice.
        grievance: 0.15,

        // Disposition - Reactance
        // Accidental injury imposes temporary constraints on autonomy and mobility, creating mild situational reactance during acute recovery phase, but with high hedonic adaptation as healing progresses.
        // Brehm & Brehm (1981). Psychological reactance: A theory of freedom and control.
        reactance: 0.15,

        // Disposition - Trust Propensity
        // Accidental injury induces temporary caution about environmental reliability and others' ability to provide support, but lacks the betrayal element central to trust propensity changes, with near-complete hedonic adaptation expected.
        // Mayer, R.C., Davis, J.H., & Schoorman, F.D. (1995). An integrative model of organizational trust.
        trust_propensity: -0.08,
    },

    chronic: ChronicFlags {
        valence: true,
        arousal: true,
        dominance: true,
        fatigue: false,
        stress: false,
        purpose: true,
        loneliness: false,
        prc: false,
        perceived_liability: true,
        self_hate: false,
        perceived_competence: false,
        depression: false,
        self_worth: false,
        hopelessness: false,
        interpersonal_hopelessness: false,
        impulse_control: false,
        empathy: false,
        aggression: false,
        grievance: false,
        reactance: false,
        trust_propensity: false,
    },

    permanence: PermanenceValues {
        valence: 0.08,
        arousal: 0.12,
        dominance: 0.08,
        fatigue: 0.05,
        stress: 0.05,
        purpose: 0.05,
        loneliness: 0.04,
        prc: 0.05,
        perceived_liability: 0.12,
        self_hate: 0.04,
        perceived_competence: 0.06,
        depression: 0.04,
        self_worth: 0.06,
        hopelessness: 0.02,
        interpersonal_hopelessness: 0.02,
        impulse_control: 0.04,
        empathy: 0.03,
        aggression: 0.04,
        grievance: 0.05,
        reactance: 0.04,
        trust_propensity: 0.02,
    },
};
