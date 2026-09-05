use neurite_protocol::{
    CapabilityId, ExecutionPlacement, NodeTypeId, OverflowPolicy, PartitionKey, PlatformId,
    ProtocolBinding, ProtocolId, ProtocolVersion, RuntimeBinding, RuntimeKindId, ScenarioId,
    StreamClass, StreamPolicy, StreamPolicyError, TransportId,
};

fn mobile_protocol() -> ProtocolBinding {
    ProtocolBinding::new(
        ProtocolId::new("hq.protocol.intent-v1").unwrap(),
        ProtocolVersion::new(1).unwrap(),
        ScenarioId::new("mobile.on-the-go").unwrap(),
    )
}

#[test]
fn android_device_runtime_is_partitioned_and_capability_scoped() {
    let mut binding = RuntimeBinding::new(
        NodeTypeId::new("hq.node.android-device").unwrap(),
        RuntimeKindId::new("hq.runtime.android").unwrap(),
        ExecutionPlacement::ClientDevice,
        PlatformId::new("android").unwrap(),
        TransportId::new("hq.transport.websocket").unwrap(),
        mobile_protocol(),
        PartitionKey::new("mobile.momo.primary").unwrap(),
    );
    binding.require_capability(CapabilityId::new("device.microphone.capture").unwrap());
    binding.require_capability(CapabilityId::new("device.notifications.publish").unwrap());

    assert_eq!(binding.placement, ExecutionPlacement::ClientDevice);
    assert_eq!(binding.platform.as_str(), "android");
    assert_eq!(binding.required_capabilities.len(), 2);
}

#[test]
fn visual_frames_can_coalesce_under_backpressure() {
    let policy =
        StreamPolicy::new(StreamClass::VisualFrame, 3, OverflowPolicy::KeepLatest).unwrap();

    assert_eq!(policy.capacity, 3);
    assert_eq!(policy.overflow, OverflowPolicy::KeepLatest);
}

#[test]
fn commands_and_authoritative_events_cannot_use_lossy_backpressure() {
    assert_eq!(
        StreamPolicy::new(StreamClass::Command, 64, OverflowPolicy::KeepLatest),
        Err(StreamPolicyError::LossyCriticalStream)
    );
    assert_eq!(
        StreamPolicy::new(
            StreamClass::AuthoritativeEvent,
            64,
            OverflowPolicy::DropOldest
        ),
        Err(StreamPolicyError::LossyCriticalStream)
    );
    assert_eq!(
        StreamPolicy::new(
            StreamClass::StatusProjection,
            0,
            OverflowPolicy::RejectNewest
        ),
        Err(StreamPolicyError::ZeroCapacity)
    );
}
