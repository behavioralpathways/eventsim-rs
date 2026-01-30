# Getting Started with Behavioral Pathways

A practical guide to using the behavioral simulation system.

---

## Core Concept

This library models how entities (people, animals, AI) think, feel, and relate over time. You create entities, add events that happen to them, and query their psychological state at any point in time.

---

## Two Pathways for Psychological Change

The system models psychological change through two distinct pathways, each grounded in empirical research:

### Short-Term Pathway: Delta + Decay

**Events create temporary deltas that decay back to baseline over time.**

This models everyday psychological fluctuations - a bad day at work lowers your mood, but you recover. Getting rejected makes you feel lonely, but that feeling fades.

```
Event occurs → Delta applied → Time passes → Delta decays → Returns to base
```

| Characteristic | Short-Term Pathway |
|----------------|-------------------|
| Trigger | Any event |
| Effect | Temporary state deviation |
| Duration | Hours to weeks |
| Mechanism | Delta + decay |
| Recovery | Automatic over time |

**Example:** Social exclusion event → Loneliness delta +0.3 → Decays over 2 weeks → Returns to baseline

### Long-Term Pathway: Formative Events + Base Shifts

**Major life events permanently alter personality traits.**

This models how trauma, major transitions, and transformative experiences change who we are at a fundamental level. Based on research by Roberts (Social Investment Theory), Tedeschi & Calhoun (Post-Traumatic Growth), and Bleidorn (Life Events Research).

```
Formative event occurs → Base shift applied → Permanent personality change
```

| Characteristic | Long-Term Pathway |
|----------------|-------------------|
| Trigger | Formative events (with_base_shift) |
| Effect | Permanent personality change |
| Duration | Permanent (with partial recovery for severe shifts) |
| Mechanism | Base shift records |
| Recovery | None (or partial for severe shifts) |

**Example:** Severe betrayal event → Agreeableness base shift -0.25 → Permanent personality change

### When to Use Each Pathway

| Scenario | Pathway | Why |
|----------|---------|-----|
| Bad day at work | Short-term | Temporary mood effect |
| Job loss | Both | Immediate distress (delta) + potential personality change (base shift) |
| Minor criticism | Short-term | Temporary, not formative |
| First job | Long-term | Role entry changes personality (Roberts) |
| Trauma | Both | Immediate PTSD symptoms (delta) + permanent effects (base shift) |
| Parenthood | Long-term | Major role change affects personality |
| Argument with friend | Short-term | Temporary relationship tension |
| Severe betrayal | Both | Immediate trust damage (delta) + permanent Agreeableness change |

### Empirical Constraints

Formative events are constrained by research:

| Constraint | Value | Source |
|------------|-------|--------|
| Max single event shift | 0.30 | Severe trauma effect size (Bleidorn) |
| Cumulative lifetime max | 1.0 per trait | Hard cap |
| Age plasticity | 1.3x (<18) to 0.6x (70+) | Caspi longitudinal data |
| Trait stability | 0.60-0.85 | HEXACO factor stability |
| Severe shift recovery | 70% retained over 180 days | Post-traumatic growth research |

---

## Basic Usage

### 1. Create a Simulation

```rust
use eventsim_rs::simulation::SimulationBuilder;
use eventsim_rs::types::Timestamp;

let reference_date = Timestamp::from_ymd_hms(2020, 1, 1, 0, 0, 0);
let mut sim = SimulationBuilder::new(reference_date).build()?;
```

### 2. Add Entities

```rust
use eventsim_rs::entity::EntityBuilder;
use eventsim_rs::enums::Species;

let person = EntityBuilder::new()
    .id("john")
    .species(Species::Human)
    .age(Duration::years(30))
    .build()?;

let anchor_time = Timestamp::from_ymd_hms(2020, 1, 1, 0, 0, 0);
sim.add_entity(person, anchor_time);
```

**What is an anchor?** The anchor is when the entity "enters" your simulation with their baseline state. All computation radiates from this point.

### 3. Add Events

```rust
use eventsim_rs::event::EventBuilder;
use eventsim_rs::enums::EventType;

let event = EventBuilder::new(EventType::ExperienceExclusionPeer)
    .severity(0.7)
    .target(person_id.clone())
    .build()?;

let event_time = Timestamp::from_ymd_hms(2020, 6, 15, 0, 0, 0);
sim.add_event(event, event_time);
```

### 4. Query State

