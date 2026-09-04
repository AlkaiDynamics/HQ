# Neurite Forensic Knowledge Graph

This directory is the structured evidence layer for the Neurite reverse-engineering program.

## Files

```text
schema.json
  graph vocabulary, status/evidence/treatment rules

nodes.jsonl
  specimens, artifacts, behaviors, states, effects, capabilities,
  contracts, fixtures, target Rust subsystems

evidence.jsonl
  provenance-bearing evidence observations

edges.jsonl
  typed relationships with provenance/status

views/
  generated or reproducible projections of the KG
```

## Authority rule

```text
source/runtime evidence
        ↓
       KG
        ↓
queries / diagrams / reports
```

Never reverse that direction. A chart is a projection of evidence, not an authoritative source.

## Evidence ladder

```text
SOURCE-DERIVED
    ↓
RUNTIME-OBSERVED
    ↓
GOLDEN-VERIFIED
    ↓
RUST-IMPLEMENTED
    ↓
DIFFERENTIAL-PASS
    ↓
PROMOTED
```

Inference/hypothesis edges are allowed, but they are explicitly marked and cannot satisfy a runtime/golden gate.

## Primary queries

The graph is designed to answer questions such as:

```text
1. Which valuable behaviors have SOURCE-CLOSED evidence but no runtime fixture?

2. Which behaviors write durable state and also produce external effects?

3. Which effects are reachable from AI-generated output?

4. Which legacy services currently possess filesystem/network/code-execution authority?

5. Which behaviors still conflate VisualEdge and SemanticReference?

6. Which contracts lack GOLDEN-VERIFIED fixtures?

7. Which contracts have no target Rust subsystem?

8. Which Rust subsystem is absorbing the most legacy responsibilities?

9. Which legacy behaviors still rely on BrowserRuntime, Node, Python or network?

10. Which persisted values currently contain authority/credentials rather than ordinary data?

11. Which external effects are destructive or non-reversible and lack strong auditability?

12. Which legacy behaviors have been implemented in Rust but have not passed differential verification?
```

## Gate queries

### Gate A

A Gate A query should require:

```text
for every selected valuable Behavior:
  runtime_status == RUNTIME-VERIFIED
  golden_status == GOLDEN-VERIFIED

and environment fixtures exist for:
  fresh profile
  existing workspace
  online/offline
  missing services
  local AI
  cloud AI
```

### Gate B

For each valuable behavior, graph reachability must exist:

```text
Interaction
→ Behavior
→ implementation/source evidence
→ State reads/writes
→ ExternalEffect/Service/API where applicable
```

and the relevant state/effect edges need source or runtime evidence.

### Gate C

For every behavior selected for migration:

```text
Behavior
→ MAPS_TO_CONTRACT
→ Contract
```

and the behavioral contract must be grounded by the required runtime/golden evidence rather than only static inference.

## Import target

JSONL is deliberately storage-neutral. It can be loaded into:

```text
SQLite/PostgreSQL tables
DuckDB
NetworkX
petgraph
Neo4j
Memgraph
Kuzu
Zef
or a native Rust graph/index
```

No graph-database product is required by the forensic contract.

## Future automated extraction

The static compiler can progressively populate:

```text
SourceArtifact
→ DECLARES Symbol
→ CALLS Symbol
→ READS/WRITES State
→ CALLS API/Service
```

Runtime instrumentation then adds:

```text
Interaction
→ observed Behavior
→ observed State delta
→ observed ExternalEffect
→ visual/timing capture
```

Differential tests add:

```text
Behavior/Contract
→ VERIFIED_BY Fixture
```

The same graph can therefore survive the transition from reverse engineering to implementation and release certification.
