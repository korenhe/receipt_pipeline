use duckdb::{Connection, params, types::ValueRef};
use crate::model::Receipt;

pub fn init(path: &str) -> anyhow::Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS receipts (
            receipt_number TEXT PRIMARY KEY,
            buyer TEXT,
            seller TEXT,
            total_amount DOUBLE,
            items TEXT,
            invoice_date TEXT,
            source_file TEXT
        )
        "#,
        [],
    )?;

    Ok(())
}

pub fn insert_receipt(path: &str, r: &Receipt) -> anyhow::Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        r#"
        INSERT OR REPLACE INTO receipts
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        params![
            r.receipt_number,
            r.buyer,
            r.seller,
            r.total_amount,
            serde_json::to_string(&r.items)?,
            r.invoice_date,
            r.source_file
        ],
    )?;

    Ok(())
}

pub fn run_query(path: &str, sql: &str) -> anyhow::Result<()> {
    let conn = duckdb::Connection::open(path)?;
    let mut stmt = conn.prepare(sql)?;

    {    let mut rows = stmt.query([])?;
         // Get first row safely
         let Some(first_row) = rows.next()? else {
             println!("(no rows)");
             return Ok(());
         };
    }

    let col_count = stmt.column_count();

    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        print_row(&row, col_count);
    }

    Ok(())
}

fn print_row(row: &duckdb::Row, col_count: usize) {
    for i in 0..col_count {
        let value = row.get_ref(i).unwrap();

        let s = match value {
            ValueRef::Null => "NULL".to_string(),
            ValueRef::Boolean(v) => v.to_string(),
            ValueRef::TinyInt(v) => v.to_string(),
            ValueRef::SmallInt(v) => v.to_string(),
            ValueRef::Int(v) => v.to_string(),
            ValueRef::BigInt(v) => v.to_string(),
            ValueRef::Float(v) => v.to_string(),
            ValueRef::Double(v) => format!("{:.2}", v),
            ValueRef::Text(v) => String::from_utf8_lossy(v).to_string(),
            ValueRef::Blob(_) => "<BLOB>".into(),
            _ => "<UNSUPPORTED>".into(),
        };

        print!(
            "{}{}",
            s,
            if i + 1 == col_count { "\n" } else { " | " }
        );
    }
}
