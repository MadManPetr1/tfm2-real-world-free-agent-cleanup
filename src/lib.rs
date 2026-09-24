use game_core::RetirementState;
use mod_api::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

const MOD_ID: &str = "tfm2_real_world_free_agent_cleanup";
const SAVE_SCHEMA_VERSION: usize = 1;
const CLIENT_RESCAN_FRAMES: usize = 300;
const CONTRACT_CORRECTIONS: &[(&str, &str, &str)] = &[
    ("zyko", "supernova", "darkzero dragonsteel"),
    ("zekas", "vivo keyd stars", "vivo keyd stars academy"),
];

fn normalized_name(name: &str) -> String {
    let mut normalized = String::with_capacity(name.len());
    for part in name.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        for character in part.chars() {
            normalized.extend(character.to_lowercase());
        }
    }
    normalized
}

fn contract_team_id(athlete: &Athlete) -> Option<usize> {
    athlete.contract.team_id()
}

fn verified_stale_contract_ids(records: &[(usize, String, String)]) -> Vec<usize> {
    let mut stale_ids = Vec::new();

    for &(player_name, canonical_team, stale_team) in CONTRACT_CORRECTIONS {
        let canonical_exists = records
            .iter()
            .any(|(_, name, team)| name == player_name && team == canonical_team);
        if !canonical_exists {
            continue;
        }

        for (athlete_id, name, team) in records {
            if name == player_name && team == stale_team && !stale_ids.contains(athlete_id) {
                stale_ids.push(*athlete_id);
            }
        }
    }

    stale_ids
}

struct CleanupServerExtension;

impl ModServerExtension for CleanupServerExtension {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        let rostered_names: HashSet<String> = ctx
            .database
            .athletes
            .iter()
            .filter(|athlete| {
                !athlete.contract.is_free_agent()
                    && !matches!(athlete.retirement, RetirementState::Retired)
            })
            .map(|athlete| normalized_name(&athlete.name))
            .filter(|name| !name.is_empty())
            .collect();

        let mut duplicates: Vec<(usize, String)> = ctx
            .database
            .athletes
            .iter()
            .filter(|athlete| {
                athlete.contract.is_free_agent()
                    && matches!(athlete.retirement, RetirementState::Active)
            })
            .filter(|athlete| rostered_names.contains(&normalized_name(&athlete.name)))
            .map(|athlete| (athlete.id, athlete.name.clone()))
            .collect();

        let contracted_records = ctx
            .database
            .athletes
            .iter()
            .filter(|athlete| {
                !athlete.contract.is_free_agent()
                    && !matches!(athlete.retirement, RetirementState::Retired)
            })
            .filter_map(|athlete| {
                let team_id = contract_team_id(athlete)?;
                let team = ctx.database.teams.get(team_id)?;
                Some((
                    athlete.id,
                    athlete.name.clone(),
                    normalized_name(&athlete.name),
                    normalized_name(&team.name),
                ))
            })
            .collect::<Vec<_>>();

        let correction_records = contracted_records
            .iter()
            .map(|(athlete_id, _, name, team)| (*athlete_id, name.clone(), team.clone()))
            .collect::<Vec<_>>();
        for stale_id in verified_stale_contract_ids(&correction_records) {
            if !duplicates.iter().any(|(id, _)| *id == stale_id) {
                let display_name = contracted_records
                    .iter()
                    .find(|(id, _, _, _)| *id == stale_id)
                    .map(|(_, display_name, _, _)| display_name.clone())
                    .unwrap_or_default();
                duplicates.push((stale_id, display_name));
            }
        }

        let duplicate_count = duplicates.len();
        for (athlete_id, _) in &duplicates {
            if let Some(athlete) = ctx.database.athletes.get_mut(*athlete_id) {
                athlete.retirement = RetirementState::Retired;
            }
        }

