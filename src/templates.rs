//! Project scaffolding templates, ported and adapted from MinecraftDev's
//! `src/main/resources/fileTemplates` (LGPL-3.0, see NOTICE.md):
//!  - main classes for Forge / NeoForge / Fabric / Quilt / Bukkit / Paper /
//!    Spigot / Sponge / Velocity / BungeeCord / Waterfall
//!  - build.gradle + gradle.properties per platform
//!  - mod metadata (fabric.mod.json, mods.toml, neoforge.mods.toml, plugin.yml,
//!    paper-plugin.yml, sponge_plugins.json)
//!  - mixin configs, pack.mcmeta, event handlers, commands, registries
//!
//! Placeholders use `{{NAME}}` and are filled by lib.rs.

use super::detect::Platform;

pub struct Template {
    pub label: String,
    pub content: String,
}

fn t(label: &str, content: &str) -> Template {
    Template {
        label: label.to_string(),
        content: content.to_string(),
    }
}

/// Replace `{{key}}` placeholders. Unknown placeholders are left visible.
pub fn fill(content: &str, vars: &[(&str, &str)]) -> String {
    let mut out = content.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("{{{{{k}}}}}"), v);
    }
    out
}

// ---------------------------------------------------------------- main classes

pub fn main_class(platform: Platform, bukkit_family: Option<Platform>) -> Template {
    match platform {
        Platform::Fabric => t("Fabric ModInitializer", FABRIC_MAIN),
        Platform::Quilt => t("Quilt ModInitializer", QUILT_MAIN),
        Platform::Forge => t("Forge @Mod", FORGE_MAIN),
        Platform::NeoForge => t("NeoForge @Mod", NEOFORGE_MAIN),
        Platform::Bukkit | Platform::Spigot | Platform::Paper => {
            let family = bukkit_family.unwrap_or(platform);
            let title = match family {
                Platform::Paper => "Paper plugin class",
                Platform::Spigot => "Spigot plugin class",
                _ => "Bukkit plugin class",
            };
            t(title, BUKKIT_MAIN)
        }
        Platform::Sponge => t("Sponge plugin class", SPONGE_MAIN),
        Platform::Velocity => t("Velocity plugin class", VELOCITY_MAIN),
        Platform::BungeeCord | Platform::Waterfall => t("BungeeCord plugin class", BUNGEECORD_MAIN),
        _ => t("Mixin / unsupported platform", MIXIN_MAIN),
    }
}

const FABRIC_MAIN: &str = r#"package {{PACKAGE}};

import net.fabricmc.api.ModInitializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class {{CLASS_NAME}} implements ModInitializer {
    public static final String MOD_ID = "{{MOD_ID}}";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

    @Override
    public void onInitialize() {
        LOGGER.info("Hello Fabric world from {{MOD_NAME}}!");
    }
}
"#;

const QUILT_MAIN: &str = r#"package {{PACKAGE}};

import org.quiltmc.loader.api.ModInitializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class {{CLASS_NAME}} implements ModInitializer {
    public static final String MOD_ID = "{{MOD_ID}}";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

    @Override
    public void onInitialize() {
        LOGGER.info("Hello Quilt world from {{MOD_NAME}}!");
    }
}
"#;

const FORGE_MAIN: &str = r#"package {{PACKAGE}};

import com.mojang.logging.LogUtils;
import net.minecraft.client.Minecraft;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.CreativeModeTabs;
import net.minecraft.world.item.Item;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.material.Material;
import net.minecraftforge.api.distmarker.Dist;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.CreativeModeTabEvent;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.eventbus.api.SubscribeEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.event.lifecycle.FMLClientSetupEvent;
import net.minecraftforge.fml.event.lifecycle.FMLCommonSetupEvent;
import net.minecraftforge.event.server.ServerStartingEvent;
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
import net.minecraftforge.registries.DeferredRegister;
import net.minecraftforge.registries.ForgeRegistries;
import net.minecraftforge.registries.RegistryObject;
import org.slf4j.Logger;

// The value here should match an entry in the META-INF/mods.toml file
@Mod({{CLASS_NAME}}.MODID)
public class {{CLASS_NAME}} {

    // Define mod id in a common place for everything to reference
    public static final String MODID = "{{MOD_ID}}";
    // Directly reference a slf4j logger
    private static final Logger LOGGER = LogUtils.getLogger();
    // Create a Deferred Register to hold Blocks which will all be registered under the "{{MOD_ID}}" namespace
    public static final DeferredRegister<Block> BLOCKS = DeferredRegister.create(ForgeRegistries.BLOCKS, MODID);
    // Create a Deferred Register to hold Items which will all be registered under the "{{MOD_ID}}" namespace
    public static final DeferredRegister<Item> ITEMS = DeferredRegister.create(ForgeRegistries.ITEMS, MODID);

    // Creates a new Block with the id "{{MOD_ID}}:example_block", combining the namespace and path
    public static final RegistryObject<Block> EXAMPLE_BLOCK = BLOCKS.register("example_block", () -> new Block(BlockBehaviour.Properties.of(Material.STONE)));
    // Creates a new BlockItem with the id "{{MOD_ID}}:example_block", combining the namespace and path
    public static final RegistryObject<Item> EXAMPLE_BLOCK_ITEM = ITEMS.register("example_block", () -> new BlockItem(EXAMPLE_BLOCK.get(), new Item.Properties()));

    public {{CLASS_NAME}}() {
        IEventBus modEventBus = FMLJavaModLoadingContext.get().getModEventBus();

        // Register the commonSetup method for modloading
        modEventBus.addListener(this::commonSetup);

        // Register the Deferred Register to the mod event bus so blocks get registered
        BLOCKS.register(modEventBus);
        // Register the Deferred Register to the mod event event bus so items get registered
        ITEMS.register(modEventBus);

        // Register ourselves for server and other game events we are interested in
        MinecraftForge.EVENT_BUS.register(this);

        // Register the item to a creative tab
        modEventBus.addListener(this::addCreative);
    }

