pub use serenity::model::id::UserId as InternalDiscordUserId;
pub use serenity::model::id::MessageId as InternalDiscordMessageId;
pub use serenity::model::id::InteractionId as InternalDiscordInteractionId;
pub use serenity::model::id::ChannelId as InternalDiscordChannelId;
pub use serenity::model::id::GuildId as InternalDiscordGuildId;
pub use serenity::model::id::RoleId as InternalDiscordRoleId;

use diesel::{
    pg::Pg,
    sql_types::Numeric,
};

use crate::wrap_i32;
use crate::{
    wrap::{
        wrap_type,
        wrap_i64,
    },
    PgU64,
};

type DB = Pg;

wrap_type! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    DiscordUserId<DB>(Numeric > PgU64 > InternalDiscordUserId)
        |pgu| {
            InternalDiscordUserId(pgu.0)
        }
        |u| {
            &PgU64(u.0)
        }
}
impl From<InternalDiscordUserId> for DiscordUserId {
    fn from(user_id: InternalDiscordUserId) -> Self {
        Self(user_id)
    }
}

wrap_type! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    DiscordMessageId<DB>(Numeric > PgU64 > InternalDiscordMessageId)
        |pgu| {
            InternalDiscordMessageId(pgu.0)
        }
        |u| {
            &PgU64(u.0)
        }
}
impl From<InternalDiscordMessageId> for DiscordMessageId {
    fn from(message_id: InternalDiscordMessageId) -> Self {
        Self(message_id)
    }
}

wrap_type! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    DiscordInteractionId<DB>(Numeric > PgU64 > InternalDiscordInteractionId)
        |pgu| {
            InternalDiscordInteractionId(pgu.0)
        }
        |u| {
            &PgU64(u.0)
        }
}
impl From<InternalDiscordInteractionId> for DiscordInteractionId {
    fn from(interaction_id: InternalDiscordInteractionId) -> Self {
        Self(interaction_id)
    }
}

wrap_type! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    DiscordChannelId<DB>(Numeric > PgU64 > InternalDiscordChannelId)
        |pgu| {
            InternalDiscordChannelId(pgu.0)
        }
        |u| {
            &PgU64(u.0)
        }
}
impl From<InternalDiscordChannelId> for DiscordChannelId {
    fn from(channel_id: InternalDiscordChannelId) -> Self {
        Self(channel_id)
    }
}

wrap_type! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    DiscordGuildId<DB>(Numeric > PgU64 > InternalDiscordGuildId)
        |pgu| {
            InternalDiscordGuildId(pgu.0)
        }
        |u| {
            &PgU64(u.0)
        }
}
impl From<InternalDiscordGuildId> for DiscordGuildId {
    fn from(guild_id: InternalDiscordGuildId) -> Self {
        Self(guild_id)
    }
}

wrap_type! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    DiscordRoleId<DB>(Numeric > PgU64 > InternalDiscordRoleId)
        |pgu| {
            InternalDiscordRoleId(pgu.0)
        }
        |u| {
            &PgU64(u.0)
        }
}
impl From<InternalDiscordRoleId> for DiscordRoleId {
    fn from(guild_id: InternalDiscordRoleId) -> Self {
        Self(guild_id)
    }
}

wrap_i64!(UserJourneyId<DB>);
wrap_i64!(UserJourneyUIId<DB>);
wrap_i64!(UIMessageId<DB>);
wrap_i64!(ActionId<DB>);

wrap_i64!(ClassChangeCommandId<DB>);

wrap_i64!(DepositCommandId<DB>);

wrap_i64!(TrainCommandId<DB>);
wrap_i64!(TrainTargetId<DB>);

wrap_i64!(SoloCommandId<DB>);

wrap_i64!(BuyCommandId<DB>);

wrap_i64!(TravelCommandId<DB>);
wrap_i64!(ItemUseCommandId<DB>);
wrap_i64!(InventoryCommandId<DB>);
wrap_i64!(SendCommandId<DB>);

wrap_i64!(TradeCommandId<DB>);

wrap_i64!(BotDescriptionId<DB>);

wrap_i32!(RoleId<DB>);
wrap_i32!(RoleUserJunctionId<DB>);
wrap_i32!(DiscordRoleRoleJunctionId<DB>);
wrap_i32!(RoleRestrictedCommandId<DB>);
