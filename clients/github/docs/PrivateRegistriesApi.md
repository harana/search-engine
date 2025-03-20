# \PrivateRegistriesApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**private_registries_slash_create_org_private_registry**](PrivateRegistriesApi.md#private_registries_slash_create_org_private_registry) | **POST** /orgs/{org}/private-registries | Create a private registry for an organization
[**private_registries_slash_delete_org_private_registry**](PrivateRegistriesApi.md#private_registries_slash_delete_org_private_registry) | **DELETE** /orgs/{org}/private-registries/{secret_name} | Delete a private registry for an organization
[**private_registries_slash_get_org_private_registry**](PrivateRegistriesApi.md#private_registries_slash_get_org_private_registry) | **GET** /orgs/{org}/private-registries/{secret_name} | Get a private registry for an organization
[**private_registries_slash_get_org_public_key**](PrivateRegistriesApi.md#private_registries_slash_get_org_public_key) | **GET** /orgs/{org}/private-registries/public-key | Get private registries public key for an organization
[**private_registries_slash_list_org_private_registries**](PrivateRegistriesApi.md#private_registries_slash_list_org_private_registries) | **GET** /orgs/{org}/private-registries | List private registries for an organization
[**private_registries_slash_update_org_private_registry**](PrivateRegistriesApi.md#private_registries_slash_update_org_private_registry) | **PATCH** /orgs/{org}/private-registries/{secret_name} | Update a private registry for an organization



## private_registries_slash_create_org_private_registry

> models::OrgPrivateRegistryConfigurationWithSelectedRepositories private_registries_slash_create_org_private_registry(org, private_registries_create_org_private_registry_request)
Create a private registry for an organization

> [!NOTE] > This endpoint is in public preview and is subject to change.  Creates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**private_registries_create_org_private_registry_request** | [**PrivateRegistriesCreateOrgPrivateRegistryRequest**](PrivateRegistriesCreateOrgPrivateRegistryRequest.md) |  | [required] |

### Return type

[**models::OrgPrivateRegistryConfigurationWithSelectedRepositories**](org-private-registry-configuration-with-selected-repositories.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_registries_slash_delete_org_private_registry

> private_registries_slash_delete_org_private_registry(org, secret_name)
Delete a private registry for an organization

> [!NOTE] > This endpoint is in public preview and is subject to change.  Delete a private registry configuration at the organization-level.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_registries_slash_get_org_private_registry

> models::OrgPrivateRegistryConfiguration private_registries_slash_get_org_private_registry(org, secret_name)
Get a private registry for an organization

> [!NOTE] > This endpoint is in public preview and is subject to change.  Get the configuration of a single private registry defined for an organization, omitting its encrypted value.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::OrgPrivateRegistryConfiguration**](org-private-registry-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_registries_slash_get_org_public_key

> models::PrivateRegistriesGetOrgPublicKey200Response private_registries_slash_get_org_public_key(org)
Get private registries public key for an organization

> [!NOTE] > This endpoint is in public preview and is subject to change.  Gets the org public key, which is needed to encrypt private registry secrets. You need to encrypt a secret before you can create or update secrets.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::PrivateRegistriesGetOrgPublicKey200Response**](private_registries_get_org_public_key_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_registries_slash_list_org_private_registries

> models::PrivateRegistriesListOrgPrivateRegistries200Response private_registries_slash_list_org_private_registries(org, per_page, page)
List private registries for an organization

> [!NOTE] > This endpoint is in public preview and is subject to change.  Lists all private registry configurations available at the organization-level without revealing their encrypted values.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::PrivateRegistriesListOrgPrivateRegistries200Response**](private_registries_list_org_private_registries_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_registries_slash_update_org_private_registry

> private_registries_slash_update_org_private_registry(org, secret_name, private_registries_update_org_private_registry_request)
Update a private registry for an organization

> [!NOTE] > This endpoint is in public preview and is subject to change.  Updates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**private_registries_update_org_private_registry_request** | [**PrivateRegistriesUpdateOrgPrivateRegistryRequest**](PrivateRegistriesUpdateOrgPrivateRegistryRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

