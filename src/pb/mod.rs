mod sf_near_v1;

use crate::pb::SignatureType::{Ed25519, Secp256k1};
use near_crypto;

use near_indexer::near_primitives;
use near_indexer::near_primitives::views as near_views;
use near_indexer::near_primitives::views::ChunkHeaderView;
use near_indexer::StreamerMessage;
pub use sf_near_v1::*;
use std::fmt::{Display, Formatter};

impl From<&near_indexer::StreamerMessage> for BlockWrapper {
    fn from(sm: &StreamerMessage) -> Self {
        let h = sm.block.header.clone();
        // if let Some(ap) = h.approvals.as_ref()[0] {}

        let block = Block {
            header: Some(BlockHeader::from(h)),
            author: sm.block.author.clone(),
            chunks: sm
                .block
                .chunks
                .clone()
                .into_iter()
                .map(|ch| ChunkHeader::from(ch))
                .collect(),
        };

        BlockWrapper {
            block: Some(block),
            shards: vec![],
            state_changes: vec![],
        }
    }
}

impl From<near_views::BlockHeaderView> for BlockHeader {
    fn from(h: near_views::BlockHeaderView) -> Self {
        BlockHeader {
            height: h.height,
            prev_height: 0, //todo: this is v3 feature, what that means?
            epoch_id: Some(CryptoHash::from(h.epoch_id)),
            next_epoch_id: Some(CryptoHash::from(h.next_epoch_id)),
            hash: Some(CryptoHash::from(h.hash)),
            prev_hash: Some(CryptoHash::from(h.prev_hash)),
            prev_state_root: Some(CryptoHash::from(h.prev_state_root)),
            chunk_receipts_root: Some(CryptoHash::from(h.chunk_receipts_root)),
            chunk_headers_root: Some(CryptoHash::from(h.chunk_headers_root)),
            chunk_tx_root: Some(CryptoHash::from(h.chunk_tx_root)),
            outcome_root: Some(CryptoHash::from(h.outcome_root)),
            chunks_included: h.chunks_included,
            challenges_root: Some(CryptoHash::from(h.challenges_root)),
            timestamp: h.timestamp,
            timestamp_nanosec: h.timestamp_nanosec,
            random_value: Some(CryptoHash::from(h.random_value)),
            validator_proposals: h
                .validator_proposals
                .into_iter()
                .map(|p| ValidatorStake::from(p))
                .collect(),
            chunk_mask: h.chunk_mask,
            gas_price: Some(BigInt::from(h.gas_price)),
            block_ordinal: 0, //todo: this is v3 feature, what that means?
            validator_reward: Some(BigInt::from(h.validator_reward)),
            total_supply: Some(BigInt::from(h.total_supply)),
            challenges_result: h
                .challenges_result
                .into_iter()
                .map(|cr| SlashedValidator::from(cr))
                .collect(),
            last_final_block: Some(CryptoHash::from(h.last_final_block)),
            last_ds_final_block: Some(CryptoHash::from(h.last_ds_final_block)),
            next_bp_hash: Some(CryptoHash::from(h.next_bp_hash)),
            block_merkle_root: Some(CryptoHash::from(h.block_merkle_root)),
            epoch_sync_data_hash: vec![], //todo: this is v3 feature, what that means?
            approvals: vec![], //todo: need to find a way to make Signature::from work with near_crypto::Signature
            signature: None, //todo: need to find a way to make Signature::from work with near_crypto::Signature
            latest_protocol_version: h.latest_protocol_version,
        }
    }
}

impl From<near_views::ChunkHeaderView> for ChunkHeader {
    fn from(ch: ChunkHeaderView) -> Self {
        ChunkHeader {
            chunk_hash: Vec::from(ch.chunk_hash),
            prev_block_hash: Vec::from(ch.prev_block_hash),
            outcome_root: Vec::from(ch.outcome_root),
            prev_state_root: Vec::from(ch.prev_state_root),
            encoded_merkle_root: Vec::from(ch.encoded_merkle_root),
            encoded_length: ch.encoded_length,
            height_created: ch.height_created,
            height_included: ch.height_included,
            shard_id: ch.shard_id,
            gas_used: ch.gas_used,
            gas_limit: ch.gas_limit,
            validator_reward: Some(BigInt::from(ch.validator_reward)),
            balance_burnt: Some(BigInt::from(ch.balance_burnt)),
            outgoing_receipts_root: Vec::from(ch.outgoing_receipts_root),
            tx_root: Vec::from(ch.tx_root),
            validator_proposals: ch
                .validator_proposals
                .into_iter()
                .map(|vp| ValidatorStake::from(vp))
                .collect(),
            signature: None, //todo: need to find a way to make Signature::from work with near_crypto::Signature
        }
    }
}

impl From<near_crypto::Signature> for Signature {
    fn from(sign: near_crypto::Signature) -> Self {
        match sign {
            near_crypto::Signature::ED25519(s) => Signature {
                r#type: Ed25519.into(),
                bytes: Vec::from(s.to_bytes()),
            } as Signature,
            near_crypto::Signature::SECP256K1(s) => {
                let data = Vec::from(<[u8; 65]>::from(s));
                Signature {
                    r#type: Secp256k1.into(),
                    bytes: data,
                }
            }
        }
    }
}

impl From<near_primitives::challenge::SlashedValidator> for SlashedValidator {
    fn from(sv: near_primitives::challenge::SlashedValidator) -> Self {
        SlashedValidator {
            account_id: sv.account_id,
            is_double_sign: sv.is_double_sign,
        }
    }
}

impl From<near_primitives::views::validator_stake_view::ValidatorStakeView> for ValidatorStake {
    fn from(sv: near_primitives::views::validator_stake_view::ValidatorStakeView) -> Self {
        ValidatorStake {
            account_id: sv.account_id,
            public_key: Some(PublicKey::from(sv.public_key.key_data())),
            stake: Some(BigInt::from(sv.stake)),
        }
    }
}

impl From<u128> for BigInt {
    fn from(i: u128) -> Self {
        BigInt {
            bytes: Vec::from(i.to_be_bytes()),
        }
    }
}

impl From<&[u8]> for PublicKey {
    fn from(data: &[u8]) -> Self {
        PublicKey {
            bytes: Vec::from(data),
        }
    }
}

impl From<near_primitives::hash::CryptoHash> for CryptoHash {
    fn from(h: near_primitives::hash::CryptoHash) -> Self {
        CryptoHash {
            bytes: Vec::from(h),
        }
    }
}

impl Display for BlockWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "here block {}",
            self.block.as_ref().unwrap().header.as_ref().unwrap().height
        )
    }
}
