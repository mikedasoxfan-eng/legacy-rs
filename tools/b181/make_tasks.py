"""Write the Codex worker task files for the b1.8.1 port (minecraft/codex/tasks/b181-*.md)."""
import json
import re
import os

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TASKS = os.path.join(os.path.dirname(ROOT), 'codex', 'tasks')
d = json.load(open(os.path.join(ROOT, 'tools/b181/changes.json')))
rows = {r['class']: r for r in d['classes']}

WORLDGEN_EXTRA = ['BiomeDecorator', 'WeightedRandom', 'WeightedRandomChoice', 'LayerIsland', 'EnumDoorHelper']
WORLD_TO_GUI = ['Minecraft', 'PlayerList', 'PlayerListEntry', 'SorterStatsItem', 'SorterStatsBlock',
                'ThreadStatSyncherReceive', 'ThreadStatSyncherSend', 'IStatType']
RENDER_WORLD = {'RenderBlocks', 'EntityRenderer', 'RenderGlobal', 'WorldRenderer', 'Tessellator', 'TerrainTextureManager',
                'ItemRenderer', 'ScreenShotHelper', 'RenderList', 'MapItemRenderer', 'TextureCompassFX', 'Timer',
                'SoundManager', 'RenderItem'}


def owned(area):
    out = []
    for r in d['classes']:
        if r['status'] == 'same':
            continue
        a, c = r['area'], r['class']
        if c in WORLDGEN_EXTRA:
            a = 'worldgen'
        elif c in WORLD_TO_GUI:
            a = 'gui'
        elif a == 'rendering':
            a = 'render-world' if c in RENDER_WORLD else 'render-entities'
        elif c in ('ChestItemRenderHelper', 'SignModel', 'TexturedQuad'):
            a = 'render-entities'
        if a == 'worldgen' and re.match(r'^(Component\w*|Structure\w*|MapGen(Village|Stronghold|Mineshaft|Structure)|EnumDoor\w*)$', c):
            a = 'structures'
        if a == area:
            out.append(r)
    return sorted(out, key=lambda r: -r['changed_lines'])


