# ActionsHostedRunner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The unique identifier of the hosted runner. | 
**name** | **String** | The name of the hosted runner. | 
**runner_group_id** | Option<**i32**> | The unique identifier of the group that the hosted runner belongs to. | [optional]
**image_details** | Option<[**models::NullableActionsHostedRunnerPoolImage**](nullable-actions-hosted-runner-pool-image.md)> |  | 
**machine_size_details** | [**models::ActionsHostedRunnerMachineSpec**](actions-hosted-runner-machine-spec.md) |  | 
**status** | **String** | The status of the runner. | 
**platform** | **String** | The operating system of the image. | 
**maximum_runners** | Option<**i32**> | The maximum amount of hosted runners. Runners will not scale automatically above this number. Use this setting to limit your cost. | [optional][default to 10]
**public_ip_enabled** | **bool** | Whether public IP is enabled for the hosted runners. | 
**public_ips** | Option<[**Vec<models::PublicIp>**](public-ip.md)> | The public IP ranges when public IP is enabled for the hosted runners. | [optional]
**last_active_on** | Option<**String**> | The time at which the runner was last used, in ISO 8601 format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


