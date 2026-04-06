use crate::dto::{ProcessedForecast, ProcessedSummary, RawDayOutputParameter, RawForecast};

const PRECIPITATION_AMOUNT_MEDIUM_THRESHOLD: f64 = 15.0;
const PRECIPITATION_AMOUNT_HIGH_THRESHOLD: f64 = 50.0;
const TEMPERATURE_HIGH_THRESHOLD: f64 = 25.0;
const RELATIVE_HUMIDITY_HIGH_THRESHOLD: f64 = 80.0;

pub fn apply_rules(data: Vec<RawForecast>) -> Vec<ProcessedForecast> {
    data.into_iter()
        .flat_map(|raw_forecast| {
            raw_forecast
                .places
                .iter()
                .map(|place| {
                    let mut alerts = vec![];
                    let mut summary = ProcessedSummary::default();

                    if place.status.is_not_found() {
                        return ProcessedForecast {
                            place: place.name.clone(),
                            municipality: place.municipality.clone(),
                            summary,
                            alerts: vec![place.status.to_string()],
                        };
                    }

                    raw_forecast.places.iter().for_each(|place| {
                        place.days.iter().for_each(|day| {
                            day.values
                                .get(&RawDayOutputParameter::Temperature)
                                .iter()
                                .for_each(|values| {
                                    values.iter().for_each(|value| {
                                        summary.add_temperature(
                                            value.time.clone(),
                                            value.value.as_f64().unwrap_or(0.0),
                                        );
                                        if value.value.as_f64() > Some(TEMPERATURE_HIGH_THRESHOLD) {
                                            alerts.push(format!(
                                                "Temperature high {} at {}",
                                                value.value.as_f64().unwrap_or(0.0),
                                                value.time
                                            ));
                                        }
                                    });
                                });
                        });
                    });

                    raw_forecast.places.iter().for_each(|place| {
                        place.days.iter().for_each(|day| {
                            day.values
                                .get(&RawDayOutputParameter::RelativeHumidity)
                                .iter()
                                .for_each(|values| {
                                    values.iter().for_each(|value| {
                                        summary.add_relative_humidity(
                                            value.time.clone(),
                                            value.value.as_f64().unwrap_or(0.0),
                                        );
                                        if value.value.as_f64()
                                            > Some(RELATIVE_HUMIDITY_HIGH_THRESHOLD)
                                        {
                                            alerts.push(format!(
                                                "Relative humidity high {} at {}",
                                                value.value.as_f64().unwrap_or(0.0),
                                                value.time,
                                            ));
                                        }
                                    });
                                });
                        });
                    });

                    raw_forecast.places.iter().for_each(|place| {
                        place.days.iter().for_each(|day| {
                            day.values
                                .get(&RawDayOutputParameter::PrecipitationAmount)
                                .iter()
                                .for_each(|values| {
                                    values.iter().for_each(|value| {
                                        summary.add_precipitation_amount(
                                            value.time.clone(),
                                            value.value.as_f64().unwrap_or(0.0),
                                        );
                                        if value.value.as_f64()
                                            > Some(PRECIPITATION_AMOUNT_HIGH_THRESHOLD)
                                        {
                                            alerts.push(format!(
                                                "Precipitation amount high {}mm at {}",
                                                value.value.as_f64().unwrap_or(0.0),
                                                value.time,
                                            ));
                                        } else if value.value.as_f64()
                                            > Some(PRECIPITATION_AMOUNT_MEDIUM_THRESHOLD)
                                        {
                                            alerts.push(format!(
                                                "Precipitation amount medium {}mm at {}",
                                                value.value.as_f64().unwrap_or(0.0),
                                                value.time,
                                            ));
                                        }
                                    });
                                })
                        })
                    });

                    ProcessedForecast {
                        place: place.name.clone(),
                        municipality: place.municipality.clone(),
                        summary,
                        alerts,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
