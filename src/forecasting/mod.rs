pub mod forecast;
pub mod forecaster;
pub mod pine_plague_risk;
pub mod weather_risk;

use crate::dto::RawForecast;
use crate::forecasting::forecast::Forecast;

pub trait Forecaster {
    fn apply(&self, data: Vec<RawForecast>) -> Vec<Forecast>;
}

pub trait RiskForecaster {
    fn apply(&self, data: Vec<Forecast>) -> Vec<Forecast>;
}
