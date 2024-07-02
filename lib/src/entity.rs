use sea_orm::{sea_query, ActiveValue, Value};

/// Wrap provided Option to ActiveValue
/// - Some(T): ActiveValue::Set(Some(T))
/// - None: ActiveValue::NotSet
pub fn opt_to_active_value_opt<T>(arg: Option<T>) -> ActiveValue<Option<T>>
where
    T: Into<Value> + sea_query::Nullable,
{
    arg.map(|i| ActiveValue::Set(Some(i)))
        .unwrap_or(ActiveValue::NotSet)
}

/// Convert provided Option to ActiveValue
/// - Some(T): ActiveValue::Set(T)
/// - None: ActiveValue::NotSet
pub fn opt_to_active_value<T>(arg: Option<T>) -> ActiveValue<T>
where
    T: Into<Value> + sea_query::Nullable,
{
    arg.map(ActiveValue::Set).unwrap_or(ActiveValue::NotSet)
}
