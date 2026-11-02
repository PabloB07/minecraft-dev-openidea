//! Project detection: reads the project's build files and works out which
//! Minecraft modding platform it uses, the mod id, the Minecraft version and
//! the mappings — the OpenIdea counterpart of MinecraftDev's facet detection.

use crate::openidea::plugin::host;
use super::versions::McVersion;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Platform {
    #[default]
    Unknown,
    Fabric,
    Quilt,
    Forge,
    NeoForge,
    Architectury,
    Bukkit,
    Spigot,
    Paper,
    Sponge,
    Velocity,
    BungeeCord,
    Waterfall,
    Mixin,
}

impl Platform {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "fabric" => Some(Self::Fabric),
            "quilt" | "quiltmc" => Some(Self::Quilt),
            "forge" | "minecraftforge" => Some(Self::Forge),
            "neoforge" => Some(Self::NeoForge),
            "architectury" => Some(Self::Architectury),
            "bukkit" => Some(Self::Bukkit),
            "spigot" => Some(Self::Spigot),
            "paper" => Some(Self::Paper),
            "sponge" => Some(Self::Sponge),
            "velocity" => Some(Self::Velocity),
            "bungeecord" => Some(Self::BungeeCord),
            "waterfall" => Some(Self::Waterfall),
            "mixin" => Some(Self::Mixin),
            _ => None,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            Self::Fabric => "Fabric",
            Self::Quilt => "Quilt",
            Self::Forge => "Forge",
            Self::NeoForge => "NeoForge",
            Self::Architectury => "Architectury",
            Self::Bukkit => "Bukkit",
            Self::Spigot => "Spigot",
            Self::Paper => "Paper",
            Self::Sponge => "Sponge",
            Self::Velocity => "Velocity",
            Self::BungeeCord => "BungeeCord",
            Self::Waterfall => "Waterfall",
            Self::Mixin => "Mixin",
            Self::Unknown => "Unknown",
        }
    }
}

/// Everything we could work out about the open project.
#[derive(Default)]
pub struct ProjectFacts {
    pub platform: Platform,
    // Bukkit-family platforms share templates; keep the real detected one here.
    pub bukkit_family: Option<Platform>,
    pub mod_id: Option<String>,
    pub mod_name: Option<String>,
    pub mc_version: Option<String>,
    pub mappings: Option<String>,
    pub loader_version: Option<String>,
    pub java_version: Option<String>,
    pub has_mixins: bool,
    pub mod_environment: Option<String>,
    pub signals: Vec<String>,
    pub files_found: Vec<String>,
}

impl ProjectFacts {
    pub fn mc_parsed(&self) -> Option<McVersion> {
        self.mc_version.as_deref().and_then(McVersion::parse)
    }

    pub fn mod_id_or(&self, fallback: &str) -> String {
        self.mod_id.clone().unwrap_or_else(|| fallback.to_string())
    }

    pub fn describe(&self) -> String {
        let mut lines = vec![format!("Platform: {}", self.platform.display())];
        if let Some(f) = &self.mod_id {
            lines.push(format!("Mod / plugin id: {f}"));
        }
        if let Some(n) = &self.mod_name {
            lines.push(format!("Name: {n}"));
        }
        if let Some(v) = &self.mc_version {
            lines.push(format!("Minecraft: {v}"));
        }
        if let Some(m) = &self.mappings {
            lines.push(format!("Mappings: {m}"));
        }
        if let Some(l) = &self.loader_version {
            lines.push(format!("Loader: {l}"));
        }
        let java = self
            .java_version
            .clone()
            .or_else(|| self.mc_parsed().map(|v| super::versions::java_target(v).to_string()));
        if let Some(j) = java {
            lines.push(format!("Java level: {j}"));
        }
        lines.push(format!("Mixins: {}", if self.has_mixins { "yes" } else { "no" }));
        if !self.files_found.is_empty() {
            lines.push(format!("Files seen: {}", self.files_found.join(", ")));
        }
        lines.join("\n")
    }
}

/// Files that betray a project's platform, most specific first.
const PROBE_FILES: &[(&str, &str)] = &[
    ("src/main/resources/fabric.mod.json", "fabric"),
    ("src/main/resources/quilt.mod.json", "quilt"),
    ("src/main/resources/META-INF/neoforge.mods.toml", "neoforge"),
    ("src/main/resources/META-INF/mods.toml", "forge"),
    ("src/main/resources/sponge_plugins.json", "sponge"),
    ("src/main/resources/plugin.yml", "bukkit"),
    ("src/main/resources/paper-plugin.yml", "paper"),
    ("velocity.toml", "velocity"),
    ("bungee.yml", "bungeecord"),
    ("build.gradle", "gradle"),
    ("build.gradle.kts", "gradle-kts"),
    ("settings.gradle", "gradle"),
    ("settings.gradle.kts", "gradle-kts"),
    ("gradle.properties", "props"),
    ("pom.xml", "maven"),
];

