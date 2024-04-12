use std::error::Error;

// Import the StockData struct from lib.rs
use websocket_ticker::StockData;

fn main() {
    // Example usage of StockData::new
    let stock = StockData::new("AAPL".to_string(), 150.0, 10000, 2.0e12);

    // Example usage of StockData::retrieve_data
    match StockData::retrieve_data("AAPL") {
        Ok(stock_data) => {
            println!("Retrieved stock data: {:?}", stock_data);

            // Example usage of StockData::calculate_technical_indicators
            stock_data.calculate_technical_indicators();

            // Example usage of StockData::visualize_trends
            stock_data.visualize_trends();

            // Example usage of StockData::generate_report
            stock_data.generate_report();
        }
        Err(e) => {
            eprintln!("Error retrieving stock data: {}", e);
        }
    }
}
