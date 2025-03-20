# CodeSecurityUpdateEnterpriseConfigurationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the code security configuration. Must be unique across the enterprise. | [optional]
**description** | Option<**String**> | A description of the code security configuration | [optional]
**advanced_security** | Option<**String**> | The enablement status of GitHub Advanced Security. Must be set to enabled if you want to enable any GHAS settings. | [optional]
**dependency_graph** | Option<**String**> | The enablement status of Dependency Graph | [optional]
**dependency_graph_autosubmit_action** | Option<**String**> | The enablement status of Automatic dependency submission | [optional]
**dependency_graph_autosubmit_action_options** | Option<[**models::CodeSecurityUpdateEnterpriseConfigurationRequestDependencyGraphAutosubmitActionOptions**](code_security_update_enterprise_configuration_request_dependency_graph_autosubmit_action_options.md)> |  | [optional]
**dependabot_alerts** | Option<**String**> | The enablement status of Dependabot alerts | [optional]
**dependabot_security_updates** | Option<**String**> | The enablement status of Dependabot security updates | [optional]
**code_scanning_default_setup** | Option<**String**> | The enablement status of code scanning default setup | [optional]
**code_scanning_default_setup_options** | Option<[**models::CodeScanningDefaultSetupOptions**](code-scanning-default-setup-options.md)> |  | [optional]
**code_scanning_delegated_alert_dismissal** | Option<**String**> | The enablement status of code scanning delegated alert dismissal | [optional][default to Disabled]
**secret_scanning** | Option<**String**> | The enablement status of secret scanning | [optional]
**secret_scanning_push_protection** | Option<**String**> | The enablement status of secret scanning push protection | [optional]
**secret_scanning_validity_checks** | Option<**String**> | The enablement status of secret scanning validity checks | [optional]
**secret_scanning_non_provider_patterns** | Option<**String**> | The enablement status of secret scanning non-provider patterns | [optional]
**secret_scanning_generic_secrets** | Option<**String**> | The enablement status of Copilot secret scanning | [optional][default to Disabled]
**secret_scanning_delegated_alert_dismissal** | Option<**String**> | The enablement status of secret scanning delegated alert dismissal | [optional][default to Disabled]
**private_vulnerability_reporting** | Option<**String**> | The enablement status of private vulnerability reporting | [optional]
**enforcement** | Option<**String**> | The enforcement status for a security configuration | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


