"""Expose unchanged LWJGL class metadata from a directory, avoiding its sealed package manifest.

The recorder is first on the classpath. No native library is loaded by the harnesses.
"""
import pathlib
import sys
import zipfile

destination = pathlib.Path(sys.argv[2])
with zipfile.ZipFile(sys.argv[1]) as archive:
    for name in archive.namelist():
        relative = pathlib.PurePosixPath(name)
        if not name.endswith(".class") or relative.is_absolute() or ".." in relative.parts:
            continue
        output = destination.joinpath(*relative.parts)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_bytes(archive.read(name))
