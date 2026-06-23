use serde_json::json;
use serde_json::Value;

use crate::ocpp::types::ChargePointStatus;

pub fn build(
    connector_id: u8,
    status: ChargePointStatus,
) -> Value {
    json!([
        2,
        crate::ocpp::message_id::next(),
          "StatusNotification",
          {
              "connectorId": connector_id,
          "errorCode": "NoError",
          "status": status.as_str(),
          "vendorErrorCode": "0x0000"
          }
    ])
}
