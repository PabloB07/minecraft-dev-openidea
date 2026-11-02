//! Minecraft version helpers.
//!
//! `pack_format` mapping is ported from MinecraftDev's
//! `platform/forge/util/ForgePackDescriptor.kt` (LGPL-3.0, see NOTICE.md).

use core::cmp::Ordering;

/// A Minecraft version like `1.21.3` or, for the year scheme, `26.1`.
/// Compared so that `1.21.3 > 1.20.4` regardless of digit count.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct McVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl PartialOrd for McVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for McVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl McVersion {
    /// Parses "1.21.3", "26.1", "1.20.4-rc1", "1.16.5-pre2"; returns None for junk.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        // Strip a range bracket like "[1.20,1.21)" if present.
        let s = s.trim_start_matches('[').split(',').next().unwrap_or(s);
        let s = s.split(['-', '+']).next().unwrap_or(s);
        let mut parts = s.split('.');
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts.next().and_then(|p| p.parse::<u32>().ok()).unwrap_or(0);
        let patch = parts.next().and_then(|p| p.parse::<u32>().ok()).unwrap_or(0);
        Some(Self { major, minor, patch })
    }

    pub fn from_parts(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

/// The Java version a mod should target for a given Minecraft version
/// (mirrors the JDK advice of the official MDKs).
pub fn java_target(version: McVersion) -> &'static str {
    if version >= McVersion::from_parts(1, 20, 5) || version.major >= 25 {
        "21"
    } else if version >= McVersion::from_parts(1, 18, 0) {
        "17"
    } else if version >= McVersion::from_parts(1, 17, 0) {
        "16"
    } else {
        "8"
    }
}

/// The resource pack `pack_format` and its Forge comment.
///
/// Ported from MinecraftDev's `ForgePackDescriptor.kt`
/// (https://minecraft.gamepedia.com/Tutorials/Creating_a_resource_pack).
pub fn pack_format(version: McVersion) -> (u32, &'static str) {
    let v = version;
    if v <= McVersion::from_parts(1, 12, 2) {
        (
            3,
            "A pack_format of 3 should be used starting with Minecraft 1.11.",
        )
    } else if v <= McVersion::from_parts(1, 14, 4) {
        (4, "A pack_format of 4 requires json lang files.")
    } else if v <= McVersion::from_parts(1, 16, 1) {
        (5, "A pack_format of 5 requires json lang files and some texture changes from 1.15.")
    } else if v < McVersion::from_parts(1, 17, 0) {
        (6, "A pack_format of 6 requires json lang files and some texture changes from 1.16.2.")
    } else if v < McVersion::from_parts(1, 18, 0) {
        (7, "")
    } else if v < McVersion::from_parts(1, 18, 2) {
        (8, "")
    } else if v < McVersion::from_parts(1, 19, 0) {
        (9, "")
    } else if v < McVersion::from_parts(1, 19, 3) {
        (10, "")
    } else if v < McVersion::from_parts(1, 20, 0) {
        (12, "")
    } else if v < McVersion::from_parts(1, 20, 2) {
        (15, "")
    } else if v < McVersion::from_parts(1, 20, 3) {
        (18, "")
    } else if v < McVersion::from_parts(1, 20, 5) {
        (26, "")
    } else if v < McVersion::from_parts(1, 21, 0) {
        (41, "")
    } else {
        (48, "")
    }
}
