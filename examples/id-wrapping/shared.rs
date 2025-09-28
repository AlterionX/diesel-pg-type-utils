use diesel::pg::Pg;

use diesel_pg_type_utils::wrap::wrap_i32;

type DB = Pg;

wrap_i32!(PremiumUserDataId<DB>);
