use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub device_id: String,
    pub temperature: f64,
    pub humidity: f64,
    pub timestamp: u64,
}

impl SensorData {
    pub fn to_string(&self) -> String {
        format!(
            "Device: {}, Time: {}, Temp: {:.1}°C, Humidity: {:.1}%",
            self.device_id, self.timestamp, self.temperature, self.humidity
        )
    }
    
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

pub struct IoTDevice {
    pub id: String,
    base_temperature: f64,
    base_humidity: f64,
    temperature_variance: f64,
    humidity_variance: f64,
}

impl IoTDevice {
    pub fn new(id: String) -> Self {
        let mut rng = rand::thread_rng();
        
        IoTDevice {
            id,
            base_temperature: rng.gen_range(18.0..25.0),
            base_humidity: rng.gen_range(30.0..60.0),
            temperature_variance: 2.0,
            humidity_variance: 5.0,
        }
    }
    
    pub fn read_sensor(&self) -> SensorData {
        let mut rng = rand::thread_rng();
        
        let temperature = self.base_temperature + 
            rng.gen_range(-self.temperature_variance..self.temperature_variance);
        
        let humidity = self.base_humidity + 
            rng.gen_range(-self.humidity_variance..self.humidity_variance);
        
        SensorData {
            device_id: self.id.clone(),
            temperature,
            humidity,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    // Simulate an anomaly in the readings
    pub fn generate_anomaly(&self) -> SensorData {
        let mut rng = rand::thread_rng();
        let anomaly_type = rng.gen_range(0..3);
        
        match anomaly_type {
            0 => {
                // Temperature spike
                SensorData {
                    device_id: self.id.clone(),
                    temperature: self.base_temperature + rng.gen_range(10.0..15.0),
                    humidity: self.base_humidity + 
                        rng.gen_range(-self.humidity_variance..self.humidity_variance),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            },
            1 => {
                // Humidity spike
                SensorData {
                    device_id: self.id.clone(),
                    temperature: self.base_temperature + 
                        rng.gen_range(-self.temperature_variance..self.temperature_variance),
                    humidity: self.base_humidity + rng.gen_range(20.0..30.0),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            },
            _ => {
                // Both temperature and humidity anomaly
                SensorData {
                    device_id: self.id.clone(),
                    temperature: self.base_temperature - rng.gen_range(8.0..12.0),
                    humidity: self.base_humidity - rng.gen_range(15.0..25.0),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            }
        }
    }
}