    private void commonSetup(final FMLCommonSetupEvent event) {
        // Some common setup code
        LOGGER.info("HELLO FROM COMMON SETUP");
        LOGGER.info("DIRT BLOCK >> {}", ForgeRegistries.BLOCKS.getKey(Blocks.DIRT));
    }

    private void addCreative(CreativeModeTabEvent.BuildContents event) {
        if (event.getTab() == CreativeModeTabs.BUILDING_BLOCKS)
            event.accept(EXAMPLE_BLOCK_ITEM);
    }

    // You can use SubscribeEvent and let the Event Bus discover methods to call
    @SubscribeEvent
    public void onServerStarting(ServerStartingEvent event) {
        LOGGER.info("HELLO from server starting");
    }

    // You can use EventBusSubscriber to automatically register all static methods in the class annotated with @SubscribeEvent
    @Mod.EventBusSubscriber(modid = MODID, bus = Mod.EventBusSubscriber.Bus.MOD, value = Dist.CLIENT)
    public static class ClientModEvents {

        @SubscribeEvent
        public static void onClientSetup(FMLClientSetupEvent event) {
            LOGGER.info("HELLO FROM CLIENT SETUP");
            LOGGER.info("MINECRAFT NAME >> {}", Minecraft.getInstance().getUser().getName());
        }
    }
}
"#;

const NEOFORGE_MAIN: &str = r#"package {{PACKAGE}};

import com.mojang.logging.LogUtils;
import net.minecraft.client.Minecraft;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.food.FoodProperties;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.CreativeModeTab;
import net.minecraft.world.item.CreativeModeTabs;
import net.minecraft.world.item.Item;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.material.MapColor;
import net.neoforged.api.distmarker.Dist;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.bus.api.SubscribeEvent;
import net.neoforged.fml.ModLoadingContext;
import net.neoforged.fml.common.Mod;
import net.neoforged.fml.config.ModConfig;
import net.neoforged.fml.event.lifecycle.FMLClientSetupEvent;
import net.neoforged.fml.event.lifecycle.FMLCommonSetupEvent;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.BuildCreativeModeTabContentsEvent;
import net.neoforged.neoforge.event.server.ServerStartingEvent;
import net.neoforged.neoforge.registries.DeferredBlock;
import net.neoforged.neoforge.registries.DeferredHolder;
import net.neoforged.neoforge.registries.DeferredItem;
import org.slf4j.Logger;

// The value here should match an entry in the META-INF/neoforge.mods.toml file
@Mod({{CLASS_NAME}}.MODID)
public class {{CLASS_NAME}} {
    public static final String MODID = "{{MOD_ID}}";
    private static final Logger LOGGER = LogUtils.getLogger();
    public static final DeferredBlock<Block> EXAMPLE_BLOCK = DeferredBlock.createBlock("example_block", "{{MOD_ID}}");
    public static final DeferredItem<Item> EXAMPLE_BLOCK_ITEM = DeferredItem.createItem("example_block", "{{MOD_ID}}");

    public {{CLASS_NAME}}(IEventBus modEventBus) {
        modEventBus.addListener(this::commonSetup);
        EXAMPLE_BLOCK.get().registerDefaultState();
        modEventBus.addListener(this::addCreative);
        NeoForge.EVENT_BUS.register(this);
    }

    private void commonSetup(final FMLCommonSetupEvent event) {
        LOGGER.info("HELLO FROM COMMON SETUP");
        LOGGER.info("DIRT BLOCK >> {}", BuiltInRegistries.BLOCK.getKey(Blocks.DIRT));
    }

    private void addCreative(BuildCreativeModeTabContentsEvent event) {
        if (event.getTabKey().equals(CreativeModeTabs.BUILDING_BLOCKS.name()))
            event.accept(EXAMPLE_BLOCK_ITEM.get());
    }

    @SubscribeEvent
    public void onServerStarting(ServerStartingEvent event) {
        LOGGER.info("HELLO from server starting");
    }
}
"#;

const BUKKIT_MAIN: &str = r#"package {{PACKAGE}};

import org.bukkit.plugin.java.JavaPlugin;

public final class {{CLASS_NAME}} extends JavaPlugin {

    @Override
    public void onEnable() {
        // Plugin startup logic
        getLogger().info("{{MOD_NAME}} enabled");
    }

    @Override
    public void onDisable() {
        // Plugin shutdown logic
    }
}
"#;

const SPONGE_MAIN: &str = r#"package {{PACKAGE}};

import com.google.inject.Inject;
import net.kyori.adventure.identity.Identity;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.LinearComponents;
import net.kyori.adventure.text.format.NamedTextColor;
import net.kyori.adventure.text.format.Style;
import net.kyori.adventure.text.format.TextDecoration;
import org.apache.logging.log4j.Logger;
import org.spongepowered.api.Server;
import org.spongepowered.api.command.Command;
import org.spongepowered.api.command.CommandResult;
import org.spongepowered.api.command.parameter.Parameter;
import org.spongepowered.api.event.Listener;
import org.spongepowered.api.event.lifecycle.ConstructPluginEvent;
import org.spongepowered.api.event.lifecycle.RegisterCommandEvent;
import org.spongepowered.api.event.lifecycle.StartingEngineEvent;
import org.spongepowered.plugin.PluginContainer;
import org.spongepowered.plugin.builtin.jvm.Plugin;

/**
 * The main class of your Sponge plugin.
 */
@Plugin("{{MOD_ID}}")
public class {{CLASS_NAME}} {

