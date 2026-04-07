use crate::forecasting::RiskForecaster;
use crate::forecasting::forecast::Forecast;

pub struct PinePlagueRiskForecaster<T: RiskForecaster> {
    risk_forecaster: T,
}

impl<T: RiskForecaster> PinePlagueRiskForecaster<T> {
    const TEMPERATURE_MEDIUM_RANGE_THRESHOLD: f64 = 25.0;
    const RELATIVE_HUMIDITY_MEDIUM_RANGE_THRESHOLD: f64 = 76.0;
    const RELATIVE_HUMIDITY_HIGH_RANGE_THRESHOLD: f64 = 80.0;

    pub fn new(risk_forecaster: T) -> Self {
        Self { risk_forecaster }
    }
}

impl<T: RiskForecaster> RiskForecaster for PinePlagueRiskForecaster<T> {
    fn apply(&self, forecasts: Vec<Forecast>) -> Vec<Forecast> {
        let forecasts = self.risk_forecaster.apply(forecasts);
        forecasts
            .into_iter()
            .map(|forecast| {
                let mut alerts = forecast.alerts;

                let dates = forecast
                    .summary
                    .temperature
                    .keys()
                    .collect::<Vec<&String>>();
                let (mean_max_temperature, mean_relative_humidity) = {
                    let (temperature_sum, relative_humidity_sum) = dates
                        .iter()
                        .map(|date| {
                            (
                                forecast.summary.get_max_temperature(date),
                                forecast.summary.get_mean_relative_humidity(date),
                            )
                        })
                        .fold((0.0_f64, 0.0_f64), |(ta, ha), (t, h)| (ta + t, ha + h));

                    (
                        temperature_sum / forecast.summary.temperature.len() as f64,
                        relative_humidity_sum / forecast.summary.relative_humidity.len() as f64,
                    )
                };

                if mean_max_temperature < Self::TEMPERATURE_MEDIUM_RANGE_THRESHOLD
                    && mean_relative_humidity > Self::RELATIVE_HUMIDITY_HIGH_RANGE_THRESHOLD
                {
                    alerts.push(format!(
                        "Temperature medium-low {}ºC and relative humidity high over {}% on {}",
                        mean_max_temperature.round(),
                        mean_relative_humidity.round(),
                        dates
                            .iter()
                            .map(|date| date.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    ));
                } else if mean_max_temperature < Self::TEMPERATURE_MEDIUM_RANGE_THRESHOLD
                    && mean_relative_humidity > Self::RELATIVE_HUMIDITY_MEDIUM_RANGE_THRESHOLD
                {
                    alerts.push(format!(
                        "Temperature medium-low {}ºC and relative humidity high over {}% on {}",
                        mean_max_temperature.round(),
                        mean_relative_humidity.round(),
                        dates
                            .iter()
                            .map(|date| date.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    ));
                }

                Forecast { alerts, ..forecast }
            })
            .collect()
    }
}
