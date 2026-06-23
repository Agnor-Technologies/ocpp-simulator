use config::ResolvedInstance;
use serde_json::Value;

pub fn build(
    config: &ResolvedInstance,
) -> Value {
    serde_json::json!([
        2,
        crate::ocpp::message_id::next(),
        "BootNotification",
        {
            "chargePointVendor":
            config.profile.vendor,

            "chargePointModel":
            config.profile.model,

            "chargePointSerialNumber":
            config.instance.id,

            "chargeBoxSerialNumber":
            config.instance.id,

            "firmwareVersion":
            config.profile.firmware_version,

            "iccid":
            config.profile.iccid,

            "imsi":
            config.profile.imsi,

            "meterType":
            config.profile.meter_type,

            "meterSerialNumber":
            config.profile.meter_serial_number
        }
    ])
}
