pub fn alert_error(message: String) -> (String, String) {
    ("HX-Trigger".to_string(), format!("{{\"alerterror\": \"{}\"}}", message))
}
pub fn alert_success(message: String) -> (String, String) {
    ("HX-Trigger".to_string(), format!("{{\"alertsuccess\": \"{}\"}}", message))
}
