import fitz  # PyMuPDF
from PIL import Image
import os
import sys
import argparse

def pdf_to_png(pdf_path, output_folder, dpi=300):
    # Open the PDF file using PyMuPDF
    doc = fitz.open(pdf_path)

    # Ensure the output folder exists
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)

    base_name = os.path.splitext(os.path.basename(pdf_path))[0]  # Get the base name of the PDF
    zoom = dpi / 72.0
    mat = fitz.Matrix(zoom, zoom)

    # Iterate through each page and save it as a PNG image
    for page_num in range(len(doc)):
        page = doc.load_page(page_num)  # Load a page by index
        pix = page.get_pixmap(matrix=mat, alpha=False)
        img = Image.frombytes("RGB", [pix.width, pix.height], pix.samples)

        # Construct the output file name based on the base name and page number
        if len(doc) > 1:
            output_path = os.path.join(output_folder, f"{base_name}_{page_num:02d}.png")
        else:
            output_path = os.path.join(output_folder, f"{base_name}.png")

        img.save(output_path)
        print(f"Saved page {page_num + 1} as {output_path}")

def main():
    parser = argparse.ArgumentParser(description="Convert a PDF file or directory of PDFs to PNG images.")
    parser.add_argument("input_path", help="Path to the input PDF file or folder")
    parser.add_argument(
        "--output_folder",
        default=None,
        help="Output folder for the PNG images (default: same as input folder if it's a folder, otherwise current working directory)",
    )

    args = parser.parse_args()

    # Determine output folder
    if os.path.isdir(args.input_path):
        if args.output_folder is None:
            args.output_folder = args.input_path
    else:
        if args.output_folder is None:
            args.output_folder = os.getcwd()

    for file in os.listdir(args.input_path):
        if file.endswith(".pdf"):
            pdf_path = os.path.join(args.input_path, file)

            # Create the output subfolder to maintain the structure
            output_subfolder = os.path.join(args.output_folder, os.path.dirname(file))
            if not os.path.exists(output_subfolder):
                os.makedirs(output_subfolder)

            pdf_to_png(pdf_path, output_subfolder)

if __name__ == "__main__":
    main()