# Analysis: a MinecraftDev clone for OpenIDEA

**Short answer:** MinecraftDev is written in **Kotlin/JVM** (an IntelliJ Platform
plugin); OpenIDEA is rebuilt in **Rust** (Tauri + TypeScript frontend) and its code
plugins are **WebAssembly components**. The clone is therefore written in **Rust**
(compiled to `wasm32-wasip2`) — not Python, not Java.

This document explains why, what has been ported, what cannot be ported yet, and
what to do next.

---

## 1. The two repositories

### MinecraftDev (minecraft-dev/MinecraftDev)

| Aspect | Value |
| --- | --- |
| Language | Kotlin (JVM), some Java for file templates; Python only for release scripts |
| Build | Gradle + `gradle-intellij-plugin`; output is a `.zip` for the JetBrains plugin repository |
| Target platform | IntelliJ Platform (IDEA 2025.3–2026.3) |
| License | LGPL-3.0-only |
| Size | ~3,100 commits; ~100 platform/subsystem packages under `src/main/kotlin` |

**Main subsystems** (as found in `src/main/kotlin`):

| Package | What it does |
| --- | --- |
| `creator/` | The "New Minecraft Project" wizard: platforms, Maven/Gradle, Parchment, generated files from `resources/fileTemplates` |
| `platform/*` | Per-platform facets (bukkit, forge, fabric, neoforge, sponge, velocity, bungeecord, architectury, mixin, mcp, adventure): module detection, frameworks, version fetching |
| `asset/` | IDE assets (icons) and semi-structural support for mod resources |
| `facet/` | Automatic detection of a Minecraft project from its build files |
| `translations/` | A custom language for `.lang` files (PSI, colors, formatting, spellchecking, sorting) |
| `platform/mcp/*` | Access Transformers (AT), SRG/MCP, related inspections |
| `inspection/`, `insight/` | Inspections, quick-fixes, intentions, code generation |
| `update/` | Checks for new versions (network) |

None of that can compile against OpenIDEA: MinecraftDev uses **PSI** (IntelliJ's
code index), IntelliJ's UI wizard API, the Gradle tooling API and the inspection
engine. It is not "a Java plugin"; it is a first-class citizen inside the IDE's JVM.

### OpenIDEA (l0ng-ai/openidea)

| Aspect | Value |
| --- | --- |
| Language | Rust (core: `crates/`, `src-tauri/`) + TypeScript/React (frontend `src/`) |
| Plugins | Sandboxed **WebAssembly components** (wasmtime, component model, WASI preview 2), declared in `plugin.toml` |
| Plugin API | WIT interface `openidea:plugin/plugin@0.1.0` (`wit/plugin.wit`) |
| Plugin SDK | Rust + `wit-bindgen` (see `plugins/hello`, `plugins/language-toy`, `plugins/lombok`, …) |
| Contribution types | commands, themes (JSON), languages (TOML) — the last two need no code |
| License | Apache-2.0 |

**The template you pointed at (`templates/theme-plugin`) is data-only and
theme-specific**: it is a package of JSON colors (`plugin.toml` + `themes/*.theme.json`)
with *no* code and nothing to compile. A plugin that ships code (like this clone)
follows the same package conventions (`plugin.toml` + relative assets) but adds
`wasm = "dist/plugin.wasm"` and `permissions = [...]` — exactly what
`plugins/README.md` documents.

---

## 2. The OpenIDEA plugin API (and its limits)

Host functions (`wit/plugin.wit`, all verified against the source):

| Host function | Permission | Rough IntelliJ/MinecraftDev equivalent |
| --- | --- | --- |
| `log`, `register-command`, `show-notification`, `register-language`, `register-theme` | always | ~ an action + notifications |
| `workspace-root()` | `project-read` | ~ facet / project detection |
| `read-file(path)` | `project-read` | ~ reading `build.gradle`, `gradle.properties`, `fabric.mod.json`, … |
| `active-file()` | `editor-read` | path of the focused file |
| `insert-text`, `replace-selection` | `editor-write` | **only** insert into the active editor |

