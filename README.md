# Minecraft Dev for OpenIDEA

A clone of the essential features of
[MinecraftDev](https://github.com/minecraft-dev/MinecraftDev) (the IntelliJ plugin
for Minecraft modding) for [OpenIDEA](https://github.com/l0ng-ai/openidea),
adapted to its plugin API (a WebAssembly component + `plugin.toml`).

The full analysis (why Kotlin → Rust, what can be ported and what cannot) is in
[`ANALYSIS.md`](ANALYSIS.md).

## What it does

- **Platform detection**: open your mod project and run
  `Minecraft Dev: Detect Platform`. It reads `build.gradle(.kts)`,
  `gradle.properties`, `fabric.mod.json`, `mods.toml`, `plugin.yml`,
  `sponge_plugins.json`, … and reports platform, mod id, Minecraft version,
  mappings and whether mixins are present.
- **Smart templates**: the insert commands use what was detected to fill
  `{{MOD_ID}}`, `{{MC_VERSION}}`, etc.
- **Minecraft file types**: `*.mcmeta` files are treated as JSON and
  `.accesswidener` as text with `#` comments.
- **"Grass Block" theme** (grass green): the package also follows the
  `templates/theme-plugin` shape for its data parts.

Platforms: Fabric, Quilt, Forge, NeoForge, Architectury, Bukkit, Spigot, Paper,
Sponge, Velocity, BungeeCord, Waterfall and Mixin.

## Install

1. Build: `./build.sh` (needs `rustup target add wasm32-wasip2`).
2. In OpenIDEA: **Settings › Plugins › gear menu › Install Plugin from Disk…** and
   pick this folder (the one containing `plugin.toml`).
3. Commands appear in **Search Everywhere › Actions** under the
   *Minecraft Dev* category, and can be bound in Settings › Keymap.

To iterate: copy (or symlink) this folder into `<project>/.openidea/plugins/` and
press **Reload** in Settings › Plugins — no reinstall needed.

## Commands

Every command accepts an optional argument (from the action palette, e.g.
`executeAction("plugin.minecraft-dev.<id>", "<arg>")`) to force the platform,
for example `fabric`.

| Command | What it inserts |
| --- | --- |
| `Detect Platform` | (nothing) the detection report |
| `Insert Main Class` | the platform's main class |
| `Insert build.gradle` | the platform's build script |
| `Insert gradle.properties` | version/mappings properties |
| `Insert Mod Metadata` | fabric.mod.json / mods.toml / neoforge.mods.toml / plugin.yml / sponge_plugins.json |
| `Insert Mixins Config` | `*.mixins.json` |
| `Insert pack.mcmeta` | correct `pack_format` for the detected MC version |
| `Insert Asset JSON` | `block-model`, `item-model`, `blockstate`, `lang`, `sounds` (inferred from the open file) |
| `Insert Registry Code` | Forge/NeoForge/Fabric (blocks and items) |
| `Insert Event Handler` | the platform's event listener |
| `Insert Command Class` | the platform's command class |
| `Insert Mixin Class` | `@Mixin` skeleton |

## Permissions

`project-read` (read build files), `editor-read` (active file), `editor-write`
(insert). Nothing else.

## Licenses

Apache-2.0 (new code). Templates derived from MinecraftDev (LGPL-3.0): see
[`NOTICE.md`](NOTICE.md).