```rust
let query_time = Timestamp::from_ymd_hms(2020, 12, 31, 0, 0, 0);
let handle = sim.entity(&person_id)?;
let state = handle.state_at(query_time);

println!("Valence: {}", state.mood().valence());
println!("Loneliness: {}", state.needs().loneliness());
```

### 5. Add Formative Events (Permanent Personality Changes)

For major life events that permanently change personality, use `with_base_shift()`:

```rust
use eventsim_rs::event::EventBuilder;
use eventsim_rs::enums::{EventType, HexacoPath};

// Severe betrayal permanently decreases Agreeableness and increases Neuroticism
let betrayal = EventBuilder::new(EventType::ExperienceBetrayalTrust)
    .target(person_id.clone())
    .severity(0.9)
    .with_base_shift(HexacoPath::Agreeableness, -0.25)
    .with_base_shift(HexacoPath::Neuroticism, 0.15)
    .build()?;

sim.add_event(betrayal, Timestamp::from_ymd_hms(2020, 8, 1, 0, 0, 0));

// Query state after the event - personality has permanently changed
let state = handle.state_at(Timestamp::from_ymd_hms(2021, 1, 1, 0, 0, 0));
// Agreeableness is now lower than anchor state
// Neuroticism is now higher than anchor state
```

**Note:** Base shifts are automatically modified by:
- **Age plasticity** - younger entities are more affected
- **Trait stability** - some traits (Extraversion) resist change more than others
- **Sensitive periods** - certain ages amplify specific trait changes
- **Diminishing returns** - repeated shifts in the same direction have less effect

### HEXACO Traits Available for Base Shifts

| Trait | Path | Stability | Description |
|-------|------|-----------|-------------|
| Openness | `HexacoPath::Openness` | 0.80 | Intellectual curiosity, creativity |
| Conscientiousness | `HexacoPath::Conscientiousness` | 0.70 | Organization, self-discipline |
| Extraversion | `HexacoPath::Extraversion` | 0.85 | Sociability, assertiveness |
| Agreeableness | `HexacoPath::Agreeableness` | 0.65 | Cooperation, patience |
| Neuroticism | `HexacoPath::Neuroticism` | 0.60 | Emotional reactivity, anxiety |
| Honesty-Humility | `HexacoPath::HonestyHumility` | 0.75 | Integrity, fairness |

Higher stability = more resistant to change (requires larger input shifts).

---

## How Anchoring Works

### The Anchor Point Matters

Events are only processed when you query across them:

```rust
// John enters simulation at age 20 (2010)
sim.add_entity(john, Timestamp::from_ymd_hms(2010, 1, 1, 0, 0, 0));

// Childhood trauma happened at age 5 (1995) - BEFORE anchor
sim.add_event(trauma, Timestamp::from_ymd_hms(1995, 3, 15, 0, 0, 0));

// Query at age 20 (same as anchor)
let state = handle.state_at(Timestamp::from_ymd_hms(2010, 1, 1, 0, 0, 0));
// ❌ Trauma NOT included - event is before anchor
```

### Solution 1: Anchor Earlier

Start the simulation when the first event happens:

```rust
// Anchor John at age 5 (1995) - BEFORE trauma
sim.add_entity(john, Timestamp::from_ymd_hms(1995, 1, 1, 0, 0, 0));

// Add childhood trauma at age 5
sim.add_event(trauma, Timestamp::from_ymd_hms(1995, 3, 15, 0, 0, 0));

// Query at age 20 (2010) - 15 years AFTER anchor
let state = handle.state_at(Timestamp::from_ymd_hms(2010, 1, 1, 0, 0, 0));
// ✅ Trauma IS included - event is after anchor
```

### Solution 2: Set Initial State

If you can't anchor earlier, set the state directly. EntityBuilder supports initializing all state components:

```rust
use eventsim_rs::state::{
    MentalHealth, Mood, Needs, SocialCognition, Disposition
};

let john = EntityBuilder::new()
    .id("john")
    .species(Species::Human)
    .age(Duration::years(30))
    .mental_health(MentalHealth::new().with_acquired_capability_base(0.8))
    .mood(Mood::new().with_valence_base(-0.2))
    .social_cognition(SocialCognition::new().with_loneliness_base(0.4))
    .build()?;

// Anchor at age 20 with trauma effects already baked into state
sim.add_entity(john, Timestamp::from_ymd_hms(2010, 1, 1, 0, 0, 0));
```

