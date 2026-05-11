use crate::types::{PointLineResponse, RatioRegressionResponse, SinglePointResponse, ratio};

#[derive(Debug, sqlx::FromRow)]
pub struct MeasurementEntry {
    pub timestamp: i64,
    pub stable: i64,
    pub lazer: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DailyEntry {
    pub date: i64,
    pub stable: i64,
    pub lazer: i64,
}

#[derive(Debug)]
pub struct RatioRegression {
    pub target_ratio: f64,
    pub was_reached: bool,
    pub estimated_timestamp: i64,
}

impl From<MeasurementEntry> for SinglePointResponse {
    fn from(value: MeasurementEntry) -> Self {
        Self {
            timestamp: value.timestamp,
            stable: value.stable,
            lazer: value.lazer,
            ratio: ratio(value.stable, value.lazer),
            sum: value.stable + value.lazer,
        }
    }
}

impl From<SinglePointResponse> for MeasurementEntry {
    fn from(value: SinglePointResponse) -> Self {
        Self {
            timestamp: value.timestamp,
            stable: value.stable,
            lazer: value.lazer,
        }
    }
}

impl From<Vec<MeasurementEntry>> for PointLineResponse {
    fn from(value: Vec<MeasurementEntry>) -> Self {
        let mut response = Self {
            timestamp: Vec::with_capacity(value.len()),
            stable: Vec::with_capacity(value.len()),
            lazer: Vec::with_capacity(value.len()),
            sum: Vec::with_capacity(value.len()),
            ratio: Vec::with_capacity(value.len()),
        };

        for entry in value {
            response.timestamp.push(entry.timestamp);
            response.stable.push(entry.stable);
            response.lazer.push(entry.lazer);
            response.sum.push(entry.stable + entry.lazer);
            response.ratio.push(ratio(entry.stable, entry.lazer));
        }

        response
    }
}

impl From<Vec<DailyEntry>> for PointLineResponse {
    fn from(value: Vec<DailyEntry>) -> Self {
        let mut response = Self {
            timestamp: Vec::with_capacity(value.len()),
            stable: Vec::with_capacity(value.len()),
            lazer: Vec::with_capacity(value.len()),
            sum: Vec::with_capacity(value.len()),
            ratio: Vec::with_capacity(value.len()),
        };
        for entry in value {
            response.timestamp.push(entry.date);
            response.stable.push(entry.stable);
            response.lazer.push(entry.lazer);
            response.sum.push(entry.stable + entry.lazer);
            response.ratio.push(ratio(entry.stable, entry.lazer));
        }
        response
    }
}

impl From<RatioRegression> for RatioRegressionResponse {
    fn from(value: RatioRegression) -> Self {
        Self {
            target_ratio: value.target_ratio,
            was_reached: value.was_reached,
            estimated_timestamp: value.estimated_timestamp,
        }
    }
}
