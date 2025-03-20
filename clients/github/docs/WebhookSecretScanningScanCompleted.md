# WebhookSecretScanningScanCompleted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**r#type** | **String** | What type of scan was completed | 
**source** | **String** | What type of content was scanned | 
**started_at** | **String** | The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | 
**completed_at** | **String** | The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | 
**secret_types** | Option<**Vec<String>**> | List of patterns that were updated. This will be empty for normal backfill scans or custom pattern updates | [optional]
**custom_pattern_name** | Option<**String**> | If the scan was triggered by a custom pattern update, this will be the name of the pattern that was updated | [optional]
**custom_pattern_scope** | Option<**String**> | If the scan was triggered by a custom pattern update, this will be the scope of the pattern that was updated | [optional]
**repository** | Option<[**models::RepositoryWebhooks**](repository-webhooks.md)> |  | [optional]
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**sender** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


