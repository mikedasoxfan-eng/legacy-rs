"""Inject only Entity's Random seed seam; restoration assertion proves byte preservation."""
import ast, pathlib

root = pathlib.Path(__file__).resolve().parents[2]
source = root / "tools/oracle11/build_entity.py"
tree = ast.parse(source.read_text())
patch = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "patch")
build = root / "target/r11/eggstate-build"
(build / "net/minecraft/src").mkdir(parents=True, exist_ok=True)
namespace = {"jar": root.parent / "retromcp-1.1/minecraft/jars/deobfuscated.jar", "build": build}
exec("import struct,zipfile", namespace)
exec(compile(ast.Module(body=[patch], type_ignores=[]), str(source), "exec"), namespace)
namespace["patch"]("Entity", "newEntityRandom")
