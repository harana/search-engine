# DependabotAlertWithRepositoryDependency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package** | Option<[**models::DependabotAlertPackage**](dependabot-alert-package.md)> |  | [optional]
**manifest_path** | Option<**String**> | The full path to the dependency manifest file, relative to the root of the repository. | [optional][readonly]
**scope** | Option<**String**> | The execution scope of the vulnerable dependency. | [optional][readonly]
**relationship** | Option<**String**> | The vulnerable dependency's relationship to your project.  > [!NOTE] > We are rolling out support for dependency relationship across ecosystems. This value will be \"unknown\" for all dependencies in unsupported ecosystems.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


