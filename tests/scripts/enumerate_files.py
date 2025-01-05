from typing import Iterable
from pathlib import Path
import os


def enumerate_files(path: Path, suffix: str | None = None) -> Iterable[Path]:
    if os.path.isfile(path):
        if suffix is None or path.suffix == suffix:
            yield path
    else:
        for child in map(Path, sorted(os.listdir(path))):
            yield from enumerate_files(Path(os.path.join(path, child)), suffix=suffix)
