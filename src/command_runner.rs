use std::process::Command;

use anyhow::Result;

use crate::dto::ForecastInput;

pub fn run_forecast(input: &ForecastInput) -> Result<String> {
    let mut cmd = Command::new("forecast");

    cmd.arg("forecast");

    log::debug!("Command: {:?}", input);

    for place in &input.places {
        cmd.arg("--places").arg(place);
    }

    cmd.arg("--start-time").arg(input.start_time.clone());
    cmd.arg("--end-time").arg(input.end_time.clone());

    if let Ok(key) = std::env::var("API_KEY") {
        cmd.env("API_KEY", key);
    }

    if let Ok(url) = std::env::var("BASE_URL") {
        cmd.env("BASE_URL", url);
    }

    let output = cmd.output()?;

    log::debug!("Command output: {:?}", output);

    Ok(String::from_utf8(output.stdout)?)
}
