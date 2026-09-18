use mod_api_stable::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

const MOD_ID: &str = "tfm2_real_world_free_agent_cleanup";
const SAVE_SCHEMA_VERSION: usize = 2;
const CONTRACT_CORRECTIONS: &[(&str, &str, &str)] = &[
    ("zyko", "supernova", "darkzero dragonsteel"),
    ("zekas", "vivo keyd stars", "vivo keyd stars academy"),
];

fn normalized_name(name: &str) -> String {
    name.split_whitespace()
        .flat_map(|part| part.chars().flat_map(char::to_lowercase).chain([' ']))
        .collect::<String>()
        .trim()
        .to_owned()
}

fn normalized_json(value: &Value) -> String {
    value
        .to_string()
        .to_lowercase()
        .replace([' ', '_', '-'], "")
}

fn is_retired(value: &Value) -> bool {
    normalized_json(value).contains("retired")
}

fn is_free_agent(value: &Value) -> bool {
    let compact = normalized_json(value);
    compact.contains("freeagent") || compact.contains("\"teamid\":null") || compact == "null"
}

fn find_team_id(value: &Value) -> Option<usize> {
    match value {
        Value::Object(map) => {
            for key in ["team_id", "teamId", "team"] {
                if let Some(id) = map.get(key).and_then(Value::as_u64) {
                    return Some(id as usize);
                }
            }
            map.values().find_map(find_team_id)
        }
        Value::Array(values) => values.iter().find_map(find_team_id),
        _ => None,
    }
}

fn retired_value_like(active: &Value, sample: Option<&Value>) -> Value {
    if let Some(sample) = sample {
        return sample.clone();
    }
    match active {
        Value::String(_) => Value::String("Retired".to_owned()),
        Value::Object(map) if map.len() == 1 => {
            let payload = map.values().next().cloned().unwrap_or(Value::Null);
            serde_json::json!({"Retired": payload})
        }
        _ => Value::String("Retired".to_owned()),
    }
}

#[derive(Clone)]
struct AthleteRecord {
    id: usize,
    display_name: String,
    name: String,
    contract: Value,
    retirement: Value,
}

fn verified_stale_contract_ids(
    athletes: &[AthleteRecord],
    team_names: &HashMap<usize, String>,
) -> Vec<usize> {
    let contracted = athletes
        .iter()
        .filter_map(|athlete| {
            let team_id = find_team_id(&athlete.contract)?;
            let team = team_names.get(&team_id)?;
            Some((athlete.id, athlete.name.as_str(), team.as_str()))
        })
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    for (player_name, canonical_team, stale_team) in CONTRACT_CORRECTIONS {
        let canonical_exists = contracted
            .iter()
            .any(|(_, name, team)| name == player_name && team == canonical_team);
        if canonical_exists {
            result.extend(
                contracted
                    .iter()
                    .filter(|(_, name, team)| name == player_name && team == stale_team)
                    .map(|(id, _, _)| *id),
            );
        }
    }
    result
}

struct CleanupServerExtension;

impl StableServerExtension for CleanupServerExtension {
    fn on_server_start(&self, ctx: &mut StableServerCtx<'_>) {
        let athletes = ctx
            .record_ids(RecordKindV1::Athlete)
            .into_iter()
            .filter_map(|id| {
                let display_name = ctx.record_get_string(RecordKindV1::Athlete, id, "name")?;
                let contract = ctx
                    .record_get_json(RecordKindV1::Athlete, id, "contract")
                    .and_then(|json| serde_json::from_str(&json).ok())?;
                let retirement = ctx
                    .record_get_json(RecordKindV1::Athlete, id, "retirement")
                    .and_then(|json| serde_json::from_str(&json).ok())?;
                Some(AthleteRecord {
                    id,
                    name: normalized_name(&display_name),
                    display_name,
                    contract,
                    retirement,
                })
            })
            .collect::<Vec<_>>();

        let team_names = ctx
            .record_ids(RecordKindV1::Team)
            .into_iter()
            .filter_map(|id| {
                ctx.record_get_string(RecordKindV1::Team, id, "name")
                    .map(|name| (id, normalized_name(&name)))
            })
            .collect::<HashMap<_, _>>();

        let rostered_names = athletes
            .iter()
            .filter(|athlete| !is_retired(&athlete.retirement))
            .filter(|athlete| !is_free_agent(&athlete.contract))
            .map(|athlete| athlete.name.clone())
            .filter(|name| !name.is_empty())
            .collect::<HashSet<_>>();

        let mut targets = athletes
            .iter()
            .filter(|athlete| !is_retired(&athlete.retirement))
            .filter(|athlete| is_free_agent(&athlete.contract))
            .filter(|athlete| rostered_names.contains(&athlete.name))
            .map(|athlete| athlete.id)
            .collect::<Vec<_>>();

        for id in verified_stale_contract_ids(&athletes, &team_names) {
            if !targets.contains(&id) {
                targets.push(id);
            }
        }

        let retired_sample = athletes
            .iter()
            .find(|athlete| is_retired(&athlete.retirement))
            .map(|athlete| athlete.retirement.clone());
        let mut cleaned_names = Vec::new();
        for id in targets {
            let Some(athlete) = athletes.iter().find(|athlete| athlete.id == id) else {
                continue;
            };
            let retired = retired_value_like(&athlete.retirement, retired_sample.as_ref());
            if ctx.record_set_json(
                RecordKindV1::Athlete,
                id,
                "retirement",
                &retired.to_string(),
            ) {
                cleaned_names.push(athlete.display_name.clone());
            }
        }

        let names = cleaned_names.join(", ");
        ctx.save_set_version(SAVE_SCHEMA_VERSION);
        let _ = ctx.save_set_string("last_cleanup_count", &cleaned_names.len().to_string());
        let _ = ctx.save_set_string("last_cleanup_names", &names);
        println!(
            "[{MOD_ID}] retired {} duplicate or verified stale athlete record(s){}",
            cleaned_names.len(),
            if names.is_empty() {
                String::new()
            } else {
                format!(": {names}")
            }
        );
    }
}

fn init(host: &StableHost) -> StableMod {
    host.log(
        LogLevel::Info,
        "Real World Free Agent Cleanup 0.3.0 stable module initialized",
    );
    let mut registration = StableMod::new(MOD_ID);
    registration.set_server_extension(CleanupServerExtension);
    registration
}

declare_stable_mod!(init);

#[cfg(test)]
mod tests {
    use super::{find_team_id, is_free_agent, is_retired, normalized_name, retired_value_like};
    use serde_json::json;

    #[test]
    fn normalizes_case_and_whitespace() {
        assert_eq!(normalized_name("  Faker\t "), "faker");
        assert_eq!(normalized_name("Lee   Sang-hyeok"), "lee sang-hyeok");
    }

    #[test]
    fn recognizes_supported_contract_shapes() {
        assert!(is_free_agent(&json!("FreeAgent")));
        assert!(is_free_agent(&json!({"team_id": null})));
        assert!(!is_free_agent(&json!({"team_id": 12})));
        assert_eq!(find_team_id(&json!({"contract":{"team_id":12}})), Some(12));
    }

    #[test]
    fn mirrors_the_games_retirement_encoding() {
        assert!(is_retired(&json!("Retired")));
        assert_eq!(retired_value_like(&json!("Active"), None), json!("Retired"));
        assert_eq!(
            retired_value_like(&json!("Active"), Some(&json!({"Retired":null}))),
            json!({"Retired":null})
        );
    }
}
