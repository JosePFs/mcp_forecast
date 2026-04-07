use crate::dto::RawDayOutputParameter;
use crate::dto::RawForecast;
use crate::forecasting::Forecaster as ForecasterTrait;
use crate::forecasting::RiskForecaster;
use crate::forecasting::forecast::{Forecast, Summary};

pub struct Forecaster<T: RiskForecaster> {
    risk_forecaster: T,
}

impl<T: RiskForecaster> Forecaster<T> {
    pub fn new(risk_forecaster: T) -> Self {
        Self { risk_forecaster }
    }
}

impl<T: RiskForecaster> ForecasterTrait for Forecaster<T> {
    fn apply(&self, raw_forecasts: Vec<RawForecast>) -> Vec<Forecast> {
        let forecasts = raw_forecasts
            .into_iter()
            .flat_map(|raw_forecast| {
                raw_forecast
                    .places
                    .iter()
                    .map(|place| {
                        let mut summary = Summary::default();

                        if place.status.is_not_found() {
                            return Forecast {
                                place: place.name.clone(),
                                municipality: place.municipality.clone(),
                                summary,
                                alerts: vec![place.status.to_string()],
                            };
                        }

                        place.days.iter().for_each(|day| {
                            if let Some(values) =
                                day.values.get(&RawDayOutputParameter::Temperature)
                            {
                                for value in values {
                                    let v = value.value.as_f64().unwrap_or(0.0);
                                    summary.add_temperature(
                                        day.date.clone(),
                                        value.time.clone(),
                                        v,
                                    );
                                }
                            }

                            if let Some(values) =
                                day.values.get(&RawDayOutputParameter::RelativeHumidity)
                            {
                                for value in values {
                                    let v = value.value.as_f64().unwrap_or(0.0);
                                    summary.add_relative_humidity(
                                        day.date.clone(),
                                        value.time.clone(),
                                        v,
                                    );
                                }
                            }

                            if let Some(values) =
                                day.values.get(&RawDayOutputParameter::PrecipitationAmount)
                            {
                                for value in values {
                                    let v = value.value.as_f64().unwrap_or(0.0);
                                    summary.add_precipitation_amount(
                                        day.date.clone(),
                                        value.time.clone(),
                                        v,
                                    );
                                }
                            }
                        });

                        Forecast {
                            place: place.name.clone(),
                            municipality: place.municipality.clone(),
                            summary,
                            alerts: vec![],
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.risk_forecaster.apply(forecasts)
    }
}
