import argparse
import json
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path
from typing import cast

import pymupdf

arg = argparse.ArgumentParser("uv run pdf_extractor.py")

arg.add_argument("--spawn", action="store_true")
arg.add_argument("--loop", action="store_true")
arg.add_argument("--files", nargs="*", help="target_files")

args = arg.parse_args()


def _process_single_pdf(path: Path) -> None | tuple[str, str]:
    if not Path(path).exists():
        return None

    try:
        doc = pymupdf.open(path)
    except Exception:  # noqa: BLE001
        return None

    try:
        # 'with' garante o fechamento do arquivo e liberação de RAM por PDF
        with pymupdf.open(path) as doc:
            text = ""
            for page in doc:
                try:
                    text += " " + cast(str, page.get_text())
                except Exception:  # noqa: BLE001, S112
                    continue
    except Exception:  # noqa: BLE001
        return None

    return str(path.absolute()), text.strip()


def extract_content_from_pdf_files() -> dict[str, str]:
    paths_as_path = [f.absolute() for f in map(Path, args.files or [])]

    mode = "loop" if args.loop else "multi-process"

    content_by_file = {}
    if mode == "multi-process":
        with ProcessPoolExecutor() as executor:
            # Benefits of using process spawnned tasks:
            # Reduction from 8seconds to 1.6seconds in a payload of 936 pdf (8pages each one) example
            result = executor.map(_process_single_pdf, paths_as_path)
            filtered_result = filter(None, result)
            content_by_file = dict(list(filtered_result))
    if mode == "loop":

        for file in paths_as_path:
            result = _process_single_pdf(file)
            if result:
                path, content = result
                content_by_file[str(path)] = content

    # this gonna be read by rust
    print(json.dumps(content_by_file, ensure_ascii=False))
    return content_by_file


if __name__ == "__main__":
    extract_content_from_pdf_files()