    private final PluginContainer container;
    private final Logger logger;

    @Inject
    {{CLASS_NAME}}(final PluginContainer container, final Logger logger) {
        this.container = container;
        this.logger = logger;
    }

    @Listener
    public void onConstructPlugin(final ConstructPluginEvent event) {
        this.logger.info("Constructing {{MOD_ID}}");
    }

    @Listener
    public void onRegisterCommands(final RegisterCommandEvent<Command.Parameterized> event) {
        final Parameter.Value<String> nameParam = Parameter.string().key("name").build();
        event.register(this.container, Command.builder()
            .addParameter(nameParam)
            .permission("{{MOD_ID}}.command.greet")
            .executor(ctx -> {
                final String name = ctx.requireOne(nameParam);
                ctx.sendMessage(Identity.nil(), LinearComponents.linear(
                    NamedTextColor.AQUA,
                    Component.text("Hello "),
                    Component.text(name, Style.style(TextDecoration.BOLD)),
                    Component.text("!")
                ));
                return CommandResult.success();
            })
            .build(), "greet", "wave");
    }
}
"#;

const VELOCITY_MAIN: &str = r#"package {{PACKAGE}};

import com.google.inject.Inject;
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent;
import com.velocitypowered.api.event.Subscribe;
import com.velocitypowered.api.plugin.Plugin;
import org.slf4j.Logger;

@Plugin(id = "{{MOD_ID}}", name = "{{MOD_NAME}}", version = "{{MOD_VERSION}}")
public class {{CLASS_NAME}} {

    @Inject private Logger logger;

    @Subscribe
    public void onProxyInitialization(ProxyInitializeEvent event) {
        logger.info("{{MOD_NAME}} online");
    }
}
"#;

const BUNGEECORD_MAIN: &str = r#"package {{PACKAGE}};

import net.md_5.bungee.api.plugin.Plugin;

public final class {{CLASS_NAME}} extends Plugin {

    @Override
    public void onEnable() {
        // Plugin startup logic
        getLogger().info("{{MOD_NAME}} enabled");
    }

    @Override
    public void onDisable() {
        // Plugin shutdown logic
    }
}
"#;

const MIXIN_MAIN: &str = r#"// Mixin detected: see the "Minecraft Dev: Insert Mixin Class" command
// and the mixins config to bolt onto this project's loader."#;

// ---------------------------------------------------------------- build files

pub fn build_gradle(platform: Platform) -> Template {
    match platform {
        Platform::Fabric => t("Fabric build.gradle (loom)", FABRIC_BUILD),
        Platform::Quilt => t("Quilt build.gradle (quilt-loom)", QUILT_BUILD),
        Platform::Forge => t("Forge build.gradle (ForgeGradle)", FORGE_BUILD),
        Platform::NeoForge => t("NeoForge build.gradle (NeoGradle)", NEOFORGE_BUILD),
        Platform::Architectury => t("Architectury build.gradle", ARCHITECTURY_BUILD),
        Platform::Bukkit | Platform::Spigot | Platform::Paper => t("Bukkit-family build.gradle", BUKKIT_BUILD),
        Platform::Sponge => t("Sponge build.gradle", SPONGE_BUILD),
        Platform::Velocity => t("Velocity build.gradle", VELOCITY_BUILD),
        Platform::BungeeCord | Platform::Waterfall => t("BungeeCord build.gradle", BUNGEECORD_BUILD),
        _ => t("plain library build.gradle", BUKKIT_BUILD),
    }
}

const FABRIC_BUILD: &str = r#"plugins {
    id 'fabric-loom' version '{{LOOM_VERSION}}'
    id 'maven-publish'
}

version = project.mod_version
group = project.maven_group

base {
    archivesName = project.archives_base_name
}

repositories {
    // Add repositories to retrieve artifacts from in here.
    // You should only use this when depending on other mods because
    // Loom adds the essential maven repositories to download Minecraft and libraries from automatically.
}

dependencies {
    // To change the versions see the gradle.properties file
    minecraft "com.mojang:minecraft:${project.minecraft_version}"
    mappings "net.fabricmc:yarn:${project.yarn_mappings}:v2"
    modImplementation "net.fabricmc:fabric-loader:${project.loader_version}"

    // Fabric API. This is technically optional, but you probably want it anyway.
    modImplementation "net.fabricmc.fabric-api:fabric-api:${project.fabric_version}"
}

processResources {
    inputs.property "version", project.version
    inputs.property "minecraft_version", project.minecraft_version
    inputs.property "loader_version", project.loader_version
    filesMatching("fabric.mod.json") {
        expand "version": project.version,
                "minecraft_version": project.minecraft_version,
                "loader_version": project.loader_version
    }
}

def targetJavaVersion = {{JAVA_VERSION}}
tasks.withType(JavaCompile).configureEach {
    it.options.encoding = "UTF-8"
    if (targetJavaVersion >= 10 || JavaVersion.current().isJava10Compatible()) {
        it.options.release.set(targetJavaVersion)
    }
}

java {
    def javaVersion = JavaVersion.toVersion(targetJavaVersion)
    if (JavaVersion.current() < javaVersion) {
        toolchain.languageVersion = JavaLanguageVersion.of(targetJavaVersion)
    }
    withSourcesJar()
}

jar {
    from("LICENSE") {
        rename { "${it}_${project.archivesBaseName}" }
    }
}
"#;

const QUILT_BUILD: &str = r#"plugins {
    id 'org.quiltmc.loom' version '{{LOOM_VERSION}}'
    id 'maven-publish'
}

version = project.mod_version
group = project.maven_group

base {
    archivesName = project.archives_base_name
}

repositories {
    // Add repositories to retrieve artifacts from in here.
}

