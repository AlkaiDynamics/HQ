#![forbid(unsafe_code)]

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierError {
    Empty,
    InvalidBoundary,
    InvalidCharacter { index: usize, character: char },
}

fn validate_identifier(value: &str) -> Result<(), IdentifierError> {
    if value.is_empty() {
        return Err(IdentifierError::Empty);
    }

    if !value
        .chars()
        .next()
        .expect("non-empty")
        .is_ascii_alphanumeric()
        || !value
            .chars()
            .last()
            .expect("non-empty")
            .is_ascii_alphanumeric()
    {
        return Err(IdentifierError::InvalidBoundary);
    }

    if let Some((index, character)) = value.char_indices().find(|(_, c)| {
        !(c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '-' | '_'))
    }) {
        return Err(IdentifierError::InvalidCharacter { index, character });
    }

    Ok(())
}

macro_rules! open_identifier {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
                let value = value.into();
                validate_identifier(&value)?;
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

open_identifier!(CapabilityId);
open_identifier!(ActionId);
open_identifier!(ClientTypeId);
open_identifier!(NodeTypeId);
open_identifier!(PartitionKey);
open_identifier!(PlatformId);
open_identifier!(ProjectionTypeId);
open_identifier!(ProtocolId);
open_identifier!(RuntimeKindId);
open_identifier!(ScenarioId);
open_identifier!(TransportId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtocolVersion(u32);

impl ProtocolVersion {
    pub fn new(value: u32) -> Result<Self, ProtocolVersionError> {
        if value == 0 {
            return Err(ProtocolVersionError::Zero);
        }
        Ok(Self(value))
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersionError {
    Zero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolBinding {
    pub protocol: ProtocolId,
    pub version: ProtocolVersion,
    pub scenario: ScenarioId,
}

impl ProtocolBinding {
    pub const fn new(protocol: ProtocolId, version: ProtocolVersion, scenario: ScenarioId) -> Self {
        Self {
            protocol,
            version,
            scenario,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientDescriptor {
    pub client_type: ClientTypeId,
    pub platform: PlatformId,
    pub transports: BTreeSet<TransportId>,
    pub projections: BTreeSet<ProjectionTypeId>,
    pub requested_capabilities: BTreeSet<CapabilityId>,
    pub offline_outbox: bool,
}

impl ClientDescriptor {
    pub fn new(client_type: ClientTypeId, platform: PlatformId) -> Self {
        Self {
            client_type,
            platform,
            transports: BTreeSet::new(),
            projections: BTreeSet::new(),
            requested_capabilities: BTreeSet::new(),
            offline_outbox: false,
        }
    }

    pub fn add_transport(&mut self, transport: TransportId) {
        self.transports.insert(transport);
    }

    pub fn add_projection(&mut self, projection: ProjectionTypeId) {
        self.projections.insert(projection);
    }

    pub fn request_capability(&mut self, capability: CapabilityId) {
        self.requested_capabilities.insert(capability);
    }

    pub fn enable_offline_outbox(&mut self) {
        self.offline_outbox = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionPlacement {
    ControlPlane,
    ExecutionWorker,
    ClientDevice,
    RemoteService,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeBinding {
    pub node_type: NodeTypeId,
    pub runtime_kind: RuntimeKindId,
    pub placement: ExecutionPlacement,
    pub platform: PlatformId,
    pub transport: TransportId,
    pub protocol: ProtocolBinding,
    pub partition: PartitionKey,
    pub required_capabilities: BTreeSet<CapabilityId>,
}

impl RuntimeBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node_type: NodeTypeId,
        runtime_kind: RuntimeKindId,
        placement: ExecutionPlacement,
        platform: PlatformId,
        transport: TransportId,
        protocol: ProtocolBinding,
        partition: PartitionKey,
    ) -> Self {
        Self {
            node_type,
            runtime_kind,
            placement,
            platform,
            transport,
            protocol,
            partition,
            required_capabilities: BTreeSet::new(),
        }
    }

    pub fn require_capability(&mut self, capability: CapabilityId) {
        self.required_capabilities.insert(capability);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamClass {
    Command,
    AuthoritativeEvent,
    StatusProjection,
    VisualFrame,
    Media,
    Telemetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowPolicy {
    RejectNewest,
    DropOldest,
    KeepLatest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamPolicy {
    pub class: StreamClass,
    pub capacity: usize,
    pub overflow: OverflowPolicy,
}

impl StreamPolicy {
    pub fn new(
        class: StreamClass,
        capacity: usize,
        overflow: OverflowPolicy,
    ) -> Result<Self, StreamPolicyError> {
        if capacity == 0 {
            return Err(StreamPolicyError::ZeroCapacity);
        }

        let critical = matches!(
            class,
            StreamClass::Command | StreamClass::AuthoritativeEvent
        );
        let lossy = matches!(
            overflow,
            OverflowPolicy::DropOldest | OverflowPolicy::KeepLatest
        );
        if critical && lossy {
            return Err(StreamPolicyError::LossyCriticalStream);
        }

        Ok(Self {
            class,
            capacity,
            overflow,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamPolicyError {
    ZeroCapacity,
    LossyCriticalStream,
}
