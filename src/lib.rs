//! Minecraft Dev for OpenIDEA — a port of the essential MinecraftDev (IntelliJ)
//! feature set to the OpenIDEA plugin API (wit/plugin.wit).
//!
//! What MinecraftDev does in IntelliJ with PSI, wizards and tool windows, this
//! plugin does with the OpenIDEA host API: read the project's build files,
//! work out the platform, and insert ported scaffolding into the editor.

wit_bindgen::generate!({
    world: "plugin",
    path: "wit",
});

use openidea::plugin::host::{self, LogLevel, NotificationLevel};

mod assets;
mod detect;
mod templates;
mod versions;

use detect::{Platform, ProjectFacts};

/// (id, title) pairs — same list is declared statically in plugin.toml.
const COMMANDS: &[(&str, &str)] = &[
    ("detect", "Minecraft Dev: Detect Platform"),
    ("insert-main", "Minecraft Dev: Insert Main Class"),
    ("insert-build-gradle", "Minecraft Dev: Insert build.gradle"),
    (
        "insert-gradle-properties",
        "Minecraft Dev: Insert gradle.properties",
    ),
    ("insert-mod-metadata", "Minecraft Dev: Insert Mod Metadata"),
    ("insert-mixins", "Minecraft Dev: Insert Mixins Config"),
    ("insert-pack-mcmeta", "Minecraft Dev: Insert pack.mcmeta"),
    ("insert-asset", "Minecraft Dev: Insert Asset JSON"),
    ("insert-registry", "Minecraft Dev: Insert Registry Code"),
    ("insert-event", "Minecraft Dev: Insert Event Handler"),
    ("insert-command", "Minecraft Dev: Insert Command Class"),
    ("insert-mixin", "Minecraft Dev: Insert Mixin Class"),
    ("help", "Minecraft Dev: Help & Supported Platforms"),
];

struct McDev;

impl Guest for McDev {
    fn activate(_ctx: Context) -> Result<(), String> {
        host::log(LogLevel::Info, "minecraft-dev activated");
        for (id, title) in COMMANDS {
            host::register_command(id, title, Some("Minecraft Dev"));
        }
        Ok(())
    }

    fn deactivate() {
        host::log(LogLevel::Info, "minecraft-dev deactivated");
    }

    fn run_command(id: String, arg: Option<String>) -> Result<Option<String>, String> {
        let arg = arg.as_deref();
        match id.as_str() {
            "help" => {
                host::show_notification(NotificationLevel::Info, "Minecraft Dev", &help_text());
                Ok(Some(help_text()))
            }
            "detect" => {
                let facts = detect::detect();
                let body = facts.describe();
                let title = if facts.platform == Platform::Unknown {
                    "No Minecraft project detected"
                } else {
                    "Minecraft project detected"
                };
                host::show_notification(NotificationLevel::Info, title, &body);
                Ok(Some(body))
            }
            "insert-main" => {
                let (platform, family) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::main_class(platform, family);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-build-gradle" => {
                let (platform, _) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::build_gradle(platform);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-gradle-properties" => {
                let (platform, _) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::gradle_properties(platform);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-mod-metadata" => {
                let (platform, family) = resolve_platform(arg)?;
                let tpl = templates::mod_metadata(platform);
                where_bukkit(&tpl, family)
            }
            "insert-mixins" => {
                let (platform, _) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::mixins_config(platform);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-pack-mcmeta" => {
                let facts = detect::detect();
                let (format, comment) = facts
                    .mc_parsed()
                    .map(|v| versions::pack_format(v))
                    .unwrap_or((48, ""));
                let comment_json = if comment.is_empty() {
                    String::new()
                } else {
                    format!(",\n    \"_comment\": \"{comment}\"")
                };
                let extra = [
                    ("PACK_FORMAT", format.to_string()),
                    ("PACK_COMMENT", comment_json),
                ];
                let tpl = templates::Template {
                    label: "pack.mcmeta".to_string(),
                    content: templates::PACK_MCMETA.to_string(),
                };
                insert_filled(&tpl, &tpl, facts, &extra)
            }
            "insert-asset" => {
                let facts = detect::detect();
                let kind = match arg {
                    Some(k) => assets::AssetKind::parse(k),
                    None => host::active_file().as_deref().and_then(assets::AssetKind::from_path),
                };
                let kind = kind.ok_or_else(|| {
                    String::from(
                        "no asset kind given (try arg: block-model, item-model, blockstate, lang, sounds); nothing was inserted",
                    )
                })?;
                let mod_id = facts.mod_id_or("examplemod");
                let content = assets::asset_content(&kind, &mod_id, "example_block");
                let label = format!("{} ({mod_id})", kind.name());
                host::insert_text(&content);
                host::show_notification(
                    NotificationLevel::Info,
                    "Minecraft Dev",
                    &format!("Inserted {label} for mod {mod_id}"),
                );
                Ok(Some(content))
            }
            "insert-registry" => {
                let (platform, _) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::registry_code(platform);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-event" => {
                let (platform, _) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::event_handler(platform);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-command" => {
                let (platform, _) = resolve_platform(arg)?;
                let facts = detect::detect();
                let tpl = templates::command_class(platform);
                insert_filled(&tpl, &tpl, facts, &[])
            }
            "insert-mixin" => {
                let facts = detect::detect();
                let tpl = templates::mixin_class();
                let extra = [("MIXIN_PACKAGE", format!("{}.mixin", package(&facts)))];
                insert_filled(&tpl, &tpl, facts, &extra)
            }
            other => Err(format!("unknown command: {other}")),
        }
    }
}

