# WebhookCodeScanningAlertAppearedInBranchAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** | The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.` | 
**dismissed_at** | Option<**String**> | The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | 
**dismissed_by** | Option<[**models::User2**](User_2.md)> |  | 
**dismissed_comment** | Option<**String**> | The dismissal comment associated with the dismissal of the alert. | [optional]
**dismissed_reason** | Option<**String**> | The reason for dismissing or closing the alert. | 
**fixed_at** | Option<[**serde_json::Value**](.md)> | The time that the alert was fixed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional]
**html_url** | **String** | The GitHub URL of the alert resource. | 
**most_recent_instance** | Option<[**models::AlertInstance**](Alert_Instance.md)> |  | [optional]
**number** | **i32** | The code scanning alert number. | 
**rule** | [**models::WebhookCodeScanningAlertAppearedInBranchAlertRule**](webhook_code_scanning_alert_appeared_in_branch_alert_rule.md) |  | 
**state** | Option<**String**> | State of a code scanning alert. Events for alerts found outside the default branch will return a `null` value until they are dismissed or fixed. | 
**tool** | [**models::WebhookCodeScanningAlertAppearedInBranchAlertTool**](webhook_code_scanning_alert_appeared_in_branch_alert_tool.md) |  | 
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


