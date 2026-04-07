use rmcp::{
    ErrorData, Json, ServerHandler,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::{
    bootstrap::bootstrap_forecaster,
    command_runner,
    dto::{ForecastInput, ForecastResponse, RawForecast},
    forecasting::Forecaster as _,
};

#[derive(Clone)]
pub struct RiskWeatherServer {
    tool_router: ToolRouter<Self>,
}

impl RiskWeatherServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl RiskWeatherServer {
    #[tool(
        description = "Get weather forecast for one or more places with risk of pine plague. IMPORTANT: places, start_time and end_time are required and must contain at least one entry in the format 'name/municipality', e.g: 'carballo/carballo', '2026-04-05T00:00:00' and '2026-04-05T23:59:59' respectively. Ask the user for the place, start time and end time if not provided."
    )]
    pub async fn get_risk_weather_forecast(
        &self,
        params: Parameters<ForecastInput>,
    ) -> Json<ForecastResponse> {
        let input = params.0;

        let raw_json = command_runner::run_forecast(&input).map_err(|e| {
            ErrorData::internal_error(
                e.to_string(),
                Some(serde_json::to_value(&input).unwrap_or_default()),
            )
        });

        let parsed: Vec<RawForecast> =
            serde_json::from_str(&raw_json.unwrap_or_default()).unwrap_or_default();

        let weather_forecaster = bootstrap_forecaster();
        let forecasts = weather_forecaster.apply(parsed);

        Json(ForecastResponse {
            forecasts: forecasts
                .into_iter()
                .map(|forecast| forecast.into())
                .collect(),
        })
    }
}

#[tool_handler]
impl ServerHandler for RiskWeatherServer {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(Default::default());

        ServerInfo::new(capabilities)
    }
}
