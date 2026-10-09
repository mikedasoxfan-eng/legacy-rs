"""Copy a jar with the 'Sealed: true' manifest attribute removed (oracle.sh: LWJGL 2.9.4).
Usage: python unseal.py in.jar out.jar
"""
import sys
import zipfile

src, dst = sys.argv[1], sys.argv[2]
with zipfile.ZipFile(src) as z, zipfile.ZipFile(dst, 'w', zipfile.ZIP_DEFLATED) as o:
    for info in z.infolist():
        data = z.read(info)
        if info.filename == 'META-INF/MANIFEST.MF':
            data = b''.join(l for l in data.splitlines(keepends=True) if not l.startswith(b'Sealed:'))
        o.writestr(info, data)
