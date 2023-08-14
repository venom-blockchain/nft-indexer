use anyhow::Result;
use async_trait::async_trait;
use indexer_repo::types::{Address, EventCategory, EventRecord, EventType};
use sqlx::PgPool;

use crate::persistence::entities::{Decode, Decoded};
use crate::{
    models::events::{DirectSellDeclined, DirectSellDeployed},
    settings::whitelist::{OfferRootType, TRUSTED_ADDRESSES},
    utils::{EventMessageInfo, KeyInfo},
};

use super::Entity;

#[async_trait]
impl Entity for DirectSellDeployed {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeployed,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: indexer_repo::actions::get_collection_by_nft(
                &self.nft.to_string().into(),
                &mut pg_pool_tx,
            )
            .await,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectSell]
            .contains(&event_record.address.0)
        {
            indexer_repo::actions::add_whitelist_address(
                &self.direct_sell.to_string().into(),
                &mut pg_pool_tx,
            )
            .await?;
        }

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save DirectSellDeployed event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl Entity for DirectSellDeclined {
    async fn save_to_db(&self, pg_pool: &PgPool, msg_info: &EventMessageInfo) -> Result<()> {
        let mut pg_pool_tx = pg_pool.begin().await?;

        let event_record = EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        };

        let save_result = indexer_repo::actions::save_event(&event_record, &mut pg_pool_tx)
            .await
            .expect("Failed to save DirectSellDeclined event");
        if save_result.rows_affected() == 0 {
            pg_pool_tx.rollback().await?;
            return Ok(());
        }

        pg_pool_tx.commit().await?;

        Ok(())
    }
}

impl Decode for DirectSellDeployed {
    fn decode(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        let emitter_address: Address = msg_info.tx_data.get_account().into();

        if TRUSTED_ADDRESSES.get().unwrap()[&OfferRootType::FactoryDirectSell]
            .contains(&emitter_address.0)
        {
            Ok(Decoded::DirectSellDeployed(emitter_address))
        } else {
            Ok(Decoded::ShouldSkip)
        }
    }

    fn decode_event(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeployed,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,
            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}

impl Decode for DirectSellDeclined {
    fn decode(&self, _msg_info: &EventMessageInfo) -> Result<Decoded> {
        Ok(Decoded::ShouldSkip)
    }

    fn decode_event(&self, msg_info: &EventMessageInfo) -> Result<Decoded> {
        Ok(Decoded::RawEventRecord(EventRecord {
            event_category: EventCategory::DirectSell,
            event_type: EventType::DirectSellDeclined,

            address: msg_info.tx_data.get_account().into(),
            created_lt: msg_info.tx_data.logical_time() as i64,
            created_at: msg_info.tx_data.get_timestamp(),
            message_hash: msg_info.message_hash.to_string(),
            nft: Some(self.nft.to_string().into()),
            collection: None,

            raw_data: serde_json::to_value(self).unwrap_or_default(),
        }))
    }
}
