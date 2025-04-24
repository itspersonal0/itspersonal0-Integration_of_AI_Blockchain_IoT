pub struct LinearRegressionModel {
    slope: f64,
    intercept: f64,
}

impl LinearRegressionModel {
    pub fn new() -> Self {
        LinearRegressionModel {
            slope: 0.0,
            intercept: 0.0,
        }
    }
    
    pub fn train(&mut self, x: &[f64], y: &[f64]) {
        if x.len() != y.len() || x.is_empty() {
            return;
        }
        
        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(x, y)| x * y).sum();
        let sum_xx: f64 = x.iter().map(|x| x * x).sum();
        
        self.slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
        self.intercept = (sum_y - self.slope * sum_x) / n;
        
        println!("Model trained: y = {}x + {}", self.slope, self.intercept);
    }
    
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
    
    pub fn predict_batch(&self, x_values: &[f64]) -> Vec<f64> {
        x_values.iter().map(|&x| self.predict(x)).collect()
    }
    
    pub fn get_parameters(&self) -> (f64, f64) {
        (self.slope, self.intercept)
    }
}

pub fn detect_anomalies(data: &[f64], threshold: f64) -> Vec<usize> {
    if data.len() < 3 {
        return Vec::new();
    }
    
    // Calculate mean and standard deviation
    let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
    
    let variance: f64 = data.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / data.len() as f64;
    
    let std_dev = variance.sqrt();
    
    // Detect anomalies (values outside threshold * std_dev from mean)
    data.iter()
        .enumerate()
        .filter(|(_, &x)| (x - mean).abs() > threshold * std_dev)
        .map(|(i, _)| i)
        .collect()
}