/// Read a file relative to the project root (or absolute). `Ok(None)` if missing.
fn try_read(root: Option<&str>, rel: &str) -> Option<String> {
    let path = match root {
        Some(r) => format!("{}/{}", r.trim_end_matches(['/', '\\']), rel),
        None => rel.to_string(),
    };
    match host::read_file(&path) {
        Ok(text) => Some(text),
        Err(_) => None,
    }
}

/// Detect the platform and mod info of the current project.
pub fn detect() -> ProjectFacts {
    let root = host::workspace_root();
    let mut facts = ProjectFacts::default();
    let root_ref = root.as_deref();
    let mut texts: Vec<(&str, String)> = Vec::new();

    for (rel, tag) in PROBE_FILES {
        if let Some(text) = try_read(root_ref, rel) {
            texts.push((tag, text));
            facts.files_found.push((*rel).to_string());
        }
    }

    // Mixins: any *.mixins.json or forge mixin config
    for rel in ["src/main/resources/examplemod.mixins.json"] {
        if try_read(root_ref, rel).is_some() {
            facts.has_mixins = true;
            facts.files_found.push(rel.to_string());
        }
    }
    for (tag, text) in &texts {
        if text.contains("\"mixins\"") || text.contains("mixins.json") {
            facts.has_mixins = true;
        }
        let _ = tag;
    }

    let get = |tag: &str| -> Option<&String> {
        texts.iter().find(|(t, _)| *t == tag).map(|(_, s)| s)
    };

    // --- metadata files first: they name the platform outright ---
    if let Some(json) = get("quilt") {
        facts.platform = Platform::Quilt;
        facts.signals.push("found src/main/resources/quilt.mod.json".into());
        facts.mod_id = json_string(json, "id");
        facts.mod_name = json_string(json, "name");
        facts.mod_environment = json_string(json, "environment");
        if let Some(dep) = json.depends_minecraft() {
            facts.mc_version = Some(dep);
            facts.signals.push("quilt.mod.json depends.minecraft".into());
        }
    } else if let Some(json) = get("fabric") {
        facts.platform = Platform::Fabric;
        facts.signals.push("found src/main/resources/fabric.mod.json".into());
        facts.mod_id = json_string(json, "id");
        facts.mod_name = json_string(json, "name");
        facts.mod_environment = json_string(json, "environment");
        if let Some(dep) = json.depends_minecraft() {
            facts.mc_version = Some(dep);
            facts.signals.push("fabric.mod.json depends.minecraft".into());
        }
        if texts.iter().any(|(_, t)| t.contains("architectury")) {
            facts.platform = Platform::Architectury;
            facts.signals.push("architectury marker in build files".into());
        }
    } else if let Some(toml) = get("neoforge") {
        facts.platform = Platform::NeoForge;
        facts.signals.push("found META-INF/neoforge.mods.toml".into());
        for_devforge_toml(toml, &mut facts);
    } else if let Some(toml) = get("forge") {
        facts.platform = Platform::Forge;
        facts.signals.push("found META-INF/mods.toml".into());
        for_devforge_toml(toml, &mut facts);
    } else if let Some(json) = get("sponge") {
        facts.platform = Platform::Sponge;
        facts.signals.push("found sponge_plugins.json".into());
        if let Some(id) = json_string(json, "id") {
            facts.mod_id = Some(id);
        }
    } else if get("paper").is_some() {
        facts.platform = Platform::Paper;
        facts.bukkit_family = Some(Platform::Paper);
        facts.signals.push("found paper-plugin.yml".into());
        facts.mod_id = get("paper").and_then(|y| yaml_key(y, "main"));
    } else if let Some(yml) = get("bukkit") {
        // Bukkit vs Spigot vs Paper is a build-dependency question
        facts.platform = Platform::Bukkit;
        facts.bukkit_family = Some(Platform::Bukkit);
        facts.signals.push("found plugin.yml".into());
        facts.mod_id = yaml_key(yml, "main");
        for (tag, text) in &texts {
            let l = text.to_ascii_lowercase();
            if l.contains("io.papermc.paper:paper-api") || l.contains("paper-api") {
                facts.platform = Platform::Paper;
                facts.bukkit_family = Some(Platform::Paper);
                facts.signals.push("paper-api dependency".into());
            } else if l.contains("org.spigotmc:spigot-api") {
                facts.platform = Platform::Spigot;
                facts.bukkit_family = Some(Platform::Spigot);
                facts.signals.push("spigot-api dependency".into());
            } else if l.contains("md-5") || l.contains("bungeecord-api") {
                facts.platform = Platform::BungeeCord;
                facts.signals.push("bungeecord-api dependency".into());
            }
            let _ = tag;
        }
    } else if get("velocity").is_some() {
        facts.platform = Platform::Velocity;
        facts.signals.push("found velocity.toml".into());
    } else if let Some(yml) = get("bungeecord") {
        facts.platform = Platform::BungeeCord;
        facts.signals.push("found bungee.yml".into());
        if let Some(name) = yaml_key(yml, "name") {
            facts.mod_name = Some(name);
        }
    }

    // --- then gradle / maven clues, even when metadata exists ---
    for text in texts.iter().map(|(_, t)| t) {
        let l = text.to_ascii_lowercase();
        if facts.platform == Platform::Unknown {
            if l.contains("fabric-loom") || l.contains("net.fabricmc") {
                facts.platform = Platform::Fabric;
                facts.signals.push("fabric-loom / fabricmc in build script".into());
            } else if l.contains("neoforged") || l.contains("neogradle") {
                facts.platform = Platform::NeoForge;
                facts.signals.push("neoforged / neogradle in build script".into());
            } else if l.contains("minecraftforge") || l.contains("forgegradle")
                || l.contains("net.minecraftforge.gradle")
            {
                facts.platform = Platform::Forge;
                facts.signals.push("ForgeGradle in build script".into());
            } else if l.contains("org.spongepowered") {
                facts.platform = Platform::Sponge;
                facts.signals.push("spongepowered plugin in build script".into());
            } else if l.contains("com.velocitypowered") {
                facts.platform = Platform::Velocity;
                facts.signals.push("velocity gradle plugin".into());
            } else if l.contains("net.md-5.bungeecord") || l.contains("bungeecord-api") {
                facts.platform = Platform::BungeeCord;
                facts.signals.push("bungeecord dependency".into());
            } else if l.contains("io.github.waterfallmc") {
                facts.platform = Platform::Waterfall;
                facts.signals.push("waterfall dependency".into());
            } else if l.contains("papermc") || l.contains("paper-api") {
                facts.platform = Platform::Bukkit;
                facts.bukkit_family = Some(Platform::Paper);
                facts.signals.push("paper dependency".into());
            } else if l.contains("spigotmc") || l.contains("spigot-api") {
                facts.platform = Platform::Bukkit;
                facts.bukkit_family = Some(Platform::Spigot);
                facts.signals.push("spigot dependency".into());
            }
        }
        if l.contains("architectury") {
            facts.signals.push("architectury marker".into());
        }
        // yarn vs mojang mappings
        if l.contains("loom.officialmojangmappings") {
            facts.mappings = Some("official Mojang mappings".into());
        } else if l.contains("yarn") {
            facts.mappings = Some("yarn".into());
        } else if l.contains("mappings channel: 'official'") || l.contains("\"official\"") {
            facts.mappings = facts.mappings.clone().or(Some("official".into()));
        } else if l.contains("parchment") {
            facts.mappings = Some("parchment".into());
        }
        // minecraft version from a gradle dependency coordinate
        if facts.mc_version.is_none() {
            if let Some(v) = find_mc_coordinate(text) {
                facts.mc_version = Some(v);
                facts.signals.push("minecraft dependency coordinate".into());
            }
        }
        // java toolchain
        if let Some(j) = find_java_level(text) {
            facts.java_version = Some(j);
        }
    }

    // --- gradle.properties values: mod_id, versions ---
    if let Some(props) = get("props") {
        if facts.mod_id.is_none() {
            facts.mod_id = props
                .kv("mod_id")
                .or_else(|| props.kv("modid"))
                .or_else(|| props.kv("archives_base_name"));
        }
        if facts.mc_version.is_none() {
            facts.mc_version = props.kv("minecraft_version");
            if facts.mc_version.is_some() {
                facts.signals.push("gradle.properties minecraft_version".into());
            }
        }
        if facts.loader_version.is_none() {
            facts.loader_version = props.kv("loader_version");
        }
        if facts.mod_name.is_none() {
            facts.mod_name = props.kv("mod_name");
        }
    }

    facts
}

