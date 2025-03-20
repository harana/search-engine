# CodeSecurityCreateConfigurationForEnterpriseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the code security configuration. Must be unique within the enterprise. | 
**description** | **String** | A description of the code security configuration | 
**advanced_security** | Option<**String**> | The enablement status of GitHub Advanced Security | [optional][default to Disabled]
**dependency_graph** | Option<**String**> | The enablement status of Dependency Graph | [optional][default to Enabled]
**dependency_graph_autosubmit_action** | Option<**String**> | The enablement status of Automatic dependency submission | [optional][default to Disabled]
**dependency_graph_autosubmit_action_options** | Option<[**models::CodeSecurityCreateConfigurationForEnterpriseRequestDependencyGraphAutosubmitActionOptions**](code_security_create_configuration_for_enterprise_request_dependency_graph_autosubmit_action_options.md)> |  | [optional]
**dependabot_alerts** | Option<**String**> | The enablement status of Dependabot alerts | [optional][default to Disabled]
**dependabot_security_updates** | Option<**String**> | The enablement status of Dependabot security updates | [optional][default to Disabled]
**code_scanning_default_setup** | Option<**String**> | The enablement status of code scanning default setup | [optional][default to Disabled]
**code_scanning_default_setup_options** | Option<[**models::CodeScanningDefaultSetupOptions**](code-scanning-default-setup-options.md)> |  | [optional]
**code_scanning_delegated_alert_dismissal** | Option<**String**> | The enablement status of code scanning delegated alert dismissal | [optional][default to Disabled]
**secret_scanning** | Option<**String**> | The enablement status of secret scanning | [optional][default to Disabled]
**secret_scanning_push_protection** | Option<**String**> | The enablement status of secret scanning push protection | [optional][default to Disabled]
**secret_scanning_validity_checks** | Option<**String**> | The enablement status of secret scanning validity checks | [optional][default to Disabled]
**secret_scanning_non_provider_patterns** | Option<**String**> | The enablement status of secret scanning non provider patterns | [optional][default to Disabled]
**secret_scanning_generic_secrets** | Option<**String**> | The enablement status of Copilot secret scanning | [optional][default to Disabled]
**secret_scanning_delegated_alert_dismissal** | Option<**String**> | The enablement status of secret scanning delegated alert dismissal | [optional][default to Disabled]
**private_vulnerability_reporting** | Option<**String**> | The enablement status of private vulnerability reporting | [optional][default to Disabled]
**enforcement** | Option<**String**> | The enforcement status for a security configuration | [optional][default to Enforced]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


