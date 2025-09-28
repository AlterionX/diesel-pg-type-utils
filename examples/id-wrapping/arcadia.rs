use diesel::pg::Pg;

use diesel_pg_type_utils::wrap::{
    wrap_i64,
    wrap_i32,
    wrap_u64,
};

type DB = Pg;

wrap_i32!(MaxTransactionAgeId<DB>);

wrap_i32!(UserId<DB>);
wrap_u64!(AssetId<DB>);

wrap_i32!(SoloScenarioId<DB>);
wrap_i32!(SoloScenarioWeightsId<DB>);
wrap_i32!(SoloScenarioResolutionStrategyId<DB>);
wrap_i32!(SoloScenarioChoiceId<DB>);
wrap_i32!(SoloScenarioChoiceRaceJunctionId<DB>);
wrap_i32!(SoloRewardsId<DB>);

wrap_i32!(SoloExpeditionId<DB>);

wrap_i32!(WeeklyScenarioId<DB>);

wrap_i32!(WeeklyExpeditionId<DB>);

wrap_i32!(ProtagonistId<DB>);
wrap_u64!(ProtagonistAssetId<DB>);
impl ProtagonistAssetId {
    /// Note: the inverse of this is technically not always correct, so is omitted.
    pub fn as_asset_id(&self) -> AssetId {
        AssetId(self.0)
    }
}

wrap_i32!(LevelingCostId<DB>);

wrap_i32!(RaceId<DB>);
wrap_i32!(ClassId<DB>);

wrap_i32!(StatId<DB>);
wrap_i32!(ModifierId<DB>);

wrap_i32!(TrainingCostId<DB>);
wrap_i64!(TrainingSessionId<DB>);
wrap_i64!(TrainingParticipantId<DB>);

wrap_i32!(ListingId<DB>);
wrap_i32!(ItemListingId<DB>);
wrap_i32!(CardListingId<DB>);
wrap_i32!(BotItemListingId<DB>);

wrap_i32!(ItemId<DB>);
wrap_u64!(CardId<DB>);
impl CardId {
    /// Note: the inverse of this is technically not always correct, so is omitted.
    pub fn as_asset_id(&self) -> AssetId {
        AssetId(self.0)
    }
}
wrap_i32!(BotItemId<DB>);
wrap_i32!(ConsumableArchetypeId<DB>);
wrap_i32!(CollectibleArchetypeId<DB>);
wrap_i32!(EquipmentArchetypeId<DB>);
wrap_i32!(ArtifactArchetypeId<DB>);
wrap_i32!(ResourceArchetypeId<DB>);

wrap_i32!(EquippedItemId<DB>);

wrap_i32!(ConsumableEffectId<DB>);

wrap_i32!(InventoryEntryId<DB>);

wrap_i32!(EquipmentSlotId<DB>);
wrap_i32!(EquipmentSlotLayoutId<DB>);
wrap_i32!(EquipmentSlotLayoutEntryId<DB>);

wrap_i32!(LocationId<DB>);

wrap_i32!(LocationTagId<DB>);
wrap_i32!(LocationTagSetId<DB>);
wrap_i32!(LocationTagSetEntryId<DB>);

wrap_i64!(WorldId<DB>);


wrap_i32!(JourneyDurationId<DB>);
wrap_i32!(JourneyId<DB>);

wrap_i32!(WanderScenarioId<DB>);
wrap_i32!(WanderId<DB>);

wrap_i32!(ItemUsageId<DB>);

wrap_i64!(CaravanId<DB>);
wrap_i64!(CaravanMemberId<DB>);

wrap_i64!(TransactionId<DB>);
wrap_i64!(ParticipantId<DB>);
wrap_i64!(DepositId<DB>);
wrap_i64!(OfferId<DB>);
wrap_i64!(PoolId<DB>);

wrap_i64!(ShopId<DB>);
wrap_i64!(ShopEntryId<DB>);
wrap_i64!(ShopLimitId<DB>);
wrap_i64!(ShopLimitShopId<DB>);
wrap_i64!(ShopLimitEntryId<DB>);
wrap_i64!(ShopPurchaseId<DB>);

wrap_i64!(Id<DB>);
