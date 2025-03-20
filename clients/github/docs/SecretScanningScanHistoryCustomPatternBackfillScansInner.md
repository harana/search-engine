# SecretScanningScanHistoryCustomPatternBackfillScansInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of scan | [optional]
**status** | Option<**String**> | The state of the scan. Either \"completed\", \"running\", or \"pending\" | [optional]
**completed_at** | Option<**String**> | The time that the scan was completed. Empty if the scan is running | [optional]
**started_at** | Option<**String**> | The time that the scan was started. Empty if the scan is pending | [optional]
**pattern_name** | Option<**String**> | Name of the custom pattern for custom pattern scans | [optional]
**pattern_scope** | Option<**String**> | Level at which the custom pattern is defined, one of \"repository\", \"organization\", or \"enterprise\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


