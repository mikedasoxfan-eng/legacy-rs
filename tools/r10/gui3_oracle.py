"""Run GUI round-three fixtures against original bytecode and an existing oracle base.

No base harness files are copied or modified. Generated instrumentation lives in target/.
"""
import os
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[2]
local_base = root / 'tools/oracle10'
default_base = local_base
base = Path(os.environ.get('ORACLE10_BASE', default_base)).resolve()
refs = base.parent / 'retromcp-r10'
jar = refs / 'minecraft/jars/deobfuscated.jar'
build = root / 'target/oracle10-gui3'
build.mkdir(parents=True, exist_ok=True)
if not (base / 'build/net/minecraft/src/Oracle.class').exists():
    sys.exit(f'Build the existing oracle base first: {base}/oracle.sh')
java_candidates = [Path(os.environ.get('APPDATA', '')) / '.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe']
java_candidates += list((Path(os.environ.get('LOCALAPPDATA', '')) / 'Packages').glob('Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe'))
java = os.environ.get('ORACLE_JAVA') or next((str(p) for p in java_candidates if p.exists()), 'java')
version = subprocess.run([java, '-version'], capture_output=True, text=True)
if '1.8.0_51' not in version.stderr:
    sys.exit('GUI oracle requires Mojang Java 8u51.')
libs = [str(p) for p in (refs / 'libraries').rglob('*.jar') if 'sources' not in p.name and 'natives' not in p.name and not p.name.startswith('lwjgl-2')]
libs.append(str(base / 'build/lwjgl-unsealed.jar'))
# Original jar precedes base build: even a patched GUI class in the base cannot shadow it.
cp = ';'.join([str(build), str(base / 'build/input'), str(jar), str(base / 'build'), *libs])
stub = (base / 'headless/org/lwjgl/opengl/GL11.java').read_text()
needle = 'public static void glCallLists(IntBuffer list){'
assert stub.count(needle) == 1
stub = stub.replace('public final class GL11 {', 'public final class GL11 {\n    public static final java.util.ArrayList<Integer> gui2Lists = new java.util.ArrayList<Integer>();')
stub = stub.replace(needle, needle + 'for(int i=list.position();i<list.limit();i++)gui2Lists.add(list.get(i));')
for name, method in {
    'glOrtho': 'public static void glOrtho(double l,double r,double b,double t,double n,double f) {}',
    'glClear': 'public static void glClear(int mask) {}',
    'glViewport': 'public static void glViewport(int x,int y,int w,int h) {}',
    'glCopyTexSubImage2D': 'public static void glCopyTexSubImage2D(int target,int level,int x,int y,int sx,int sy,int w,int h) {}',
}.items():
    if ('void ' + name + '(') not in stub:
        stub = stub.replace('public final class GL11 {', 'public final class GL11 {\n    ' + method)
# Only this fixture's GL stub is instrumented; original game classes are first on the classpath.
stub = stub.replace('public final class GL11 {', 'public final class GL11 {\n    public static final java.util.ArrayList<float[]> gui3Ops = new java.util.ArrayList<float[]>();\n    public static final java.util.ArrayList<int[]> gui3Viewports = new java.util.ArrayList<int[]>();\n    public static final java.util.ArrayList<int[]> gui3Uploads = new java.util.ArrayList<int[]>();\n    public static final java.util.ArrayList<byte[]> gui3Bytes = new java.util.ArrayList<byte[]>();')
stub = stub.replace('public static void glViewport(int x,int y,int w,int h) {}', 'public static void glViewport(int x,int y,int w,int h) {gui3Viewports.add(new int[]{x,y,w,h});}')
stub = stub.replace('FirstPersonOp(0,x,y,z,0);', 'gui3Ops.add(new float[]{matrixMode,0,x,y,z,0});FirstPersonOp(0,x,y,z,0);')
stub = stub.replace('FirstPersonOp(1,x,y,z,0);', 'gui3Ops.add(new float[]{matrixMode,1,x,y,z,0});FirstPersonOp(1,x,y,z,0);')
stub = stub.replace('FirstPersonOp(2,a,x,y,z);', 'gui3Ops.add(new float[]{matrixMode,2,a,x,y,z});FirstPersonOp(2,a,x,y,z);')
stub = stub.replace('uploaded=new byte[bytes.remaining()];bytes.duplicate().get(uploaded);}', 'uploaded=new byte[bytes.remaining()];bytes.duplicate().get(uploaded);}')
needle = 'public static void glTexSubImage2D(int target,int level,int x,int y,int width,int height,int format,int type,ByteBuffer bytes){'
stub = stub.replace(needle, needle+'gui3Uploads.add(new int[]{texture,level,x,y,width,height});byte[] copy=new byte[bytes.remaining()];bytes.duplicate().get(copy);gui3Bytes.add(copy);')
stub_path = build / 'generated/org/lwjgl/opengl/GL11.java'
stub_path.parent.mkdir(parents=True, exist_ok=True)
stub_path.write_text(stub)
sources = sorted((root / 'tools/oracle10/gui2').glob('*.java'))  # kept out of src: they need the instrumented GL11 above
sources += sorted((root / 'tools/oracle10/gui3').glob('*.java'))
sources += [stub_path]
subprocess.run(['javac', '-encoding', 'UTF-8', '-nowarn', '-cp', cp, '-d', str(build), *map(str, sources)], check=True)
mode, output = sys.argv[1:3]
output = str(Path(output).resolve())
Path(output).parent.mkdir(parents=True, exist_ok=True)
subprocess.run([java, '-Djava.awt.headless=true', '-cp', cp, 'net.minecraft.src.Mode_' + mode, output], cwd=build, check=True)