dependencies {
    minecraft "com.mojang:minecraft:${project.minecraft_version}"
    mappings "org.quiltmc:quilt-mappings:${project.quilt_mappings}:intermediary"
    modImplementation "org.quiltmc:quilt-loader:${project.loader_version}"
}

processResources {
    inputs.property "version", project.version
    inputs.property "minecraft_version", project.minecraft_version
    filesMatching("quilt.mod.json") {
        expand "version": project.version, "minecraft_version": project.minecraft_version
    }
}

def targetJavaVersion = {{JAVA_VERSION}}
tasks.withType(JavaCompile).configureEach {
    it.options.encoding = "UTF-8"
    it.options.release.set(targetJavaVersion)
}

java {
    withSourcesJar()
}
"#;

const FORGE_BUILD: &str = r#"plugins {
    id 'net.minecraftforge.gradle' version '[6.0,6.2)'
    id 'eclipse'
    id 'maven-publish'
}

version = project.mod_version
group = project.maven_group

base {
    archivesName = project.archives_base_name
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
    }
    withSourcesJar()
}

minecraft {
    mappings channel: 'official', version: '{{MC_VERSION}}'

    runs {
        client {
            workingDirectory file('run')
            property 'forge.logging.markers', 'REGISTRIES'
            property 'forge.logging.console.level', 'debug'
            mods {
                "{{MOD_ID}}" {
                    source sourceSets.main
                }
            }
        }
        server {
            workingDirectory file('run/server')
            property 'forge.logging.markers', 'REGISTRIES'
            mods {
                "{{MOD_ID}}" {
                    source sourceSets.main
                }
            }
        }
    }
}

repositories {
    // Add repositories to retrieve artifacts from in here.
}

dependencies {
    // The version of ForgeGradle above must match your gradle.properties settings
}

processResources {
    inputs.property "version", project.version
    inputs.property "mod_id", project.mod_id
    filesMatching("META-INF/mods.toml") {
        expand "version": project.version, "mod_id": project.mod_id
    }
}

jar {
    manifest {
        attributes([
                "Specification-Title"     : project.mod_id,
                "Specification-Vendor"    : "{{MOD_NAME}}",
                "Specification-Version"   : "1",
                "Implementation-Title"    : project.name,
                "Implementation-Version"  : project.jar.archiveVersion,
                "Implementation-Vendor"   : "{{MOD_NAME}}",
        ])
    }
}
"#;

const NEOFORGE_BUILD: &str = r#"plugins {
    id 'net.neoforged.gradle.userdev' version '{{NEOFORGE_GRADLE_VERSION}}'
    id 'eclipse'
    id 'maven-publish'
}

version = project.mod_version
group = project.maven_group

base {
    archivesName = project.archives_base_name
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
    }
    withSourcesJar()
}

minecraft {
    accessTransformers {
        // file('src/main/resources/META-INF/accesstransformer.cfg')
    }
    runs {
        client {
            workingDirectory file('run')
            property 'neoforge.logging.markers', 'REGISTRIES'
            mods {
                "{{MOD_ID}}" {
                    source sourceSets.main
                }
            }
        }
        server {
            workingDirectory file('run/server')
            property 'neoforge.logging.markers', 'REGISTRIES'
            mods {
                "{{MOD_ID}}" {
                    source sourceSets.main
                }
            }
        }
    }
}

processResources {
    inputs.property "version", project.version
    filesMatching("META-INF/neoforge.mods.toml") {
        expand "version": project.version
    }
}

jar {
    manifest {
        attributes([
                "Specification-Title"   : project.mod_id,
                "Specification-Vendor"  : "{{MOD_NAME}}",
                "Specification-Version" : "1",
                "Implementation-Title"  : project.name,
                "Implementation-Version": project.jar.archiveVersion,
                "Implementation-Vendor" : "{{MOD_NAME}}",
        ])
    }
}
"#;

const ARCHITECTURY_BUILD: &str = r#"// Architectury is a multi-module setup: common/ + fabric/ + forge/ + neoforge/.
// Root build.gradle — the shared plugins for every subproject.
plugins {
    id 'architectury-plugin' version '{{ARCHITECTURY_VERSION}}'
    id 'dev.architectury.architectury' version '{{ARCHITECTURY_VERSION}}'
}

allprojects {
    group = project.maven_group
    version = project.mod_version
}

subprojects {
    apply plugin: 'dev.architectury.loom'
    apply plugin: 'architectury-plugin'
    apply plugin: 'maven-publish'

    base {
        archivesName = project.archives_base_name
    }

    java {
        toolchain {
            languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
        }
    }
}
"#;

const BUKKIT_BUILD: &str = r#"plugins {
    id 'java'
    id 'maven-publish'
}

group = project.maven_group
version = project.mod_version

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
    }
    withSourcesJar()
}

repositories {
    mavenCentral()
    // Bukkit/Spigot/Paper snapshots: pick the one for your platform
    maven { url = 'https://repo.papermc.io/repository/maven-public/' }
    maven { url = 'https://hub.spigotmc.org/nexus/content/repositories/snapshots/' }
}

dependencies {
    // Paper (recommended) — pick the matching version for your Minecraft version:
    // compileOnly 'io.papermc.paper:paper-api:{{MC_VERSION}}-R0.1-SNAPSHOT'
    // Spigot:
    // compileOnly 'org.spigotmc:spigot-api:{{MC_VERSION}}-R0.1-SNAPSHOT'
}

processResources {
    inputs.property "version", project.version
    filesMatching('plugin.yml') {
        expand 'version': project.version
    }
}
"#;

const SPONGE_BUILD: &str = r#"plugins {
    id 'java'
    id 'org.spongepowered.gradle.plugin' version '2.2.0'
    id 'maven-publish'
}

