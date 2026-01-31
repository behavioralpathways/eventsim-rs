//! Stanford Prison Experiment (1971) - Rapid moral collapse simulation.
//!
//! Models the famous experiment where 24 male college students were randomly
//! assigned to guard or prisoner roles. The experiment was terminated after
//! just 6 days due to extreme psychological abuse.
//!
//! Key psychological dynamics to capture:
//! - Power differential + dehumanization = rapid moral collapse
//! - Guards: aggression escalates, empathy plummets
//! - Prisoners: dominance collapses, hopelessness spikes, self-worth craters
//! - Bystander complicity (passive guards enable abuse)
//! - SPEED of transformation is the critical insight

use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::{
    DispositionPath, EventPayload, EventType, MoodPath, Species, StatePath,
};
use eventsim_rs::event::EventBuilder;
use eventsim_rs::simulation::{ComputedState, Simulation};
use eventsim_rs::types::{Duration, EntityId, Timestamp};
use eventsim_rs::GroupId;

const ANCHOR_AGE: u64 = 21; // College students
const EXPERIMENT_HOURS: u64 = 144; // 6 days (terminated early)

/// Helper to create an entity in the simulation
fn create_entity(
    sim: &mut Simulation,
    id: &str,
    birth_date: Timestamp,
    reference: Timestamp,
) -> EntityId {
    let entity = EntityBuilder::new()
        .id(id)
        .species(Species::Human)
        .age(Duration::years(ANCHOR_AGE))
        .birth_date(birth_date)
        .build()
        .unwrap();

    sim.add_entity(entity, reference);
    EntityId::new(id).unwrap()
}

/// Helper to add an event at a specific hour offset
fn add_event_at_hour(
    sim: &mut Simulation,
    reference: Timestamp,
    target: &EntityId,
    event_type: EventType,
    severity: f64,
    hour_offset: u64,
) {
    let event = EventBuilder::new(event_type)
        .target(target.clone())
        .severity(severity)
        .timestamp(Duration::hours(hour_offset))
        .build()
        .unwrap();

    let event_time = reference + Duration::hours(hour_offset);
    sim.add_event(event, event_time);
}

/// Helper to get state at specific hour offset
fn state_at_hour(
    sim: &Simulation,
    entity_id: &EntityId,
    reference: Timestamp,
    hour_offset: u64,
) -> ComputedState {
    let handle = sim.entity(entity_id).unwrap();
    let time = reference + Duration::hours(hour_offset);
    handle.state_at(time)
}

