"""Class-level change map from b1.7.3 to b1.8.1 (both decompiled with RetroMCP).

Writes tools/b181/changes.json: every b1.8.1 class with its status (new / changed / same),
its line count, how many lines differ from b1.7.3, and the area it belongs to. The progress
script (tools/progress.py) weighs work by these line counts.
Usage: python tools/b181/changemap.py
"""
import difflib
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OLD = os.path.join(ROOT, 'tools/retromcp/minecraft/src/net/minecraft')
NEW = os.path.join(ROOT, 'tools/retromcp-b181/minecraft/src/net/minecraft')

# Out of scope, as for b1.7.3: multiplayer networking, the applet/launcher, crash panels, the
# JSON library and the isometric map viewer.
SKIP = re.compile(r'^(Packet\d*\w*|NetClientHandler|NetHandler|NetworkManager|Network\w+Thread|PacketCounter|'
                  r'GuiMultiplayer|GuiConnecting|GuiConnectFailed|GuiDownloadTerrain|GuiSleepMP|WorldClient|'
                  r'EntityClientPlayerMP|EntityOtherPlayerMP|PlayerControllerMP|SaveHandlerMP|ChunkProviderClient|'
                  r'Thread(ConnectToServer|CloseConnection|CheckHasPaid|RunIsoClient|SleepForever|Download\w*)|'
                  r'J_\w+|EnumJson\w+|CanvasIsomPreview|IsoImageBuffer|ImageBufferDownload|'
                  r'Canvas\w+|Panel\w+|MinecraftApplet\w*|MinecraftAppletImpl|MinecraftImpl|GameWindowListener|'
                  r'UnexpectedThrowable|OsMap|EnumOS\w*|ServerList\w*|ServerNBTStorage|GuiScreenAddServer|'
                  r'GuiSlotServer|ThreadPollServers|GuiScreenServerList|ServerData)$')

AREAS = [
    ('worldgen', r'^(GenLayer\w*|MapGen\w*|WorldGen\w*|BiomeGen\w*|ChunkProvider\w*|NoiseGenerator\w*|'
                 r'WorldChunkManager\w*|ComponentVillage\w*|ComponentStronghold\w*|ComponentMineshaft\w*|'
                 r'Structure\w*|StructureBoundingBox|StructurePieceBlockSelector\w*|BiomeCache\w*|IntCache|'
                 r'SpawnListEntry|StructureStart|StructureComponent|StructureMineshaft\w*|StructureStronghold\w*|'
                 r'StructureVillage\w*|EnumDoor|WorldType\w*|LongHashMap\w*|ChunkCoordIntPair|Village\w*)$'),
    ('player', r'^(EntityPlayer\w*|FoodStats|PlayerController\w*|InventoryPlayer|PlayerCapabilities|'
               r'MovementInput\w*|EnumGameType|WorldSettings|EnumStatus|ItemInWorldManager)$'),
    ('gui', r'^(Gui\w*|GuiSlot\w*|ScaledResolution|FontRenderer|ContainerCreative|SlotCreative\w*|'
            r'GameSettings|EnumOptions\w*|KeyBinding|StatCollector|StringTranslate|LoadingScreenRenderer|'
            r'ChatLine|ChatAllowedCharacters|Achievement\w*|Stat\w*|TexturePack\w*|RenderEngine|'
            r'Container\w*|Slot\w*|Inventory\w*|Craft\w*|Recipe\w*|Furnace\w*|ShapedRecipes|'
            r'ShapelessRecipes|IRecipe|ICrafting|IInventory|IInvBasic)$'),
    ('rendering', r'^(Render\w*|Model\w*|Entity\w*FX|EffectRenderer|EntityRenderer|Tessellator|'
                  r'WorldRenderer|RenderGlobal|RenderBlocks|RenderHelper|ItemRenderer|TileEntity\w*Renderer|'
                  r'TextureFX|Texture\w+FX|ColorizerFoliage|ColorizerGrass|ColorizerWater|Frustrum|Clipping\w*|'
                  r'ICamera|ActiveRenderInfo|GLAllocation|OpenGlHelper|OpenGlCapsChecker|ThreadDownloadImage\w*|'
                  r'ImageBuffer|EntitySorter|RenderSorter|RenderList|MapItemRenderer|SoundManager|SoundPool\w*|'
                  r'CodecMus|MusInputStream|TerrainTextureManager|ScreenShotHelper|MouseHelper|Timer)$'),
    ('blocks', r'^(Block\w*|Item\w*|Material\w*|StepSound\w*|EnumToolMaterial|EnumArmorMaterial|'
               r'TileEntity\w*|ItemStack|EnumArt|MapData\w*|MapCoord|MapColor|PistonBlockTextures)$'),
    ('entities', r'^(Entity\w*|SpawnerAnimals|PathEntity|PathPoint|Path|Pathfinder|IMob|IAnimals|'
                 r'EnumCreature\w*|EnumMobType|EnumEntitySize|DamageSource\w*|EntityDamageSource\w*|'
                 r'Explosion|DataWatcher|WatchableObject|IProjectile|AxisAlignedBB|Vec3D|MovingObjectPosition|'
                 r'EnumMovingObjectType|ChunkPosition|ChunkCoordinates|PotionEffect|Potion\w*)$'),
]


def area(name):
    for a, pat in AREAS:
        if re.match(pat, name):
            return a
    return 'world'  # World, Chunk, saves, lighting, ticking and everything else


def read(path):
    if not os.path.exists(path):
        return None
    with open(path, encoding='utf-8', errors='replace') as f:
        return [l.rstrip() for l in f]


def main():
    rows = []
    for sub in ('src', 'client'):
        new_dir = os.path.join(NEW, sub)
        for fn in sorted(os.listdir(new_dir)):
            if not fn.endswith('.java'):
                continue
            name = fn[:-5]
            if SKIP.match(name):
                continue
            new = read(os.path.join(new_dir, fn))
            old = read(os.path.join(OLD, sub, fn))
            code = [l for l in new if l.strip()]
            if old is None:
                status, changed = 'new', len(code)
            else:
                sm = difflib.SequenceMatcher(None, old, new, autojunk=False)
                changed = sum(max(j2 - j1, i2 - i1) for tag, i1, i2, j1, j2 in sm.get_opcodes() if tag != 'equal')
                status = 'same' if changed == 0 else 'changed'
            rows.append({'class': name, 'package': sub, 'status': status, 'lines': len(code),
                         'changed_lines': changed, 'area': area(name)})
    removed = []
    for sub in ('src', 'client'):
        for fn in sorted(os.listdir(os.path.join(OLD, sub))):
            if fn.endswith('.java') and not os.path.exists(os.path.join(NEW, sub, fn)) and not SKIP.match(fn[:-5]):
                removed.append(fn[:-5])
    out = os.path.join(ROOT, 'tools/b181/changes.json')
    json.dump({'classes': rows, 'removed': removed}, open(out, 'w'), indent=1)
    from collections import Counter
    c = Counter(r['status'] for r in rows)
    print('classes', len(rows), dict(c), 'removed', len(removed))
    for a in sorted({r['area'] for r in rows}):
        sel = [r for r in rows if r['area'] == a and r['status'] != 'same']
        print(f"{a:10} new {sum(r['status']=='new' for r in sel):3} changed {sum(r['status']=='changed' for r in sel):3} "
              f"work lines {sum(r['lines'] for r in sel):6} changed lines {sum(r['changed_lines'] for r in sel):6}")


main()
