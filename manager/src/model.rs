use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_number: String,
    pub buyer: String,
    pub seller: String,
    pub total_amount: f64,
    pub items: Vec<String>,
    pub invoice_date: Option<String>, // yyyy-mm-dd
    pub source_file: Option<String>,
}