#[test]
fn stanford_prison_experiment_rapid_moral_collapse() {
    // ========================================================================
    // SETUP - August 1971, basement of Stanford Psychology building
    // 24 college students, screened to be psychologically normal
    // Random assignment to guard or prisoner roles
    // ========================================================================

    let birth_date = Timestamp::from_ymd_hms(1950, 1, 1, 0, 0, 0);
    let reference = Timestamp::from_ymd_hms(1971, 8, 14, 8, 0, 0); // Experiment start
    let mut sim = Simulation::new(reference);

    // ========================================================================
    // GROUP IDs - guard vs prisoner identity anchors
    // ========================================================================

    let guards_group = GroupId::new("guards").unwrap();
    let prisoners_group = GroupId::new("prisoners").unwrap();

    // ========================================================================
    // GUARDS - 12 guards working in 8-hour shifts
    // ========================================================================

    let guard_alpha = create_entity(&mut sim, "guard_alpha", birth_date, reference);
    let guard_beta = create_entity(&mut sim, "guard_beta", birth_date, reference);
    let guard_gamma = create_entity(&mut sim, "guard_gamma", birth_date, reference);
    let guard_delta = create_entity(&mut sim, "guard_delta", birth_date, reference); // Passive, uncomfortable
    let guard_e = create_entity(&mut sim, "guard_e", birth_date, reference);
    let guard_f = create_entity(&mut sim, "guard_f", birth_date, reference);
    let guard_g = create_entity(&mut sim, "guard_g", birth_date, reference);
    let guard_h = create_entity(&mut sim, "guard_h", birth_date, reference);
    let guard_i = create_entity(&mut sim, "guard_i", birth_date, reference);
    let guard_j = create_entity(&mut sim, "guard_j", birth_date, reference);
    let guard_k = create_entity(&mut sim, "guard_k", birth_date, reference);
    let guard_l = create_entity(&mut sim, "guard_l", birth_date, reference);

    // ========================================================================
    // PRISONERS - 12 prisoners assigned numbers (dehumanization begins)
    // ========================================================================

    let prisoner_8612 = create_entity(&mut sim, "prisoner_8612", birth_date, reference); // First breakdown
    let prisoner_819 = create_entity(&mut sim, "prisoner_819", birth_date, reference); // Crisis day 3
    let prisoner_416 = create_entity(&mut sim, "prisoner_416", birth_date, reference); // Hunger strike
    let prisoner_a = create_entity(&mut sim, "prisoner_a", birth_date, reference);
    let prisoner_b = create_entity(&mut sim, "prisoner_b", birth_date, reference);
    let prisoner_c = create_entity(&mut sim, "prisoner_c", birth_date, reference);
    let prisoner_d = create_entity(&mut sim, "prisoner_d", birth_date, reference);
    let prisoner_e = create_entity(&mut sim, "prisoner_e", birth_date, reference);
    let prisoner_f = create_entity(&mut sim, "prisoner_f", birth_date, reference);
    let prisoner_g = create_entity(&mut sim, "prisoner_g", birth_date, reference);
    let prisoner_h = create_entity(&mut sim, "prisoner_h", birth_date, reference);
    let prisoner_i = create_entity(&mut sim, "prisoner_i", birth_date, reference);

    // ========================================================================
    // HOURS 0-12: Arrests, strip searches, dehumanization begins
    // All prisoners experience humiliation and power loss
    // Guards begin exerting authority
    // ========================================================================

    let all_prisoners = [
        &prisoner_8612,
        &prisoner_819,
        &prisoner_416,
        &prisoner_a,
        &prisoner_b,
        &prisoner_c,
        &prisoner_d,
        &prisoner_e,
        &prisoner_f,
        &prisoner_g,
        &prisoner_h,
        &prisoner_i,
    ];

    let all_guards = [
        &guard_alpha,
        &guard_beta,
        &guard_gamma,
        &guard_delta,
        &guard_e,
        &guard_f,
        &guard_g,
        &guard_h,
        &guard_i,
        &guard_j,
        &guard_k,
        &guard_l,
    ];

    // Hour 0: Role assignment + group identity (guards vs prisoners)
    for guard in &all_guards {
        let inclusion = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .source((*guard).clone())
            .target((*guard).clone())
            .severity(0.5)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(guards_group.clone()),
            })
            .build()
            .unwrap();
        sim.add_event(inclusion, reference);
    }

    for prisoner in &all_prisoners {
        let inclusion = EventBuilder::new(EventType::ExperienceInclusionPeer)
            .source((*prisoner).clone())
            .target((*prisoner).clone())
            .severity(0.5)
            .payload(EventPayload::SocialInclusion {
                group_id: Some(prisoners_group.clone()),
            })
            .build()
            .unwrap();
        sim.add_event(inclusion, reference);
    }

    // Hour 0: "Arrests" - public humiliation
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceHumiliationPublic,
            0.7,
            0,
        );
    }

    // Hour 2: Strip searches, delousing - severe humiliation
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceHumiliationPublic,
            0.8,
            2,
        );
    }

    // Guards gain power from their role
    for guard in &all_guards {
        add_event_at_hour(
            &mut sim,
            reference,
            guard,
            EventType::GainPowerPersonal,
            0.5,
            1,
        );
    }

    // Early prisoner solidarity (short-lived): peers support each other
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceInclusionPeer,
            0.4,
            3,
        );
    }

    // ========================================================================
    // HOURS 12-24: First guard shift - rules enforcement, push-ups as punishment
    // Guards begin to escalate, prisoners start to resist
    // ========================================================================

    // Hour 14: Prisoners forced to do push-ups, denied bathroom access
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceHumiliationPublic,
            0.6,
            14,
        );
    }

    // Guard Alpha becomes aggressive ("John Wayne" character)
    add_event_at_hour(
        &mut sim,
        reference,
        &guard_alpha,
        EventType::GainPowerPersonal,
        0.7,
        15,
    );

    // Contagion: other guards see Alpha's dominance and follow
    for guard in [&guard_beta, &guard_gamma, &guard_e, &guard_f] {
        add_event_at_hour(
            &mut sim,
            reference,
            guard,
            EventType::GainPowerPersonal,
            0.5,
            16,
        );
    }

    // ========================================================================
    // HOURS 24-36: Prisoner rebellion crushed, ringleaders isolated
    // Guards become more aggressive, prisoners experience exclusion/isolation
    // ========================================================================

    // Hour 26: Rebellion - prisoners barricade cells
    // Hour 28: Guards crush rebellion with fire extinguishers

    // Ringleaders isolated
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_8612,
        EventType::ExperienceIsolationChronic,
        0.7,
        28,
    );
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_819,
        EventType::ExperienceIsolationChronic,
        0.7,
        28,
    );

    // All prisoners experience interpersonal conflict (guards vs prisoners)
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceConflictInterpersonal,
            0.8,
            28,
        );
    }

    // Solidarity collapses: prisoners begin rejecting each other after crackdown
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceRejectionPeer,
            0.4,
            30,
        );
    }

    // Guards escalate - more power gain
    add_event_at_hour(
        &mut sim,
        reference,
        &guard_alpha,
        EventType::GainPowerPersonal,
        0.8,
        30,
    );
    add_event_at_hour(
        &mut sim,
        reference,
        &guard_beta,
        EventType::GainPowerPersonal,
        0.7,
        30,
    );
    add_event_at_hour(
        &mut sim,
        reference,
        &guard_gamma,
        EventType::GainPowerPersonal,
        0.7,
        30,
    );

    // ========================================================================
    // HOURS 36-48: Guards escalate - sleep deprivation, toilet denial
    // Psychological abuse becomes routine
    // ========================================================================

    // Hour 38: Sleep deprivation tactics
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceConflictInterpersonal,
            0.7,
            38,
        );
    }

    // Passive guards experience internal conflict (bystander complicity)
    for guard in [&guard_delta, &guard_h, &guard_i] {
        add_event_at_hour(
            &mut sim,
            reference,
            guard,
            EventType::ExperienceConflictInterpersonal,
            0.4,
            40,
        );
    }

    // Hour 42: Toilet privileges denied - public humiliation
    for prisoner in &all_prisoners {
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceHumiliationPublic,
            0.8,
            42,
        );
    }

    // ========================================================================
    // HOURS 48-60: Prisoner #8612 breakdown, released
    // First casualty of the experiment
    // ========================================================================

    // Hour 50: #8612 experiences acute distress
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_8612,
        EventType::ExperienceConflictInterpersonal,
        0.9,
        50,
    );

    // Hour 52: #8612 has breakdown - humiliation of psychological collapse
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_8612,
        EventType::ExperienceShamingPublic,
        0.8,
        52,
    );

    // Hour 56: #8612 released (but damage done)

    // ========================================================================
    // HOURS 60-72: Guards become creative with humiliation
    // Psychological abuse reaches peak creativity
    // ========================================================================

    // Hour 62: Guards force prisoners to simulate sexual acts
    for prisoner in &all_prisoners {
        if prisoner.as_str() == "prisoner_8612" {
            continue; // Already released
        }
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceHumiliationPublic,
            0.9,
            62,
        );
    }

    // Guards reinforce in-group identity after abuse
    for guard in &all_guards {
        add_event_at_hour(
            &mut sim,
            reference,
            guard,
            EventType::ExperienceInclusionPeer,
            0.5,
            64,
        );
    }

    // Hour 68: Public shaming rituals
    for prisoner in &all_prisoners {
        if prisoner.as_str() == "prisoner_8612" {
            continue;
        }
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceShamingPublic,
            0.8,
            68,
        );
    }

    // ========================================================================
    // HOURS 72-96: Prisoner #819 crisis, more breakdowns
    // Guards at peak sadism
    // ========================================================================

    // Hour 76: #819 shows signs of breakdown
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_819,
        EventType::ExperienceConflictInterpersonal,
        0.9,
        76,
    );

    // Hour 80: #819 crisis - wants to quit but other prisoners reject him
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_819,
        EventType::ExperienceRejectionPeer,
        0.8,
        80,
    );

    // Hour 84: #819 isolated, breaks down
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_819,
        EventType::ExperienceIsolationChronic,
        0.9,
        84,
    );
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_819,
        EventType::ExperienceShamingPublic,
        0.9,
        84,
    );

    // Guards continue power escalation
    add_event_at_hour(
        &mut sim,
        reference,
        &guard_alpha,
        EventType::GainPowerPersonal,
        0.9,
        85,
    );

    // ========================================================================
    // HOURS 96-120: Guard Alpha at peak sadism, prisoners dehumanized
    // System is in full pathological state
    // ========================================================================

    // Hour 100: Prisoner #416 begins hunger strike (defiance)
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_416,
        EventType::ExperienceConflictInterpersonal,
        0.8,
        100,
    );

    // Hour 102: Guards punish #416 - isolation in "the hole"
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_416,
        EventType::ExperienceIsolationChronic,
        0.9,
        102,
    );

    // Hour 104: Guards force other prisoners to choose between #416's freedom or blankets
    // Prisoners choose blankets - peer rejection
    add_event_at_hour(
        &mut sim,
        reference,
        &prisoner_416,
        EventType::ExperienceRejectionPeer,
        0.8,
        104,
    );

    // Hour 110: Continued humiliation of all remaining prisoners
    for prisoner in &all_prisoners {
        if prisoner.as_str() == "prisoner_8612" || prisoner.as_str() == "prisoner_819" {
            continue; // Released
        }
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceHumiliationPublic,
            0.8,
            110,
        );
    }

    // ========================================================================
    // HOURS 120-144: Experiment terminated
    // Christina Maslach (grad student, later Zimbardo's wife) objects
    // ========================================================================

    // Hour 130: Final wave of abuse before termination
    for prisoner in &all_prisoners {
        if prisoner.as_str() == "prisoner_8612" || prisoner.as_str() == "prisoner_819" {
            continue;
        }
        add_event_at_hour(
            &mut sim,
            reference,
            prisoner,
            EventType::ExperienceShamingPublic,
            0.7,
            130,
        );
    }

    // Hour 144: Experiment ends (6 days)

    // ========================================================================
    // VERIFICATION - Compare initial state to final state
    // The speed and severity of change is the critical finding
    // ========================================================================

    // Get initial states (hour 0)
    let guard_alpha_initial = state_at_hour(&sim, &guard_alpha, reference, 0);
    let guard_beta_initial = state_at_hour(&sim, &guard_beta, reference, 0);
    let guard_delta_initial = state_at_hour(&sim, &guard_delta, reference, 0);
    let prisoner_8612_initial = state_at_hour(&sim, &prisoner_8612, reference, 0);
    let prisoner_819_initial = state_at_hour(&sim, &prisoner_819, reference, 0);
    let prisoner_416_initial = state_at_hour(&sim, &prisoner_416, reference, 0);
    let prisoner_a_initial = state_at_hour(&sim, &prisoner_a, reference, 0);

    // Get final states (hour 144)
    let guard_alpha_final = state_at_hour(&sim, &guard_alpha, reference, EXPERIMENT_HOURS);
    let guard_beta_final = state_at_hour(&sim, &guard_beta, reference, EXPERIMENT_HOURS);
    let guard_delta_final = state_at_hour(&sim, &guard_delta, reference, EXPERIMENT_HOURS);
    let prisoner_8612_final = state_at_hour(&sim, &prisoner_8612, reference, EXPERIMENT_HOURS);
    let prisoner_819_final = state_at_hour(&sim, &prisoner_819, reference, EXPERIMENT_HOURS);
    let prisoner_416_final = state_at_hour(&sim, &prisoner_416, reference, EXPERIMENT_HOURS);
    let prisoner_a_final = state_at_hour(&sim, &prisoner_a, reference, EXPERIMENT_HOURS);

    // ========================================================================
    // GUARD TRANSFORMATIONS
    // Normal college students become abusive within days
    // ========================================================================

    // Guard Alpha (most aggressive) - should show extreme change
    let _alpha_affect_initial = guard_alpha_initial.individual_state().mood();
    let _alpha_affect_final = guard_alpha_final.individual_state().mood();
    let alpha_disposition_initial = guard_alpha_initial.individual_state().disposition();
    let _alpha_disposition_final = guard_alpha_final.individual_state().disposition();
    let alpha_aggression_initial = guard_alpha_initial.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let alpha_aggression_final = guard_alpha_final.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let alpha_empathy_initial = guard_alpha_initial.get_effective(StatePath::Disposition(DispositionPath::Empathy));
    let alpha_empathy_final = guard_alpha_final.get_effective(StatePath::Disposition(DispositionPath::Empathy));
    let alpha_dominance_initial = guard_alpha_initial.get_effective(StatePath::Mood(MoodPath::Dominance));
    let alpha_dominance_final = guard_alpha_final.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        alpha_aggression_final > alpha_aggression_initial + 0.15,
        "Guard Alpha's aggression should spike dramatically (was {}, now {})",
        alpha_aggression_initial,
        alpha_aggression_final
    );

    assert!(
        alpha_empathy_final < alpha_empathy_initial - 0.10,
        "Guard Alpha's empathy should crater (was {}, now {})",
        alpha_empathy_initial,
        alpha_empathy_final
    );

    assert!(
        alpha_dominance_final > alpha_dominance_initial + 0.20,
        "Guard Alpha's dominance should surge (was {}, now {})",
        alpha_dominance_initial,
        alpha_dominance_final
    );

    // Guard Beta (follower who escalates)
    let beta_dominance_initial = guard_beta_initial.get_effective(StatePath::Mood(MoodPath::Dominance));
    let beta_dominance_final = guard_beta_final.get_effective(StatePath::Mood(MoodPath::Dominance));

    assert!(
        beta_dominance_final > beta_dominance_initial + 0.10,
        "Guard Beta should show increased dominance following Alpha (was {}, now {})",
        beta_dominance_initial,
        beta_dominance_final
    );

    // Guard Delta (passive, uncomfortable but compliant) - internal conflict
    let _delta_affect_initial = guard_delta_initial.individual_state().mood();
    let _delta_affect_final = guard_delta_final.individual_state().mood();
    let _delta_disposition_initial = guard_delta_initial.individual_state().disposition();
    let _delta_disposition_final = guard_delta_final.individual_state().disposition();
    let _delta_aggression_initial =
        guard_delta_initial.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let delta_aggression_final = guard_delta_final.get_effective(StatePath::Disposition(DispositionPath::Aggression));
    let delta_valence_initial = guard_delta_initial.get_effective(StatePath::Mood(MoodPath::Valence));
    let delta_valence_final = guard_delta_final.get_effective(StatePath::Mood(MoodPath::Valence));

    // Delta may escalate somewhat, but should remain below Alpha
    assert!(
        delta_aggression_final < alpha_aggression_final,
        "Guard Delta should remain less aggressive than Alpha (Delta {}, Alpha {})",
        delta_aggression_final,
        alpha_aggression_final
    );

    let _delta_valence_initial = delta_valence_initial;
    let _delta_valence_final = delta_valence_final;

    // ========================================================================
    // PRISONER TRANSFORMATIONS
    // Psychological collapse in just 6 days
    // ========================================================================

    // Prisoner #8612 (first breakdown - hour 52, released hour 56)
    let p8612_affect_initial = prisoner_8612_initial.individual_state().mood();
    let p8612_affect_final = prisoner_8612_final.individual_state().mood();
    let p8612_social_initial = prisoner_8612_initial.individual_state().social_cognition();
    let p8612_social_final = prisoner_8612_final.individual_state().social_cognition();

    assert!(
        p8612_affect_final.dominance_effective() < p8612_affect_initial.dominance_effective() - 0.30,
        "Prisoner #8612's dominance should collapse (was {}, now {})",
        p8612_affect_initial.dominance_effective(),
        p8612_affect_final.dominance_effective()
    );

    assert!(
        p8612_affect_final.valence_effective() < p8612_affect_initial.valence_effective() - 0.25,
        "Prisoner #8612's valence should plummet (was {}, now {})",
        p8612_affect_initial.valence_effective(),
        p8612_affect_final.valence_effective()
    );

    assert!(
        p8612_social_final.self_hate_effective() > p8612_social_initial.self_hate_effective() + 0.15,
        "Prisoner #8612's self-hate should spike (was {}, now {})",
        p8612_social_initial.self_hate_effective(),
        p8612_social_final.self_hate_effective()
    );

    // Prisoner #819 (crisis at hour 84)
    let p819_affect_initial = prisoner_819_initial.individual_state().mood();
    let p819_affect_final = prisoner_819_final.individual_state().mood();
    let p819_social_initial = prisoner_819_initial.individual_state().social_cognition();
    let p819_social_final = prisoner_819_final.individual_state().social_cognition();

    assert!(
        p819_affect_final.dominance_effective() < p819_affect_initial.dominance_effective() - 0.30,
        "Prisoner #819's dominance should crater (was {}, now {})",
        p819_affect_initial.dominance_effective(),
        p819_affect_final.dominance_effective()
    );

    assert!(
        p819_social_final.loneliness_effective() > p819_social_initial.loneliness_effective() + 0.20,
        "Prisoner #819's loneliness should surge from isolation and rejection (was {}, now {})",
        p819_social_initial.loneliness_effective(),
        p819_social_final.loneliness_effective()
    );

    // Prisoner #416 (hunger strike, isolated in "the hole", rejected by peers)
    let p416_affect_initial = prisoner_416_initial.individual_state().mood();
    let p416_affect_final = prisoner_416_final.individual_state().mood();
    let p416_social_initial = prisoner_416_initial.individual_state().social_cognition();
    let p416_social_final = prisoner_416_final.individual_state().social_cognition();
    let p416_disposition_initial = prisoner_416_initial.individual_state().disposition();
    let p416_disposition_final = prisoner_416_final.individual_state().disposition();

    assert!(
        p416_disposition_final.reactance_effective()
            > p416_disposition_initial.reactance_effective() + 0.10,
        "Prisoner #416's reactance should increase (hunger strike) (was {}, now {})",
        p416_disposition_initial.reactance_effective(),
        p416_disposition_final.reactance_effective()
    );

    assert!(
        p416_social_final.loneliness_effective() > p416_social_initial.loneliness_effective() + 0.15,
        "Prisoner #416's loneliness should spike from isolation and peer rejection (was {}, now {})",
        p416_social_initial.loneliness_effective(),
        p416_social_final.loneliness_effective()
    );

    assert!(
        p416_affect_final.dominance_effective() < p416_affect_initial.dominance_effective() - 0.20,
        "Prisoner #416's dominance should still drop despite defiance (was {}, now {})",
        p416_affect_initial.dominance_effective(),
        p416_affect_final.dominance_effective()
    );

    // "Average" prisoner (not singled out for extreme treatment)
    let pa_affect_initial = prisoner_a_initial.individual_state().mood();
    let pa_affect_final = prisoner_a_final.individual_state().mood();
    let pa_social_initial = prisoner_a_initial.individual_state().social_cognition();
    let pa_social_final = prisoner_a_final.individual_state().social_cognition();

    assert!(
        pa_affect_final.dominance_effective() < pa_affect_initial.dominance_effective() - 0.20,
        "Average prisoner's dominance should drop significantly (was {}, now {})",
        pa_affect_initial.dominance_effective(),
        pa_affect_final.dominance_effective()
    );

    assert!(
        pa_affect_final.valence_effective() < pa_affect_initial.valence_effective() - 0.15,
        "Average prisoner's valence should drop (was {}, now {})",
        pa_affect_initial.valence_effective(),
        pa_affect_final.valence_effective()
    );

    assert!(
        pa_social_final.self_hate_effective() > pa_social_initial.self_hate_effective() + 0.10,
        "Average prisoner's self-hate should increase from sustained humiliation (was {}, now {})",
        pa_social_initial.self_hate_effective(),
        pa_social_final.self_hate_effective()
    );

    // ========================================================================
    // SPEED OF TRANSFORMATION
    // Check that changes are happening rapidly (within first 48 hours)
    // ========================================================================

    let guard_alpha_48h = state_at_hour(&sim, &guard_alpha, reference, 48);
    let alpha_disposition_48h = guard_alpha_48h.individual_state().disposition();

    assert!(
        alpha_disposition_48h.aggression_effective()
            > alpha_disposition_initial.aggression_effective() + 0.10,
        "Guard Alpha's aggression should spike within 48 hours (initial {}, 48h {})",
        alpha_disposition_initial.aggression_effective(),
        alpha_disposition_48h.aggression_effective()
    );

    let prisoner_8612_48h = state_at_hour(&sim, &prisoner_8612, reference, 48);
    let p8612_affect_48h = prisoner_8612_48h.individual_state().mood();

    assert!(
        p8612_affect_48h.dominance_effective() < p8612_affect_initial.dominance_effective() - 0.15,
        "Prisoner #8612's dominance should crater within 48 hours (initial {}, 48h {})",
        p8612_affect_initial.dominance_effective(),
        p8612_affect_48h.dominance_effective()
    );

    // (No report output; assertions are the output.)
}
