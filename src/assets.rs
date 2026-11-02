//! Minecraft resource asset templates (block models, blockstates, language
//! files, sounds, pack.mcmeta) — the text half of MinecraftDev's asset support.

pub enum AssetKind {
    BlockModel,
    ItemModel,
    Blockstate,
    LangEntry,
    Sounds,
}

impl AssetKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().replace('_', "-").as_str() {
            "block-model" | "blockmodel" | "model-block" => Some(Self::BlockModel),
            "item-model" | "itemmodel" | "model-item" => Some(Self::ItemModel),
            "blockstate" | "block-states" => Some(Self::Blockstate),
            "lang" | "lang-entry" | "language" => Some(Self::LangEntry),
            "sounds" | "sounds-json" => Some(Self::Sounds),
            _ => None,
        }
    }

    /// Guess what the active file is, so one command is enough.
    pub fn from_path(path: &str) -> Option<Self> {
        let p = path.replace('\\', "/");
        if p.contains("/blockstates/") {
            Some(Self::Blockstate)
        } else if p.contains("/models/block/") {
            Some(Self::BlockModel)
        } else if p.contains("/models/item/") {
            Some(Self::ItemModel)
        } else if p.contains("/lang/") {
            Some(Self::LangEntry)
        } else if p.ends_with("sounds.json") {
            Some(Self::Sounds)
        } else {
            None
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::BlockModel => "block model",
            Self::ItemModel => "item model",
            Self::Blockstate => "blockstate",
            Self::LangEntry => "language entry",
            Self::Sounds => "sounds.json entry",
        }
    }
}

/// The JSON content for an asset. Placeholders are inserted directly.
pub fn asset_content(kind: &AssetKind, mod_id: &str, block_name: &str) -> String {
    let namespaced = format!("{mod_id}:{block_name}");
    match kind {
        AssetKind::BlockModel => r#"{
  "parent": "minecraft:block/cube_all",
  "textures": {
    "particle": "PLACEHOLDER",
    "all": "PLACEHOLDER"
  }
}"#
        .replace("PLACEHOLDER", &namespaced),
        AssetKind::ItemModel => r#"{
  "parent": "PLACEHOLDER"
}"#
        .replace("PLACEHOLDER", &namespaced),
        AssetKind::Blockstate => r#"{
  "variants": {
    "": { "model": "PLACEHOLDER" }
  }
}"#
        .replace("PLACEHOLDER", &namespaced),
        AssetKind::LangEntry => format!(
            r#"{{
  "block.{mod_id}.{block_name}": "{mod_id}.{block_name}",
  "item.{mod_id}.{block_name}": "{block_name}"
}}"#
        ),
        AssetKind::Sounds => format!(
            r#"{{
  "{block_name}": {{
    "category": "master",
    "sounds": [
      {{
        "name": "{namespaced}",
        "stream": false
      }}
    ]
  }}
}}"#
        ),
    }
}
