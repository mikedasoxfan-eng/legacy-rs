"""Generate test assets from the user's jar; no Mojang assets are committed."""
from pathlib import Path
from io import BytesIO
import os, sys, zipfile
from PIL import Image
folder = Path(sys.argv[1]) / "texturepacks"
folder.mkdir(parents=True, exist_ok=True)
jar = Path(os.environ.get("BETA_JAR", str(Path(os.environ["APPDATA"]) / ".minecraft/versions/b1.8.1/b1.8.1.jar")))
with zipfile.ZipFile(jar) as source, zipfile.ZipFile(folder / "Oracle.zip", "w", zipfile.ZIP_DEFLATED) as pack:
    pack.writestr("pack.txt", "A generated oracle texture pack, truncated here\r\nSecond description line\r\nIgnored third line")
    for name in ["pack.png", "gui/gui.png", "gui/background.png", "gui/icons.png", "gui/items.png", "terrain.png", "title/mclogo.png"]:
        image = Image.open(BytesIO(source.read(name))).convert("RGBA")
        r, g, b, a = image.split()
        image = Image.merge("RGBA", (b, r, g, a))
        buf = BytesIO(); image.save(buf, format="PNG"); pack.writestr(name, buf.getvalue())
    pack.writestr("oracle-resource.txt", b"pack resource override")
(folder / "Broken.zip").write_bytes(b"vanilla keeps malformed zip entries")
