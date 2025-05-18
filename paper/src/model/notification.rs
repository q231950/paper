#[derive(uniffi::Record, Debug)]
pub struct Notification {
    pub notification_type: NotificationType,
    pub message: String,
}

#[derive(uniffi::Enum, Debug, PartialEq)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
}
