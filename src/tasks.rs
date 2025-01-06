use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub date: DateTime<Utc>,
    pub done: bool,
}