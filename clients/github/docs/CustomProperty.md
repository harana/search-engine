# CustomProperty

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**property_name** | **String** | The name of the property | 
**url** | Option<**String**> | The URL that can be used to fetch, update, or delete info about this property via the API. | [optional]
**source_type** | Option<**String**> | The source type of the property | [optional]
**value_type** | **String** | The type of the value for the property | 
**required** | Option<**bool**> | Whether the property is required. | [optional]
**default_value** | Option<[**models::CustomPropertyDefaultValue**](custom_property_default_value.md)> |  | [optional]
**description** | Option<**String**> | Short description of the property | [optional]
**allowed_values** | Option<**Vec<String>**> | An ordered list of the allowed values of the property. The property can have up to 200 allowed values. | [optional]
**values_editable_by** | Option<**String**> | Who can edit the values of the property | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


