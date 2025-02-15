use jmt::KeyHash;
use penumbra_crypto::{asset, note, Nullifier};
use penumbra_tct::Root;

pub fn token_supply(asset_id: &asset::Id) -> KeyHash {
    format!("shielded_pool/assets/{}/token_supply", asset_id).into()
}

pub fn known_assets() -> KeyHash {
    "shielded_pool/known_assets".into()
}

pub fn denom_by_asset(asset_id: &asset::Id) -> KeyHash {
    format!("shielded_pool/assets/{}/denom", asset_id).into()
}

pub fn note_source(note_commitment: &note::Commitment) -> KeyHash {
    format!("shielded_pool/note_source/{}", note_commitment).into()
}

pub fn compact_block(height: u64) -> KeyHash {
    format!("shielded_pool/compact_block/{}", height).into()
}

pub fn anchor_by_height(height: &u64) -> KeyHash {
    format!("shielded_pool/anchor/{}", height).into()
}

pub fn anchor_lookup(anchor: &Root) -> KeyHash {
    format!("shielded_pool/valid_anchors/{}", anchor).into()
}

pub fn spent_nullifier_lookup(nullifier: &Nullifier) -> KeyHash {
    format!("shielded_pool/spent_nullifiers/{}", nullifier).into()
}

pub fn commission_amounts(height: u64) -> KeyHash {
    format!("staking/commission_amounts/{}", height).into()
}
