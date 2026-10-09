# Documentation

Start with the [project overview](../README.md), [playing guide](playing.md) or [build and verification guide](development.md).

## Versions and verification

| Document | Covers |
|---|---|
| [Beta 1.8.1](beta-1.8.1.md) | Adventure Update architecture and historical subsystem audits |
| [Minecraft 1.0](minecraft-1.0.md) | Features, layout and release verification history |
| [Minecraft 1.1](minecraft-1.1.md) | New features, resources, data directory and implementation |
| [Minecraft 1.1 verification](minecraft-1.1-verification.md) | Completed class audit, original-code comparisons and scope |
| [Report index](../reports/README.md) | Release checks, retained machine evidence and historical reports |
| [Changelog](../CHANGELOG.md) | Changes by legacy-rs release |

Beta 1.7.3 is the original foundation in `crates/worldgen` and `crates/client`. The subsystem notes below were written for that version. Later versions adapt those systems; consult their own evidence for version-specific behavior.

## Subsystems

| Document | Covers |
|---|---|
| [Containers](containers.md) | Inventory, crafting, chests, furnaces, dispensers and screen contracts |
| [Pistons and tile entities](pistons.md) | Piston behavior, note blocks and tile lifecycle |
| [World effects](effects.md) | Particles, sound, records and ordered events |
| [First-person view](first-person.md) | Camera, FOV, fog, hands, held items and player rendering |
| [Vehicles](vehicles.md) | Minecarts, boats, riding and inventories |
| [Decorations and fishing](decorations.md) | Signs, paintings, maps and fishing |
| [Texture packs, chat and anaglyph](texture-packs-chat-anaglyph.md) | Resource loading, refresh, chat and stereo rendering |
| [Video settings](video-settings.md) | Lighting, distances, frame limits and GUI scale |
| [Weather](weather.md) | Rain, snow, splashes, lightning and sky darkening |

## Source-level notes

- [Entity simulation](../crates/worldgen/src/entity/README.md)
- [Player simulation](../crates/worldgen/src/entity/player/README.md)
- [Container logic](../crates/worldgen/src/container/README.md)
- [Entity rendering](../crates/client/src/render/entity/README.md)

The [report index](../reports/README.md) distinguishes historical audits from the latest release checks. Generated class ledgers and original oracle scripts remain available for reproducibility; the project README no longer carries development progress tables.
