use async_graphql::Enum;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum ChartDatasetRange {
    OneDay,
    OneWeek,
    OneMonth,
    All,
}
