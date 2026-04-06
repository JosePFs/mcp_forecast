use std::{collections::HashMap, fmt::Display};

use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ForecastInput {
    #[schemars(
        description = "REQUIRED. List of places in format 'name/municipality', e.g: ['carballo/carballo', 'coruña/coruña']"
    )]
    pub places: Vec<String>,
    #[schemars(
        description = "REQUIRED. Start time in format 'YYYY-MM-DDTHH:MM:SS', e.g: '2026-04-05T00:00:00'"
    )]
    pub start_time: String,
    #[schemars(
        description = "REQUIRED. End time in format 'YYYY-MM-DDTHH:MM:SS', e.g: '2026-04-05T23:59:59'"
    )]
    pub end_time: String,
    #[schemars(description = "List of types, e.g: ['locality', 'beach']")]
    pub types: Vec<String>,
    #[schemars(description = "Language, e.g: 'gl' (Galician), 'es' (Spanish), 'en' (English)")]
    pub lang: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct RawForecast {
    pub places: Vec<RawPlaceOutput>,
}

#[derive(Deserialize, JsonSchema, PartialEq, Eq)]
pub enum RawPlaceStatus {
    #[schemars(description = "Place forecast found")]
    Found,
    #[schemars(description = "Place not found")]
    LocationNotFound,
    #[schemars(description = "Forecast info not found")]
    ForecastInfoNotFound,
}

impl RawPlaceStatus {
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            RawPlaceStatus::LocationNotFound | RawPlaceStatus::ForecastInfoNotFound
        )
    }
}

impl TryFrom<String> for RawPlaceStatus {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Found" => Ok(RawPlaceStatus::Found),
            "LocationNotFound" => Ok(RawPlaceStatus::LocationNotFound),
            "ForecastInfoNotFound" => Ok(RawPlaceStatus::ForecastInfoNotFound),
            _ => Err(format!("Invalid status: '{}'", s)),
        }
    }
}

impl Display for RawPlaceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawPlaceStatus::Found => write!(f, "Found"),
            RawPlaceStatus::LocationNotFound => write!(f, "Location not found"),
            RawPlaceStatus::ForecastInfoNotFound => write!(f, "Forecast info not found"),
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct RawPlaceOutput {
    pub name: String,
    pub municipality: String,
    pub days: Vec<RawDayOutput>,
    pub status: RawPlaceStatus,
}

#[derive(Deserialize, JsonSchema, PartialEq, Eq, Hash)]
pub enum RawDayOutputParameter {
    #[schemars(description = "Temperature in ºC")]
    #[serde(rename = "temperature")]
    Temperature,
    #[schemars(description = "Relative humidity in %")]
    #[serde(rename = "relative_humidity")]
    RelativeHumidity,
    #[schemars(description = "Precipitation amount in mm")]
    #[serde(rename = "precipitation_amount")]
    PrecipitationAmount,
}

#[derive(Deserialize, JsonSchema)]
pub struct RawDayOutput {
    pub date: String,
    pub values: HashMap<RawDayOutputParameter, Vec<RawValueOutput>>,
}

#[derive(Deserialize, JsonSchema)]
pub struct RawValueOutput {
    pub time: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, JsonSchema)]
pub struct ProcessedSummaryRow {
    pub time: String,
    pub value: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct ProcessedSummary {
    pub temperature: Vec<ProcessedSummaryRow>,
    pub relative_humidity: Vec<ProcessedSummaryRow>,
    pub precipitation_amount: Vec<ProcessedSummaryRow>,
}

impl Default for ProcessedSummary {
    fn default() -> Self {
        Self {
            temperature: vec![],
            relative_humidity: vec![],
            precipitation_amount: vec![],
        }
    }
}

impl ProcessedSummary {
    pub fn add_temperature(&mut self, time: String, value: f64) {
        self.temperature.push(ProcessedSummaryRow { time, value });
    }

    pub fn add_relative_humidity(&mut self, time: String, value: f64) {
        self.relative_humidity
            .push(ProcessedSummaryRow { time, value });
    }

    pub fn add_precipitation_amount(&mut self, time: String, value: f64) {
        self.precipitation_amount
            .push(ProcessedSummaryRow { time, value });
    }
}

#[derive(Serialize, JsonSchema)]
pub struct ProcessedForecast {
    pub place: String,
    pub municipality: String,
    pub summary: ProcessedSummary,
    pub alerts: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct ForecastResponse {
    pub forecasts: Vec<ProcessedForecast>,
}
