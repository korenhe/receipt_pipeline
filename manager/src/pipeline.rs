use std::fs;
use std::path::Path;

use crate::db;
use crate::model::Receipt;
use crate::{ocr, ollama};

pub async fn run_scan(input_dir: &str, db_path: &str) -> anyhow::Result<()> {
    db::init(db_path)?;

    let path = Path::new(input_dir);
    let pngs = ocr::pdf_to_png(&path)?;

    for png in pngs {
        println!("processing png: {}", png.display());
        let ocr_text = ocr::run_ocr(&png)?;
        println!("ocr text: {}", ocr_text);

        let prompt = format!(
            r#"You are a data extraction engine.

Task:
Extract the buyer and seller company names from OCR text.

Rules:
- Company names appear after the prefix "名称："
- The BUYER is the FIRST company name in the text
- The SELLER is the LAST company name in the text
- Ignore section headers like "购买方信息" or "销售方信息"
- The total_amount should after the "计税合计", and this value should be the sum of the value after "合计"
- Do NOT guess or translate names
- If a value is missing, use null

Output format:
Output MUST be valid JSON.
DO NOT include explanations.
DO NOT include markdown.
DO NOT include extra fields.
DO NOT translate company names.
If a field is missing, use null.

Schema:
{{
  "receipt_number": string,
  "buyer": string,
  "seller": string,
  "total_amount": number,
  "items": [string]
  "invoice_date" : string,
}}

Text:
<<<
{}
>>>
"#,
            ocr_text
        );

        let result = ollama::extract_receipt(
            "http://localhost:11434",
            "qwen2.5:7b",
            &prompt,
        ).await?;

        println!("result from ollama: {}", result);

        // convert result into receipt
        let receipt: Receipt = serde_json::from_str(&result)?;

        db::insert_receipt(db_path, &receipt)?;
    }

    Ok(())
}
