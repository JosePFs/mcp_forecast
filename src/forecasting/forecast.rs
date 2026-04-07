use std::collections::HashMap;

pub struct SummaryRow {
    pub time: String,
    pub value: f64,
}

pub struct Summary {
    pub temperature: HashMap<String, Vec<SummaryRow>>,
    pub relative_humidity: HashMap<String, Vec<SummaryRow>>,
    pub precipitation_amount: HashMap<String, Vec<SummaryRow>>,
}

impl Default for Summary {
    fn default() -> Self {
        Self {
            temperature: HashMap::new(),
            relative_humidity: HashMap::new(),
            precipitation_amount: HashMap::new(),
        }
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
}

pub struct Forecast {
    pub place: String,
    pub municipality: String,
    pub summary: Summary,
    pub alerts: Vec<String>,
}