**Available state builders:**
- `.mood(Mood)` - PAD dimensions (valence, arousal, dominance)
- `.needs(Needs)` - Fatigue, stress, purpose
- `.mental_health(MentalHealth)` - Depression, hopelessness, acquired capability
- `.social_cognition(SocialCognition)` - Loneliness, perceived caring, liability, self-hate
- `.disposition(Disposition)` - Empathy, aggression, impulse control

### The Rule

**Events before the anchor are only processed if you query a time before the anchor (backward regression).**

For forward queries (after the anchor), only events after the anchor matter.

---

## Hydrating from Persisted State

If you persist entity state (e.g., NPC genesis files), you can fully hydrate an entity and replay events on top:

```rust
// Load persisted genesis state (your serialization format)
let genesis: GenesisState = load_from_json("npc_001_genesis.json")?;

// Build entity with full initial state
let entity = EntityBuilder::new()
    .id(&genesis.id)
    .species(Species::Human)
    .birth_date(genesis.birth_date)
    .age(genesis.age)
    .hexaco(genesis.hexaco)
    .person_characteristics(genesis.person_characteristics)
    .mood(genesis.mood)
    .needs(genesis.needs)
    .mental_health(genesis.mental_health)
    .social_cognition(genesis.social_cognition)
    .disposition(genesis.disposition)
    .build()?;

// Add to simulation and replay events
sim.add_entity(entity, genesis.anchor_timestamp);
for event in stored_events {
    sim.add_event(event.event, event.timestamp);
}

// Query current state
let current_state = sim.entity(&genesis.id)?.state_at(Timestamp::now());
```

This pattern allows you to:
1. Persist entity state at creation time
2. Store events as they happen
3. Recompute current state on demand by replaying events

---

## Event Severity: Your Responsibility

The library doesn't know about your game's context (wealth, status, inventory). **You** compute severity based on your domain:

```rust
// Poor person wins lottery (achieves major goal)
let severity = if person.wealth < 10_000 {
    0.95  // Life-changing
} else if person.wealth > 1_000_000 {
    0.3   // Nice but minor
} else {
    0.7   // Significant
};

let lottery = EventBuilder::new(EventType::AchieveGoalMajor)
    .severity(severity)
    .build()?;
```

The library **does** personalize based on psychology:
- Personality traits (high emotionality → stronger reactions)
- Current state (high arousal → events more salient)
- Relationship context (trust affects interpretation)

But domain context (rich vs poor) is yours to handle.

---

## Event System

The event system gives you two ways to model psychological change:

- **Option A:** Use pre-built EventTypes with researched impact values
- **Option B:** Create custom events with your own specifications

Both approaches are first-class citizens. The built-in types are conveniences based on psychological research, not requirements. You have full control to define any psychological impact your application needs.

---

### Option A: Pre-built EventTypes

The library includes researched event types with impact values across 22 psychological dimensions. These are based on psychological literature and provide sensible defaults for common life events.

```rust
use eventsim_rs::event::EventBuilder;
use eventsim_rs::enums::EventType;

let event = EventBuilder::new(EventType::EndRelationshipRomantic)
    .severity(0.8)
    .target(person_id.clone())
    .build()?;

sim.add_event(event, event_time);
```

**Available EventTypes:**

| EventType | Description |
|-----------|-------------|
| `AchieveGoalMajor` | Achievement of a major life goal (graduating, career milestone) |
| `DevelopIllnessChronic` | Development of a chronic illness (diabetes, heart disease) |
| `EndRelationshipRomantic` | End of a romantic relationship (breakup, divorce) |
| `EngageSelfharmNonsuicidal` | Non-suicidal self-injury (NSSI) |
| `ExperienceAwarenessMortality` | Acute awareness of one's own mortality |
| `ExperienceBetrayalTrust` | Significant betrayal of trust by someone close |
| `ExperienceCombatMilitary` | Direct participation in military combat |
| `ExperienceConflictFamily` | Conflict within the family unit |
| `ExperienceConflictInterpersonal` | Interpersonal conflict outside of family |
| `ExperienceExclusionGroup` | Exclusion from a group or organization |
| `ExperienceExclusionPeer` | Exclusion by peers (social rejection) |
| `ExperienceHumiliationPublic` | Public humiliation or embarrassment |
| `ExperienceInclusionPeer` | Inclusion and acceptance by peers |
| `ExperienceIsolationChronic` | Chronic social isolation over extended period |

