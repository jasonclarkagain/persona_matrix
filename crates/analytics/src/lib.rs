use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize, Clone)]
pub struct MetricPoint {
    pub timestamp: DateTime<Utc>,
    pub operator_stability: f32,
    pub active_persona: String,
}

pub struct AnalyticsEngine {
    pub history: Vec<MetricPoint>,
}

impl AnalyticsEngine {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn record(&mut self, stability: f32, persona: &str) {
        self.history.push(MetricPoint {
            timestamp: Utc::now(),
            operator_stability: stability,
            active_persona: persona.to_string(),
        });
    }
}