group = project.maven_group
version = project.mod_version

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
    }
    withSourcesJar()
}

repositories {
    mavenCentral()
    maven { url = 'https://repo.spongepowered.org/repository/maven-public/' }
}

sponge {
    apiVersion("8.2.0")
    license("{{LICENSE}}")
    loader {
        name(JavaPlatform.HEADER)
        version("1.0")
    }
    plugin("{{MOD_ID}}") {
        displayName("{{MOD_NAME}}")
        version("${project.version}")
        mainClass("{{PACKAGE}}.{{CLASS_NAME}}")
    }
}
"#;

const VELOCITY_BUILD: &str = r#"plugins {
    id 'java'
    id 'com.github.johnrengelman.shadow' version '8.1.1'
}

group = project.maven_group
version = project.mod_version

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
    }
    withSourcesJar()
}

repositories {
    mavenCentral()
    maven { url = 'https://repo.papermc.io/repository/maven-public/' }
}

dependencies {
    implementation 'com.velocitypowered:velocity-api:{{VELOCITY_API_VERSION}}'
    annotationProcessor 'com.velocitypowered:velocity-api:{{VELOCITY_API_VERSION}}'
}
"#;

const BUNGEECORD_BUILD: &str = r#"plugins {
    id 'java'
    id 'maven-publish'
}

group = project.maven_group
version = project.mod_version

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of({{JAVA_VERSION}})
    }
    withSourcesJar()
}

repositories {
    mavenCentral()
    maven { url = 'https://oss.sonatype.org/content/repositories/snapshots' }
}

dependencies {
    // BungeeCord:
    compileOnly 'net.md-5:bungeecord-api:1.20-R0.2-SNAPSHOT'
    // Waterfall:
    // compileOnly 'io.github.waterfallmc:waterfall-api:1.20-R0.2-SNAPSHOT'
}
"#;

// ---------------------------------------------------------- gradle.properties

pub fn gradle_properties(platform: Platform) -> Template {
    match platform {
        Platform::Fabric => t("Fabric gradle.properties", FABRIC_PROPS),
        Platform::Quilt => t("Quilt gradle.properties", QUILT_PROPS),
        Platform::Forge => t("Forge gradle.properties", FORGE_PROPS),
        Platform::NeoForge => t("NeoForge gradle.properties", NEOFORGE_PROPS),
        Platform::Architectury => t("Architectury gradle.properties", ARCHITECTURY_PROPS),
        _ => t("Bukkit-family gradle.properties", BUKKIT_PROPS),
    }
}

const FABRIC_PROPS: &str = r#"# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx1G

# Fabric Properties
	# check these on https://fabricmc.net/develop
	minecraft_version={{MC_VERSION}}
	yarn_mappings={{MC_VERSION}}+build.1
	loader_version=0.16.5

# Mod Properties
	mod_version={{MOD_VERSION}}
	maven_group=com.example
	archives_base_name={{MOD_ID}}

# Dependencies
	# check this on https://fabricmc.net/develop
	fabric_version=0.102.0+{{MC_VERSION}}
"#;

const QUILT_PROPS: &str = r#"# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx1G

# Quilt Properties
	# check these on https://quiltmc.org/en/usage/
	minecraft_version={{MC_VERSION}}
	quilt_mappings={{MC_VERSION}}+build.32
	loader_version=0.26.1-beta.2

# Mod Properties
	mod_version={{MOD_VERSION}}
	maven_group=com.example
	archives_base_name={{MOD_ID}}
"#;

const FORGE_PROPS: &str = r#"# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx1G

# Forge properties
org.gradle.parallel=true
minecraft_version={{MC_VERSION}}
minecraft_version_range=[1.21,1.22)
mapping_channel=official
mapping_version={{MC_VERSION}}

# Mod properties
mod_id={{MOD_ID}}
mod_name={{MOD_NAME}}
mod_license={{LICENSE}}
mod_version={{MOD_VERSION}}
mod_authors=you
mod_description={{MOD_DESCRIPTION}}

maven_group=com.example
archives_base_name={{MOD_ID}}
"#;

const NEOFORGE_PROPS: &str = r#"# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx1G
org.gradle.parallel=true

# NeoForge properties
minecraft_version={{MC_VERSION}}
minecraft_version_range=[{{MC_VERSION}},1.22)
neoforge_version=21.0.+
mapping_channel=official
mapping_version={{MC_VERSION}}

# Mod properties
mod_id={{MOD_ID}}
mod_name={{MOD_NAME}}
mod_license={{LICENSE}}
mod_version={{MOD_VERSION}}
mod_authors=you
mod_description={{MOD_DESCRIPTION}}

maven_group=com.example
archives_base_name={{MOD_ID}}
"#;

const ARCHITECTURY_PROPS: &str = r#"# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx1G

minecraft_version={{MC_VERSION}}

# Mod properties
mod_id={{MOD_ID}}
mod_name={{MOD_NAME}}
mod_license={{LICENSE}}
mod_version={{MOD_VERSION}}
maven_group=com.example
archives_base_name={{MOD_ID}}

# per-loader versions
fabric_loader_version=0.16.5
fabric_api_version=0.102.0+{{MC_VERSION}}
yarn_mappings={{MC_VERSION}}+build.1
forge_version={{MC_VERSION}}-52.0.0
neoforge_version=21.0.+
"#;

const BUKKIT_PROPS: &str = r#"# Done to increase the memory available to gradle.
org.gradle.jvmargs=-Xmx2G

# Plugin properties
plugin_version={{MOD_VERSION}}
api_version={{MC_VERSION}}
maven_group=com.example
"#;

// ------------------------------------------------------------- mod metadata

