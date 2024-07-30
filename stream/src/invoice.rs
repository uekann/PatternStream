use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
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

#[derive(Debug, Serialize, Clone)]
pub struct Stock {
    #[serde(rename = "StockCode")]
    stock_code: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "UnitPrice")]
    unit_price: f64,

    #[serde(rename = "Quantity")]
    quantity: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Invoice {
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
    pub fn new(record: Record) -> Self {
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

    pub fn add_stock(&mut self, record: Record) -> Result<()> {
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

pub struct InvoiceIterator<'a> {
    current_invoice: Invoice,
    deserialize: csv::DeserializeRecordsIter<'a, File, Record>,
}

impl<'a> InvoiceIterator<'a> {
    pub fn new(reader: &'a mut csv::Reader<File>) -> Self {
        let mut deserialize = reader.deserialize();
        let inovice = Invoice::new(deserialize.next().unwrap().unwrap());
        InvoiceIterator {
            current_invoice: inovice,
            deserialize,
        }
    }
}

impl<'a> Iterator for InvoiceIterator<'a> {
    type Item = Result<Invoice>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let record = match self.deserialize.next() {
                Some(Ok(record)) => record,
                Some(Err(err)) => return Some(Err(err.into())),
                None => return None,
            };
            if record.unit_price == 0.0 {
                continue;
            }
            match self.current_invoice.add_stock(record.clone()) {
                Ok(_) => {}
                Err(_) => {
                    let result = self.current_invoice.clone();
                    self.current_invoice = Invoice::new(record);
                    return Some(Ok(result));
                }
            }
        }
    }
}
