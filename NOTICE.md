# NOTICE

## Minecraft Dev for OpenIDEA

Copyright 2026 OpenDevMC contributors

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at

    http://www.apache.org/licenses/LICENSE-2.0

## Third-party components

### Derived content from MinecraftDev (minecraft-dev/MinecraftDev)

Parts of this plugin are ports of content from
[MinecraftDev](https://github.com/minecraft-dev/MinecraftDev), licensed under
**LGPL-3.0-only** (https://www.gnu.org/licenses/lgpl-3.0.html):

- `src/templates.rs` — main classes, build scripts, gradle.properties, mod
  metadata and mixin configs adapted from
  `src/main/resources/fileTemplates/j2ee/**` and
  `src/main/resources/liveTemplates/Bukkit.xml`.
- `src/versions.rs` — the `pack_format` mapping is a port of
  `src/main/kotlin/platform/forge/util/ForgePackDescriptor.kt`.

MinecraftDev is © minecraft-dev contributors. The derived material is redistributed
under the terms of the LGPL-3.0; the source of these upstream files is available at
the repository linked above.

### OpenIDEA plugin API

The `wit/plugin.wit` file is copied from
[OpenIDEA](https://github.com/l0ng-ai/openidea) (Apache-2.0), by the l0ng-ai
contributors.
