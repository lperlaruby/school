use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use ureq;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Bitcoin {}

#[derive(Debug, Deserialize)]
struct Ethereum {}

#[derive(Debug, Deserialize)]
struct SP500 {}

trait Pricing {
    fn fetch_price(&self) -> Result<String, String>;
    fn save_to_file(&self, price: &str, file_name: &str);
    fn get_name(&self) -> &'static str;
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<String, String> {
        let url = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
        if response.status() == 200 {
            let json: serde_json::Value = serde_json::from_reader(response.into_reader())
                .map_err(|e| e.to_string())?;
            let price = json["bpi"]["USD"]["rate"]
                .as_str()
                .unwrap()
                .replace(",", "");
            Ok(price)
        } else {
            Err(format!("Failed to fetch Bitcoin price. Status: {}", response.status()))
        }
    }

    fn save_to_file(&self, price: &str, file_name: &str) {
        let mut file = File::create(file_name).unwrap();
        writeln!(file, "Bitcoin price: ${}", price).unwrap();
    }

    fn get_name(&self) -> &'static str {
        "Bitcoin"
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<String, String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
        if response.status() == 200 {
            let json: serde_json::Value = serde_json::from_reader(response.into_reader())
                .map_err(|e| e.to_string())?;
            let price = json["ethereum"]["usd"].to_string();
            Ok(price)
        } else {
            Err(format!("Failed to fetch Ethereum price. Status: {}", response.status()))
        }
    }

    fn save_to_file(&self, price: &str, file_name: &str) {
        let mut file = File::create(file_name).unwrap();
        writeln!(file, "Ethereum price: ${}", price).unwrap();
    }

    fn get_name(&self) -> &'static str {
        "Ethereum"
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<String, String> {
        let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=SPY&apikey=YOUR_API_KEY";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
        if response.status() == 200 {
            let json: serde_json::Value = serde_json::from_reader(response.into_reader())
                .map_err(|e| e.to_string())?;
            let price = json["Global Quote"]["05. price"]
                .as_str()
                .ok_or("Missing price field in response")?
                .to_string();
            Ok(price)
        } else {
            Err(format!("Failed to fetch SP500 value. Status: {}", response.status()))
        }
    }

    fn save_to_file(&self, value: &str, file_name: &str) {
        let mut file = File::create(file_name).unwrap();
        writeln!(file, "SP500 value: ${}", value).unwrap();
    }

    fn get_name(&self) -> &'static str {
        "SP500"
    }
}

fn main() {
    let bitcoin = Bitcoin {};
    let ethereum = Ethereum {};
    let sp500 = SP500 {};

    let assets: Vec<&dyn Pricing> = vec![&bitcoin, &ethereum, &sp500];

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    let file_name = match asset.get_name() {
                        "Bitcoin" => "bitcoin.txt",
                        "Ethereum" => "ethereum.txt",
                        "SP500" => "sp500.txt",
                        _ => "unknown.txt",
                    };
                    asset.save_to_file(&price, file_name);
                    println!("Saved {} data to {}", asset.get_name(), file_name);
                }
                Err(e) => println!("Error fetching {} data: {}", asset.get_name(), e),
            }
        }
        sleep(Duration::from_secs(10));
    }
}

