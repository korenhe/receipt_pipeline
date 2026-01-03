use std::path::{Path, PathBuf};
use std::process::Command;

pub fn pdf_to_png(pdf: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let output = Command::new("python")
        .arg("/app/script/pdf_png.py")
        .arg(pdf)
        .output()?;

    println!("pdf folder is {}", pdf.display());
    if !output.status.success() {
        anyhow::bail!("PDF to PNG failed");
    }

    let mut pngs = Vec::new();

    for entry in std::fs::read_dir(pdf)? {
        let p = entry?.path();
        if p.extension().and_then(|s| s.to_str()) == Some("png") {
            pngs.push(p);
        }
    }

    Ok(pngs)
}

pub fn run_ocr(png: &Path) -> anyhow::Result<String> {
    let output = Command::new("python")
        .arg("/app/script/test_ocr.py")
        .arg(png)
        .output()?;

    if !output.status.success() {
        anyhow::bail!("OCR failed");
    }

    Ok(String::from_utf8(output.stdout)?)
}