**Hard sandbox limits (by design):**

1. **No filesystem access.** The "New Minecraft Project" wizard (creating ~30
   files) is impossible today; this plugin inserts templates into the file the
   user has open.
2. **No PSI / analysis.** No inspections, completions, intentions, quick-fixes,
   navigation or Find Usages.
3. **No processes / subprocess.** Cannot run `./gradlew` or fetch versions
   (network is forbidden: no version-fetching at all).
4. **No plugin UI.** Notifications + Search Everywhere + keymap only.

---

## 3. Porting map (what each piece does here)

| MinecraftDev feature | Here | OpenIDEA mechanism |
| --- | --- | --- |
| Facet / platform detection | `src/detect.rs` | reads `build.gradle(.kts)`, `settings.gradle(.kts)`, `gradle.properties`, `fabric.mod.json`, `quilt.mod.json`, `META-INF/mods.toml`, `neoforge.mods.toml`, `sponge_plugins.json`, `plugin.yml`, `paper-plugin.yml`, `velocity.toml`, `bungee.yml` |
| Project wizard → project files | `insert-build-gradle`, `insert-gradle-properties`, `insert-mod-metadata`, `insert-mixins` | commands that insert the content (ported from `resources/fileTemplates/j2ee/*`) into the editor |
| Per-platform main classes | `insert-main` | `templates.rs` (ported from the Forge/NeoForge/Fabric/Quilt/Bukkit/Spigot/Paper/Sponge/Velocity/BungeeCord file templates) |
| `pack.mcmeta` + `pack_format` | `insert-pack-mcmeta` | `versions.rs::pack_format`, a literal port of `ForgePackDescriptor.kt` |
| Assets (models, blockstates, lang, sounds) | `insert-asset` | `assets.rs`: infers the kind from the active path (`blockstates/`, `models/block/`, `lang/`, `sounds.json`) |
| MDK events / commands / registries | `insert-event`, `insert-command`, `insert-registry` | per-platform MDK templates |
| Mixins | `insert-mixin` + `insert-mixins` | `@Mixin` skeleton + mixin configs (package, Java compatibility level) |
| Bukkit live templates | covered by `insert-event` / `insert-command` | — |
| MinecraftDev color/icons | — | no visual identity port (not applicable to OpenIDEA) |
| `.mcmeta` files (`pack.mcmeta`) | `languages/mcmeta.toml` | `*.mcmeta` was not mapped to JSON; now it is |
| Access wideners | `languages/accesswidener.toml` | syntax mapping (`#` comments, extension) |
| Java target per MC version | `versions.rs::java_target` | derived from the MDKs and used to fill templates |
| Theme | `themes/grass-block.*` | `[[contributes.themes]]` (grass green), same JSON shape as `templates/theme-plugin` |

**Detection:** signals go from most to least specific. A typical Fabric project
yields `fabric.mod.json` (mod id, `depends.minecraft`) + `gradle.properties`
(`minecraft_version`, `loader_version`) + `build.gradle` (loom). The notification
reports platform, mod id, Minecraft version, mappings, loader, Java level and
whether mixins are present — and the templates fill themselves (`{{MOD_ID}}`,
`{{MC_VERSION}}`, …).

## 4. What is NOT ported (and what it would take)

| MinecraftDev feature | Blocker | What OpenIDEA's API would need |
| --- | --- | --- |
| Creating the project file tree | no file writes | `write-file(path, content)`, `create-dir` |
| JSON model validation (inspections) | no diagnostics channel | a "publish diagnostic" channel (e.g. via LSP) |
| Registry completions (Forge/Fabric), `@Shadow`, AT | no PSI | a completions API or a per-plugin LSP |
| Forge events / AT / MCP mappings | no processes, no network | an allowed subprocess, or a host command |
| Version tracking (latest fabric/forge/neoforge) | no network in the sandbox | host fetch with cache + confirmation |
| Translation files (`.lang`) | no PSI, no grammar contributions in the port | grammar/tree-sitter contribution — already possible via `register-language` |
| Wizards UI | no webviews | a plugin panel/dialog API |

