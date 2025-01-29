use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ScriptType {
    SimpleScript,
    PlutusScriptV1,
    PlutusScriptV2,
    PlutusScriptV3,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScriptDetails {
    cbor_hex: String,
    description: String,
    #[serde(rename = "type")]
    script_type: ScriptType,
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    script_language: String,
    script: ScriptDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    #[serde(default)]
    lovelace: u64,
    #[serde(flatten)]
    assets: HashMap<String, HashMap<String, i64>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TxOut {
    address: String,
    value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_script: Option<Option<Script>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datumhash: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_datum: Option<Option<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_datumhash: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_datum_raw: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datum: Option<Option<String>>,
}

type UTxO = HashMap<String, TxOut>;

fn validate_script(script: &Script) -> Result<(), String> {
    if !validate_hex_string(&script.script.cbor_hex) {
        return Err("Invalid hex in script".to_string());
    }

    if script.script_language.is_empty() {
        return Err("Script language cannot be empty".to_string());
    }

    Ok(())
}

fn validate_hex_string(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_value(value: &Value) -> Result<(), String> {
    for (policy_id, assets) in &value.assets {
        if policy_id.len() != 56 || !validate_hex_string(policy_id) {
            return Err(format!("Invalid Policy ID: {}", policy_id));
        }

        if assets.is_empty() {
            return Err(format!(
                "Asset map for policy {} cannot be empty",
                policy_id
            ));
        }

        for (asset_name, _) in assets {
            if asset_name.len() > 64 || !validate_hex_string(asset_name) {
                return Err(format!("Invalid asset name: {}", asset_name));
            }
        }
    }

    Ok(())
}

pub fn validate_json(json: String) -> Result<(), Box<dyn std::error::Error>> {
    let key_regex = Regex::new(r"^[0-9a-f]{64}#[0-9]+$")?;

    let json: UTxO = serde_json::from_str(&json)?;

    for (utxo_ref, tx_out) in json.iter() {
        if !key_regex.is_match(utxo_ref) {
            return Err(format!("Invalid UTxO ref: {}", utxo_ref).into());
        }

        if tx_out.address.is_empty() {
            return Err(format!("Empty address in UTxO: {}", utxo_ref).into());
        }

        validate_value(&tx_out.value)
            .map_err(|e| format!("Failed to validate value in UtxO {}: {}", utxo_ref, e))?;

        if let Some(Some(script)) = &tx_out.reference_script {
            validate_script(script)
                .map_err(|e| format!("Failed to validate script in UTxO {}: {}", utxo_ref, e))?;
        }

        if let Some(Some(datumhash)) = &tx_out.datumhash {
            if !validate_hex_string(datumhash) {
                return Err(format!("Invalid datumhash format in UTxO: {}", utxo_ref).into());
            }
        }

        if let Some(Some(inline_datumhash)) = &tx_out.inline_datumhash {
            if !validate_hex_string(inline_datumhash) {
                return Err(
                    format!("Invalid inline_datumhash format in UTxO: {}", utxo_ref).into(),
                );
            }
        }

        if let Some(Some(datum)) = &tx_out.datum {
            if !validate_hex_string(datum) {
                return Err(format!("Invalid datum format in UTxO: {}", utxo_ref).into());
            }
        }
    }

    Ok(())
}
