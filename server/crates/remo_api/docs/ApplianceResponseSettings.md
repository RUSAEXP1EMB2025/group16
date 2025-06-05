# ApplianceResponseSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**button** | Option<**String**> | Button. Specify 'power-off' always if you want the air conditioner powered off. Empty means powered on. | [optional]
**dir** | Option<**String**> | AC air direction. Empty means automatic. | [optional]
**dirh** | Option<**String**> | AC horizontal air direction. | [optional]
**mode** | Option<**String**> | AC operation mode. The range of operation modes which the air conditioner accepts depends on the air conditioner model. Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model. | [optional]
**temp** | Option<**String**> | Temperature. The temperature in string format. The unit is described in Aircon object. The range of Temperatures which the air conditioner accepts depends on the air conditioner model and operation mode. Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model and operation mode. | [optional]
**temp_unit** | Option<**String**> | Temperature unit. 'c' or 'f' or '' for unknown. | [optional]
**updated_at** | Option<**String**> |  | [optional]
**vol** | Option<**String**> | AC air volume. Empty means automatic. Numbers express the amount of volume. The range of AirVolumes which the air conditioner accepts depends on the air conditioner model and operation mode. Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model and operation mode. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


