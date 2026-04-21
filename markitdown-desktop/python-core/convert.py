#!/usr/bin/env python3
"""MarkItDown Sidecar - Document to Markdown converter.

Usage:
    python convert.py convert <file_path>
    python convert.py batch <file_path> [<file_path> ...]
    python convert.py info
"""

import argparse
import io
import json
import os
import sys
import traceback
from pathlib import Path

# Force UTF-8 on stdout to prevent encoding issues (especially on Windows)
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

from markitdown import MarkItDown


def make_response(success: bool, data=None, error: str | None = None) -> str:
    """Create a standardized JSON response."""
    resp = {"success": success}
    if data is not None:
        resp["data"] = data
    if error is not None:
        resp["error"] = error
    return json.dumps(resp, ensure_ascii=False)


def convert_single(file_path: str) -> str:
    """Convert a single file to Markdown."""
    path = Path(file_path)
    if not path.exists():
        return make_response(False, error=f"File not found: {file_path}")
    if not path.is_file():
        return make_response(False, error=f"Not a file: {file_path}")

    try:
        image_paths = []

        suffix = path.suffix.lower()
        if suffix == ".pdf":
            markdown_content = convert_pdf_with_tables(path)
        elif suffix in (".docx", ".doc"):
            markdown_content = convert_docx_with_tables(path)
        else:
            md = MarkItDown()
            result = md.convert(str(path))
            markdown_content = result.text_content or ""

        output_path = path.with_suffix(".md")
        output_path.write_text(markdown_content, encoding="utf-8")

        file_size = path.stat().st_size

        data = {
            "filename": path.name,
            "source_path": str(path.resolve()),
            "output_path": str(output_path.resolve()),
            "markdown_content": markdown_content,
            "image_paths": image_paths,
            "file_size": file_size,
            "status": "completed",
        }
        return make_response(True, data=data)
    except Exception as e:
        return make_response(
            False,
            error=f"Conversion failed for {path.name}: {str(e)}",
        )


def convert_docx_with_tables(path: Path) -> str:
    """Convert DOCX to markdown preserving table structure using python-docx.
    Falls back to markitdown if python-docx is not available."""
    try:
        from docx import Document
    except ImportError:
        md = MarkItDown()
        result = md.convert(str(path))
        return result.text_content or ""

    doc = Document(str(path))
    parts = []

    for element in doc.element.body:
        tag = element.tag.split("}")[-1] if "}" in element.tag else element.tag

        if tag == "p":
            from docx.text.paragraph import Paragraph
            para = Paragraph(element, doc)
            text = para.text.strip()
            if text:
                style_name = (para.style.name or "").lower() if para.style else ""
                if "heading 1" in style_name:
                    parts.append(f"# {text}")
                elif "heading 2" in style_name:
                    parts.append(f"## {text}")
                elif "heading 3" in style_name:
                    parts.append(f"### {text}")
                elif "heading 4" in style_name:
                    parts.append(f"#### {text}")
                else:
                    parts.append(text)

        elif tag == "tbl":
            from docx.table import Table
            table = Table(element, doc)
            rows_data = []
            for row in table.rows:
                row_cells = [cell.text.strip().replace("\n", " ") for cell in row.cells]
                rows_data.append(row_cells)
            if rows_data:
                gfm = table_to_gfm(rows_data)
                if gfm:
                    parts.append(gfm)

    return "\n\n".join(parts)


def convert_pdf_with_tables(path: Path) -> str:
    """Convert PDF to markdown using pdfplumber for table structure.
    Falls back to markitdown if pdfplumber is not available."""
    try:
        import pdfplumber
    except ImportError:
        md = MarkItDown()
        result = md.convert(str(path))
        return result.text_content or ""

    parts = []

    with pdfplumber.open(str(path)) as pdf:
        for page in pdf.pages:
            tables = page.find_tables()

            if not tables:
                text = page.extract_text()
                if text:
                    parts.append(text)
                continue

            # Sort tables by vertical position
            tables.sort(key=lambda t: t.bbox[1])

            prev_bottom = 0
            for table in tables:
                top = table.bbox[1]
                bottom = table.bbox[3]

                # Text above this table
                if top > prev_bottom + 1:
                    region = page.crop((0, prev_bottom, page.width, top))
                    text = region.extract_text()
                    if text and text.strip():
                        parts.append(text.strip())

                # Table as GFM
                rows = table.extract()
                if rows:
                    gfm = table_to_gfm(rows)
                    if gfm:
                        parts.append(gfm)

                prev_bottom = bottom

            # Text after last table
            if prev_bottom < page.height - 1:
                region = page.crop((0, prev_bottom, page.width, page.height))
                text = region.extract_text()
                if text and text.strip():
                    parts.append(text.strip())

    return "\n\n".join(parts)


def table_to_gfm(rows: list) -> str:
    """Convert extracted table rows to GFM markdown."""
    if not rows:
        return ""

    num_cols = max(len(row) for row in rows)
    if num_cols == 0:
        return ""

    lines = []
    for i, row in enumerate(rows):
        cells = list(row) + [None] * (num_cols - len(row))
        cell_strs = [
            str(cell or "").replace("|", "\\|").replace("\n", " ").strip()
            for cell in cells
        ]
        lines.append("| " + " | ".join(cell_strs) + " |")

        if i == 0:
            lines.append("| " + " | ".join(["---"] * num_cols) + " |")

    return "\n".join(lines)


def batch_convert(file_paths: list[str]) -> str:
    """Convert multiple files, skipping corrupted ones."""
    results = []
    for fp in file_paths:
        result_json = convert_single(fp)
        results.append(json.loads(result_json))
    return make_response(True, data={"results": results})


def get_info() -> str:
    """Return supported formats."""
    formats = [
        ".pdf", ".docx", ".xlsx", ".pptx", ".html", ".htm",
        ".csv", ".json", ".xml", ".zip", ".txt", ".md",
        ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff",
        ".wav", ".mp3",
    ]
    return make_response(True, data={"supported_formats": formats})


def main():
    parser = argparse.ArgumentParser(description="MarkItDown Sidecar")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # convert subcommand
    convert_parser = subparsers.add_parser("convert", help="Convert a single file")
    convert_parser.add_argument("file", help="Path to the file to convert")

    # batch subcommand
    batch_parser = subparsers.add_parser("batch", help="Batch convert multiple files")
    batch_parser.add_argument("paths", nargs="+", help="Paths to files to convert")

    # info subcommand
    subparsers.add_parser("info", help="Get supported formats")

    args = parser.parse_args()

    if args.command == "convert":
        print(convert_single(args.file))
    elif args.command == "batch":
        print(batch_convert(args.paths))
    elif args.command == "info":
        print(get_info())


if __name__ == "__main__":
    main()
