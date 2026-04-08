use std::collections::HashMap;

pub struct SummaryRow {
    pub time: String,
    pub value: f64,
}

pub struct WindSummaryRow {
    pub time: String,
    pub speed: f64,
    pub direction: f64,
}

pub struct Summary {
    pub temperature: HashMap<String, Vec<SummaryRow>>,
    pub relative_humidity: HashMap<String, Vec<SummaryRow>>,
    pub precipitation_amount: HashMap<String, Vec<SummaryRow>>,
    pub wind: HashMap<String, Vec<WindSummaryRow>>,
}

impl Default for Summary {
    fn default() -> Self {
        Self {
            temperature: HashMap::new(),
            relative_humidity: HashMap::new(),
            precipitation_amount: HashMap::new(),
            wind: HashMap::new(),
        }
    }
}

fn classify_wind_sector(degrees: f64) -> &'static str {
    let normalized = degrees.rem_euclid(360.0);
    match normalized {
        d if d >= 337.5 || d < 22.5 => "N",
        d if d < 67.5 => "NE",
        d if d < 112.5 => "E",
        d if d < 157.5 => "SE",
        d if d < 202.5 => "S",
        d if d < 247.5 => "SW",
        d if d < 292.5 => "W",
        _ => "NW",
    }
}

impl Summary {
    pub fn add_temperature(&mut self, date: String, time: String, value: f64) {
        self.temperature
            .entry(date)
            .or_insert(vec![])
            .push(SummaryRow { time, value });
    }

    pub fn add_relative_humidity(&mut self, date: String, time: String, value: f64) {
        self.relative_humidity
            .entry(date)
            .or_insert(vec![])
            .push(SummaryRow { time, value });
    }

    pub fn add_precipitation_amount(&mut self, date: String, time: String, value: f64) {
        self.precipitation_amount
            .entry(date)
            .or_insert(vec![])
            .push(SummaryRow { time, value });
    }

    pub fn add_wind(&mut self, date: String, time: String, speed: f64, direction: f64) {
        self.wind
            .entry(date)
            .or_insert(vec![])
            .push(WindSummaryRow {
                time,
                speed,
                direction,
            });
    }

    pub fn get_max_temperature(&self, date: &String) -> f64 {
        self.temperature
            .get(date)
            .unwrap()
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .unwrap()
            .value
    }

    pub fn get_min_temperature(&self, date: &String) -> f64 {
        self.temperature
            .get(date)
            .unwrap()
            .iter()
            .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .unwrap()
            .value
    }

    pub fn get_mean_temperature(&self, date: &String) -> f64 {
        self.temperature
            .get(date)
            .unwrap()
            .iter()
            .map(|row| row.value)
            .sum::<f64>()
            / self.temperature.get(date).unwrap().len() as f64
    }

    pub fn get_max_relative_humidity(&self, date: String) -> f64 {
        self.relative_humidity
            .get(&date)
            .unwrap()
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .unwrap()
            .value
    }

    pub fn get_mean_relative_humidity(&self, date: &String) -> f64 {
        self.relative_humidity
            .get(date)
            .unwrap()
            .iter()
            .map(|row| row.value)
            .sum::<f64>()
            / self.relative_humidity.get(date).unwrap().len() as f64
    }

    pub fn get_max_precipitation_amount(&self, date: String) -> f64 {
        self.precipitation_amount
            .get(&date)
            .unwrap()
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .unwrap()
            .value
    }

    pub fn get_mean_precipitation_amount(&self, date: &String) -> f64 {
        self.precipitation_amount
            .get(date)
            .unwrap()
            .iter()
            .map(|row| row.value)
            .sum::<f64>()
            / self.precipitation_amount.get(date).unwrap().len() as f64
    }

    pub fn get_precipitation_amount_accumulated(&self, date: &String) -> f64 {
        self.precipitation_amount
            .get(date)
            .unwrap()
            .iter()
            .fold(0.0, |acc, row| acc + row.value)
    }

    pub fn get_max_wind_speed(&self, date: String) -> f64 {
        self.wind
            .get(&date)
            .unwrap()
            .iter()
            .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
            .unwrap()
            .speed
    }

    pub fn get_mean_wind_speed(&self, date: &String) -> f64 {
        self.wind
            .get(date)
            .unwrap()
            .iter()
            .map(|row| row.speed)
            .sum::<f64>()
            / self.wind.get(date).unwrap().len() as f64
    }

    pub fn get_predominant_wind_direction(&self, date: &String) -> String {
        let mut sector_counts: HashMap<&str, usize> = HashMap::new();
        let wind_data = self.wind.get(date).unwrap();
        let total = wind_data.len();

        for row in wind_data {
            let sector = classify_wind_sector(row.direction);
            *sector_counts.entry(sector).or_insert(0) += 1;
        }

        let mut sectors: Vec<_> = sector_counts.into_iter().collect();
        sectors.sort_by(|a, b| b.1.cmp(&a.1));

        sectors
            .into_iter()
            .map(|(sector, count)| {
                let percentage = (count as f64 / total as f64 * 100.0).round() as u32;
                format!("{}({}%)", sector, percentage)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub struct Forecast {
    pub place: String,
    pub municipality: String,
    pub summary: Summary,
    pub alerts: Vec<String>,
}
