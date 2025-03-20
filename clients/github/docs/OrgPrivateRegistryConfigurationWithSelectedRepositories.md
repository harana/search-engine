# OrgPrivateRegistryConfigurationWithSelectedRepositories

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the private registry configuration. | 
**registry_type** | **String** | The registry type. | 
**username** | Option<**String**> | The username to use when authenticating with the private registry. | [optional]
**visibility** | **String** | Which type of organization repositories have access to the private registry. `selected` means only the repositories specified by `selected_repository_ids` can access the private registry. | 
**selected_repository_ids** | Option<**Vec<i32>**> | An array of repository IDs that can access the organization private registry when `visibility` is set to `selected`. | [optional]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