        let names = duplicates
            .iter()
            .map(|(_, name)| name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        ctx.database
            .mod_save_data
            .set_version(MOD_ID, SAVE_SCHEMA_VERSION);
        ctx.database.mod_save_data.set_string(
            MOD_ID,
            "last_cleanup_count",
            duplicate_count.to_string(),
        );
        ctx.database
            .mod_save_data
            .set_string(MOD_ID, "last_cleanup_names", names.clone());

        println!(
            "[{MOD_ID}] retired {duplicate_count} duplicate free agent(s){}",
            if names.is_empty() {
                String::new()
            } else {
                format!(": {names}")
            }
        );
    }
}

struct CleanupClientExtension {
    last_database_id: AtomicUsize,
    update_counter: AtomicUsize,
}

impl ModExtension for CleanupClientExtension {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else {
            self.last_database_id.store(usize::MAX, Ordering::Release);
            self.update_counter.store(0, Ordering::Release);
            return;
        };

        let database_id = data.db().id;
        let database_changed =
            self.last_database_id.swap(database_id, Ordering::AcqRel) != database_id;
        let update = self.update_counter.fetch_add(1, Ordering::AcqRel);
        if !database_changed && !update.is_multiple_of(CLIENT_RESCAN_FRAMES) {
            return;
        }

        let duplicate_ids = {
            let database = data.db();
            let active_names: HashSet<String> = database
                .athletes
                .values()
                .filter(|athlete| {
                    !athlete.contract.is_free_agent()
                        && !matches!(athlete.retirement, RetirementState::Retired)
                })
                .map(|athlete| normalized_name(&athlete.name))
                .filter(|name| !name.is_empty())
                .collect();

            let mut duplicate_ids = database
                .athletes
                .values()
                .filter(|athlete| {
                    athlete.contract.is_free_agent()
                        && active_names.contains(&normalized_name(&athlete.name))
                })
                .map(|athlete| athlete.id)
                .collect::<Vec<_>>();

            let contracted_records = database
                .athletes
                .values()
                .filter(|athlete| {
                    !athlete.contract.is_free_agent()
                        && !matches!(athlete.retirement, RetirementState::Retired)
                })
                .filter_map(|athlete| {
                    let team_id = contract_team_id(athlete)?;
                    let team = database.teams.get(&team_id)?;
                    Some((
                        athlete.id,
                        normalized_name(&athlete.name),
                        normalized_name(&team.name),
                    ))
                })
                .collect::<Vec<_>>();

            for athlete_id in verified_stale_contract_ids(&contracted_records) {
                if !duplicate_ids.contains(&athlete_id) {
                    duplicate_ids.push(athlete_id);
                }
            }

            duplicate_ids
        };

        if duplicate_ids.is_empty() {
            return;
        }

        let mut database = data.db_mut();
        let mut removed = 0;
        for athlete_id in duplicate_ids {
            removed += usize::from(database.athletes.remove(&athlete_id).is_some());
        }

        println!("[{MOD_ID}] removed {removed} duplicate record(s) from the client snapshot");
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(CleanupClientExtension {
        last_database_id: AtomicUsize::new(usize::MAX),
        update_counter: AtomicUsize::new(0),
    });
    reg.set_server_extension(CleanupServerExtension);
    reg
}

declare_mod!(init);

#[cfg(test)]
mod tests {
    use super::{normalized_name, verified_stale_contract_ids};

    #[test]
    fn normalizes_case_and_whitespace() {
        assert_eq!(normalized_name("  Faker\t "), "faker");
        assert_eq!(normalized_name("Lee   Sang-hyeok"), "lee sang-hyeok");
    }

    #[test]
    fn applies_only_verified_contract_pair_corrections() {
        let records = vec![
            (1, "zyko".to_string(), "supernova".to_string()),
            (2, "zyko".to_string(), "darkzero dragonsteel".to_string()),
            (3, "zyko".to_string(), "unrelated team".to_string()),
            (
                4,
                "zekas".to_string(),
                "vivo keyd stars academy".to_string(),
            ),
        ];

        assert_eq!(verified_stale_contract_ids(&records), vec![2]);
    }
}
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
