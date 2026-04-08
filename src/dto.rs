use std::{collections::HashMap, fmt::Display};

use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

use crate::forecasting::forecast::{Forecast, Summary};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ForecastInput {
    #[schemars(
        example = "['carballo/carballo', 'coruña/coruña']",
        description = "List of places in format 'name/municipality'."
    )]
    pub places: Vec<String>,
    #[schemars(
        description = "Start time in format 'YYYY-MM-DDTHH:MM:SS'.",
        example = "2026-04-05T00:00:00"
    )]
    pub start_time: String,
    #[schemars(
        description = "End time in format 'YYYY-MM-DDTHH:MM:SS'.",
        example = "2026-04-05T23:59:59"
    )]
    pub end_time: String,
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
    #[schemars(description = "Wind speed in km/h")]
    #[serde(rename = "wind")]
    Wind,
}

#[derive(Deserialize, JsonSchema)]
pub struct RawDayOutput {
    pub date: String,
    pub values: HashMap<RawDayOutputParameter, Vec<RawValueOutput>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WindOrScalar {
    Wind { speed: f64, direction: f64 },
    Scalar(f64),
}

#[derive(Deserialize, JsonSchema)]
pub struct RawValueOutput {
    pub time: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, JsonSchema)]
pub struct ProcessedSummary {
    #[serde(serialize_with = "serialize_rounded")]
    #[schemars(description = "Maximum temperature in ºC")]
    pub temperature_max: f64,
    #[serde(serialize_with = "serialize_rounded")]
    #[schemars(description = "Minimum temperature in ºC")]
    pub temperature_min: f64,
    #[serde(serialize_with = "serialize_rounded")]
    #[schemars(description = "Mean temperature in ºC")]
    pub temperature_mean: f64,
    #[serde(serialize_with = "serialize_rounded")]
    #[schemars(description = "Mean relative humidity in %")]
    pub relative_humidity_mean: f64,
    #[serde(serialize_with = "serialize_rounded")]
    #[schemars(description = "Precipitation amount accumulated in mm")]
    pub precipitation_amount_accumulated: f64,
    #[serde(serialize_with = "serialize_rounded")]
    #[schemars(description = "Mean wind speed in km/h")]
    pub wind_speed_mean: f64,
    #[schemars(description = "Predominant wind directions ordered by frequency")]
    pub wind_direction_predominant: String,
}

fn serialize_rounded<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(value.round())
}

#[derive(Serialize, JsonSchema)]
pub struct ProcessedForecast {
    pub place: String,
    pub municipality: String,
    pub summary: HashMap<String, ProcessedSummary>,
    pub alerts: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct ForecastResponse {
    pub forecasts: Vec<ProcessedForecast>,
}

impl From<Forecast> for ProcessedForecast {
    fn from(forecast: Forecast) -> Self {
        Self {
            place: forecast.place,
            municipality: forecast.municipality,
            summary: forecast.summary.into(),
            alerts: forecast.alerts,
        }
    }
}

impl From<Summary> for HashMap<String, ProcessedSummary> {
    fn from(summary: Summary) -> Self {
        summary
            .temperature
            .keys()
            .map(|date| {
                (
                    date.clone(),
                    ProcessedSummary {
                        temperature_max: summary.get_max_temperature(&date),
                        temperature_min: summary.get_min_temperature(&date),
                        temperature_mean: summary.get_mean_temperature(&date),
                        relative_humidity_mean: summary.get_mean_relative_humidity(&date),
                        precipitation_amount_accumulated: summary
                            .get_precipitation_amount_accumulated(&date),
                        wind_speed_mean: summary.get_mean_wind_speed(&date),
                        wind_direction_predominant: summary.get_predominant_wind_direction(&date),
                    },
                )
            })
            .collect()
    }
}
