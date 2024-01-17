> Copied from https://github.com/hashicorp/nomad-openapi/tree/main/clients/rust/reqwest/v1

# Rust API client for nomad_client

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.1.4
- Package version: 1.1.4
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `nomad_client` and add the following to `Cargo.toml` under `[dependencies]`:

```
nomad_client = { path = "./nomad_client" }
```

## Documentation for API Endpoints

All URIs are relative to *http://127.0.0.1:4646/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ACLApi* | [**delete_acl_policy**](docs/ACLApi.md#delete_acl_policy) | **DELETE** /acl/policy/{policyName} | 
*ACLApi* | [**delete_acl_token**](docs/ACLApi.md#delete_acl_token) | **DELETE** /acl/token/{tokenAccessor} | 
*ACLApi* | [**get_acl_policies**](docs/ACLApi.md#get_acl_policies) | **GET** /acl/policies | 
*ACLApi* | [**get_acl_policy**](docs/ACLApi.md#get_acl_policy) | **GET** /acl/policy/{policyName} | 
*ACLApi* | [**get_acl_token**](docs/ACLApi.md#get_acl_token) | **GET** /acl/token/{tokenAccessor} | 
*ACLApi* | [**get_acl_token_self**](docs/ACLApi.md#get_acl_token_self) | **GET** /acl/token | 
*ACLApi* | [**get_acl_tokens**](docs/ACLApi.md#get_acl_tokens) | **GET** /acl/tokens | 
*ACLApi* | [**post_acl_bootstrap**](docs/ACLApi.md#post_acl_bootstrap) | **POST** /acl/bootstrap | 
*ACLApi* | [**post_acl_policy**](docs/ACLApi.md#post_acl_policy) | **POST** /acl/policy/{policyName} | 
*ACLApi* | [**post_acl_token**](docs/ACLApi.md#post_acl_token) | **POST** /acl/token/{tokenAccessor} | 
*ACLApi* | [**post_acl_token_onetime**](docs/ACLApi.md#post_acl_token_onetime) | **POST** /acl/token/onetime | 
*ACLApi* | [**post_acl_token_onetime_exchange**](docs/ACLApi.md#post_acl_token_onetime_exchange) | **POST** /acl/token/onetime/exchange | 
*AllocationsApi* | [**get_allocation**](docs/AllocationsApi.md#get_allocation) | **GET** /allocation/{allocID} | 
*AllocationsApi* | [**get_allocation_services**](docs/AllocationsApi.md#get_allocation_services) | **GET** /allocation/{allocID}/services | 
*AllocationsApi* | [**get_allocations**](docs/AllocationsApi.md#get_allocations) | **GET** /allocations | 
*AllocationsApi* | [**post_allocation_stop**](docs/AllocationsApi.md#post_allocation_stop) | **POST** /allocation/{allocID}/stop | 
*DeploymentsApi* | [**get_deployment**](docs/DeploymentsApi.md#get_deployment) | **GET** /deployment/{deploymentID} | 
*DeploymentsApi* | [**get_deployment_allocations**](docs/DeploymentsApi.md#get_deployment_allocations) | **GET** /deployment/allocations/{deploymentID} | 
*DeploymentsApi* | [**get_deployments**](docs/DeploymentsApi.md#get_deployments) | **GET** /deployments | 
*DeploymentsApi* | [**post_deployment_allocation_health**](docs/DeploymentsApi.md#post_deployment_allocation_health) | **POST** /deployment/allocation-health/{deploymentID} | 
*DeploymentsApi* | [**post_deployment_fail**](docs/DeploymentsApi.md#post_deployment_fail) | **POST** /deployment/fail/{deploymentID} | 
*DeploymentsApi* | [**post_deployment_pause**](docs/DeploymentsApi.md#post_deployment_pause) | **POST** /deployment/pause/{deploymentID} | 
*DeploymentsApi* | [**post_deployment_promote**](docs/DeploymentsApi.md#post_deployment_promote) | **POST** /deployment/promote/{deploymentID} | 
*DeploymentsApi* | [**post_deployment_unblock**](docs/DeploymentsApi.md#post_deployment_unblock) | **POST** /deployment/unblock/{deploymentID} | 
*EnterpriseApi* | [**create_quota_spec**](docs/EnterpriseApi.md#create_quota_spec) | **POST** /quota | 
*EnterpriseApi* | [**delete_quota_spec**](docs/EnterpriseApi.md#delete_quota_spec) | **DELETE** /quota/{specName} | 
*EnterpriseApi* | [**get_quota_spec**](docs/EnterpriseApi.md#get_quota_spec) | **GET** /quota/{specName} | 
*EnterpriseApi* | [**get_quotas**](docs/EnterpriseApi.md#get_quotas) | **GET** /quotas | 
*EnterpriseApi* | [**post_quota_spec**](docs/EnterpriseApi.md#post_quota_spec) | **POST** /quota/{specName} | 
*EvaluationsApi* | [**get_evaluation**](docs/EvaluationsApi.md#get_evaluation) | **GET** /evaluation/{evalID} | 
*EvaluationsApi* | [**get_evaluation_allocations**](docs/EvaluationsApi.md#get_evaluation_allocations) | **GET** /evaluation/{evalID}/allocations | 
*EvaluationsApi* | [**get_evaluations**](docs/EvaluationsApi.md#get_evaluations) | **GET** /evaluations | 
*JobsApi* | [**delete_job**](docs/JobsApi.md#delete_job) | **DELETE** /job/{jobName} | 
*JobsApi* | [**get_job**](docs/JobsApi.md#get_job) | **GET** /job/{jobName} | 
*JobsApi* | [**get_job_allocations**](docs/JobsApi.md#get_job_allocations) | **GET** /job/{jobName}/allocations | 
*JobsApi* | [**get_job_deployment**](docs/JobsApi.md#get_job_deployment) | **GET** /job/{jobName}/deployment | 
*JobsApi* | [**get_job_deployments**](docs/JobsApi.md#get_job_deployments) | **GET** /job/{jobName}/deployments | 
*JobsApi* | [**get_job_evaluations**](docs/JobsApi.md#get_job_evaluations) | **GET** /job/{jobName}/evaluations | 
*JobsApi* | [**get_job_scale_status**](docs/JobsApi.md#get_job_scale_status) | **GET** /job/{jobName}/scale | 
*JobsApi* | [**get_job_summary**](docs/JobsApi.md#get_job_summary) | **GET** /job/{jobName}/summary | 
*JobsApi* | [**get_job_versions**](docs/JobsApi.md#get_job_versions) | **GET** /job/{jobName}/versions | 
*JobsApi* | [**get_jobs**](docs/JobsApi.md#get_jobs) | **GET** /jobs | 
*JobsApi* | [**post_job**](docs/JobsApi.md#post_job) | **POST** /job/{jobName} | 
*JobsApi* | [**post_job_dispatch**](docs/JobsApi.md#post_job_dispatch) | **POST** /job/{jobName}/dispatch | 
*JobsApi* | [**post_job_evaluate**](docs/JobsApi.md#post_job_evaluate) | **POST** /job/{jobName}/evaluate | 
*JobsApi* | [**post_job_parse**](docs/JobsApi.md#post_job_parse) | **POST** /jobs/parse | 
*JobsApi* | [**post_job_periodic_force**](docs/JobsApi.md#post_job_periodic_force) | **POST** /job/{jobName}/periodic/force | 
*JobsApi* | [**post_job_plan**](docs/JobsApi.md#post_job_plan) | **POST** /job/{jobName}/plan | 
*JobsApi* | [**post_job_revert**](docs/JobsApi.md#post_job_revert) | **POST** /job/{jobName}/revert | 
*JobsApi* | [**post_job_scaling_request**](docs/JobsApi.md#post_job_scaling_request) | **POST** /job/{jobName}/scale | 
*JobsApi* | [**post_job_stability**](docs/JobsApi.md#post_job_stability) | **POST** /job/{jobName}/stable | 
*JobsApi* | [**post_job_validate_request**](docs/JobsApi.md#post_job_validate_request) | **POST** /validate/job | 
*JobsApi* | [**register_job**](docs/JobsApi.md#register_job) | **POST** /jobs | 
*MetricsApi* | [**get_metrics_summary**](docs/MetricsApi.md#get_metrics_summary) | **GET** /metrics | 
*NamespacesApi* | [**create_namespace**](docs/NamespacesApi.md#create_namespace) | **POST** /namespace | 
*NamespacesApi* | [**delete_namespace**](docs/NamespacesApi.md#delete_namespace) | **DELETE** /namespace/{namespaceName} | 
*NamespacesApi* | [**get_namespace**](docs/NamespacesApi.md#get_namespace) | **GET** /namespace/{namespaceName} | 
*NamespacesApi* | [**get_namespaces**](docs/NamespacesApi.md#get_namespaces) | **GET** /namespaces | 
*NamespacesApi* | [**post_namespace**](docs/NamespacesApi.md#post_namespace) | **POST** /namespace/{namespaceName} | 
*NodesApi* | [**get_node**](docs/NodesApi.md#get_node) | **GET** /node/{nodeId} | 
*NodesApi* | [**get_node_allocations**](docs/NodesApi.md#get_node_allocations) | **GET** /node/{nodeId}/allocations | 
*NodesApi* | [**get_nodes**](docs/NodesApi.md#get_nodes) | **GET** /nodes | 
*NodesApi* | [**update_node_drain**](docs/NodesApi.md#update_node_drain) | **POST** /node/{nodeId}/drain | 
*NodesApi* | [**update_node_eligibility**](docs/NodesApi.md#update_node_eligibility) | **POST** /node/{nodeId}/eligibility | 
*NodesApi* | [**update_node_purge**](docs/NodesApi.md#update_node_purge) | **POST** /node/{nodeId}/purge | 
*OperatorApi* | [**delete_operator_raft_peer**](docs/OperatorApi.md#delete_operator_raft_peer) | **DELETE** /operator/raft/peer | 
*OperatorApi* | [**get_operator_autopilot_configuration**](docs/OperatorApi.md#get_operator_autopilot_configuration) | **GET** /operator/autopilot/configuration | 
*OperatorApi* | [**get_operator_autopilot_health**](docs/OperatorApi.md#get_operator_autopilot_health) | **GET** /operator/autopilot/health | 
*OperatorApi* | [**get_operator_raft_configuration**](docs/OperatorApi.md#get_operator_raft_configuration) | **GET** /operator/raft/configuration | 
*OperatorApi* | [**get_operator_scheduler_configuration**](docs/OperatorApi.md#get_operator_scheduler_configuration) | **GET** /operator/scheduler/configuration | 
*OperatorApi* | [**post_operator_scheduler_configuration**](docs/OperatorApi.md#post_operator_scheduler_configuration) | **POST** /operator/scheduler/configuration | 
*OperatorApi* | [**put_operator_autopilot_configuration**](docs/OperatorApi.md#put_operator_autopilot_configuration) | **PUT** /operator/autopilot/configuration | 
*PluginsApi* | [**get_plugin_csi**](docs/PluginsApi.md#get_plugin_csi) | **GET** /plugin/csi/{pluginID} | 
*PluginsApi* | [**get_plugins**](docs/PluginsApi.md#get_plugins) | **GET** /plugins | 
*RegionsApi* | [**get_regions**](docs/RegionsApi.md#get_regions) | **GET** /regions | 
*ScalingApi* | [**get_scaling_policies**](docs/ScalingApi.md#get_scaling_policies) | **GET** /scaling/policies | 
*ScalingApi* | [**get_scaling_policy**](docs/ScalingApi.md#get_scaling_policy) | **GET** /scaling/policy/{policyID} | 
*SearchApi* | [**get_fuzzy_search**](docs/SearchApi.md#get_fuzzy_search) | **POST** /search/fuzzy | 
*SearchApi* | [**get_search**](docs/SearchApi.md#get_search) | **POST** /search | 
*StatusApi* | [**get_status_leader**](docs/StatusApi.md#get_status_leader) | **GET** /status/leader | 
*StatusApi* | [**get_status_peers**](docs/StatusApi.md#get_status_peers) | **GET** /status/peers | 
*SystemApi* | [**put_system_gc**](docs/SystemApi.md#put_system_gc) | **PUT** /system/gc | 
*SystemApi* | [**put_system_reconcile_summaries**](docs/SystemApi.md#put_system_reconcile_summaries) | **PUT** /system/reconcile/summaries | 
*VariablesApi* | [**delete_variable**](docs/VariablesApi.md#delete_variable) | **DELETE** /var/{path} | 
*VariablesApi* | [**get_variable_query**](docs/VariablesApi.md#get_variable_query) | **GET** /var/{path} | 
*VariablesApi* | [**get_variables_list_request**](docs/VariablesApi.md#get_variables_list_request) | **GET** /vars | 
*VariablesApi* | [**post_variable**](docs/VariablesApi.md#post_variable) | **POST** /var/{path} | 
*VariablesApi* | [**put_variable**](docs/VariablesApi.md#put_variable) | **PUT** /var/{path} | 
*VolumesApi* | [**create_volume**](docs/VolumesApi.md#create_volume) | **POST** /volume/csi/{volumeId}/{action} | 
*VolumesApi* | [**delete_snapshot**](docs/VolumesApi.md#delete_snapshot) | **DELETE** /volumes/snapshot | 
*VolumesApi* | [**delete_volume_registration**](docs/VolumesApi.md#delete_volume_registration) | **DELETE** /volume/csi/{volumeId} | 
*VolumesApi* | [**detach_or_delete_volume**](docs/VolumesApi.md#detach_or_delete_volume) | **DELETE** /volume/csi/{volumeId}/{action} | 
*VolumesApi* | [**get_external_volumes**](docs/VolumesApi.md#get_external_volumes) | **GET** /volumes/external | 
*VolumesApi* | [**get_snapshots**](docs/VolumesApi.md#get_snapshots) | **GET** /volumes/snapshot | 
*VolumesApi* | [**get_volume**](docs/VolumesApi.md#get_volume) | **GET** /volume/csi/{volumeId} | 
*VolumesApi* | [**get_volumes**](docs/VolumesApi.md#get_volumes) | **GET** /volumes | 
*VolumesApi* | [**post_snapshot**](docs/VolumesApi.md#post_snapshot) | **POST** /volumes/snapshot | 
*VolumesApi* | [**post_volume**](docs/VolumesApi.md#post_volume) | **POST** /volumes | 
*VolumesApi* | [**post_volume_registration**](docs/VolumesApi.md#post_volume_registration) | **POST** /volume/csi/{volumeId} | 


## Documentation For Models

 - [AclPolicy](docs/AclPolicy.md)
 - [AclPolicyListStub](docs/AclPolicyListStub.md)
 - [AclToken](docs/AclToken.md)
 - [AclTokenListStub](docs/AclTokenListStub.md)
 - [AclTokenRoleLink](docs/AclTokenRoleLink.md)
 - [Affinity](docs/Affinity.md)
 - [AllocDeploymentStatus](docs/AllocDeploymentStatus.md)
 - [AllocStopResponse](docs/AllocStopResponse.md)
 - [AllocatedCpuResources](docs/AllocatedCpuResources.md)
 - [AllocatedDeviceResource](docs/AllocatedDeviceResource.md)
 - [AllocatedMemoryResources](docs/AllocatedMemoryResources.md)
 - [AllocatedResources](docs/AllocatedResources.md)
 - [AllocatedSharedResources](docs/AllocatedSharedResources.md)
 - [AllocatedTaskResources](docs/AllocatedTaskResources.md)
 - [Allocation](docs/Allocation.md)
 - [AllocationListStub](docs/AllocationListStub.md)
 - [AllocationMetric](docs/AllocationMetric.md)
 - [Attribute](docs/Attribute.md)
 - [AutopilotConfiguration](docs/AutopilotConfiguration.md)
 - [ChangeScript](docs/ChangeScript.md)
 - [CheckRestart](docs/CheckRestart.md)
 - [Constraint](docs/Constraint.md)
 - [Consul](docs/Consul.md)
 - [ConsulConnect](docs/ConsulConnect.md)
 - [ConsulExposeConfig](docs/ConsulExposeConfig.md)
 - [ConsulExposePath](docs/ConsulExposePath.md)
 - [ConsulGateway](docs/ConsulGateway.md)
 - [ConsulGatewayBindAddress](docs/ConsulGatewayBindAddress.md)
 - [ConsulGatewayProxy](docs/ConsulGatewayProxy.md)
 - [ConsulGatewayTlsConfig](docs/ConsulGatewayTlsConfig.md)
 - [ConsulIngressConfigEntry](docs/ConsulIngressConfigEntry.md)
 - [ConsulIngressListener](docs/ConsulIngressListener.md)
 - [ConsulIngressService](docs/ConsulIngressService.md)
 - [ConsulLinkedService](docs/ConsulLinkedService.md)
 - [ConsulMeshGateway](docs/ConsulMeshGateway.md)
 - [ConsulProxy](docs/ConsulProxy.md)
 - [ConsulSidecarService](docs/ConsulSidecarService.md)
 - [ConsulTerminatingConfigEntry](docs/ConsulTerminatingConfigEntry.md)
 - [ConsulUpstream](docs/ConsulUpstream.md)
 - [CsiControllerInfo](docs/CsiControllerInfo.md)
 - [CsiInfo](docs/CsiInfo.md)
 - [CsiMountOptions](docs/CsiMountOptions.md)
 - [CsiNodeInfo](docs/CsiNodeInfo.md)
 - [CsiPlugin](docs/CsiPlugin.md)
 - [CsiPluginListStub](docs/CsiPluginListStub.md)
 - [CsiSnapshot](docs/CsiSnapshot.md)
 - [CsiSnapshotCreateRequest](docs/CsiSnapshotCreateRequest.md)
 - [CsiSnapshotCreateResponse](docs/CsiSnapshotCreateResponse.md)
 - [CsiSnapshotListResponse](docs/CsiSnapshotListResponse.md)
 - [CsiTopology](docs/CsiTopology.md)
 - [CsiTopologyRequest](docs/CsiTopologyRequest.md)
 - [CsiVolume](docs/CsiVolume.md)
 - [CsiVolumeCapability](docs/CsiVolumeCapability.md)
 - [CsiVolumeCreateRequest](docs/CsiVolumeCreateRequest.md)
 - [CsiVolumeExternalStub](docs/CsiVolumeExternalStub.md)
 - [CsiVolumeListExternalResponse](docs/CsiVolumeListExternalResponse.md)
 - [CsiVolumeListStub](docs/CsiVolumeListStub.md)
 - [CsiVolumeRegisterRequest](docs/CsiVolumeRegisterRequest.md)
 - [Deployment](docs/Deployment.md)
 - [DeploymentAllocHealthRequest](docs/DeploymentAllocHealthRequest.md)
 - [DeploymentPauseRequest](docs/DeploymentPauseRequest.md)
 - [DeploymentPromoteRequest](docs/DeploymentPromoteRequest.md)
 - [DeploymentState](docs/DeploymentState.md)
 - [DeploymentUnblockRequest](docs/DeploymentUnblockRequest.md)
 - [DeploymentUpdateResponse](docs/DeploymentUpdateResponse.md)
 - [DesiredTransition](docs/DesiredTransition.md)
 - [DesiredUpdates](docs/DesiredUpdates.md)
 - [DispatchPayloadConfig](docs/DispatchPayloadConfig.md)
 - [DnsConfig](docs/DnsConfig.md)
 - [DrainMetadata](docs/DrainMetadata.md)
 - [DrainSpec](docs/DrainSpec.md)
 - [DrainStrategy](docs/DrainStrategy.md)
 - [DriverInfo](docs/DriverInfo.md)
 - [EphemeralDisk](docs/EphemeralDisk.md)
 - [EvalOptions](docs/EvalOptions.md)
 - [Evaluation](docs/Evaluation.md)
 - [EvaluationStub](docs/EvaluationStub.md)
 - [FieldDiff](docs/FieldDiff.md)
 - [FuzzyMatch](docs/FuzzyMatch.md)
 - [FuzzySearchRequest](docs/FuzzySearchRequest.md)
 - [FuzzySearchResponse](docs/FuzzySearchResponse.md)
 - [GaugeValue](docs/GaugeValue.md)
 - [HostNetworkInfo](docs/HostNetworkInfo.md)
 - [HostVolumeInfo](docs/HostVolumeInfo.md)
 - [Job](docs/Job.md)
 - [JobAcl](docs/JobAcl.md)
 - [JobChildrenSummary](docs/JobChildrenSummary.md)
 - [JobDeregisterResponse](docs/JobDeregisterResponse.md)
 - [JobDiff](docs/JobDiff.md)
 - [JobDispatchRequest](docs/JobDispatchRequest.md)
 - [JobDispatchResponse](docs/JobDispatchResponse.md)
 - [JobEvaluateRequest](docs/JobEvaluateRequest.md)
 - [JobListStub](docs/JobListStub.md)
 - [JobPlanRequest](docs/JobPlanRequest.md)
 - [JobPlanResponse](docs/JobPlanResponse.md)
 - [JobRegisterRequest](docs/JobRegisterRequest.md)
 - [JobRegisterResponse](docs/JobRegisterResponse.md)
 - [JobRevertRequest](docs/JobRevertRequest.md)
 - [JobScaleStatusResponse](docs/JobScaleStatusResponse.md)
 - [JobStabilityRequest](docs/JobStabilityRequest.md)
 - [JobStabilityResponse](docs/JobStabilityResponse.md)
 - [JobSummary](docs/JobSummary.md)
 - [JobValidateRequest](docs/JobValidateRequest.md)
 - [JobValidateResponse](docs/JobValidateResponse.md)
 - [JobVersionsResponse](docs/JobVersionsResponse.md)
 - [JobsParseRequest](docs/JobsParseRequest.md)
 - [LogConfig](docs/LogConfig.md)
 - [MetricsSummary](docs/MetricsSummary.md)
 - [MigrateStrategy](docs/MigrateStrategy.md)
 - [Multiregion](docs/Multiregion.md)
 - [MultiregionRegion](docs/MultiregionRegion.md)
 - [MultiregionStrategy](docs/MultiregionStrategy.md)
 - [Namespace](docs/Namespace.md)
 - [NamespaceCapabilities](docs/NamespaceCapabilities.md)
 - [NetworkResource](docs/NetworkResource.md)
 - [Node](docs/Node.md)
 - [NodeCpuResources](docs/NodeCpuResources.md)
 - [NodeDevice](docs/NodeDevice.md)
 - [NodeDeviceLocality](docs/NodeDeviceLocality.md)
 - [NodeDeviceResource](docs/NodeDeviceResource.md)
 - [NodeDiskResources](docs/NodeDiskResources.md)
 - [NodeDrainUpdateResponse](docs/NodeDrainUpdateResponse.md)
 - [NodeEligibilityUpdateResponse](docs/NodeEligibilityUpdateResponse.md)
 - [NodeEvent](docs/NodeEvent.md)
 - [NodeListStub](docs/NodeListStub.md)
 - [NodeMemoryResources](docs/NodeMemoryResources.md)
 - [NodePurgeResponse](docs/NodePurgeResponse.md)
 - [NodeReservedCpuResources](docs/NodeReservedCpuResources.md)
 - [NodeReservedDiskResources](docs/NodeReservedDiskResources.md)
 - [NodeReservedMemoryResources](docs/NodeReservedMemoryResources.md)
 - [NodeReservedNetworkResources](docs/NodeReservedNetworkResources.md)
 - [NodeReservedResources](docs/NodeReservedResources.md)
 - [NodeResources](docs/NodeResources.md)
 - [NodeScoreMeta](docs/NodeScoreMeta.md)
 - [NodeUpdateDrainRequest](docs/NodeUpdateDrainRequest.md)
 - [NodeUpdateEligibilityRequest](docs/NodeUpdateEligibilityRequest.md)
 - [ObjectDiff](docs/ObjectDiff.md)
 - [OneTimeToken](docs/OneTimeToken.md)
 - [OneTimeTokenExchangeRequest](docs/OneTimeTokenExchangeRequest.md)
 - [OperatorHealthReply](docs/OperatorHealthReply.md)
 - [ParameterizedJobConfig](docs/ParameterizedJobConfig.md)
 - [PeriodicConfig](docs/PeriodicConfig.md)
 - [PeriodicForceResponse](docs/PeriodicForceResponse.md)
 - [PlanAnnotations](docs/PlanAnnotations.md)
 - [PointValue](docs/PointValue.md)
 - [Port](docs/Port.md)
 - [PortMapping](docs/PortMapping.md)
 - [PreemptionConfig](docs/PreemptionConfig.md)
 - [QuotaLimit](docs/QuotaLimit.md)
 - [QuotaSpec](docs/QuotaSpec.md)
 - [RaftConfiguration](docs/RaftConfiguration.md)
 - [RaftServer](docs/RaftServer.md)
 - [RequestedDevice](docs/RequestedDevice.md)
 - [RescheduleEvent](docs/RescheduleEvent.md)
 - [ReschedulePolicy](docs/ReschedulePolicy.md)
 - [RescheduleTracker](docs/RescheduleTracker.md)
 - [Resources](docs/Resources.md)
 - [RestartPolicy](docs/RestartPolicy.md)
 - [SampledValue](docs/SampledValue.md)
 - [ScalingEvent](docs/ScalingEvent.md)
 - [ScalingPolicy](docs/ScalingPolicy.md)
 - [ScalingPolicyListStub](docs/ScalingPolicyListStub.md)
 - [ScalingRequest](docs/ScalingRequest.md)
 - [SchedulerConfiguration](docs/SchedulerConfiguration.md)
 - [SchedulerConfigurationResponse](docs/SchedulerConfigurationResponse.md)
 - [SchedulerSetConfigurationResponse](docs/SchedulerSetConfigurationResponse.md)
 - [SearchRequest](docs/SearchRequest.md)
 - [SearchResponse](docs/SearchResponse.md)
 - [ServerHealth](docs/ServerHealth.md)
 - [Service](docs/Service.md)
 - [ServiceCheck](docs/ServiceCheck.md)
 - [ServiceRegistration](docs/ServiceRegistration.md)
 - [SidecarTask](docs/SidecarTask.md)
 - [Spread](docs/Spread.md)
 - [SpreadTarget](docs/SpreadTarget.md)
 - [Task](docs/Task.md)
 - [TaskArtifact](docs/TaskArtifact.md)
 - [TaskCsiPluginConfig](docs/TaskCsiPluginConfig.md)
 - [TaskDiff](docs/TaskDiff.md)
 - [TaskEvent](docs/TaskEvent.md)
 - [TaskGroup](docs/TaskGroup.md)
 - [TaskGroupDiff](docs/TaskGroupDiff.md)
 - [TaskGroupScaleStatus](docs/TaskGroupScaleStatus.md)
 - [TaskGroupSummary](docs/TaskGroupSummary.md)
 - [TaskHandle](docs/TaskHandle.md)
 - [TaskLifecycle](docs/TaskLifecycle.md)
 - [TaskState](docs/TaskState.md)
 - [Template](docs/Template.md)
 - [UpdateStrategy](docs/UpdateStrategy.md)
 - [Variable](docs/Variable.md)
 - [VariableMetadata](docs/VariableMetadata.md)
 - [Vault](docs/Vault.md)
 - [VolumeMount](docs/VolumeMount.md)
 - [VolumeRequest](docs/VolumeRequest.md)
 - [WaitConfig](docs/WaitConfig.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@hashicorp.com