pub fn mod_metadata(platform: Platform) -> Template {
    match platform {
        Platform::Fabric => t("fabric.mod.json", FABRIC_MOD_JSON),
        Platform::Quilt => t("quilt.mod.json", FABRIC_MOD_JSON),
        Platform::Forge => t("META-INF/mods.toml", FORGE_MODS_TOML),
        Platform::NeoForge => t("META-INF/neoforge.mods.toml", NEOFORGE_MODS_TOML),
        Platform::Sponge => t("sponge_plugins.json", SPONGE_PLUGINS_JSON),
        Platform::Paper => t("paper-plugin.yml", PAPER_PLUGIN_YML),
        Platform::Bukkit | Platform::Spigot => t("plugin.yml", BUKKIT_PLUGIN_YML),
        Platform::BungeeCord | Platform::Waterfall => t("plugin.yml + bungee.yml", BUKKIT_PLUGIN_YML),
        Platform::Velocity => t("velocity-plugin.json / annotation", VELOCITY_METADATA),
        _ => t("META-INF/mods.toml", FORGE_MODS_TOML),
    }
}

const FABRIC_MOD_JSON: &str = r#"{
  "schemaVersion": 1,
  "id": "{{MOD_ID}}",
  "version": "{{MOD_VERSION}}",

  "name": "{{MOD_NAME}}",
  "description": "{{MOD_DESCRIPTION}}",
  "authors": [],
  "contact": {},

  "license": "{{LICENSE}}",
  "icon": "assets/{{MOD_ID}}/icon.png",

  "environment": "*",
  "entrypoints": {
    "main": [
      "{{PACKAGE}}.{{CLASS_NAME}}"
    ]
  },
  "mixins": [
  ],
  "depends": {
    "fabricloader": ">=0.14.0",
    "minecraft": "~{{MC_VERSION}}",
    "java": ">={{JAVA_VERSION}}"
  }
}
"#;

const FORGE_MODS_TOML: &str = r#"modLoader="javafml"
loaderVersion="[43,)"
license="{{LICENSE}}"

[[mods]]
modId="{{MOD_ID}}"
version="{{MOD_VERSION}}"
displayName="{{MOD_NAME}}"
description='''
{{MOD_DESCRIPTION}}
'''
authors="you"

[[dependencies.{{MOD_ID}}]]
    modId="forge"
    mandatory=true
    versionRange="[{{MC_VERSION}},)"
    ordering="NONE"
    side="BOTH"

[[dependencies.{{MOD_ID}}]]
    modId="minecraft"
    mandatory=true
    versionRange="[{{MC_VERSION}},1.22)"
    ordering="NONE"
    side="BOTH"

[[mixins]]
config="{{MOD_ID}}.mixins.json"
"#;

const NEOFORGE_MODS_TOML: &str = r#"modLoader="javafml"
loaderVersion="[4,)"
license="{{LICENSE}}"

[[mods]]
modId="{{MOD_ID}}"
version="{{MOD_VERSION}}"
displayName="{{MOD_NAME}}"
description='''
{{MOD_DESCRIPTION}}
'''
authors="you"

[[dependencies.{{MOD_ID}}]]
    modId="neoforge"
    mandatory=true
    versionRange="[{{NEOFORGE_VERSION}},)"
    ordering="NONE"
    side="BOTH"

[[mixins]]
config="{{MOD_ID}}.mixins.json"
"#;

const BUKKIT_PLUGIN_YML: &str = r#"name: {{MOD_ID}}
version: '{{MOD_VERSION}}'
main: {{PACKAGE}}.{{CLASS_NAME}}
api-version: '{{MC_VERSION}}'
authors: [you]
description: {{MOD_DESCRIPTION}}
"#;

const PAPER_PLUGIN_YML: &str = r#"name: {{MOD_ID}}
version: '{{MOD_VERSION}}'
main: {{PACKAGE}}.{{CLASS_NAME}}
api-version: '{{MC_VERSION}}'
authors: [you]
description: {{MOD_DESCRIPTION}}
database: false
load: POSTWORLD
"#;

const SPONGE_PLUGINS_JSON: &str = r#"{
  "loader": {
    "name": "java_plain",
    "version": "1.0"
  },
  "license": "{{LICENSE}}",
  "plugins": [
    {
      "id": "{{MOD_ID}}",
      "name": "{{MOD_NAME}}",
      "version": "{{MOD_VERSION}}",
      "entrypoint": "{{PACKAGE}}.{{CLASS_NAME}}",
      "description": "{{MOD_DESCRIPTION}}",
      "branding": {},
      "contributors": [
        {
          "name": "you",
          "description": "Author"
        }
      ],
      "dependencies": [
        {
          "id": "spongeapi",
          "version": "8.2.0",
          "load-order": "after",
          "optional": false
        }
      ]
    }
  ]
}
"#;

const VELOCITY_METADATA: &str = r#"// Velocity reads its metadata from the @Plugin annotation on the main class:
@Plugin(id = "{{MOD_ID}}", name = "{{MOD_NAME}}", version = "{{MOD_VERSION}}")
// Dependencies:
// @Plugin(
//   id = "{{MOD_ID}}",
//   dependencies = { @Dependency(id = "minecraft") }
// )"#;

// ------------------------------------------------------------------- mixins

pub fn mixins_config(platform: Platform) -> Template {
    let base = r#"{
  "required": true,
  "minVersion": "0.8",
  "package": "{{MIXIN_PACKAGE}}",
  "compatibilityLevel": "JAVA_{{JAVA_VERSION}}",
  "mixins": [],
  "client": [],
  "injectors": {
    "defaultRequire": 1
  }
}"#;
    match platform {
        Platform::Fabric | Platform::Quilt => t("{{MOD_ID}}.mixins.json", base),
        Platform::Forge | Platform::NeoForge => t("{{MOD_ID}}.mixins.json", base),
        _ => t("mixins.json (add to your loader)", base),
    }
}

