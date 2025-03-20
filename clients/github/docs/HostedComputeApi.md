# \HostedComputeApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hosted_compute_slash_create_network_configuration_for_org**](HostedComputeApi.md#hosted_compute_slash_create_network_configuration_for_org) | **POST** /orgs/{org}/settings/network-configurations | Create a hosted compute network configuration for an organization
[**hosted_compute_slash_delete_network_configuration_from_org**](HostedComputeApi.md#hosted_compute_slash_delete_network_configuration_from_org) | **DELETE** /orgs/{org}/settings/network-configurations/{network_configuration_id} | Delete a hosted compute network configuration from an organization
[**hosted_compute_slash_get_network_configuration_for_org**](HostedComputeApi.md#hosted_compute_slash_get_network_configuration_for_org) | **GET** /orgs/{org}/settings/network-configurations/{network_configuration_id} | Get a hosted compute network configuration for an organization
[**hosted_compute_slash_get_network_settings_for_org**](HostedComputeApi.md#hosted_compute_slash_get_network_settings_for_org) | **GET** /orgs/{org}/settings/network-settings/{network_settings_id} | Get a hosted compute network settings resource for an organization
[**hosted_compute_slash_list_network_configurations_for_org**](HostedComputeApi.md#hosted_compute_slash_list_network_configurations_for_org) | **GET** /orgs/{org}/settings/network-configurations | List hosted compute network configurations for an organization
[**hosted_compute_slash_update_network_configuration_for_org**](HostedComputeApi.md#hosted_compute_slash_update_network_configuration_for_org) | **PATCH** /orgs/{org}/settings/network-configurations/{network_configuration_id} | Update a hosted compute network configuration for an organization



## hosted_compute_slash_create_network_configuration_for_org

> models::NetworkConfiguration hosted_compute_slash_create_network_configuration_for_org(org, hosted_compute_create_network_configuration_for_org_request)
Create a hosted compute network configuration for an organization

Creates a hosted compute network configuration for an organization.  OAuth app tokens and personal access tokens (classic) need the `write:network_configurations` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hosted_compute_create_network_configuration_for_org_request** | [**HostedComputeCreateNetworkConfigurationForOrgRequest**](HostedComputeCreateNetworkConfigurationForOrgRequest.md) |  | [required] |

### Return type

[**models::NetworkConfiguration**](network-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosted_compute_slash_delete_network_configuration_from_org

> hosted_compute_slash_delete_network_configuration_from_org(org, network_configuration_id)
Delete a hosted compute network configuration from an organization

Deletes a hosted compute network configuration from an organization.  OAuth app tokens and personal access tokens (classic) need the `write:network_configurations` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**network_configuration_id** | **String** | Unique identifier of the hosted compute network configuration. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosted_compute_slash_get_network_configuration_for_org

> models::NetworkConfiguration hosted_compute_slash_get_network_configuration_for_org(org, network_configuration_id)
Get a hosted compute network configuration for an organization

Gets a hosted compute network configuration configured in an organization.  OAuth app tokens and personal access tokens (classic) need the `read:network_configurations` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**network_configuration_id** | **String** | Unique identifier of the hosted compute network configuration. | [required] |

### Return type

[**models::NetworkConfiguration**](network-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosted_compute_slash_get_network_settings_for_org

> models::NetworkSettings hosted_compute_slash_get_network_settings_for_org(org, network_settings_id)
Get a hosted compute network settings resource for an organization

Gets a hosted compute network settings resource configured for an organization.  OAuth app tokens and personal access tokens (classic) need the `read:network_configurations` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**network_settings_id** | **String** | Unique identifier of the hosted compute network settings. | [required] |

### Return type

[**models::NetworkSettings**](network-settings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosted_compute_slash_list_network_configurations_for_org

> models::HostedComputeListNetworkConfigurationsForOrg200Response hosted_compute_slash_list_network_configurations_for_org(org, per_page, page)
List hosted compute network configurations for an organization

Lists all hosted compute network configurations configured in an organization.  OAuth app tokens and personal access tokens (classic) need the `read:network_configurations` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::HostedComputeListNetworkConfigurationsForOrg200Response**](hosted_compute_list_network_configurations_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hosted_compute_slash_update_network_configuration_for_org

> models::NetworkConfiguration hosted_compute_slash_update_network_configuration_for_org(org, network_configuration_id, hosted_compute_update_network_configuration_for_org_request)
Update a hosted compute network configuration for an organization

Updates a hosted compute network configuration for an organization.  OAuth app tokens and personal access tokens (classic) need the `write:network_configurations` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**network_configuration_id** | **String** | Unique identifier of the hosted compute network configuration. | [required] |
**hosted_compute_update_network_configuration_for_org_request** | [**HostedComputeUpdateNetworkConfigurationForOrgRequest**](HostedComputeUpdateNetworkConfigurationForOrgRequest.md) |  | [required] |

### Return type

[**models::NetworkConfiguration**](network-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

