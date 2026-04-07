use crate::forecasting::forecaster::Forecaster;
use crate::forecasting::pine_plague_risk::PinePlagueRiskForecaster;
use crate::forecasting::weather_risk::WeatherRiskForecaster;

pub fn bootstrap_forecaster() -> Forecaster<PinePlagueRiskForecaster<WeatherRiskForecaster>> {
    let risk_weather_forecaster = WeatherRiskForecaster::new();
    let pine_plague_risk_forecaster = PinePlagueRiskForecaster::new(risk_weather_forecaster);

    Forecaster::new(pine_plague_risk_forecaster)
}
