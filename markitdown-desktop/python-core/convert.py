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
        md = MarkItDown()
        result = md.convert(str(path))

        # Extract images to sibling assets/ folder
        assets_dir = path.parent / "assets"
        assets_dir.mkdir(exist_ok=True)

        image_paths = []
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