With those five extensions (file writes, diagnostics, completions, optional
subprocess, simple panel) roughly 90% of what MinecraftDev users touch daily could
be ported.

## 5. Build environment (this machine, no root privileges)

Neither Rust nor a C toolchain was present. What was done:

1. `rustup` (stable toolchain) + target `wasm32-wasip2`.
2. Without `sudo`, the `gcc`/`libc6-dev` `.deb` packages were fetched with
   `apt-get install --print-uris` + `curl`, unpacked into `~/toolchain`
   (`dpkg -x`), and a `~/bin/cc` wrapper was created that adds `LIBRARY_PATH`
   and `C_INCLUDE_PATH` and rewrites the extracted `libc.so` linker scripts
   (they referenced absolute `/usr/lib/...` paths that only exist with the dev
   package installed).
3. `build.sh` (mirroring OpenIDEA's `plugins/build.sh`) compiles with
   `cargo build --release --target wasm32-wasip2` and copies the component to
   `dist/plugin.wasm` (~155 KB).

On any normal machine none of this is needed: with `rustup target add
wasm32-wasip2` and an ordinary `cc`, `./build.sh` just works.

## 6. Delivered package layout

```
OpenDevMC/            # the plugin package itself (installable as-is)
├── plugin.toml       # manifest: id, wasm, permissions, commands, themes, languages
├── Cargo.toml        # Rust crate (cdylib)
├── build.sh          # Rust → WebAssembly
├── wit/plugin.wit    # OpenIDEA plugin API (vendored)
├── src/
│   ├── lib.rs        # command catalog, routing, placeholders
│   ├── detect.rs     # platform detection (the facet counterpart)
│   ├── templates.rs  # templates ported from MinecraftDev
│   ├── versions.rs   # MC version parsing, pack_format, Java target
│   └── assets.rs     # JSON asset templates
├── languages/        # mcmeta.toml, accesswidener.toml
├── themes/           # grass-block.theme.json + .scheme.json
├── dist/plugin.wasm  # built component
├── README.md         # usage
├── NOTICE.md         # licenses and attribution
└── ANALYSIS.md       # this document
```

## 7. Verification performed

- Clean release build (no warnings); `dist/plugin.wasm` checked for component
  framing and the `activate` / `run-command` / `read-file` exports.
- `plugin.toml` parsed as valid TOML (id, version, wasm path, permissions,
  13 commands, 1 theme, 2 languages).
- Both theme JSONs parse (comments stripped; 113 UI tokens + 55 editor-scheme
  tokens; no token overlap between theme and scheme).
- Logic tests in an auxiliary crate:
  - `pack_format` boundaries tested for 20 versions — the implementation matches
    `ForgePackDescriptor.kt` exactly (the initial test table was wrong, not the
    code; e.g. 1.20.0–1.20.1 → 15, 1.20.2 → 18, 1.20.3–1.20.4 → 26, 1.20.5–1.20.6 → 41).
  - `java_target`: 1.16.5 → 8, 1.17.1 → 16, 1.18.2 → 17, 1.20.5+ → 21, 26.1 → 21.
  - `AssetKind::from_path` correctly maps `blockstates/`, `models/block/`,
    `models/item/`, `lang/`, `sounds.json`, and rejects unrelated paths.
  - A real bug was found and fixed: asset templates used `format!` with `{{ }}`
  braces, so placeholders collapsed to `{name}` instead of the value.

## 8. Licenses

New code: Apache-2.0. Templates and data derived from MinecraftDev (contents of
`fileTemplates`, `ForgePackDescriptor`): MinecraftDev is LGPL-3.0; see `NOTICE.md`.
Theme tokens follow OpenIDEA's `tokens.css` (Apache-2.0).
