import os
import sys
from paddleocr import PaddleOCR
import subprocess
import pprint

def ocr_image(ocr, img_path):
    print(f"\n=== OCR: {img_path} ===")
    result = ocr.ocr(img_path)
    # pprint.pprint(result)

    # PaddleX OCR returns: List[Dict] (one dict per page/image)
    all_text = []
    for page in result:
        texts = page.get("rec_texts", [])
        scores = page.get("rec_scores", [])

        for text, score in zip(texts, scores):
            all_text.append({
                "text": text,
                "score": score
            })

    #pprint.pprint(all_text)
    return all_text

def clean_ocr(ocr_lines, min_score=0.8):
    return [
        line["text"]
        for line in ocr_lines
        if line["score"] >= min_score
    ]

def process_image_file(img_path, ocr):
    ocr_result = ocr_image(ocr, img_path)
    clean_text = clean_ocr(ocr_result, min_score=0.8)
    clean_text_str = "\n".join(clean_text)
    print(f"=== Clean Text: {img_path} ===")
    pprint.pprint(clean_text_str)

def main(path):
    ocr = PaddleOCR(
        lang="ch",
        use_textline_orientation=True
    )

    if os.path.isfile(path) and path.endswith(".png"):
        process_image_file(path, ocr)
    elif os.path.isdir(path):
        for root, _, files in os.walk(path):
            for file_name in files:
                if file_name.lower().endswith('.png'):
                    img_path = os.path.join(root, file_name)
                    process_image_file(img_path, ocr)

if __name__ == "__main__":
    main(sys.argv[1])