use neurite_protocol::{
    CapabilityId, ClientDescriptor, ClientTypeId, NodeTypeId, PlatformId, ProjectionTypeId,
    ProtocolBinding, ProtocolId, ProtocolVersion, ScenarioId, TransportId,
};

#[test]
fn future_node_and_platform_types_do_not_require_core_enum_changes() {
    let node_type = NodeTypeId::new("vendor.future.spatial-ui").unwrap();
    let platform = PlatformId::new("future-mobile-os").unwrap();

    assert_eq!(node_type.as_str(), "vendor.future.spatial-ui");
    assert_eq!(platform.as_str(), "future-mobile-os");
}

#[test]
fn protocol_bindings_are_explicitly_scenario_scoped() {
    let protocol = ProtocolId::new("hq.protocol.questn").unwrap();
    let version = ProtocolVersion::new(1).unwrap();
    let research = ProtocolBinding::new(
        protocol.clone(),
        version,
        ScenarioId::new("research.deep").unwrap(),
    );
    let scheduling = ProtocolBinding::new(
        protocol,
        version,
        ScenarioId::new("scheduling.daily").unwrap(),
    );

    assert_ne!(research.scenario, scheduling.scenario);
    assert_eq!(research.protocol, scheduling.protocol);
}

#[test]
fn android_client_declares_projection_transport_and_requests_without_receiving_grants() {
    let mut android = ClientDescriptor::new(
        ClientTypeId::new("hq.client.android").unwrap(),
        PlatformId::new("android").unwrap(),
    );
    android.add_transport(TransportId::new("hq.transport.websocket").unwrap());
    android.add_projection(ProjectionTypeId::new("hq.projection.mobile-canvas").unwrap());
    android.request_capability(CapabilityId::new("device.camera.capture").unwrap());
    android.request_capability(CapabilityId::new("device.location.read").unwrap());
    android.enable_offline_outbox();

    assert_eq!(android.platform.as_str(), "android");
    assert!(android
        .requested_capabilities
        .contains(&CapabilityId::new("device.camera.capture").unwrap()));
    assert!(android.offline_outbox);
}

#[test]
fn invalid_extension_identifiers_are_rejected_at_the_boundary() {
    assert!(NodeTypeId::new("").is_err());
    assert!(CapabilityId::new("device camera").is_err());
    assert!(ProtocolVersion::new(0).is_err());
}
