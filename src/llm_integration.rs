use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use crate::iot_device::SensorData;

pub struct LLMAnalyzer {
    api_key: String,
    client: Client,
}

impl LLMAnalyzer {
    pub fn new(api_key: String) -> Self {
        LLMAnalyzer {
            api_key,
            client: Client::new(),
        }
    }
    
    // Original OpenAI method (keep as fallback)
    pub async fn analyze_sensor_data(
        &self,
        sensor_data: &[SensorData]
    ) -> Result<String, Box<dyn Error>> {
        if sensor_data.is_empty() {
            return Ok("No data to analyze".to_string());
        }
        
        // Create a structured representation of the data
        let data_points = sensor_data.iter()
            .take(10) // Limit to 10 data points
            .map(|d| format!(
                "Time: {}, Device: {}, Temp: {:.1}°C, Humidity: {:.1}%",
                d.timestamp, d.device_id, d.temperature, d.humidity
            ))
            .collect::<Vec<String>>()
            .join("\n");
        
        let prompt = format!(
            "Analyze this IoT sensor data:\n\n{}\n\nProvide a brief analysis of patterns and anomalies.",
            data_points
        );
        
        println!("Sending request to OpenAI API...");
        
        let response = match self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.7,
                "max_tokens": 500
            }))
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => {
                    println!("API request error: {}", e);
                    return Ok("Error making API request: ".to_string() + &e.to_string());
                }
            };
        
        println!("Response status: {}", response.status());
        
        let response_text = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                println!("Error reading response: {}", e);
                return Ok("Error reading API response: ".to_string() + &e.to_string());
            }
        };
        
        println!("Raw response: {}", response_text);
        
        let response_json: Result<Value, _> = serde_json::from_str(&response_text);
        
        match response_json {
            Ok(json) => {
                // Extract the response content
                if let Some(choices) = json.get("choices") {
                    if let Some(first_choice) = choices.get(0) {
                        if let Some(message) = first_choice.get("message") {
                            if let Some(content) = message.get("content") {
                                if let Some(content_str) = content.as_str() {
                                    return Ok(content_str.to_string());
                                }
                            }
                        }
                    }
                }
                
                println!("Failed to extract content from JSON: {:?}", json);
                Ok("Failed to parse LLM response JSON structure".to_string())
            },
            Err(e) => {
                println!("JSON parsing error: {}", e);
                Ok("Failed to parse LLM response as JSON: ".to_string() + &e.to_string())
            }
        }
    }
    
    // New Groq API method
    pub async fn analyze_sensor_data_groq(
        &self,
        sensor_data: &[SensorData]
    ) -> Result<String, Box<dyn Error>> {
        if sensor_data.is_empty() {
            return Ok("No data to analyze".to_string());
        }
        
        // Create a structured representation of the data
        let data_points = sensor_data.iter()
            .take(10)
            .map(|d| format!(
                "Time: {}, Device: {}, Temp: {:.1}°C, Humidity: {:.1}%",
                d.timestamp, d.device_id, d.temperature, d.humidity
            ))
            .collect::<Vec<String>>()
            .join("\n");
        
        let prompt = format!(
            "Analyze this IoT sensor data:\n\n{}\n\nProvide a brief analysis of patterns and anomalies.",
            data_points
        );
        
        println!("Sending request to Groq API...");
        
        let response = match self.client.post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key)) // Use Groq API key
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "llama2-70b-4096",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.7,
                "max_tokens": 500
            }))
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => {
                    println!("API request error: {}", e);
                    return Ok("Error making API request: ".to_string() + &e.to_string());
                }
            };
        
        println!("Response status: {}", response.status());
        
        let response_text = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                println!("Error reading response: {}", e);
                return Ok("Error reading API response: ".to_string() + &e.to_string());
            }
        };
        
        println!("Raw response: {}", response_text);
        
        // Parse Groq response (similar to OpenAI format)
        match serde_json::from_str::<Value>(&response_text) {
            Ok(json) => {
                if let Some(choices) = json.get("choices") {
                    if let Some(first_choice) = choices.get(0) {
                        if let Some(message) = first_choice.get("message") {
                            if let Some(content) = message.get("content") {
                                if let Some(content_str) = content.as_str() {
                                    return Ok(content_str.to_string());
                                }
                            }
                        }
                    }
                }
                
                println!("Failed to extract content from JSON: {:?}", json);
                Ok("Failed to parse Groq response".to_string())
            },
            Err(e) => {
                println!("JSON parsing error: {}", e);
                Ok("Failed to parse Groq response as JSON: ".to_string() + &e.to_string())
            }
        }
    }
    
    // Keep the fallback analysis method
    pub fn fallback_analysis(&self, sensor_data: &[SensorData]) -> String {
        if sensor_data.is_empty() {
            return "No data to analyze".to_string();
        }
        
        // Calculate basic statistics
        let temp_values: Vec<f64> = sensor_data.iter().map(|d| d.temperature).collect();
        let humid_values: Vec<f64> = sensor_data.iter().map(|d| d.humidity).collect();
        
        let temp_avg = temp_values.iter().sum::<f64>() / temp_values.len() as f64;
        let humid_avg = humid_values.iter().sum::<f64>() / humid_values.len() as f64;
        
        let temp_min = temp_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let temp_max = temp_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let humid_min = humid_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let humid_max = humid_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        // Generate a simple analysis
        format!(
            "Data Analysis Summary:\n\
             - Temperature: avg {:.1}°C (range: {:.1}°C to {:.1}°C)\n\
             - Humidity: avg {:.1}% (range: {:.1}% to {:.1}%)\n\
             - Total readings: {}\n\
             - Devices: {}\n\
             - Time span: {} seconds",
            temp_avg, temp_min, temp_max,
            humid_avg, humid_min, humid_max,
            sensor_data.len(),
            sensor_data.iter().map(|d| &d.device_id).collect::<std::collections::HashSet<_>>().len(),
            sensor_data.last().unwrap().timestamp - sensor_data.first().unwrap().timestamp
        )
    }
}
