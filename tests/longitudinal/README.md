# Longitudinal Tests

These tests are written to exercise the library strictly through the public API, the same way an external developer would use it. That means:

- ONE TEST PER FILE
- Use only exported types and functions from the public crate surface (e.g., `eventsim_rs::...`).
- Do not call internal modules, private helpers, or crate-private items.
- Build scenarios using `EntityBuilder`, `Simulation`, `EventBuilder`, and other public entry points.
- Assert outcomes via public getters and query methods (e.g., `state_at`, `entity`, `age_at_timestamp`).

If a test requires internal access to verify behavior, it does not belong here. Add it to unit tests instead.