COMMON = '''# Task: Minecraft Beta 1.8.1 port, area "{area}"

You are one of 9 parallel workers porting **Minecraft Beta 1.8.1** into beta-rs, a Rust port of
Minecraft Beta 1.7.3 that is **bit-exact** with the original, proven against Mojang's original
bytecode (the "oracle"). b1.7.3 is finished (v1.0.0). b1.8.1 now lives beside it.

## Layout (read first)

- `README.md` (how verification works). b1.7.3 code: `crates/worldgen`, `crates/client`,
  `tools/oracle`, `tools/verify*.sh`. **Never edit any of those** — b1.7.3 must stay exact.
- b1.8.1 code (yours to change): `crates/worldgen181` (simulation; the lib is still named
  `worldgen`, binaries `compare181`, `container_compare181`) and `crates/client181` (the client,
  a library run by `beta.exe --mc b1.8.1`, data folder `b1.8.1/`). Both start as copies of the
  b1.7.3 crates: port them **forward** to b1.8.1, change by change.
- Original b1.8.1 source (decompiled, MCP names): `tools/retromcp-b181/minecraft/src/net/minecraft/src/*.java`.
  b1.7.3 source for diffing: `tools/retromcp/minecraft/src/net/minecraft/src/*.java`. Diff the two
  versions of each class you own (`git diff --no-index --stat` or `diff -u`) and port every change.
- Names differ between the two decompilations: `python tools/b181/renames.py <old_name> ...` prints
  the b1.8.1 name of a b1.7.3 identifier (`tools/b181/renames.json`). `tools/b181/changes.json` is the
  class-level change map.
- Oracle for b1.8.1: `tools/oracle181/` (`oracle.sh <mode> ...`, Java harnesses calling the original
  b1.8.1 classes; it compiles every `src/net/minecraft/src/*.java`). **Add new oracle modes as new
  classes** `Mode_<name>.java` with `public static void main(String[] a)` — `Oracle.java` dispatches
  unknown modes to `Mode_<mode>` by reflection, so you never need to touch the dispatch. Prefer new
  files over editing shared ones.
- Checks: `tools/verify181.sh` (all sections; generated from verify.sh) plus `verify_gui181.sh`,
  `verify_containers181.sh`, `verify_vehicles181.sh`, `verify_decorations181.sh`. They take a lock in
  `target/verify181.lock`. Add your new checks as a delimited block at the END of `verify181.sh`
  (`# --- {area} (b1.8.1) ---` ... `# --- end {area} ---`) so merges stay clean; GUI scenes go to
  the end of `tools/oracle181/gui/scenes.txt` likewise. Run only the sections you need while
  iterating (copy the relevant lines into a scratch script); the full run is ~1h and the integrator
  runs it.
- Many existing verify181 sections fail at first simply because the Rust side is still b1.7.3
  code: the ones touching your classes are yours to turn back to EXACT.
- Build: `cargo build --release -p worldgen181 -p client` (beta.exe contains both versions).
- Java: the oracle uses Mojang's Java 8u51 like b1.7.3 (see oracle.sh).

## Your area: {area}

{goal}

Classes you own (b1.8.1 name: changed-line count, * = new class). "Done" for a class means: its
b1.8.1 behaviour is ported in Rust **and** an oracle check covering it ends EXACT:

{classes}

## Rules

- Port literally; float vs double exactly as Java; keep vanilla bugs. No approximations.
- Other workers own the other areas (worldgen, structures, world, player, entities, blocks, gui, render-world,
  render-entities). If you need something from another area (a new block id, a DamageSource, a
  field on Entity), add the **minimum** you need in the most local place and say so in your
  report; don't rewrite another area's files.
- Commit identity: author `logiu`, email `224190594+mikedasoxfan-eng@users.noreply.github.com`
  (never any other email, in commits or in files). Git is blocked by the sandbox: leave
  `target/commit-b181-{area}.sh` that stages your files and commits with
  `git -c user.name=logiu -c user.email=224190594+mikedasoxfan-eng@users.noreply.github.com commit`
  and the trailers `Co-Authored-By: Codex (gpt-6.1-sol) <noreply@openai.com>` and
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Update it as you finish stages.
- Work fast and in stages; you have about 75 minutes. Biggest/most-visible changes first.
- At the end, write a report: per class, done (with the check that proves it EXACT) or not; the
  exact verify181 sections you made EXACT; what's left.
'''

