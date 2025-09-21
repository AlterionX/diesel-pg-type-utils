use diesel::pg::Pg;

use crate::wrap::wrap_i32;

type DB = Pg;

wrap_i32!(PremiumUserDataId<DB>);
