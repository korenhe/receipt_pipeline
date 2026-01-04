# Receipt Recognition Pipeline (Rust)

## Overview

This project implements a **Rust-based pipeline framework** for automated
receipt recognition and information extraction.

The system processes receipt PDFs end-to-end: converting documents into images,
performing OCR using a GPU-accelerated engine, extracting structured information
via an LLM service, and persisting the results into an analytical database.

The design emphasizes **pipeline composition**, **clear stage boundaries**, and
**practical integration of heterogeneous systems** (GPU OCR, LLM inference,
and SQL storage).

---

## Functionality

The pipeline consists of the following stages:

1. **Input**
   - Accepts a folder containing receipt PDF files.

2. **PDF to Image Conversion**
   - Each PDF is converted into one or more PNG images for downstream processing.

3. **OCR (Optical Character Recognition)**
   - Text is extracted from images using **PaddleOCR with GPU acceleration**.

4. **Information Extraction**
   - The recognized text is sent to an **Ollama** server.
   - Ollama converts unstructured OCR output into structured receipt data
     (e.g. merchant, date, total amount, line items).

5. **Storage**
   - Structured data is stored in a **DuckDB** database.
   - Results are written into a `receipts` table for further analysis or querying.

---

## Target

The primary goals of this project are:

- To explore a **modular Rust pipeline architecture** for real-world data
  processing tasks.
- To integrate **GPU-accelerated OCR** and **LLM-based information extraction**
  in a reproducible workflow.
- To provide a foundation for scalable receipt ingestion and analytics.
- To serve as a learning and experimentation platform for systems programming,
  data engineering, and AI-assisted extraction.

---

## Expected Result

Given a directory of receipt PDFs, the system is expected to:

- Automatically process all PDFs without manual intervention.
- Extract readable text from receipt images using OCR.
- Transform raw OCR text into **structured receipt records**.
- Persist those records into DuckDB for querying and downstream use.

Example outcome:

- A populated `receipts` table containing normalized receipt data
  derived from heterogeneous PDF inputs.

- Stored receipt info (example):

| receipt_number  | buyer     | seller     | total_amount | items     | invoice_date   | source_file |
|-----------------|-----------|------------|--------------|-----------|----------------|-------------|
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 178.98       | `<ITEMS>` | 2026年01月02日 | NULL        |
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 184.00       | `<ITEMS>` | 2026年01月02日 | NULL        |
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 1798.00      | `<ITEMS>` | 2026年01月03日 | NULL        |
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 225.12       | `<ITEMS>` | 2026年01月02日 | NULL        |
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 5999.00      | `<ITEMS>` | 2026年01月02日 | NULL        |
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 7500.00      | `<ITEMS>` | 2026年01月03日 | NULL        |
| `<RECEIPT NO.>` | `<BUYER>` | `<SELLER>` | 1080.00      | `<ITEMS>` | 2025年12月31日 | NULL        |
---

## Subcommands

The application provides a command-line interface with two primary subcommands
for ingesting receipt data and querying stored results.

### SUBCOMMAND: scan

Scans a directory of receipt documents and runs the full processing pipeline.

USAGE:
app scan --input-dir <RECEIPT_DIRECTORY>

DETAILS:
- Accepts a directory containing receipt PDF files.
- Converts PDFs into images for processing.
- Performs OCR using the configured OCR backend.
- Extracts structured information via the LLM service.
- Persists the extracted data into the receipt database.

### SUBCOMMAND: query

Queries stored receipt data using SQL.

USAGE:
app query --db <DATABASE_FILE> --sql "<SQL_QUERY>"

DETAILS:
- Executes a user-provided SQL query against the receipt database.
- Outputs query results to standard output.
- Useful for inspection, debugging, and analysis of stored receipt records.


## Current Stage & Limitations

This project is currently in an **early / draft framework stage**.

Known limitations and requirements:

- **CUDA 13.0 is required**
  - PaddleOCR is configured to use GPU acceleration and currently targets
    CUDA 13.0.
  - The runtime environment must support NVIDIA GPUs and the appropriate
    CUDA runtime.
  - CPU-only execution may be supported for experimental or testing purposes.

- **Ollama server must be running**
  - An external Ollama service must be established.
  - The Ollama API port must be exposed and reachable by the pipeline.

- **Limited error handling**
  - Failure recovery, retries, and partial pipeline execution are minimal.

- **Schema and prompts are evolving**
  - The structure of extracted receipt data and LLM prompts are subject to change.

- **Not production-ready**
  - Performance tuning, security hardening, and large-scale ingestion
    have not yet been addressed.

---

## Status

🚧 **Work in Progress**

This repository currently focuses on establishing the core pipeline
and validating integration between OCR, LLM extraction, and storage.
Future work will refine robustness, configurability, and extensibility.
