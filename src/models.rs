use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct UsageSection {
    pub percentage: f64,
    #[serde(serialize_with = "serialize_system_time")]
    pub resets_at: Option<SystemTime>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UsageData {
    pub session: UsageSection,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AppUsageData {
    pub claude_code: Option<UsageData>,
    pub codex: Option<UsageData>,
    pub polled_at: Option<String>,
}

fn serialize_system_time<S>(value: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value.and_then(system_time_to_iso8601) {
        Some(value) => serializer.serialize_some(&value),
        None => serializer.serialize_none(),
    }
}

fn system_time_to_iso8601(value: SystemTime) -> Option<String> {
    let duration = value.duration_since(UNIX_EPOCH).ok()?;
    Some(format_iso8601(duration.as_secs()))
}

pub fn current_utc_timestamp() -> Option<String> {
    system_time_to_iso8601(SystemTime::now())
}

fn format_iso8601(unix_secs: u64) -> String {
    let days = unix_secs / 86_400;
    let secs_of_day = unix_secs % 86_400;

    let (year, month, day) = civil_from_days(days as i64);
    let hour = secs_of_day / 3_600;
    let minute = (secs_of_day % 3_600) / 60;
    let second = secs_of_day % 60;

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}+00:00")
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };

    (year as i32, month as u32, day as u32)
}