// ------------------------------------------------------------- pack.mcmeta

pub const PACK_MCMETA: &str = r#"{
  "pack": {
    "description": "{{MOD_ID}} resources",
    "pack_format": {{PACK_FORMAT}}{{PACK_COMMENT}}
  }
}"#;

// --------------------------------------------------------- events, commands, registries

pub fn event_handler(platform: Platform) -> Template {
    match platform {
        Platform::Forge => t("Forge event handler", FORGE_EVENT),
        Platform::NeoForge => t("NeoForge event handler", NEOFORGE_EVENT),
        Platform::Fabric | Platform::Quilt => t("Fabric event handler", FABRIC_EVENT),
        Platform::Bukkit | Platform::Spigot | Platform::Paper => t("Bukkit event handler", BUKKIT_EVENT),
        Platform::Sponge => t("Sponge listener", SPONGE_EVENT),
        Platform::Velocity => t("Velocity listener", VELOCITY_EVENT),
        Platform::BungeeCord | Platform::Waterfall => t("BungeeCord listener", BUNGEE_EVENT),
        _ => t("Forge event handler", FORGE_EVENT),
    }
}

const FORGE_EVENT: &str = r#"@Mod.EventBusSubscriber(modid = "{{MOD_ID}}", bus = Mod.EventBusSubscriber.Bus.FORGE)
public class {{CLASS_NAME}}Event {

    @SubscribeEvent
    public static void onServerStarting(ServerStartingEvent event) {
        // Fires when the server starts
    }
}
"#;

const NEOFORGE_EVENT: &str = r#"@EventBusSubscriber(modid = "{{MOD_ID}}", bus = EventBusSubscriber.Bus.GAME)
public class {{CLASS_NAME}}Event {

    @SubscribeEvent
    public static void onServerStarting(ServerStartingEvent event) {
        // Fires when the server starts
    }
}
"#;

const FABRIC_EVENT: &str = r#"import net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents;

public class {{CLASS_NAME}}Event {

    public static void register() {
        ServerLifecycleEvents.SERVER_STARTED.register(server -> {
            // Fires when the dedicated/integrated server starts
        });

        ServerTickEvents.END_SERVER_TICK.register(server -> {
            // Every server tick
        });
    }
}
"#;

const BUKKIT_EVENT: &str = r#"import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;

public class {{CLASS_NAME}}Listener implements Listener {

    @EventHandler
    public void onPlayerJoin(PlayerJoinEvent event) {
        event.getPlayer().sendMessage("Welcome, " + event.getPlayer().getName() + "!");
    }
}
"#;

const SPONGE_EVENT: &str = r#"import org.spongepowered.api.event.Listener;
import org.spongepowered.api.event.network.ServerConnectedEvent;

public class {{CLASS_NAME}}Listener {

    @Listener
    public void onServerConnected(final ServerConnectedEvent event) {
        // Fires when something connects
    }
}
"#;

const VELOCITY_EVENT: &str = r#"import com.velocitypowered.api.event.Subscribe;
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent;

public class {{CLASS_NAME}}Listener {

    @Subscribe
    public void onProxyInitialize(ProxyInitializeEvent event) {
        // Fires when the proxy initialises
    }
}
"#;

const BUNGEE_EVENT: &str = r#"import net.md_5.bungee.api.plugin.Listener;
import net.md_5.bungee.api.event.PostLoginEvent;
import net.md_5.bungee.event.EventHandler;

public class {{CLASS_NAME}}Listener implements Listener {

    @EventHandler
    public void onPostLogin(PostLoginEvent event) {
        event.getPlayer().sendMessage("Welcome!");
    }
}
"#;

pub fn command_class(platform: Platform) -> Template {
    match platform {
        Platform::Forge => t("Forge command (1.21+)", FORGE_COMMAND),
        Platform::NeoForge => t("NeoForge command", NEOFORGE_COMMAND),
        Platform::Fabric | Platform::Quilt => t("Fabric command (fabric-command-api-v2)", FABRIC_COMMAND),
        Platform::Bukkit | Platform::Spigot | Platform::Paper => t("Bukkit command", BUKKIT_COMMAND),
        Platform::Sponge => t("Sponge command", SPONGE_COMMAND),
        Platform::Velocity => t("Velocity command", VELOCITY_COMMAND),
        Platform::BungeeCord | Platform::Waterfall => t("BungeeCord command", BUNGEE_COMMAND),
        _ => t("Bukkit command", BUKKIT_COMMAND),
    }
}

const FORGE_COMMAND: &str = r#"@Mod.EventBusSubscriber(modid = "{{MOD_ID}}", bus = Mod.EventBusSubscriber.Bus.FORGE)
public class {{CLASS_NAME}}Command {

    @SubscribeEvent
    public static void registerCommands(RegisterCommandsEvent event) {
        event.getDispatcher().register(
            Commands.literal("hello")
                .executes(ctx -> {
                    ctx.getSource().sendSuccess(() -> Component.literal("Hello from {{MOD_NAME}}!"), false);
                    return 1;
                })
                .build()
        );
    }
}
"#;

const NEOFORGE_COMMAND: &str = r#"@EventBusSubscriber(modid = "{{MOD_ID}}", bus = EventBusSubscriber.Bus.GAME)
public class {{CLASS_NAME}}Command {

    @SubscribeEvent
    public static void registerCommands(RegisterCommandsEvent event) {
        event.getDispatcher().register(
            Commands.literal("hello")
                .executes(ctx -> {
                    ctx.getSource().sendSuccess(() -> Component.literal("Hello from {{MOD_NAME}}!"), false);
                    return 1;
                })
                .build()
        );
    }
}
"#;

