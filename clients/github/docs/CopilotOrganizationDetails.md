# CopilotOrganizationDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seat_breakdown** | [**models::CopilotOrganizationSeatBreakdown**](copilot-organization-seat-breakdown.md) |  | 
**public_code_suggestions** | **String** | The organization policy for allowing or blocking suggestions matching public code (duplication detection filter). | 
**ide_chat** | Option<**String**> | The organization policy for allowing or disallowing Copilot Chat in the IDE. | [optional]
**platform_chat** | Option<**String**> | The organization policy for allowing or disallowing Copilot features on GitHub.com. | [optional]
**cli** | Option<**String**> | The organization policy for allowing or disallowing Copilot in the CLI. | [optional]
**seat_management_setting** | **String** | The mode of assigning new seats. | 
**plan_type** | Option<**String**> | The Copilot plan of the organization, or the parent enterprise, when applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