/// Fill modId / displayName / minecraft version from a Forge/NeoForge mods.toml.
fn for_devforge_toml(toml: &str, facts: &mut ProjectFacts) {
    let mut current_table = String::new();
    for raw in toml.lines() {
        let line = raw.trim();
        let t = line;
        if line.starts_with("[[") && line.ends_with("]]") {
            current_table = line.trim_matches(|c| c == '[' || c == ']').to_string();
            continue;
        }
        if current_table == "mods" {
            if t.starts_with("modId") {
                facts.mod_id = toml_value(t);
            } else if t.starts_with("displayName") {
                facts.mod_name = toml_value(t);
            } else if t.starts_with("version") {
                // keep it only until another source is found (unused for now)
            }
        } else if current_table.starts_with("dependencies.") {
            if t.starts_with("versionRange") {
                if let Some(v) = toml_value(t) {
                    if current_table.ends_with("minecraft") {
                        facts.mc_version = Some(v);
                        facts.signals.push("mods.toml minecraft versionRange".into());
                    }
                }
            }
        }
    }
}

// --- tiny helpers, no serde ---

/// First `"key": "value"` (or `"key":"value"`) in a JSON document.
pub fn json_string(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let idx = json.find(&needle)?;
    let rest = json[idx + needle.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let value = rest.strip_prefix('"')?;
    let end = value.find('"')?;
    Some(value[..end].to_string())
}

trait FabricExt {
    fn depends_minecraft(&self) -> Option<String>;
}

impl FabricExt for String {
    /// Entry of `depends.minecraft` inside `"depends": { "minecraft": "~1.21.1" }`.
    fn depends_minecraft(&self) -> Option<String> {
        let dep_pos = self.find("\"depends\"")?;
        let mine_pos = self[dep_pos..].find("\"minecraft\"")? + dep_pos;
        // Skip past the key itself, then the ':' and the opening quote.
        let rest = &self[mine_pos + "\"minecraft\"".len()..];
        let rest = rest.trim_start().strip_prefix(':')?;
        let rest = rest.trim_start().strip_prefix('"')?;
        let end = rest.find('"')?;
        let value = &rest[..end];
        // Version range like ">=1.20 <1.21" or "~1.21.1": take the first version token.
        let token = value
            .split(|c: char| !c.is_ascii_digit() && c != '.' && c != 'x' && c != '~')
            .find(|s| s.contains("1.") || s.starts_with('2'))
            .unwrap_or(value)
            .trim_matches(|c| c == '~' || c == '>' || c == '.' || c == 'x');
        if token.is_empty() {
            return Some(value.trim_start_matches('~').to_string());
        }
        Some(token.to_string())
    }
}

/// `key = "value"` / `key=value` from gradle.properties.
trait PropsExt {
    fn kv(&self, key: &str) -> Option<String>;
}

impl PropsExt for String {
    fn kv(&self, key: &str) -> Option<String> {
        for raw in self.lines() {
            let line = raw.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                if k.trim() == key {
                    let v = v.trim();
                    return Some(v.trim_matches('"').trim_matches('\'').to_string());
                }
            }
        }
        None
    }
}

