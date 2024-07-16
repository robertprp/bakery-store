use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use entity::notification::{NotificationStatusEnum, NotificationTypeEnum};
use serde::Serialize;

use crate::queries::PageInput;

#[derive(Clone, Debug, Serialize, InputObject, Default)]
pub struct NotificationsInput {
    pub page: Option<PageInput>,
    pub filter: Option<NotificationsFilterInput>,
}

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct NotificationsFilterInput {
    pub vault: Option<String>,
    #[graphql(name = "type")]
    pub notification_type: Option<NotificationTypeEnum>,
    pub status: Option<NotificationStatusEnum>,
}

#[derive(Clone, Debug, Serialize, InputObject, Default)]
pub struct UpdateNotificationStatusInput {
    pub vault: Option<String>,
    #[graphql(name = "type")]
    pub notification_type: Option<NotificationTypeEnum>,
    pub before_date: DateTime<Utc>,
}
