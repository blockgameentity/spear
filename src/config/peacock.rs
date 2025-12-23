use octocrab::Octocrab;
use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct PeacockOption {
    pub key: String,
    pub value: String,
    pub description: String,
    pub possible_values: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct PeacockCategory {
    pub name: String,
    pub options: Vec<PeacockOption>,
}

#[derive(Deserialize)]
struct FlagDef {
    category: String,
    title: String,
    desc: String,
    default: Value,
    #[serde(rename = "possibleValues")]
    possible_values: Option<Vec<String>>,
    #[serde(rename = "showIngame")]
    show_ingame: Option<bool>,
}

#[derive(Deserialize)]
struct SectionDef {
    flags: HashMap<String, FlagDef>,
}

#[derive(Deserialize)]
struct DefaultFlags {
    peacock: SectionDef,
}

pub fn get_peacock_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("spear");
    path.push("peacock");
    path.push("options.ini");
    path
}

fn extract_object(text: &str) -> Option<String> {
    let start = text.find("export const defaultFlags")?;
    let after = &text[start..];
    let eq_pos = after.find('=')?;
    let after_eq = &after[eq_pos + 1..];
    let brace_start = after_eq.find('{')?;
    let mut brace_count = 0;
    let mut end_pos = 0;
    for (i, c) in after_eq[brace_start..].char_indices() {
        match c {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    end_pos = brace_start + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    if end_pos > 0 {
        Some(after_eq[brace_start..end_pos].to_string())
    } else {
        None
    }
}

const HARDCODED_FLAGS_TS: &str = r#"/*
 *     The Peacock Project - a HITMAN server replacement.
 *     Copyright (C) 2021-2025 The Peacock Project Team
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Affero General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU Affero General Public License for more details.
 *
 *     You should have received a copy of the GNU Affero General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { existsSync, readFileSync, writeFileSync } from "fs"
import type { FlagSection, Flags } from "./types/types"
import { log, LogLevel } from "./loggingInterop"
import { IIniObjectSection, parse } from "js-ini"
import type { IIniObject } from "js-ini/lib/interfaces/ini-object"

let tempFlags: IIniObject = {}
let flags: IIniObject = {}

export const defaultFlags: Flags = {
    peacock: {
        title: "Peacock",
        desc: "All options for Peacock.",
        flags: {
            gameplayUnlockAllShortcuts: {
                category: "Gameplay",
                title: "gameplayUnlockAllShortcuts",
                desc: "When set to true, all shortcuts will always be unlocked.",
                default: false,
            },
            gameplayUnlockAllFreelancerMasteries: {
                category: "Gameplay",
                title: "gameplayUnlockAllFreelancerMasteries",
                desc: "When set to true, all Freelancer unlocks will always be available.",
                default: false,
            },
            mapDiscoveryState: {
                category: "Gameplay",
                title: "mapDiscoveryState",
                desc: 'Decides what to do with the discovery state of the maps. REVEALED will reset all map locations to discovered, CLOUDED will reset all maps to undiscovered, and KEEP will keep your current discovery state. Note that these actions will take effect every time you connect to Peacock. Your progress of the "Discover [Location]" challenges will not be affected by this option.',
                possibleValues: ["REVEALED", "CLOUDED", "KEEP"],
                default: "KEEP",
            },
            enableMasteryProgression: {
                category: "Gameplay",
                title: "enableMasteryProgression",
                desc: "When set to false, mastery progression will be disabled and all unlockables will be awarded at the beginning",
                default: true,
            },
            enableIsolatedUnlockables: {
                category: "Gameplay",
                title: "enableIsolatedUnlockables",
                desc: "Decides if items are unlocked when there are no associated unlocking approaches. Requires enableMasteryProgression to be true.",
                default: false,
            },
            elusivesAreShown: {
                category: "Gameplay",
                title: "elusivesAreShown",
                desc: "Show elusive targets in instinct like normal targets would appear on normal missions. (for speedrunners who are submitting to speedrun.com, just as a reminder, this tool is for practice only!)",
                default: false,
            },
            enableContractsModeSaving: {
                category: "Gameplay",
                title: "enableContractsModeSaving",
                desc: "Enable saving in Contracts Mode, including both usercreated and featured contracts. Please note that even if you enable this, the saving function is consistent with the setting in the contract.",
                default: false,
            },
            legacyNoticedKillScoring: {
                category: "Gameplay",
                title: "legacyNoticedKillScoring",
                desc: 'In the HITMAN 2016 engine, if noticed kills should behave in the official way ("vanilla"), or how they were previously handled by Peacock ("sane")',
                possibleValues: ["vanilla", "sane"],
                default: "vanilla",
            },
            legacyElusivesEnableSaving: {
                category: "Services",
                title: "legacyElusivesEnableSaving",
                desc: 'When set to true, playing elusive target missions in Hitman 2016 will share the same restarting/replanning/saving rules with normal missions, but the "Elusive Target [Location]" challenges will not be completable. These challenges will only be completable when this option is set to false.',
                default: false,
            },
            getDefaultSuits: {
                category: "Services",
                title: "getDefaultSuits",
                desc: 'Set this to true to add all the default starting suits to your inventory. Note: If you set both this and "enableMasteryProgression" to "true" at the same time, a starting suit that is also the unlock for a challenge/mastery will be locked behind its challenge/mastery.',
                default: false,
            },
            jokes: {
                category: "Services",
                title: "jokes",
                desc: "The Peacock server window will tell you a joke on startup if this is set to true.",
                default: false,
            },
            leaderboards: {
                category: "Services",
                title: "leaderboards",
                desc: "Allow your times to be submitted to the ingame leaderboards. If you do not want your times on the leaderboards, change this to false.",
                default: true,
            },
            updateChecking: {
                category: "Services",
                title: "updateChecking",
                desc: "Allow Peacock to check for updates on startup.",
                default: true,
            },
            loadoutSaving: {
                category: "Services",
                title: "loadoutSaving",
                desc: "Default loadout mode - either PROFILES (loadout profiles) or LEGACY for per-user saving",
                possibleValues: ["PROFILES", "LEGACY"],
                default: "PROFILES",
            },
            legacyContractDownloader: {
                category: "Services",
                title: "legacyContractDownloader",
                desc: "When set to true, the official servers will be used for contract downloading in H3, which only works for the platform you are playing on. When false, the HITMAPS servers will be used instead. Note that this option only pertains to H3. Official servers will be used for H1 and H2 regardless of the value of this option.",
                default: false,
            },
            imageLoading: {
                category: "Services",
                title: "imageLoading",
                desc: "How images are loaded. SAVEASREQUESTED will fetch images from online when needed (and save them in the images folder), ONLINE will fetch them without saving, and OFFLINE will load them from the image folder",
                possibleValues: ["SAVEASREQUESTED", "ONLINE", "OFFLINE"],
                default: "SAVEASREQUESTED",
            },
            liveSplit: {
                category: "Splitter",
                title: "LiveSplit",
                desc: "Toggle LiveSplit support on or off",
                default: false,
            },
            autoSplitterCampaign: {
                category: "Splitter",
                title: "Campaign for AutoSplitter",
                desc: "Which (main) campaign to use for the AutoSplitter. Can be set to 1, 2, 3, or 'trilogy'.",
                possibleValues: ["1", "2", "3", "trilogy"],
                default: "trilogy",
            },
            autoSplitterRacetimegg: {
                category: "Splitter",
                title: "AutoSplitter with racetime.gg",
                desc: "When set to true, autosplitter is set in a special mode for use with livesplit integration for racetime.gg realtime races.",
                default: false,
            },
            autoSplitterForceSilentAssassin: {
                category: "Splitter",
                title: "Only split when Silent Assassin",
                desc: "When set to true, the autosplitter will only accept missions completed with silent assassin to be valid completions. When false, any completion will split.",
                default: true,
            },
            discordRp: {
                category: "Discord",
                title: "Discord rich presence",
                desc: "Toggle Discord rich presence on or off.",
                default: false,
            },
            discordRpAppTime: {
                category: "Discord",
                title: "discordRpAppTime",
                desc: "For Discord Rich Presence, if set to false, the time playing the current level will be shown, and if set to true, the total time using Peacock will be shown.",
                default: false,
            },
            overrideFrameworkChecks: {
                category: "Modding",
                title: "overrideFrameworkChecks",
                desc: "Forcibly disable installed mod checks",
                default: false,
            },
            frameworkDeploySummaryPath: {
                category: "Modding",
                title: "frameworkDeploySummaryPath",
                desc: 'The path of Simple Mod Framework\'s deploy summary file. By default, it is set to "AUTO", which will attempt to locate the file in predefined locations. Alternatively, you can specify a custom path.',
                default: "AUTO",
                showIngame: false,
            },
            experimentalHMR: {
                category: "Experimental",
                title: "experimentalHMR",
                desc: "Toggle hot reloading of contracts",
                default: false,
            },
            developmentPluginDevHost: {
                category: "Development",
                title: "developmentPluginDevHost",
                desc: "[Workspace required] Toggle loading of plugins with a .ts/.cts extension inside the /plugins folder",
                default: false,
            },
            leaderboardsHost: {
                category: "Development",
                title: "leaderboardsHost",
                desc: "Please do not modify - intended for development only",
                default: "https://backend.rdil.rocks",
                showIngame: false,
            },
            developmentLogRequests: {
                category: "Development",
                title: "developmentLogRequests",
                desc: "When set to true, will log the body of all requests the game makes. This can cause huge log files!",
                default: false,
            },
        },
    },
}

const FLAGS_FILE = "options.ini"

/**
 * Get a flag from the flag file.
 *
 * @param flagId The flag's name.
 * @returns The flag's value.
 */
export function getFlag(flagId: string): string | boolean | number {
    const { section, flag } = convertFlagId(flagId)

    const tempSection = flags[section] as IIniObjectSection

    if (!tempSection) {
        return defaultFlags[section].flags[flag].default
    }

    return (
        (tempSection[flag] as string | boolean | number) ??
        defaultFlags[section].flags[flag].default
    )
}

export function setFlag(
    flagId: string,
    value: string | boolean | number,
): void {
    const { section, flag } = convertFlagId(flagId)

    const tempSection = flags[section] as IIniObjectSection
    tempSection[flag] = value
}

function convertFlagId(flagId: string) {
    const splittedFlagId = flagId.split(".")
    const sectionKey =
        splittedFlagId.length === 1 ? "peacock" : splittedFlagId[0]
    const flagKey =
        splittedFlagId.length === 1 ? splittedFlagId[0] : splittedFlagId[1]

    return {
        section: sectionKey,
        flag: flagKey,
    }
}

export function saveFlags() {
    const lines: string[] = []

    Object.keys(defaultFlags).forEach((sectionKey) => {
        const defaultSection = defaultFlags[sectionKey]
        const section = flags[sectionKey] as IIniObjectSection

        lines.push(`; ${defaultSection.title} - ${defaultSection.desc}`)
        lines.push(`[${sectionKey}]`)

        const flagsKeys = Object.keys(defaultSection.flags)

        for (const flagKey of flagsKeys) {
            const defaultFlag = defaultSection.flags[flagKey]
            const flag = section[flagKey]

            const category = defaultFlag.category
                ? `[${defaultFlag.category}] `
                : ""

            lines.push(
                `; ${category}${defaultFlag.title || flag} - ${defaultFlag.desc}`,
            )
            lines.push(`${flagKey}=${flag}`)
            lines.push("")
        }
    })

    writeFileSync(FLAGS_FILE, lines.join("\n"))
}

/**
 * Loads all flags.
 */
export function loadFlags(): void {
    if (!existsSync(FLAGS_FILE)) {
        writeFileSync(FLAGS_FILE, "")
    }

    // Load the current INI-file
    tempFlags = parse(readFileSync(FLAGS_FILE).toString())

    if (!tempFlags["peacock"]) {
        // This is an options file from before the rewrite.
        tempFlags = {
            peacock: tempFlags,
        }
    }

    // Create a new INI-file
    flags = {}

    // Re-create the default flags in the new INI-file, but keep the existing values from the current INI-file.
    // NOTE: This will intentionally drop any non-existing sections/flags!
    Object.keys(defaultFlags).forEach(loadFlagSection)

    log(LogLevel.DEBUG, "Loaded all default flags.")
}

function loadFlagSection(sectionKey: string) {
    flags[sectionKey] = {}

    const defaultFlagKeys = Object.keys(defaultFlags[sectionKey].flags)

    defaultFlagKeys.forEach((flag) => {
        const currentFlagValue = tempFlags[sectionKey]
            ? (tempFlags[sectionKey] as IIniObjectSection)[flag]
            : undefined

        const tempSection = flags[sectionKey] as IIniObjectSection
        tempSection[flag] =
            currentFlagValue ?? defaultFlags[sectionKey].flags[flag].default
    })
}

export function registerFlagSection(sectionKey: string, section: FlagSection) {
    defaultFlags[sectionKey] = section

    loadFlagSection(sectionKey)
}

/**
 * Get the values of all flags. Only intended for debugging purposes, since this could cause memory issues.
 *
 * @internal
 * @return The flags.
 */
export function getAllFlags(): IIniObject {
    return flags
}
"#;

pub fn parse_peacock_defaults() -> Vec<PeacockCategory> {
    log::info!("[+] Parsing Peacock defaults");
    // Try to fetch from GitHub
    match fetch_peacock_defaults() {
        Ok(categories) => {
            log::info!(
                "[+] Fetched defaults from GitHub, categories: {}",
                categories.len()
            );
            return categories;
        }
        Err(e) => {
            log::error!("[+] Fetch failed with error: {}", e);
            log::info!("[+] Using hardcoded flags.ts");
        }
    }
    
    let obj_str = extract_object(HARDCODED_FLAGS_TS)
        .ok_or("Failed to extract object")
        .unwrap();
    let default_flags: DefaultFlags = json5::from_str(&obj_str).unwrap();
    let mut options = vec![];
    for (key, flag) in &default_flags.peacock.flags {
        let value = match &flag.default {
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => "".to_string(),
        };
        options.push(PeacockOption {
            key: key.clone(),
            value,
            description: flag.desc.clone(),
            possible_values: flag.possible_values.clone(),
        });
    }
    log::info!("[+] Parsed hardcoded flags.ts, options: {}", options.len());
    vec![PeacockCategory {
        name: "peacock".to_string(),
        options,
    }]
}

fn fetch_peacock_defaults() -> Result<Vec<PeacockCategory>, Box<dyn std::error::Error>> {
    log::info!("[+] Fetching Peacock defaults from GitHub");
    let config = crate::config::spear::load_spear_config();
    let repo = &config.peacock_github_repo;
    log::info!("[+] Using repo: {}", repo);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let tag = rt
        .block_on(async {
            let octocrab = Octocrab::builder().build().map_err(|e| {
                log::error!("[+] Failed to build Octocrab: {}", e);
                e
            })?;
            let release = octocrab
                .repos(
                    repo.split('/').next().unwrap().to_string(),
                    repo.split('/').nth(1).unwrap().to_string(),
                )
                .releases()
                .get_latest()
                .await
                .map_err(|e| {
                    log::error!("[+] Failed to get latest release from GitHub API: {}", e);
                    e
                })?;
            Ok::<String, Box<dyn std::error::Error>>(release.tag_name)
        })
        .map_err(|e| {
            log::error!("[+] Octocrab fetch failed: {}", e);
            e
        })?;
    log::info!("[+] Latest tag: {}", tag);

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://raw.githubusercontent.com/{}/refs/tags/{}/components/flags.ts",
        repo, tag
    );
    log::info!("[+] Fetching flags.ts from: {}", url);
    let text = client
        .get(&url)
        .send()
        .map_err(|e| {
            log::error!("[+] Failed to send GET request to {}: {}", url, e);
            e
        })?
        .text()
        .map_err(|e| {
            log::error!("[+] Failed to read text from response: {}", e);
            e
        })?;
    log::info!("[+] Downloaded flags.ts, length: {}", text.len());
    let obj_str = extract_object(&text).ok_or_else(|| {
        log::error!("[+] Failed to extract object from flags.ts");
        "Failed to extract object"
    })?;
    log::info!("[+] Extracted object, length: {}", obj_str.len());
    let default_flags: DefaultFlags = json5::from_str(&obj_str).map_err(|e| {
        log::error!("[+] Failed to parse extracted object as JSON5: {}", e);
        e
    })?;
    log::info!(
        "[+] Parsed flags, count: {}",
        default_flags.peacock.flags.len()
    );
    let mut options = vec![];
    for (key, flag) in &default_flags.peacock.flags {
        let value = match &flag.default {
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => "".to_string(),
        };
        options.push(PeacockOption {
            key: key.clone(),
            value,
            description: flag.desc.clone(),
            possible_values: flag.possible_values.clone(),
        });
    }
    log::info!("[+] Created options, count: {}", options.len());
    Ok(vec![PeacockCategory {
        name: "peacock".to_string(),
        options,
    }])
}

pub fn load_peacock_config() -> Vec<PeacockCategory> {
    let path = get_peacock_config_path();
    log::info!("[+] Loading Peacock config from: {:?}", path);
    let mut defaults = parse_peacock_defaults();
    if path.exists() {
        log::info!("[+] Config file exists, loading values");
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current_section: Option<String> = None;
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                current_section = Some(line[1..line.len() - 1].to_string());
            } else if !line.is_empty() && !line.starts_with(';') {
                if let Some(eq_idx) = line.find('=') {
                    let key = line[..eq_idx].trim().to_string();
                    let value = line[eq_idx + 1..].trim().to_string();
                    if let Some(sec) = &current_section {
                        if let Some(cat) = defaults.iter_mut().find(|c| &c.name == sec) {
                            if let Some(opt) = cat.options.iter_mut().find(|o| o.key == key) {
                                opt.value = value;
                            }
                        }
                    }
                }
            }
        }
    } else {
        log::info!("[+] Config file does not exist, saving defaults");
        save_peacock_config(&defaults);
    }
    log::info!("[+] Loaded Peacock config, categories: {}", defaults.len());
    defaults
}

pub fn save_peacock_config(categories: &Vec<PeacockCategory>) {
    let path = get_peacock_config_path();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut content = String::new();
    for cat in categories {
        content.push_str(&format!("[{}]\n", cat.name));
        for opt in &cat.options {
            content.push_str(&format!("{}={}\n", opt.key, opt.value));
        }
        content.push('\n');
    }
    fs::write(&path, content).unwrap();
}
