use crate::forecasting::RiskForecaster;
use crate::forecasting::forecast::Forecast;

pub struct WeatherRiskForecaster;

impl WeatherRiskForecaster {
    const PRECIPITATION_AMOUNT_MEDIUM_THRESHOLD: f64 = 15.0;
    const PRECIPITATION_AMOUNT_HIGH_THRESHOLD: f64 = 40.0;
    const TEMPERATURE_MEDIUM_RANGE_THRESHOLD: f64 = 25.0;
    const TEMPERATURE_HIGH_RANGE_THRESHOLD: f64 = 30.0;
    const RELATIVE_HUMIDITY_MEDIUM_HIGH_RANGE_THRESHOLD: f64 = 70.0;
    const RELATIVE_HUMIDITY_HIGH_RANGE_THRESHOLD: f64 = 85.0;
    const WIND_SPEED_MEDIUM_THRESHOLD: f64 = 10.0;
    const WIND_SPEED_HIGH_THRESHOLD: f64 = 20.0;

    pub fn new() -> Self {
        Self
    }
}

impl RiskForecaster for WeatherRiskForecaster {
    fn apply(&self, data: Vec<Forecast>) -> Vec<Forecast> {
        data.into_iter()
            .map(|forecast| {
                let mut alerts = forecast.alerts;

                for date in forecast.summary.temperature.keys() {
                    let max_temperature = forecast.summary.get_max_temperature(date);
                    if max_temperature > Self::TEMPERATURE_HIGH_RANGE_THRESHOLD {
                        alerts.push(format!(
                            "Max temperature high {}ºC at {}",
                            max_temperature.round(),
                            date
                        ));
                    } else if max_temperature < Self::TEMPERATURE_MEDIUM_RANGE_THRESHOLD {
                        alerts.push(format!(
                            "Max temperature medium-low {}ºC at {}",
                            max_temperature.round(),
                            date
                        ));
                    }

                    let mean_relative_humidity = forecast.summary.get_mean_relative_humidity(date);
                    if mean_relative_humidity > Self::RELATIVE_HUMIDITY_HIGH_RANGE_THRESHOLD {
                        alerts.push(format!(
                            "Mean relative humidity high {}% at {}",
                            mean_relative_humidity.round(),
                            date
                        ));
                    } else if mean_relative_humidity
                        > Self::RELATIVE_HUMIDITY_MEDIUM_HIGH_RANGE_THRESHOLD
                    {
                        alerts.push(format!(
                            "Mean relative humidity medium-high {}% at {}",
                            mean_relative_humidity.round(),
                            date
                        ));
                    }

                    let mean_precipitation_amount =
                        forecast.summary.get_mean_precipitation_amount(date);
                    if mean_precipitation_amount > Self::PRECIPITATION_AMOUNT_MEDIUM_THRESHOLD {
                        alerts.push(format!(
                            "Precipitation amount medium {}mm at {}",
                            mean_precipitation_amount.round(),
                            date
                        ));
                    } else if mean_precipitation_amount > Self::PRECIPITATION_AMOUNT_HIGH_THRESHOLD
                    {
                        alerts.push(format!(
                            "Precipitation amount high {}mm at {}",
                            mean_precipitation_amount.round(),
                            date
                        ));
                    }

                    let mean_wind_speed = forecast.summary.get_mean_wind_speed(date);
                    if mean_wind_speed > Self::WIND_SPEED_HIGH_THRESHOLD {
                        alerts.push(format!(
                            "Mean wind speed medium {}km/h at {}",
                            mean_wind_speed.round(),
                            date
                        ));
                    } else if mean_wind_speed > Self::WIND_SPEED_MEDIUM_THRESHOLD {
                        alerts.push(format!(
                            "Mean wind speed high {}km/h at {}",
                            mean_wind_speed.round(),
                            date
                        ));
                    }
                }

                Forecast { alerts, ..forecast }
            })
            .collect()
    }
}
