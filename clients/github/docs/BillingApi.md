# \BillingApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**billing_slash_get_github_actions_billing_org**](BillingApi.md#billing_slash_get_github_actions_billing_org) | **GET** /orgs/{org}/settings/billing/actions | Get GitHub Actions billing for an organization
[**billing_slash_get_github_actions_billing_user**](BillingApi.md#billing_slash_get_github_actions_billing_user) | **GET** /users/{username}/settings/billing/actions | Get GitHub Actions billing for a user
[**billing_slash_get_github_billing_usage_report_org**](BillingApi.md#billing_slash_get_github_billing_usage_report_org) | **GET** /organizations/{org}/settings/billing/usage | Get billing usage report for an organization
[**billing_slash_get_github_packages_billing_org**](BillingApi.md#billing_slash_get_github_packages_billing_org) | **GET** /orgs/{org}/settings/billing/packages | Get GitHub Packages billing for an organization
[**billing_slash_get_github_packages_billing_user**](BillingApi.md#billing_slash_get_github_packages_billing_user) | **GET** /users/{username}/settings/billing/packages | Get GitHub Packages billing for a user
[**billing_slash_get_shared_storage_billing_org**](BillingApi.md#billing_slash_get_shared_storage_billing_org) | **GET** /orgs/{org}/settings/billing/shared-storage | Get shared storage billing for an organization
[**billing_slash_get_shared_storage_billing_user**](BillingApi.md#billing_slash_get_shared_storage_billing_user) | **GET** /users/{username}/settings/billing/shared-storage | Get shared storage billing for a user



## billing_slash_get_github_actions_billing_org

> models::ActionsBillingUsage billing_slash_get_github_actions_billing_org(org)
Get GitHub Actions billing for an organization

Gets the summary of the free and paid GitHub Actions minutes used.  Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see \"[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)\".  OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsBillingUsage**](actions-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_actions_billing_user

> models::ActionsBillingUsage billing_slash_get_github_actions_billing_user(username)
Get GitHub Actions billing for a user

Gets the summary of the free and paid GitHub Actions minutes used.  Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see \"[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)\".  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::ActionsBillingUsage**](actions-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_billing_usage_report_org

> models::BillingUsageReport billing_slash_get_github_billing_usage_report_org(org, year, month, day, hour)
Get billing usage report for an organization

Gets a report of the total usage for an organization. To use this endpoint, you must be an administrator of an organization within an enterprise or an organization account.  **Note:** This endpoint is only available to organizations with access to the enhanced billing platform. For more information, see \"[About the enhanced billing platform](https://docs.github.com/billing/using-the-new-billing-platform).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**year** | Option<**i32**> | If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year. |  |
**month** | Option<**i32**> | If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. If no year is specified the default `year` is used. |  |
**day** | Option<**i32**> | If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used. |  |
**hour** | Option<**i32**> | If specified, only return results for a single hour. The value of `hour` is an integer between `0` and `23`. If no `year`, `month`, or `day` is specified, the default `year`, `month`, and `day` are used. |  |

### Return type

[**models::BillingUsageReport**](billing-usage-report.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_packages_billing_org

> models::PackagesBillingUsage billing_slash_get_github_packages_billing_org(org)
Get GitHub Packages billing for an organization

Gets the free and paid storage used for GitHub Packages in gigabytes.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::PackagesBillingUsage**](packages-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_packages_billing_user

> models::PackagesBillingUsage billing_slash_get_github_packages_billing_user(username)
Get GitHub Packages billing for a user

Gets the free and paid storage used for GitHub Packages in gigabytes.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::PackagesBillingUsage**](packages-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_shared_storage_billing_org

> models::CombinedBillingUsage billing_slash_get_shared_storage_billing_org(org)
Get shared storage billing for an organization

Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::CombinedBillingUsage**](combined-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_shared_storage_billing_user

> models::CombinedBillingUsage billing_slash_get_shared_storage_billing_user(username)
Get shared storage billing for a user

Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::CombinedBillingUsage**](combined-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

