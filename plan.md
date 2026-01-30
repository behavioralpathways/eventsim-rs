# Event Impact Matrix Implementation Plan

## Executive Summary

Replace the current category-based event routing with an event-type-specific impact matrix that explicitly defines values across ALL 22 psychological dimensions, with per-dimension temporal behavior (chronic flags and permanence values).

---

## Part 1: The Problem

### Current Architecture

Events are routed by **category**, not by **event type**:

```rust
match category {
    EventCategory::SocialBelonging => {
        // Only hits TB pathway
        loneliness_delta = 0.2 * severity;
    }
    EventCategory::BurdenPerception => {
        // Only hits PB pathway
        perceived_liability_delta = 0.25 * severity;
    }
    // ...
}
```

### The Flaw

Multi-pathway events get incorrectly processed:

- `JobLoss` is categorized as `BurdenPerception`
- Therefore it ONLY hits PB pathway
- But job loss ALSO affects TB (loss of work community, daily social interaction)
- The boolean flags are IGNORED by the category-based routing

### Impact

The boolean flags `(affects_tb, affects_pb, affects_ac)` are essentially dead code. The actual processing ignores them and routes purely by category.

---

## Part 2: The Solution

### Core Insight

**Events are just containers for impact values.** An event type is:
- A label (for convenience and research documentation)
- Impact magnitudes across ALL 22 psychological dimensions
- Per-dimension temporal behavior (chronic vs acute, permanence)

### Data Structures

Located in `src/event/event_spec.rs`:

```rust
/// Impact values for all 22 psychological dimensions.
/// Each value represents the maximum delta at severity 1.0.
pub struct EventImpact {
    // Mood (PAD) - 3 dimensions
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,

    // Needs - 3 dimensions
    pub fatigue: f32,
    pub stress: f32,
    pub purpose: f32,

    // Social Cognition - 5 dimensions
    pub loneliness: f32,
    pub prc: f32,
    pub perceived_liability: f32,
    pub self_hate: f32,
    pub perceived_competence: f32,

    // Mental Health - 5 dimensions
    pub depression: f32,
    pub self_worth: f32,
    pub hopelessness: f32,
    pub interpersonal_hopelessness: f32,
    pub acquired_capability: f32,

    // Disposition - 6 dimensions
    pub impulse_control: f32,
    pub empathy: f32,
    pub aggression: f32,
    pub grievance: f32,
    pub reactance: f32,
    pub trust_propensity: f32,
}

/// Per-dimension chronic flags.
/// true = routes to chronic_delta (4x slower decay)
/// false = routes to delta (normal decay)
/// Note: AC has no entry - it's always permanent (no decay).
pub struct ChronicFlags {
    pub valence: bool,
    pub arousal: bool,
    // ... 21 bool fields (no acquired_capability)
}

/// Per-dimension permanence values (0.0 to 1.0).
/// Represents what portion of the impact becomes a permanent base shift.
/// Note: AC has no entry - system enforces 100% permanence.
pub struct PermanenceValues {
    pub valence: f32,
    pub arousal: f32,
    // ... 21 f32 fields (no acquired_capability)
}

/// Complete event specification.
pub struct EventSpec {
    pub impact: EventImpact,
    pub chronic: ChronicFlags,
    pub permanence: PermanenceValues,
}

/// Result of applying an event at a given severity.
pub struct AppliedDeltas {
    pub permanent: EventImpact,  // Goes to base (never decays)
    pub acute: EventImpact,      // Goes to delta (normal decay)
    pub chronic: EventImpact,    // Goes to chronic_delta (4x slower decay)
}

impl EventSpec {
    pub fn apply(&self, severity: f32) -> AppliedDeltas {
        let s = severity.clamp(0.0, 1.0);
        let scaled = self.impact.scale(s);
        let temp_portion = self.permanence.inverse();
        let temp_scaled = scaled.mul_permanence(&temp_portion);

        AppliedDeltas {
            permanent: scaled.mul_permanence(&self.permanence),
            acute: self.chronic.mask_inverse(&temp_scaled),
            chronic: self.chronic.mask(&temp_scaled),
        }
    }
}
```

### How It Works

Given an event with severity 0.8:

1. **Scale impact by severity**: `impact * 0.8`
2. **Split by permanence**:
   - Permanent portion: `scaled * permanence` -> goes to base
   - Temporary portion: `scaled * (1 - permanence)` -> continues to step 3