Use `EventType::all()` to list all available types programmatically.

---

### Option B: Custom Events

For events specific to your domain, or when you need precise control over psychological impacts, create custom events with your own EventSpec:

```rust
use eventsim_rs::event::{EventBuilder, Event};
use eventsim_rs::event::event_spec::{EventSpec, EventImpact, ChronicFlags, PermanenceValues};

let spec = EventSpec {
    impact: EventImpact {
        valence: -0.5,
        arousal: 0.3,
        dominance: -0.2,
        stress: 0.4,
        loneliness: 0.2,
        ..Default::default()  // All other dimensions = 0.0
    },
    chronic: ChronicFlags {
        stress: true,  // Stress decays slowly (weeks/months)
        ..Default::default()  // All others decay fast (hours/days)
    },
    permanence: PermanenceValues {
        loneliness: 0.1,  // 10% of loneliness impact is permanent
        ..Default::default()  // All others are fully temporary
    },
};

let event = Event::custom(spec);

// Or via builder with severity and target:
let event = EventBuilder::custom(spec)
    .severity(0.7)
    .target(person_id.clone())
    .build()?;
```

Custom events work exactly like built-in types. They go through the same processing pipeline and can model any psychological scenario your application requires.

---

### Understanding EventSpec

Every event (built-in or custom) is defined by an EventSpec with three components:

| Component | Description |
|-----------|-------------|
| `impact` | Base impact values for all 22 dimensions (-1.0 to 1.0) |
| `chronic` | Per-dimension flags: `true` = slow decay (weeks/months), `false` = fast decay (hours/days) |
| `permanence` | Per-dimension values (0.0 to 1.0): fraction of impact that becomes a permanent base shift |

**Inspecting a built-in spec:**

```rust
let spec = EventType::EndRelationshipRomantic.spec();
println!("Valence impact: {}", spec.impact.valence);      // -0.55
println!("Loneliness impact: {}", spec.impact.loneliness); // 0.35
```

**How `spec.apply(severity)` works:**

When an event is applied, it splits into three delta types based on chronic flags and permanence values:

```rust
let deltas = spec.apply(0.8);  // Apply at 80% severity

deltas.permanent  // Permanent base shifts (impact * severity * permanence)
deltas.acute      // Fast-decaying temporary effects (non-chronic, non-permanent portion)
deltas.chronic    // Slow-decaying temporary effects (chronic, non-permanent portion)
```

This allows a single event to have both immediate effects that fade quickly and lasting effects that persist.

---

### The 22 Psychological Dimensions

Events can impact any of these dimensions:

**Mood (PAD)**
- `valence` - Positive/negative feeling (-1 to 1)
- `arousal` - Activation level (-1 to 1)
- `dominance` - Sense of control (-1 to 1)

**Needs**
- `fatigue` - Energy depletion (0 to 1)
- `stress` - Accumulated pressure (0 to 1)
- `purpose` - Sense of meaning (-1 to 1)

**Social Cognition**
- `loneliness` - Social disconnection (0 to 1)
- `prc` - Perceived reciprocal caring (-1 to 1)
- `perceived_liability` - Feeling like a burden (0 to 1)
- `self_hate` - Self-directed negativity (0 to 1)
- `perceived_competence` - Self-efficacy (-1 to 1)

**Mental Health**
- `depression` - Persistent low mood (0 to 1)
- `self_worth` - Global self-valuation (-1 to 1)
- `hopelessness` - Future pessimism (0 to 1)
- `interpersonal_hopelessness` - Relationship pessimism (0 to 1)
- `acquired_capability` - Habituation to pain/fear (0 to 1, always permanent)

**Disposition**
- `impulse_control` - Self-regulation (-1 to 1)
- `empathy` - Responsiveness to others (-1 to 1)
- `aggression` - Hostile tendency (0 to 1)
- `grievance` - Accumulated injustice (0 to 1)
- `reactance` - Resistance to control (0 to 1)
- `trust_propensity` - Baseline trust (-1 to 1)

---

## Time Queries: Past, Present, Future

```rust
// Query past (backward regression)
let past_state = handle.state_at(anchor_time - Duration::years(5));

// Query present
let current_state = handle.state_at(Timestamp::now());

// Query future (forward projection)
let future_state = handle.state_at(Timestamp::now() + Duration::years(5));
```

