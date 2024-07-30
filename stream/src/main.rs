use anyhow::{bail, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "InvoiceNo")]
    invoice_no: String,

    #[serde(rename = "StockCode")]
    stock_code: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Quantity")]
    quantity: i32,

    #[serde(rename = "InvoiceDate")]
    invoice_date: String,

    #[serde(rename = "UnitPrice")]
    unit_price: f64,

    #[serde(rename = "CustomerID")]
    customer_id: String,

    #[serde(rename = "Country")]
    country: String,
}

#[derive(Debug, Serialize)]
struct Stock {
    #[serde(rename = "StockCode")]
    stock_code: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "UnitPrice")]
    unit_price: f64,

    #[serde(rename = "Quantity")]
    quantity: i32,
}

#[derive(Debug, Serialize)]
struct Invoice {
    #[serde(rename = "InvoiceNo")]
    invoice_no: String,

    #[serde(rename = "CustomerID")]
    customer_id: String,

    #[serde(rename = "Country")]
    country: String,

    #[serde(rename = "InvoiceDate")]
    invoice_date: String,

    #[serde(rename = "Stocks")]
    stocks: Vec<Stock>,
}

impl Invoice {
    fn new(record: Record) -> Self {
        let stock = Stock {
            stock_code: record.stock_code,
            description: record.description,
            unit_price: record.unit_price,
            quantity: record.quantity,
        };
        Invoice {
            invoice_no: record.invoice_no,
            customer_id: record.customer_id,
            country: record.country,
            invoice_date: record.invoice_date,
            stocks: vec![stock],
        }
    }

    fn add_stock(&mut self, record: Record) -> Result<()> {
        if self.invoice_no != record.invoice_no {
            bail!("Invoice number does not match");
        }
        let stock = Stock {
            stock_code: record.stock_code,
            description: record.description,
            unit_price: record.unit_price,
            quantity: record.quantity,
        };
        self.stocks.push(stock);
        Ok(())
    }
}

fn main() -> Result<()> {
    const FILE_PATH: &str = "data/OnlineRetail.csv";
    let mut reader = Reader::from_path(FILE_PATH).unwrap();
    let mut current_invoice: Option<Invoice> = None;
    for result in reader.deserialize() {
        let record: Record = result?;
        if let Some(invoice) = current_invoice.as_mut() {
            match invoice.add_stock(record) {
                Ok(_) => {}
                Err(e) => {
                    let json = serde_json::to_string(&invoice).unwrap();
                    println!("{}", json);
                    return Ok(());
                }
            }
        } else {
            current_invoice = Some(Invoice::new(record));
        }
    }
    Ok(())
}