GOALS = {
    'worldgen': 'The new b1.8.1 terrain: GenLayer biome stack (zoom, islands, rivers, temperature/rainfall, '
                'Voronoi), the new BiomeGenBase (ids, heights, decorators via BiomeDecorator), the new '
                'ChunkProviderGenerate density/surface, MapGenCaves/MapGenRavine, structure generation calls (the structures worker ports villages/strongholds/mineshafts), new trees/features '
                '(big mushrooms, swamp trees, sand/clay discs), Nether changes. Turn the raw terrain, '
                'population/decoration, spawn search and region-file checks in verify181.sh EXACT. Use '
                'block ids for new blocks directly (blocks worker owns block behaviour).',
    'structures': 'Villages, strongholds and mineshafts: MapGenStructure, StructureStart, StructureComponent, '
                  'StructureBoundingBox, every Component*/Structure* piece class (piece weights, random piece '
                  'selection, chest loot via StructurePieceTreasure, stronghold stones selector), as generated '
                  'into chunks by ChunkProviderGenerate (the worldgen worker owns the terrain/biomes; call into '
                  'structure generation the way the original does). Add a structure oracle mode that dumps '
                  'structure starts/pieces/bounding boxes and the generated blocks for several seeds, and make '
                  'it EXACT; then make the raw-terrain checks with structures enabled EXACT together with worldgen.',
    'world': 'World (706 changed lines: lightmap-era light values, weather, new spawn rules, '
             'game type, entity ticking), Chunk (height map/light/new fields), ChunkCache, WorldInfo '
             '(game type, map features, sizeOnDisk...), saves (CompressedStreamTools, RegionFile, ChunkLoader, '
             'save format/converter), Teleporter, MathHelper, NibbleArray, WorldProvider. Turn the save, '
             'lighting, ticking, block-update and teleporter checks in verify181.sh EXACT.',
    'player': 'Hunger (FoodStats: food level, saturation, exhaustion, starvation, regeneration), sprinting '
              '(double-tap forward, sprint speed, FOV), experience (levels, xp bar maths, orb pickup), '
              'creative mode (PlayerControllerTest / PlayerCapabilities: flying, instant break, no damage), '
              'the new EntityPlayer/EntityPlayerSP logic, block-hit and blocking with sword, eating '
              '(item use duration). Extend the player oracle (PlayerOracle / new Mode_ classes) and '
              'make the player checks EXACT.',
    'entities': 'New mobs (Enderman incl. block carrying/teleporting, Cave Spider, Silverfish), XP orbs, '
                'DamageSource / EntityDamageSource / EntityDamageSourceIndirect, PotionEffect/Potion '
                '(as used in b1.8.1), EntityLiving changes (damage, knockback, xp drop, armor), '
                'SpawnerAnimals (new spawn lists), changes in every existing mob, arrows (crits), '
                'fireballs, boats, lightning. Make the entity/spawn checks EXACT and add new ones for '
                'the new mobs.',
    'blocks': 'New blocks/items: vines, stems + melon/pumpkin growth, glass pane + iron bars, fence gate, '
              'stone bricks + silverfish block, huge mushroom caps, brick/stone-brick stairs, web material, '
              'new item behaviours (bow charging, food eating with use duration, dye/bonemeal changes, '
              'doors, buckets...), Block/Item base changes, chests (new chest logic + lid animation '
              'state), dispensers, pistons, fluids. Register everything in the block/item tables. '
              'Make the block tick/behaviour/item checks EXACT and add new ones.',
    'gui': 'GuiIngame (hunger bar, XP bar and level, armor, air, sprint/creative HUD differences), '
           'creative inventory (GuiContainerCreative/ContainerCreative), the new title screen and create '
           'world screen (game mode, map features, seed), achievements/stats changes, '
           'Container/slot click changes, key bindings, options, Minecraft.java main-loop changes '
           '(creative, sprint key handling, debug screen). Pixel-check every changed screen with '
           'verify_gui181.sh scenes (GuiShot on the b1.8.1 jar vs the port).',
    'render-world': 'The lightmap renderer: b1.8.1 renders blocks and entities with a 2D lightmap texture '
                    '(OpenGlHelper multitexture, EntityRenderer.updateLightmap with sky/block light, '
                    'torch flicker, night vision none yet), RenderBlocks (2,409 changed lines: brightness '
                    'via getMixedBrightnessForBlock, new block render types: vines, stems, panes, fence '
                    'gates, huge mushrooms...), Tessellator setBrightness, RenderGlobal (sky, clouds, '
                    'sorting), EntityRenderer (fog, FOV with sprint, camera), ItemRenderer, '
                    'TerrainTextureManager. Make the mesh/renderer/sky checks (rendermesh, showcase, '
                    'skycolor, first person) EXACT against the b1.8.1 oracle.',
    'render-entities': 'Entity renderers and models: Enderman, Silverfish, Cave spider (scaled spider), XP orb '
                       'renderer, chest tile-entity renderer (ModelChest/ModelLargeChest, lid), arrow crits, '
                       'every changed Model*/Render* class (lightmap brightness per entity), the new particles '
                       '(crit, magic crit, cloud, large/huge explosion, aura, suspend, portal changes) and '
                       'EffectRenderer. Make models/renderers/livingrenderers/particles checks EXACT.',
}


def main():
    total = 0
    for area, goal in GOALS.items():
        cls = owned(area)
        total += len(cls)
        lines = []
        for r in cls:
            lines.append(f"{r['class']}{'*' if r['status'] == 'new' else ''}: {r['changed_lines']}")
        text = COMMON.format(area=area, goal=goal, classes=', '.join(lines))
        open(os.path.join(TASKS, f'b181-{area}.md'), 'w', encoding='utf-8').write(text)
        print(area, len(cls), sum(r['changed_lines'] for r in cls))
    print('classes assigned', total, 'of', sum(1 for r in d['classes'] if r['status'] != 'same'))


main()