**All queries compute fresh from the anchor state plus events.**

---

## ITS Risk Monitoring

The library includes Joiner's Interpersonal Theory of Suicide (ITS) risk assessment. This tracks three proximal factors:

- **TB (Thwarted Belongingness)**: Feeling disconnected from others
- **PB (Perceived Burdensomeness)**: Believing oneself to be a burden
- **AC (Acquired Capability)**: Habituation to pain/fear of death

### Checking Factor Convergence

```rust
use eventsim_rs::processor::compute_its_factors;
use eventsim_rs::enums::ItsAlert;

let state = handle.state_at(Timestamp::now());
let factors = compute_its_factors(&state);

// Check convergence status
if factors.convergence_status.is_three_factor_convergent {
    // HIGH RISK: All three factors elevated
}

// Get specific alert type from risk matrix
if let Some(alert) = ItsAlert::from_convergence(&factors.convergence_status) {
    match alert {
        ItsAlert::ThreeFactorConvergence => {
            // Immediate intervention needed
            println!("Risk Level: {}", alert.risk_level()); // 3
            println!("Intervention: {}", alert.intervention_focus());
        }
        ItsAlert::DesireWithoutCapability => {
            // Suicidal desire present, but no capability yet
            // Priority: prevent capability acquisition
        }
        ItsAlert::SingleFactorAc => {
            // Dormant capability (e.g., combat veteran)
            // Monitor for desire development
        }
        _ => {}
    }
}
```

### Using Events for ITS Pathway Tracking

When modeling life events, choose event types that map to the relevant ITS pathways:

```rust
use eventsim_rs::enums::EventType;

// TB pathway events (Thwarted Belongingness)
EventType::ExperienceExclusionPeer      // Social rejection
EventType::ExperienceExclusionGroup     // Group exclusion
EventType::ExperienceIsolationChronic   // Long-term isolation
EventType::EndRelationshipRomantic      // Breakup, divorce

// PB pathway events (Perceived Burdensomeness)
EventType::ExperienceHumiliationPublic  // Public shame
EventType::DevelopIllnessChronic        // Illness affecting self-perception
EventType::ExperienceConflictFamily     // Family conflict

// AC pathway events (Acquired Capability)
EventType::EngageSelfharmNonsuicidal    // NSSI (highest specificity)
EventType::ExperienceCombatMilitary     // Military combat exposure
EventType::ExperienceAwarenessMortality // Death awareness

// Multi-pathway events
EventType::ExperienceBetrayalTrust      // Affects trust, belongingness
```

Check the EventSpec for each type to see its full impact across all ITS-related dimensions:

```rust
let spec = EventType::ExperienceIsolationChronic.spec();
println!("Loneliness: {}", spec.impact.loneliness);              // TB indicator
println!("Perceived liability: {}", spec.impact.perceived_liability); // PB indicator
println!("Acquired capability: {}", spec.impact.acquired_capability); // AC indicator
```

---

## Next Steps

- **Simple example**: See `tests/simulation/affect/pad_dimensions/`
- **Complex example**: See `tests/longitudinal/tribal_dynamics_ten_year.rs`

---

## Common Pitfalls

### 1. Events Before Anchor Not Applied

**Problem:** Added an event before the anchor, but queries at/after anchor don't see it.

**Solution:** Either anchor earlier, or set the initial state to reflect past events.

### 2. Expecting Automatic Context

**Problem:** "Why doesn't the library know a lottery win matters more to poor people?"

**Solution:** The library models psychology, not your domain. You compute severity based on game context.

### 3. No Matching Event Type

**Problem:** Can't find the right EventType for your specific scenario.

**Solution:** Either map to the closest psychological effect, or create a custom event:

**Option A: Use closest match**
- Minor social slight -> `ExperienceConflictInterpersonal`
- Major physical trauma -> `ExperienceCombatMilitary` (for pain exposure)
- Social acceptance -> `ExperienceInclusionPeer`

**Option B: Create custom event**
```rust
let spec = EventSpec {
    impact: EventImpact {
        stress: 0.3,
        fatigue: 0.2,
        ..Default::default()
    },
    ..Default::default()
};
let event = EventBuilder::custom(spec).severity(0.5).build()?;
```

The type determines which psychological pathways are affected, not the literal event name.

---

## Questions?

File an issue at the [GitHub repository](https://github.com/behavioralpathways/eventsim-rs/issues).
