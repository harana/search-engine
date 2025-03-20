# Artifact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**name** | **String** | The name of the artifact. | 
**size_in_bytes** | **i32** | The size in bytes of the artifact. | 
**url** | **String** |  | 
**archive_download_url** | **String** |  | 
**expired** | **bool** | Whether or not the artifact has expired. | 
**created_at** | Option<**String**> |  | 
**expires_at** | Option<**String**> |  | 
**updated_at** | Option<**String**> |  | 
**digest** | Option<**String**> | The SHA256 digest of the artifact. This field will only be populated on artifacts uploaded with upload-artifact v4 or newer. For older versions, this field will be null. | [optional]
**workflow_run** | Option<[**models::ArtifactWorkflowRun**](artifact_workflow_run.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


