# Mobile and Android Extensibility Boundary

## Objective

HQ must remain useful away from the primary desktop without turning the Android application into a second control plane or creating a mobile-only implementation of every capability.

Android support is therefore a protocol and capability boundary before it is a screen layout.

## Supported roles

### Projection and control client

An Android client may render permitted projections, issue structured intents, receive observed-state updates, maintain a bounded offline outbox, and resume a spatial/navigation context across devices.

It does not own authoritative system state merely because it displays it.

### Device capability provider

An Android device may expose individually brokered capabilities, including:

- camera capture;
- microphone capture;
- coarse or precise location;
- notifications;
- share intents and deep links;
- motion and environmental sensors;
- media selection/capture;
- scoped local storage;
- biometric approval signals;
- foreground/background execution within Android platform limits.

Every capability is independently requested, admitted, revocable, observable, and attributable to a principal and partition. Visibility never implies device authority.

### Optional on-device runtime host

Later Android builds may host approved WASM components, local models, device automations, or agent workers. These remain execution-plane runtimes behind the same `RuntimeBinding` contract; they do not move backend logic into the presentation layer.

## Contract decisions already represented in Rust

- `PlatformId` is open, so Android and future mobile platforms do not require a kernel enum change.
- `ClientDescriptor` separates projections, transports, requested capabilities, and offline-outbox support.
- `RuntimeBinding` records placement, platform, transport, scenario protocol, partition, and required capabilities.
- `ProtocolBinding` makes QuestN/Q10, research, reasoning, orchestration, scheduling, and future interaction rules explicit scenario inputs.
- `StreamPolicy` prevents lossy queues for commands and authoritative events while allowing visual frames and status projections to degrade safely.
- `CapabilityEnvelope` grants only to a matching principal and partition.
- `AdmissionPolicy` derives requirements from a trusted action registry; clients do not declare the requirements used to authorize themselves.

## Transport independence

The mobile contract must not depend on a single network technology. WebSocket, QUIC, local IPC, USB/ADB development transport, or a future peer-to-peer transport may implement the same protocol without changing node semantics.

## Offline behavior

Offline Android work requires a durable, bounded outbox with stable intent IDs, visible synchronization state, idempotent replay, conflict reporting, and explicit reconciliation. Offline caching must remain permission-aware and exclude secrets that the active projection is not allowed to retain.

The current code declares offline-outbox support but does not yet implement persistence or replay.

## Android acceptance path

1. Compile the shared Rust protocol/core crates for an Android target.
2. Connect an Android projection client to the control plane using the same versioned protocol as desktop.
3. Create and move a permitted spatial entity from Android and observe the same event on desktop.
4. Deny a camera/location request without a matching capability grant.
5. Grant one scoped device capability without granting the others.
6. Queue intents offline, reconnect, replay idempotently, and show conflicts rather than silently overwriting.
7. Suspend background work within Android lifecycle constraints and resume without losing user context.
8. Demonstrate keyboard, touch, stylus, voice, and accessibility input through the shared action model.

## Explicit non-goals for the current slice

- no Android UI toolkit selection;
- no background-service implementation;
- no device permission prompts;
- no network transport implementation;
- no offline database;
- no claim of Android build support yet.

Those choices remain downstream of the stable protocol and authority boundary established here.