export!(McDev);

// ------------------------------------------------------------------ helpers

fn resolve_platform(arg: Option<&str>) -> Result<(Platform, Option<Platform>), String> {
    if let Some(a) = arg {
        if let Some(p) = Platform::from_str(a) {
            return Ok((p, None));
        }
    }
    let facts = detect::detect();
    if facts.platform != Platform::Unknown {
        Ok((facts.platform, facts.bukkit_family))
    } else {
        Err(
            "no Minecraft platform detected — open a mod project or pass the platform \
             (fabric, quilt, forge, neoforge, architectury, bukkit, spigot, paper, sponge, \
             velocity, bungeecord, waterfall)"
                .to_string(),
        )
    }
}

/// For plugins where metadata depends on the Bukkit-family variant (Spigot vs Paper).
fn where_bukkit(
    tpl: &templates::Template,
    family: Option<Platform>,
) -> Result<Option<String>, String> {
    let resolved = match family {
        Some(Platform::Paper) => templates::mod_metadata(Platform::Paper),
        Some(Platform::Spigot) => templates::mod_metadata(Platform::Spigot),
        _ => templates::Template {
            label: tpl.label.clone(),
            content: tpl.content.clone(),
        },
    };
    let facts = detect::detect();
    insert_filled(tpl, &resolved, facts, &[])
}

/// Fill a template with project-aware placeholders, insert it and notify.
fn insert_filled(
    original: &templates::Template,
    chosen: &templates::Template,
    facts: ProjectFacts,
    extra: &[(&str, String)],
) -> Result<Option<String>, String> {
    let vars = template_vars(&facts, extra);
    let refs: Vec<(&str, &str)> = vars
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let content = templates::fill(&chosen.content, &refs);
    host::insert_text(&content);
    let used = if chosen.label == original.label {
        chosen.label.clone()
    } else {
        format!("{} (chose {})", original.label, chosen.label)
    };
    host::show_notification(
        NotificationLevel::Info,
        "Minecraft Dev",
        &format!("Inserted {used}"),
    );
    host::log(
        LogLevel::Info,
        &format!("inserted {} ({} bytes)", used, content.len()),
    );
    Ok(Some(content))
}

fn package(facts: &ProjectFacts) -> String {
    format!("com.example.{}", facts.mod_id_or("examplemod").replace(['-', '.'], "_"))
}

fn template_vars(facts: &ProjectFacts, extra: &[(&str, String)]) -> Vec<(String, String)> {
    let mod_id = facts.mod_id_or("examplemod");
    let mod_name = facts
        .mod_name
        .clone()
        .unwrap_or_else(|| pascal_case(&mod_id));
    let mc = facts.mc_version.clone().unwrap_or_else(|| "1.21.1".into());
    let java = facts
        .java_version
        .clone()
        .unwrap_or_else(|| {
            facts
                .mc_parsed()
                .map(|v| versions::java_target(v).to_string())
                .unwrap_or_else(|| "21".into())
        });
    let mut vars: Vec<(String, String)> = vec![
        ("MOD_ID".into(), mod_id.clone()),
        ("MOD_NAME".into(), mod_name.clone()),
        ("MOD_DESCRIPTION".into(), "An example mod.".into()),
        ("MOD_VERSION".into(), "1.0.0".into()),
        ("LICENSE".into(), "All Rights Reserved".into()),
        ("CLASS_NAME".into(), format!("{mod_name}Mod")),
        ("PACKAGE".into(), package(facts)),
        ("MIXIN_PACKAGE".into(), format!("{}.mixin", package(facts))),
        ("MC_VERSION".into(), mc),
        ("JAVA_VERSION".into(), java),
        ("BLOCK_NAME".into(), "example_block".into()),
        ("NEOFORGE_VERSION".into(), "21.0.0".into()),
        ("NEOFORGE_GRADLE_VERSION".into(), "7.0.145".into()),
        ("LOOM_VERSION".into(), "1.10-SNAPSHOT".into()),
        ("VELOCITY_API_VERSION".into(), "3.4.0-SNAPSHOT".into()),
        ("ARCHITECTURY_VERSION".into(), "3.4.164".into()),
    ];
    for (k, v) in extra {
        vars.push(((*k).to_string(), v.clone()));
    }
    vars
}

fn pascal_case(s: &str) -> String {
    s.split(['-', '_', '.'])
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn help_text() -> String {
    "Minecraft Dev for OpenIDEA — commands (Search Everywhere → Actions → Minecraft Dev):\n\
     \n\
     • Detect Platform — reads build.gradle, gradle.properties, fabric.mod.json, mods.toml, plugin.yml…\n\
     • Insert Main Class / build.gradle / gradle.properties / Mod Metadata / Mixins Config\n\
     • Insert pack.mcmeta (pack_format chosen from the detected Minecraft version)\n\
     • Insert Asset JSON — block-model, item-model, blockstate, lang, sounds\n\
     • Insert Registry Code / Event Handler / Command Class / Mixin Class\n\
     \n\
     Platforms: Fabric, Quilt, Forge, NeoForge, Architectury, Bukkit, Spigot, Paper, Sponge, Velocity, BungeeCord, Waterfall, Mixin.\n\
     Most insert commands take an optional platform argument: e.g. execute plugert commands take an optional platform argument: e.g. execute plugin.minecraft-dev.insert-main with \"fabric\"."
        .to_string()
}