3. **Route by chronic flags**:
   - Where `chronic = true`: temporary portion -> `chronic_delta`
   - Where `chronic = false`: temporary portion -> `delta`

Example for valence with impact=-0.55, permanence=0.05, chronic=false, severity=0.8:
- Scaled: -0.55 * 0.8 = -0.44
- Permanent: -0.44 * 0.05 = -0.022 -> base
- Temporary: -0.44 * 0.95 = -0.418 -> delta (because chronic=false)

### Special Handling: Acquired Capability

AC is always:
- **100% permanent** - Goes entirely to base, never decays
- **Non-negative** - You cannot un-learn habituation to pain/death
- System enforces this automatically - permanence field is ignored for AC

---

## Part 3: Event File Structure

### Location

```
src/event/types/
  mod.rs                        <- Module exports
  end_relationship_romantic.rs  <- One file per event type
  lose_job_layoff.rs
  lose_job_fired.rs
  ...
```

### File Format

Each event is a Rust source file exporting `pub const SPEC: EventSpec`:

```rust
//! EndRelationshipRomantic event specification.
//!
//! End of a romantic relationship (breakup, divorce).

use crate::event::event_impact::{ChronicFlags, EventImpact, EventSpec, PermanenceValues};

pub const SPEC: EventSpec = EventSpec {
    impact: EventImpact {
        // Mood - Valence
        // Romantic breakups produce severe but temporary negative affect
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        valence: -0.55,

        // Mood - Arousal
        // Significant sympathetic nervous system activation
        // https://pmc.ncbi.nlm.nih.gov/articles/PMC11290389/
        arousal: 0.55,

        // ... all 22 dimensions with rationale and citation
    },

    chronic: ChronicFlags {
        valence: false,
        arousal: false,
        // ... 21 bool values
    },

    permanence: PermanenceValues {
        valence: 0.05,
        arousal: 0.05,
        // ... 21 f32 values
    },
};
```

### Comment Format

Each field uses a multi-line comment block:
```rust
// Category - Dimension
// {One sentence rationale}
// {Research URL or citation}
field: value,
```

### Naming Convention

**snake_case file names** matching enum variants:
- `end_relationship_romantic.rs` -> `EventType::EndRelationshipRomantic`
- `lose_job_layoff.rs` -> `EventType::LoseJobLayoff`
- `suffer_violence_physical.rs` -> `EventType::SufferViolencePhysical`

---

## Part 4: Research Infrastructure

### Research Agents

Located in `.claude/agents/research/`, one agent per dimension:

```
.claude/agents/research/
  valence.md
  arousal.md
  dominance.md
  fatigue.md
  stress.md
  purpose.md
  loneliness.md
  prc.md
  perceived-liability.md
  self-hate.md
  perceived-competence.md
  depression.md
  self-worth.md
  hopelessness.md
  interpersonal-hopelessness.md
  acquired-capability.md
  impulse-control.md
  empathy.md
  aggression.md
  grievance.md
  reactance.md
  trust-propensity.md
```

Each agent contains:
- Dimension definition and range
- Research focus and considerations
- Impact calibration scale
- Permanence calibration scale (with accumulation test)
- Output format
- Key academic sources

### Research Command

`/event-research-update {event_name}` dispatches 22 haiku agents in parallel to:
1. Read dimension-specific research guidance
2. Web search for psychological literature
3. Return calibrated impact, chronic, and permanence values
4. Update the Rust source file directly

### Permanence Calibration

All research agents use this permanence scale:

| Permanence | Meaning | Example Events |
|------------|---------|----------------|
| 0.00-0.02 | Negligible | Minor setbacks, daily stressors |
| 0.03-0.08 | Small | Breakup, job loss, friendship ending |
| 0.10-0.15 | Moderate | Divorce, serious illness recovery |
| 0.20-0.30 | Significant | Death of parent, permanent disability |
| 0.40-0.50 | Severe | Death of child, violent trauma |

**Accumulation test**: If this event happens 5 times, is the total permanent shift realistic?

---

## Part 5: Rust Implementation

### Event Type Enum

