# ActionsUpdateHostedRunnerForOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the runner. Must be between 1 and 64 characters and may only contain upper and lowercase letters a-z, numbers 0-9, '.', '-', and '_'. | [optional]
**runner_group_id** | Option<**i32**> | The existing runner group to add this runner to. | [optional]
**maximum_runners** | Option<**i32**> | The maximum amount of runners to scale up to. Runners will not auto-scale above this number. Use this setting to limit your cost. | [optional]
**enable_static_ip** | Option<**bool**> | Whether this runner should be updated with a static public IP. Note limit on account. To list limits on account, use `GET actions/hosted-runners/limits` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


