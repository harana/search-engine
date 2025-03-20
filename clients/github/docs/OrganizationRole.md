# OrganizationRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The unique identifier of the role. | 
**name** | **String** | The name of the role. | 
**description** | Option<**String**> | A short description about who this role is for or what permissions it grants. | [optional]
**base_role** | Option<**String**> | The system role from which this role inherits permissions. | [optional]
**source** | Option<**String**> | Source answers the question, \"where did this role come from?\" | [optional]
**permissions** | **Vec<String>** | A list of permissions included in this role. | 
**organization** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**created_at** | **String** | The date and time the role was created. | 
**updated_at** | **String** | The date and time the role was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