```rust
// src/enums/event_type.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    // TB Pathway
    ExperienceExclusionPeer,
    ExperienceInclusionPeer,
    ExperienceRejectionRomantic,
    EndRelationshipRomantic,
    // ...

    // PB Pathway
    ReceiveFeedbackBurden,
    ExperienceShamingPublic,
    // ...

    // AC Pathway
    SufferViolencePhysical,
    SurviveAttemptSuicide,
    // ...

    // Multi-Pathway
    LoseJobLayoff,
    LoseJobFired,
    LosePersonDeath,
    // ...

    /// Custom event with developer-provided EventSpec.
    Custom,
}

impl EventType {
    /// Returns the EventSpec for this event type.
    pub fn spec(&self) -> EventSpec {
        match self {
            Self::EndRelationshipRomantic => end_relationship_romantic::SPEC,
            Self::LoseJobLayoff => lose_job_layoff::SPEC,
            // ...
            Self::Custom => EventSpec::default(),
        }
    }
}
```

### Event Struct

```rust
// src/event/event.rs
pub struct Event {
    event_type: EventType,
    severity: f32,
    target: EntityId,
    spec: EventSpec,  // For custom events or override
}

impl Event {
    /// Creates a standard event.
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            severity: 1.0,
            target: EntityId::default(),
            spec: event_type.spec(),
        }
    }

    /// Creates a custom event with explicit spec.
    pub fn custom(spec: EventSpec) -> Self {
        Self {
            event_type: EventType::Custom,
            severity: 1.0,
            target: EntityId::default(),
            spec,
        }
    }

    /// Applies the event and returns deltas to apply.
    pub fn apply(&self) -> AppliedDeltas {
        self.spec.apply(self.severity)
    }
}
```

### Processor Integration

```rust
// src/processor/event.rs
pub fn process_event(event: &Event, state: &mut EntityState) {
    let deltas = event.apply();

    // Apply permanent changes to base
    state.affect_mut().valence_mut().shift_base(deltas.permanent.valence);
    state.affect_mut().arousal_mut().shift_base(deltas.permanent.arousal);
    // ... all 22 dimensions

    // Apply acute changes to delta
    state.affect_mut().valence_mut().add_delta(deltas.acute.valence);
    // ... all dimensions where chronic=false

    // Apply chronic changes to chronic_delta
    state.affect_mut().valence_mut().add_chronic_delta(deltas.chronic.valence);
    // ... all dimensions where chronic=true
}
```

---

## Part 6: Testing Strategy

### What TO Test

1. **EventSpec::apply() logic** - Given impact/chronic/permanence, verify correct splitting
2. **Accumulation behavior** - Multiple events produce realistic total permanent shift
3. **AC special handling** - Verify AC is always 100% permanent, never negative
4. **Matrix completeness** - All EventType variants have defined specs

### What NOT to Test

- Don't test each event type's specific values (those are research-determined)
- Don't create 58 separate test files for each event
- Don't test that "JobLoss does the right thing" - that's just a lookup

### Example Tests

```rust
#[test]
fn apply_splits_impact_by_permanence() {
    let spec = EventSpec {
        impact: EventImpact { valence: -1.0, ..Default::default() },
        chronic: ChronicFlags::default(),
        permanence: PermanenceValues { valence: 0.10, ..Default::default() },
    };

    let deltas = spec.apply(1.0);

    assert_eq!(deltas.permanent.valence, -0.10);  // 10% permanent
    assert_eq!(deltas.acute.valence, -0.90);      // 90% temporary
    assert_eq!(deltas.chronic.valence, 0.0);      // chronic=false
}

#[test]
fn chronic_flag_routes_to_chronic_delta() {
    let spec = EventSpec {
        impact: EventImpact { stress: 0.50, ..Default::default() },
        chronic: ChronicFlags { stress: true, ..Default::default() },
        permanence: PermanenceValues { stress: 0.0, ..Default::default() },
    };

    let deltas = spec.apply(1.0);

    assert_eq!(deltas.acute.stress, 0.0);      // Not acute
    assert_eq!(deltas.chronic.stress, 0.50);   // All goes to chronic
}

#[test]
fn ac_is_always_fully_permanent() {
    let spec = EventSpec {
        impact: EventImpact { acquired_capability: 0.80, ..Default::default() },
        chronic: ChronicFlags::default(),
        permanence: PermanenceValues::default(),  // Ignores permanence for AC
    };

    let deltas = spec.apply(1.0);

    assert_eq!(deltas.permanent.acquired_capability, 0.80);  // All permanent
    assert_eq!(deltas.acute.acquired_capability, 0.0);       // None to delta
    assert_eq!(deltas.chronic.acquired_capability, 0.0);     // None to chronic
}

#[test]
fn five_breakups_produce_realistic_permanent_shift() {
    let spec = end_relationship_romantic::SPEC;
    let mut total_permanent_valence = 0.0;

    for _ in 0..5 {
        let deltas = spec.apply(1.0);
        total_permanent_valence += deltas.permanent.valence;
    }

    // 5 breakups at permanence=0.05 -> 5 * (-0.55 * 0.05) = -0.1375
    // This is a small but realistic lifetime accumulation
    assert!(total_permanent_valence > -0.20);
    assert!(total_permanent_valence < -0.10);
}
```

