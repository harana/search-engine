# SecretScanningAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number** | Option<**i32**> | The security alert number. | [optional][readonly]
**created_at** | Option<**String**> | The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional][readonly]
**updated_at** | Option<**String**> | The time that the alert was last updated in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional][readonly]
**url** | Option<**String**> | The REST API URL of the alert resource. | [optional][readonly]
**html_url** | Option<**String**> | The GitHub URL of the alert resource. | [optional][readonly]
**locations_url** | Option<**String**> | The REST API URL of the code locations for this alert. | [optional]
**state** | Option<[**models::SecretScanningAlertState**](secret-scanning-alert-state.md)> |  | [optional]
**resolution** | Option<[**models::SecretScanningAlertResolution**](secret-scanning-alert-resolution.md)> |  | [optional]
**resolved_at** | Option<**String**> | The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional]
**resolved_by** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**resolution_comment** | Option<**String**> | An optional comment to resolve an alert. | [optional]
**secret_type** | Option<**String**> | The type of secret that secret scanning detected. | [optional]
**secret_type_display_name** | Option<**String**> | User-friendly name for the detected secret, matching the `secret_type`. For a list of built-in patterns, see \"[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets).\" | [optional]
**secret** | Option<**String**> | The secret that was detected. | [optional]
**push_protection_bypassed** | Option<**bool**> | Whether push protection was bypassed for the detected secret. | [optional]
**push_protection_bypassed_by** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**push_protection_bypassed_at** | Option<**String**> | The time that push protection was bypassed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional]
**push_protection_bypass_request_reviewer** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**push_protection_bypass_request_reviewer_comment** | Option<**String**> | An optional comment when reviewing a push protection bypass. | [optional]
**push_protection_bypass_request_comment** | Option<**String**> | An optional comment when requesting a push protection bypass. | [optional]
**push_protection_bypass_request_html_url** | Option<**String**> | The URL to a push protection bypass request. | [optional]
**validity** | Option<**String**> | The token status as of the latest validity check. | [optional]
**publicly_leaked** | Option<**bool**> | Whether the detected secret was publicly leaked. | [optional]
**multi_repo** | Option<**bool**> | Whether the detected secret was found in multiple repositories under the same organization or enterprise. | [optional]
**is_base64_encoded** | Option<**bool**> | A boolean value representing whether or not alert is base64 encoded | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


