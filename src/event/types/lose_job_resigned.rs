//! LoseJobResigned event specification.
//!
//! Voluntary resignation from employment. Unlike involuntary job loss (fired/laid off),
//! resignation involves personal agency and choice, which fundamentally changes the
//! psychological impact. The person chose to leave, reducing shame and preserving control.

use crate::event::event_spec::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Voluntary resignation typically produces mild positive valence from regained agency and relief from workplace stressor, with near-complete hedonic adaptation within 6-12 months as uncertainty resolves.
        // Clark, A.E. & Oswald, A.J. (1994). Unhappiness and unemployment. Economic Journal; Winkelmann & Winkelmann (1995) on voluntary vs involuntary job separation.
        valence: 0.15,

        // Mood - Arousal
        // Voluntary resignation typically reduces anxiety-based arousal through restored agency, with mild positive arousal from new opportunity; stress relief is temporary with quick adaptation.
        // Thayer, R.E. (1989). The biopsychology of mood and arousal; Russell, J.A. (1980). A circumplex model of affect.
        arousal: 0.15,

        // Mood - Dominance
        // Voluntary resignation immediately increases sense of personal agency and control through autonomous choice, but this effect is largely temporary with minimal permanent base shift.
        // Deci, E.L. & Ryan, R.M. (1985). Intrinsic motivation and self-determination theory - autonomy as fundamental psychological need.
        dominance: 0.35,

        // Needs - Fatigue
        // Voluntary resignation removes a chronic stressor, providing relief and mild energy restoration despite short-term transition fatigue; most people recover fully within 1-2 months.
        // Sonnentag & Zijlstra (2006) on job characteristics and recovery; Hockey (2013) on stress-induced fatigue depletion.
        fatigue: -0.25,

        // Needs - Stress
        // Voluntary resignation typically produces immediate stress relief from leaving a stressor, but subsequent uncertainty about employment creates compensatory stress; net effect is modest reduction.
        // McEwen, B.S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load.
        stress: -0.15,

        // Needs - Purpose
        // Voluntary resignation creates temporary purpose disruption from role loss, but preserved agency and self-direction limit duration and permanence compared to involuntary job loss.
        // Schlossberg, N.K. (1981). A model for analyzing human adaptation to transition; Wrzesniewski et al. (2003). Jobs, careers, and callings.
        purpose: -0.15,

        // Social Cognition - Loneliness
        // Voluntary resignation disrupts workplace social networks and daily interpersonal interactions, but the volitional nature and reduced shame produce milder loneliness than involuntary job loss.
        // Cacioppo, J.T. & Patrick, B. (2008). Loneliness: Human nature and the need for social connection.
        loneliness: 0.18,

        // Social Cognition - PRC
        // Voluntary resignation creates ambiguous social signals (loss of daily workplace relationships offset by retention of agency), resulting in mild temporary decrease in perceived reciprocal care.
        // Joiner, T. (2005). Why People Die by Suicide; Baumeister & Leary (1995) on belonging needs.
        prc: -0.12,

        // Social Cognition - Perceived Liability
        // Voluntary resignation demonstrates agency and control, reducing perceived liability through restored self-efficacy; relief from burnout/toxic environment decreases burden perception.
        // Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide - autonomy and control are protective factors.
        perceived_liability: -0.15,

        // Social Cognition - Self Hate
        // Voluntary resignation provides agentic control that prevents deep shame; mild temporary self-directed negativity only if resignation was avoidance-based, with quick recovery.
        // Joiner, T. (2005). Why People Die by Suicide - distinguishes personal failure (high shame) from agentic choice (low shame).
        self_hate: -0.05,

        // Social Cognition - Perceived Competence
        // Voluntary resignation involves agency and potential mastery experience but creates temporary identity uncertainty; most recovery occurs within 6-12 months as new role provides competence re-calibration.
        // Deci, E.L. & Ryan, R.M. (2000). Self-determination theory and the facilitation of intrinsic motivation.
        perceived_competence: 0.15,

        // Mental Health - Depression
        // Voluntary resignation produces moderate depressive symptoms from employment/identity loss, but maintained sense of agency and absence of blame prevents the substantial impact of involuntary job loss.
        // Kendler, K.S. et al. (1999). Causal relationship between stressful life events and depression; Paul, K.I. & Moser, K. (2009). Unemployment impairs mental health.
        depression: -0.20,

        // Mental Health - Self Worth
        // Voluntary resignation typically produces mild, transient self-worth boost through restored agency and relief from adverse conditions, with substantial recovery over months despite initial uncertainty.
        // Crocker & Wolfe (2001). Contingencies of self-worth - agency preservation protects self-worth.
        self_worth: 0.05,

        // Mental Health - Hopelessness
        // Voluntary resignation confers agency and perceived control, reducing immediate hopelessness through empowerment, but effect is temporary if underlying burnout/depression persist.
        // Dekker & Schaufeli (1995) on job loss dimensionality; Joiner's ITS on perceived control reducing hopelessness.
        hopelessness: -0.35,

        // Mental Health - Interpersonal Hopelessness
        // Voluntary resignation preserves agency and typically doesn't damage beliefs in relationship support; temporary stress may slightly reduce help-seeking attitudes with complete recovery expected.
        // Wanberg, C.R. et al. (2012). Job search success and individual outcomes. Journal of Applied Psychology.
        interpersonal_hopelessness: -0.05,

        // Mental Health - Acquired Capability
        // Voluntary job resignation does not expose the individual to physical pain, death, or injury, therefore does not increase habituation to pain and death.
        // Joiner, T. (2005). Why People Die by Suicide; Van Orden, K. et al. (2010). The Interpersonal Theory of Suicide.
        acquired_capability: 0.0,

        // Disposition - Impulse Control
        // Voluntary resignation moderately improves impulse control through assertion of agency and relief from stress, with minimal permanent shift due to hedonic adaptation.
        // Baumeister, R.F. et al. (1998). Ego depletion; Deci & Ryan (1985) Self-Determination Theory on autonomy restoration.
        impulse_control: -0.08,

        // Disposition - Empathy
        // Voluntary resignation reduces stress-induced self-focus compared to involuntary job loss, but post-resignation uncertainty and identity transition still produce mild, temporary empathy reduction.
        // Decety, J. & Jackson, P.L. (2004). The functional architecture of human empathy. Behavioral and Cognitive Neuroscience Reviews.
        empathy: -0.05,

        // Disposition - Aggression
        // Voluntary resignation removes the frustrating environment and restores agency, producing mild aggression reduction that is largely temporary as the person adapts to their new situation.
        // Berkowitz, L. (1989). Frustration-Aggression Hypothesis - control/agency reduces aggression responses.
        aggression: -0.15,

        // Disposition - Grievance
        // Voluntary resignation reduces grievance because personal agency and choice eliminate the injustice narrative that arises from involuntary job loss.
        // Lind, E.A. & Tyler, T.R. (1988). The social psychology of procedural justice - voice and control reduce injustice perceptions.
        grievance: -0.15,

        // Disposition - Reactance
        // Voluntary resignation exercises autonomy and restores freedom from constraint, reducing reactance through choice restoration rather than imposing external control.
        // Brehm, J.W. (1966). A theory of psychological reactance - distinguishes between imposed and chosen constraints.
        reactance: -0.15,

        // Disposition - Trust Propensity
        // Voluntary resignation slightly decreases trust propensity due to situational stress and workplace disillusionment, but the volitional nature preserves agency and prevents severe trust damage.
        // Rotter (1967) interpersonal trust scale; Mayer et al. (1995) organizational trust model.
        trust_propensity: -0.05,
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        dominance: false,
        fatigue: false,
        stress: false,
        purpose: false,
        loneliness: false,
        prc: false,
        perceived_liability: false,
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
        valence: 0.05,
        arousal: 0.04,
        dominance: 0.05,
        fatigue: 0.04,
        stress: 0.04,
        purpose: 0.05,
        loneliness: 0.05,
        prc: 0.05,
        perceived_liability: 0.04,
        self_hate: 0.03,
        perceived_competence: 0.06,
        depression: 0.05,
        self_worth: 0.04,
        hopelessness: 0.25,
        interpersonal_hopelessness: 0.02,
        impulse_control: 0.04,
        empathy: 0.02,
        aggression: 0.04,
        grievance: 0.04,
        reactance: 0.04,
        trust_propensity: 0.04,
    },
};
