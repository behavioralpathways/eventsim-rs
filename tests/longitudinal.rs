//! Longitudinal test modules.
//!
//! Complex, long-running simulation tests spanning years or decades.
//! See tests/longitudinal/README.md for details and running instructions.
//!
//! All tests are marked #[ignore] - run explicitly with:
//! `cargo test --test longitudinal -- --ignored --nocapture`

#[path = "longitudinal/adverse_childhood.rs"]
mod adverse_childhood;
#[path = "longitudinal/adolescent_exclusion_shaming.rs"]
mod adolescent_exclusion_shaming;
#[path = "longitudinal/chronic_illness.rs"]
mod chronic_illness;
#[path = "longitudinal/chronic_vs_intermittent_isolation.rs"]
mod chronic_vs_intermittent_isolation;
#[path = "longitudinal/cult_formation.rs"]
mod cult_formation;
#[path = "longitudinal/great_depression_cohort.rs"]
mod great_depression_cohort;
#[path = "longitudinal/immigrant_relocation.rs"]
mod immigrant_relocation;
#[path = "longitudinal/intergenerational_trauma.rs"]
mod intergenerational_trauma;
#[path = "longitudinal/legal_charges.rs"]
mod legal_charges;
#[path = "longitudinal/lifespan_trajectory.rs"]
mod lifespan_trajectory;
#[path = "longitudinal/long_term_incarceration.rs"]
mod long_term_incarceration;
#[path = "longitudinal/mentorship_achievement.rs"]
mod mentorship_achievement;
#[path = "longitudinal/post_war_prosperity.rs"]
mod post_war_prosperity;
#[path = "longitudinal/repeated_financial_strain.rs"]
mod repeated_financial_strain;
#[path = "longitudinal/secure_childhood_buffer.rs"]
mod secure_childhood_buffer;
#[path = "longitudinal/ww2_combat_exposure.rs"]
mod ww2_combat_exposure;
#[path = "longitudinal/stanford_prison.rs"]
mod stanford_prison;
#[path = "longitudinal/shooter_pathway.rs"]
mod shooter_pathway;
