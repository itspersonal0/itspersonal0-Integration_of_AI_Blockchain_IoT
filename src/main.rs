mod blockchain;
mod iot_device;
mod ai_model;
mod integration;
mod llm_integration;

use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv().ok();
    
    // Run the integrated system
    integration::run_system().await?;
    
    Ok(())
}
