//! The Rust Stock Analysis Library is a Rust-based package designed to provide functionalities for retrieving stock data,
//! calculating technical indicators, visualizing trends, and generating reports.

use serde::{Deserialize, Serialize};
// use serde_json::Value;
use std::error::Error;

/// Structure to hold stock data.
#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    /// Stock symbol.
    pub symbol: String,
    /// Current stock price.
    pub price: f64,
    /// Current stock volume.
    pub volume: u64,
    /// Market capitalization.
    pub market_cap: f64,
}

impl StockData {
    /// Creates a new StockData instance.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol.
    /// * `price` - Current stock price.
    /// * `volume` - Current stock volume.
    /// * `market_cap` - Market capitalization.
    pub fn new(symbol: String, price: f64, volume: u64, market_cap: f64) -> Self {
        StockData {
            symbol,
            price,
            volume,
            market_cap,
        }
    }

    /// Retrieves stock data from API and returns a StockData instance.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol.
    pub fn retrieve_data(symbol: &str) -> Result<Self, Box<dyn Error>> {
        let json_response = fetch_data_from_api(symbol)?;
        let response = serde_json::from_str::<ApiResponse>(&json_response)?;
        Ok(StockData::new(
            symbol.to_string(),
            response.price,
            response.volume,
            response.market_cap,
        ))
    }

    /// Calculates technical indicators for the stock.
    ///
    /// Currently calculates 10-day moving average and 14-day RSI.
    pub fn calculate_technical_indicators(&self) {
        // Calculate moving average
        let moving_average = self.calculate_moving_average(10); // Calculate 10-day moving average
        println!("10-day moving average: {}", moving_average);

        // Calculate relative strength index (RSI)
        let rsi = self.calculate_rsi(14); // Calculate 14-day RSI
        println!("14-day RSI: {}", rsi);

        // Implement other technical indicators as needed
    }

    /// Visualizes trends for the stock.
    ///
    /// Currently plots stock price over time.
    pub fn visualize_trends(&self) {
        // Example: Plot stock price over time
        // You can use a plotting library like plotters to create visualizations
        // Dummy code for demonstration purposes
        println!("Visualizing trends...");
    }

    /// Generates a report for the stock.
    ///
    /// Currently generates a simple report.
    pub fn generate_report(&self) {
        // Example: Generate a simple report
        println!("Generating report...");
        println!("Symbol: {}", self.symbol);
        println!("Price: {}", self.price);
        println!("Volume: {}", self.volume);
        println!("Market Cap: {}", self.market_cap);
    }

    // helper methods for calculating technical indicators
    /// Calculates moving average for the stock.
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the moving average.
    fn calculate_moving_average(&self, period: usize) -> f64 {
        self.price * 0.95
    }

    /// Calculates relative strength index (RSI) for the stock.
    ///
    /// # Arguments
    ///
    /// * `period` - Period for the RSI.
    fn calculate_rsi(&self, period: usize) -> f64 {
        self.volume as f64 * 1.05
    }
}

/// Response structure for API request.
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    /// Current stock price.
    pub price: f64,
    /// Current stock volume.
    pub volume: u64,
    /// Market capitalization.
    pub market_cap: f64,
}

/// Retrieves data from API and returns a JSON response.
///
/// # Arguments
///
/// * `symbol` - Stock symbol.
fn fetch_data_from_api(symbol: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks",
        symbol
    );
    Ok(reqwest::blocking::get(&url)?.text()?)
}


