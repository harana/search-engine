# PrivateRegistriesUpdateOrgPrivateRegistryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**registry_type** | Option<**String**> | The registry type. | [optional]
**username** | Option<**String**> | The username to use when authenticating with the private registry. This field should be omitted if the private registry does not require a username for authentication. | [optional]
**encrypted_value** | Option<**String**> | The value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get private registries public key for an organization](https://docs.github.com/rest/private-registries/organization-configurations#get-private-registries-public-key-for-an-organization) endpoint. | [optional]
**key_id** | Option<**String**> | The ID of the key you used to encrypt the secret. | [optional]
**visibility** | Option<**String**> | Which type of organization repositories have access to the private registry. `selected` means only the repositories specified by `selected_repository_ids` can access the private registry. | [optional]
**selected_repository_ids** | Option<**Vec<i32>**> | An array of repository IDs that can access the organization private registry. You can only provide a list of repository IDs when `visibility` is set to `selected`. This field should be omitted if `visibility` is set to `all` or `private`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


