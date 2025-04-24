use crate::blockchain::Blockchain;
use crate::iot_device::{IoTDevice, SensorData};
use crate::ai_model::{LinearRegressionModel, detect_anomalies};
use crate::llm_integration::LLMAnalyzer;
use std::time::Duration;
use std::env;
use std::error::Error;

pub async fn run_system() -> Result<(), Box<dyn Error>> {
    // Get API key - now using GROQ_API_KEY instead of OPENAI_API_KEY
    let api_key = env::var("GROQ_API_KEY").unwrap_or_else(|_| {
        println!("GROQ_API_KEY not found, using default key");
        "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".to_string()
    });
    
    // Initialize blockchain
    let mut blockchain = Blockchain::new();
    
    // Initialize IoT devices
    let devices = vec![
        IoTDevice::new("device_001".to_string()),
        IoTDevice::new("device_002".to_string()),
    ];
    
    // Initialize AI model
    let mut model = LinearRegressionModel::new();
    
    // Initialize LLM analyzer with Groq API key
    let llm_analyzer = LLMAnalyzer::new(api_key);
    
    // Collect sensor data
    let mut all_sensor_data: Vec<SensorData> = Vec::new();
    let mut temperature_data: Vec<f64> = Vec::new();
    let mut timestamps: Vec<u64> = Vec::new();
    
    println!("Starting IoT data collection and blockchain integration...");
    
    // Simulate data collection for 5 cycles
    for i in 0..5 {
        println!("\n--- Cycle {} ---", i + 1);
        
        // Collect data from each device
        for device in &devices {
            // Occasionally generate an anomaly (20% chance)
            let reading = if rand::random::<f64>() < 0.2 {
                println!("Generating anomaly for device {}", device.id);
                device.generate_anomaly()
            } else {
                device.read_sensor()
            };
            
            println!("Reading: {}", reading.to_string());
            
            // Add to blockchain
            let block_data = reading.to_json();
            let block = blockchain.add_block(block_data);
            println!("Added to blockchain: {}", block);
            
            // Store data for analysis
            all_sensor_data.push(reading.clone());
            temperature_data.push(reading.temperature);
            timestamps.push(reading.timestamp);
        }
        
        // Train model after collecting enough data
        if temperature_data.len() > 5 {
            // Use timestamps as x values (normalized)
            let base_time = *timestamps.first().unwrap() as f64;
            let x_values: Vec<f64> = timestamps.iter()
                .map(|&t| (t as f64 - base_time) / 3600.0) // Convert to hours
                .collect();
            
            // Train model on temperature data
            model.train(&x_values, &temperature_data);
            
            // Check for anomalies
            let anomalies = detect_anomalies(&temperature_data, 2.0);
            if !anomalies.is_empty() {
                println!("Detected {} temperature anomalies", anomalies.len());
            }
        }
        
        // On the last cycle, use LLM to analyze the data
        if i == 4 && !all_sensor_data.is_empty() {
            println!("\nPerforming LLM analysis of sensor data...");
            
            // Try Groq API first
            match llm_analyzer.analyze_sensor_data_groq(&all_sensor_data).await {
                Ok(analysis) if !analysis.starts_with("Failed") && !analysis.starts_with("Error") => {
                    println!("\n=== AI Analysis (Groq) ===\n{}\n", analysis);
                    
                    // Add the analysis to the blockchain
                    let analysis_block = blockchain.add_block(format!("ANALYSIS: {}", analysis));
                    println!("Added analysis to blockchain: {}", analysis_block);
                },
                _ => {
                    // Use fallback analysis if Groq API call fails
                    let fallback = llm_analyzer.fallback_analysis(&all_sensor_data);
                    println!("\n=== Fallback Analysis ===\n{}\n", fallback);
                    
                    // Add the fallback analysis to the blockchain
                    let analysis_block = blockchain.add_block(format!("ANALYSIS: {}", fallback));
                    println!("Added fallback analysis to blockchain: {}", analysis_block);
                }
            }
        }
        
        // Verify blockchain integrity
        if !blockchain.is_valid() {
            println!("WARNING: Blockchain integrity compromised!");
        }
        
        // Pause between cycles
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    println!("\nFinal blockchain length: {}", blockchain.get_chain_length());
    println!("Blockchain is valid: {}", blockchain.is_valid());
    
    Ok(())
}