const FABRIC_COMMAND: &str = r#"import static net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.literal;

// In your ModInitializer.onInitialize(): literal("{{MOD_ID}}").register();
public class {{CLASS_NAME}}Command {

    public static void register() {
        literal("hello")
            .executes(ctx -> {
                ctx.getSource().sendMessage(Text.literal("Hello from {{MOD_NAME}}!"));
                return 1;
            })
            .register();
    }
}
"#;

const BUKKIT_COMMAND: &str = r#"import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;

public class {{CLASS_NAME}}Command implements CommandExecutor {

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        sender.sendMessage("Hello from {{MOD_NAME}}!");
        return true;
    }
}
"#;

const SPONGE_COMMAND: &str = r#"// In your plugin's RegisterCommandEvent:
final Parameter.Value<String> nameParam = Parameter.string().key("name").build();
event.register(this.container, Command.builder()
    .addParameter(nameParam)
    .executor(ctx -> {
        final String name = ctx.requireOne(nameParam);
        ctx.sendMessage(Identity.nil(), Component.text("Hello " + name + "!"));
        return CommandResult.success();
    })
    .build(), "hello");
"#;

const VELOCITY_COMMAND: &str = r#"import com.mojang.brigadier.Command;
import com.mojang.brigadier.builder.LiteralArgumentBuilder;
import com.velocitypowered.api.command.BrigadierCommand;
import com.velocitypowered.api.event.Subscribe;
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent;
import com.velocitypowered.api.proxy.ProxyServer;
import net.kyori.adventure.text.Component;

public class {{CLASS_NAME}}Command {

    @Subscribe
    public void onProxyInitialize(ProxyInitializeEvent event) {
        ProxyServer server = event.getProxyServer();
        LiteralArgumentBuilder<CommandSource> root = BrigadierCommand.literalArgumentBuilder("hello")
            .executes(ctx -> {
                ctx.getSource().sendMessage(Component.text("Hello from {{MOD_NAME}}!"));
                return Command.SINGLE_SUCCESS;
            })
            .build();
        server.getCommandManager().register(server.getCommandManager().metaBuilder("hello").build(),
            BrigadierCommand.toRawComponent(root));
    }
}
"#;

const BUNGEE_COMMAND: &str = r#"import net.md_5.bungee.api.CommandSender;
import net.md_5.bungee.api.plugin.Command;

public class {{CLASS_NAME}}Command extends Command {

    public {{CLASS_NAME}}Command() {
        super("hello", "{{MOD_ID}}.command.hello", "h");
    }

    @Override
    public void execute(CommandSender sender, String[] args) {
        sender.sendMessage("Hello from {{MOD_NAME}}!");
    }
}
"#;

pub fn registry_code(platform: Platform) -> Template {
    match platform {
        Platform::Forge => t("Forge DeferredRegister (block + item)", FORGE_REGISTRY),
        Platform::NeoForge => t("NeoForge DeferredRegister (block + item)", NEOFORGE_REGISTRY),
        Platform::Fabric | Platform::Quilt => t("Fabric Registry.register", FABRIC_REGISTRY),
        _ => t("Forge DeferredRegister (block + item)", FORGE_REGISTRY),
    }
}

const FORGE_REGISTRY: &str = r#"// In your mod class:
public static final DeferredRegister<Block> BLOCKS = DeferredRegister.create(ForgeRegistries.BLOCKS, MODID);
public static final DeferredRegister<Item> ITEMS = DeferredRegister.create(ForgeRegistries.ITEMS, MODID);

public static final RegistryObject<Block> EXAMPLE_BLOCK = BLOCKS.register("example_block",
    () -> new Block(BlockBehaviour.Properties.of().strength(2.0f).requiresCorrectToolForDrops()));

public static final RegistryObject<Item> EXAMPLE_BLOCK_ITEM = ITEMS.register("example_block",
    () -> new BlockItem(EXAMPLE_BLOCK.get(), new Item.Properties()));

// In the constructor: BLOCKS.register(modEventBus); ITEMS.register(modEventBus);
"#;

const NEOFORGE_REGISTRY: &str = r#"// In your mod class (neoforge registers automatically):
public static final DeferredBlock<Block> EXAMPLE_BLOCK =
    DeferredBlock.createBlock("example_block", "{{MOD_ID}}");

public static final DeferredItem<Item> EXAMPLE_BLOCK_ITEM =
    DeferredItem.createItem("example_block", "{{MOD_ID}}");

public static final DeferredBlock<Block> EXAMPLE_BLOCK2 =
    DeferredBlock.createBlock("example_block2", "{{MOD_ID}}", block -> new Block(BlockBehaviour.Properties.of().strength(2.0f)));
"#;

const FABRIC_REGISTRY: &str = r#"// In your ModInitializer:
public static final Block EXAMPLE_BLOCK = Blocks.register("{{MOD_ID}}:example_block",
    new Block(AbstractBlock.Settings.create().strength(4.0f));

public static final Item EXAMPLE_BLOCK_ITEM = Items.register("{{MOD_ID}}:example_block",
    new BlockItem(EXAMPLE_BLOCK, new Item.Settings()));
"#;

/// A mixin class skeleton (works on Fabric / Forge / NeoForge).
pub fn mixin_class() -> Template {
    t(
        "Mixin class",
        r#"package {{MIXIN_PACKAGE}};

import net.minecraft.world.entity.player.Player;
import org.objectweb.asm.Opcodes;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

@Mixin(Player.class)
public class {{CLASS_NAME}} {

    @Inject(method = "tick", at = @At("HEAD"))
    private void onTick(CallbackInfo ci) {
        // runs at the start of Player#tick
    }
}
"#,
    )
}