---

## Part 7: Implementation Steps

### Phase 1: Foundation (Done)

- [x] Create `src/event/event_spec.rs` with EventImpact, ChronicFlags, PermanenceValues, EventSpec, AppliedDeltas
- [x] Create `src/event/types/` directory structure
- [x] Create first event file: `end_relationship_romantic.rs`
- [x] Create 22 research agents in `.claude/agents/research/`
- [x] Create `/event-research-update` command

### Phase 2: Research Events

- [ ] Run `/event-research-update` for each event type
- [ ] Review and validate research results
- [ ] Ensure all events have realistic permanence values (accumulation test)

### Phase 3: Integration

- [ ] Add `src/event/types/mod.rs` exporting all event modules
- [ ] Update `EventType` enum to use new spec() method
- [ ] Wire EventSpec into event processor
- [ ] Remove old category-based routing

### Phase 4: Cleanup

- [ ] Remove `EventType::category()` method
- [ ] Remove `EventType::its_pathways()` method
- [ ] Remove `EventCategory` enum (or keep for documentation only)
- [ ] Update all tests to use new architecture

### Phase 5: Validation

- [ ] Run `make test` - ensure 100% coverage
- [ ] API surface review
- [ ] Theory expert validation

---

## Part 8: File Inventory

### Core Files

```
src/event/
  event_impact.rs              <- EventImpact, ChronicFlags, PermanenceValues, EventSpec
  types/
    mod.rs                     <- Module exports
    end_relationship_romantic.rs
    lose_job_layoff.rs
    lose_job_fired.rs
    ... (58 event files)
```

### Research Infrastructure

```
.claude/
  agents/
    research/
      valence.md
      arousal.md
      ... (22 dimension agents)
  commands/
    event-research-update.md
```

### Files to Modify

```
src/enums/event_type.rs        <- Add spec() method
src/event/event.rs             <- Use EventSpec
src/processor/event.rs         <- Replace category routing
```

---

## Appendix: The 22 Dimensions

| Category | Dimension | Range | Description |
|----------|-----------|-------|-------------|
| **Mood** | valence | -1 to +1 | Positive vs negative affect |
| | arousal | -1 to +1 | Calm vs activated |
| | dominance | -1 to +1 | Controlled vs in-control |
| **Needs** | fatigue | -1 to +1 | Energized vs exhausted |
| | stress | -1 to +1 | Relaxed vs stressed |
| | purpose | -1 to +1 | Meaningless vs meaningful |
| **Social** | loneliness | -1 to +1 | Connected vs isolated |
| | prc | -1 to +1 | Perceived reciprocal caring |
| | perceived_liability | -1 to +1 | Contributor vs burden |
| | self_hate | -1 to +1 | Self-compassion vs self-loathing |
| | perceived_competence | -1 to +1 | Incompetent vs capable |
| **Mental Health** | depression | -1 to +1 | Euthymic vs depressed |
| | self_worth | -1 to +1 | Worthless vs valuable |
| | hopelessness | -1 to +1 | Optimistic vs hopeless |
| | interpersonal_hopelessness | -1 to +1 | Help possible vs pointless |
| | acquired_capability | 0 to +1 | No habituation vs fully habituated |
| **Disposition** | impulse_control | -1 to +1 | Impulsive vs controlled |
| | empathy | -1 to +1 | Low vs high empathy |
| | aggression | -1 to +1 | Peaceful vs aggressive |
| | grievance | -1 to +1 | Fair treatment vs victimized |
| | reactance | -1 to +1 | Accepting vs resistant |
| | trust_propensity | -1 to +1 | Distrustful vs trusting |

**Computed values (not in EventImpact):**
- TB (Thwarted Belongingness) = f(loneliness, prc)
- PB (Perceived Burdensomeness) = f(perceived_liability, self_hate)