/// Top-level `key:` from a YAML document (plugin.yml).
fn yaml_key(yaml: &str, key: &str) -> Option<String> {
    for raw in yml_lines(yaml) {
        if let Some((k, v)) = raw.split_once(':') {
            if k.trim() == key {
                return Some(v.trim().trim_matches(|c| c == '"' || c == '\'').to_string());
            }
        }
    }
    None
}

fn yml_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().filter(|l| !l.trim_start().starts_with('#'))
}

/// RHS of `key = "value"` in a TOML document.
fn toml_value(line: &str) -> Option<String> {
    let (_, v) = line.split_once('=')?;
    let v = v.trim().trim_end_matches(',');
    Some(v.trim_matches('"').trim_matches('\'').to_string())
}

/// `com.mojang:minecraft:1.20.1` (or with a +suffix) inside a build script.
fn find_mc_coordinate(text: &str) -> Option<String> {
    let needle = "com.mojang:minecraft:";
    let mut start = 0;
    while let Some(pos) = text[start..].find(needle) {
        let after = start + pos + needle.len();
        let rest = &text[after.min(text.len())..];
        let token: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        if let Some(v) = McVersion::parse(&token) {
            let _ = v;
            return Some(token.trim_end_matches('.').to_string());
        }
        start = after;
    }
    None
}

/// `JavaLanguageVersion.of(21)` / `JavaVersion.toVersion(21)`.
fn find_java_level(text: &str) -> Option<String> {
    for needle in [
        "JavaLanguageVersion.of(",
        "JavaVersion.toVersion(",
        "release.set(",
    ] {
        if let Some(pos) = text.find(needle) {
            let rest = &text[pos + needle.len()..];
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                let n: u32 = digits.parse().ok()?;
                return Some(n.to_string());
            }
        }
    }
    None
}
