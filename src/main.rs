use regex::Regex;
use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let response = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;

    let mut markets: Vec<String> = Vec::new();
    if let Some(list) = json["result"]["list"].as_array() {
        for item in list {
            if let Some(symbol) = item["symbol"].as_str() {
                if symbol.ends_with("USDT") {
                    markets.push(format!("BYBIT:{}", symbol));
                }
            }
        }
    }

    // Özel sıralama fonksiyonu
    markets.sort_by(|a, b| {
        let re = Regex::new(r"(\d+)|(\D+)").unwrap();
        let a_parts: Vec<_> = re.find_iter(a).map(|m| m.as_str()).collect();
        let b_parts: Vec<_> = re.find_iter(b).map(|m| m.as_str()).collect();

        for (a_part, b_part) in a_parts.iter().zip(b_parts.iter()) {
            if a_part.parse::<u32>().is_ok() && b_part.parse::<u32>().is_ok() {
                let a_num = a_part.parse::<u32>().unwrap();
                let b_num = b_part.parse::<u32>().unwrap();
                if a_num != b_num {
                    return b_num.cmp(&a_num);
                }
            } else {
                let cmp = a_part.cmp(b_part);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
        }
        a.cmp(b)
    });

    let mut file = File::create("bybit_usdt_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler başarıyla 'bybit_usdt_markets.txt' dosyasına yazıldı.");
    Ok(())
}
