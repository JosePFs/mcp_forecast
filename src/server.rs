use rmcp::{
    ErrorData, Json, ServerHandler,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::{
    command_runner,
    dto::{ForecastInput, ForecastResponse, RawForecast},
    rules,
};

#[derive(Clone)]
pub struct WeatherServer {
    tool_router: ToolRouter<Self>,
}

impl WeatherServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl WeatherServer {
    #[tool(
        description = "Get weather forecast for one or more places. IMPORTANT: places, start_time and end_time are required and must contain at least one entry in the format 'name/municipality', e.g: 'carballo/carballo', '2026-04-05T00:00:00' and '2026-04-05T23:59:59' respectively. Ask the user for the place, start time and end time if not provided."
    )]
    pub async fn get_weather_forecast(
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

        let forecasts = rules::apply_rules(parsed);

        Json(ForecastResponse { forecasts })
    }
}

#[tool_handler]
impl ServerHandler for WeatherServer {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(Default::default());

        ServerInfo::new(capabilities)
    }
}
