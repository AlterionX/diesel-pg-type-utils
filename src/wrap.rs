#[macro_export]
macro_rules! impl_sql_convert {
    (
        <$db:ty>
        $sql:ty > $intermediate:ty > $final:ty
            |$forward:ident| $convert_forward:block
            |$backward:ident| $convert_backward:block
    ) => {
        impl $crate::diesel::deserialize::FromSql<$sql, $db> for $final {
            fn from_sql(bytes: <$db as $crate::diesel::backend::Backend>::RawValue<'_>) -> $crate::diesel::deserialize::Result<Self> {
                let intermediate = <
                    $intermediate
                    as
                    $crate::diesel::deserialize::FromSql<$sql, $db>
                >::from_sql(bytes)?;
                let $forward = intermediate;
                let converted = $convert_forward;
                Ok(converted)
            }
        }

        impl $crate::diesel::serialize::ToSql<$sql, $db> for $final {
            fn to_sql<'b>(&'b self, out: &mut $crate::diesel::serialize::Output<'b, '_, $db>) -> $crate::diesel::serialize::Result {
                let $backward = self;
                let intermediate = $convert_backward;
                <$intermediate as $crate::diesel::serialize::ToSql<$sql, $db>>::to_sql(
                    &intermediate,
                    &mut out.reborrow()
                )
            }
        }

    }
}

#[macro_export]
macro_rules! wrap_type {
    {
        $(#[derive($($trait:ident),+)])*
        $name:ident <$db: ty> ( $sql:ty > $intermediate:ty > $wrapped:ty )
            |$forward:ident| $convert_forward:block
            |$backward:ident| $convert_backward:block
    } => {
        $(#[derive($($trait),+)])*
        #[derive($crate::serde::Serialize, $crate::serde::Deserialize)]
        #[derive($crate::diesel::AsExpression, $crate::diesel::FromSqlRow)]
        #[diesel(sql_type = $sql)]
        pub struct $name($wrapped);

        impl $crate::diesel::deserialize::FromSql<$sql, $db> for $name {
            fn from_sql(bytes: <$db as $crate::diesel::backend::Backend>::RawValue<'_>) -> $crate::diesel::deserialize::Result<Self> {
                let intermediate = <
                    $intermediate
                    as
                    $crate::diesel::deserialize::FromSql<$sql, $db>
                >::from_sql(bytes)?;
                let $forward = intermediate;
                let converted = $convert_forward;
                Ok($name(converted))
            }
        }

        impl $crate::diesel::serialize::ToSql<$sql, $db> for $name {
            fn to_sql<'b>(&'b self, out: &mut $crate::diesel::serialize::Output<'b, '_, $db>) -> $crate::diesel::serialize::Result {
                let $backward = self.0;
                let intermediate = $convert_backward;
                <$intermediate as $crate::diesel::serialize::ToSql<$sql, $db>>::to_sql(
                    &intermediate,
                    &mut out.reborrow()
                )
            }
        }

        impl From<$name> for $wrapped {
            fn from(f: $name) -> Self {
                f.0
            }
        }

        impl $name {
            pub fn assume_valid(f: $wrapped) -> Self {
                Self(f)
            }

            pub fn inner(&self) -> &$wrapped {
                &self.0
            }
        }
    };
    (
        $(#[derive($($trait:ident),+)])*
        $name:ident < $db:ty > ( $sql:ty > $wrapped:ty )
    ) => {
        $crate::wrap::wrap_type! {
            $(#[derive($($trait),+)])*
            $name<$db>($sql > $wrapped > $wrapped) |junk| { junk } |junk| { &junk }
        }
    };
}

#[macro_export]
macro_rules! wrap_i32 {
    {
        $(#[derive($($trait:ident),+)])*
        $name:ident<$db:ty>
    } => {
        $crate::wrap::wrap_type! {
            $(#[derive($($trait),+)])*
            #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
            $name<$db>($crate::diesel::sql_types::Integer > i32)
        }
    };
}

#[macro_export]
macro_rules! wrap_i64 {
    {
        $(#[derive($($trait:ident),+)])*
        $name:ident<$db:ty>
    } => {
        $crate::wrap::wrap_type! {
            $(#[derive(($trait),+)])*
            #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
            $name<$db>($crate::diesel::sql_types::BigInt > i64)
        }
    };
}

#[macro_export]
macro_rules! wrap_u32 {
    {
        $(#[derive($($trait:ident),+)])*
        $name:ident<$db:ty>
    } => {
        $crate::wrap::wrap_type! {
            $(#[derive($($trait),+)])*
            #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
            $name<$db>($crate::diesel::sql_types::BigInt > $crate::PgU32 > u32)
            |u| {
                u.into()
            }
            |u| {
                &$crate::PgU32::from(u)
            }
        }
    };
}

#[macro_export]
macro_rules! wrap_u64 {
    {
        $(#[derive($($trait:ident),+)])*
        $name:ident<$db:ty>
    } => {
        $crate::wrap::wrap_type! {
            $(#[derive($($trait),+)])*
            #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
            $name<$db>($crate::diesel::sql_types::Numeric > $crate::PgU64 > u64)
            |u| {
                u.into()
            }
            |u| {
                &$crate::PgU64::from(u)
            }
        }
    };
}

pub use impl_sql_convert;
pub use wrap_type;
pub use wrap_i32;
pub use wrap_i64;
pub use wrap_u64;
pub use wrap_u32;

#[cfg(test)]
mod test {
    use crate::wrap;
    use diesel::pg::Pg;

    // Test each of the specialized impls.
    wrap::wrap_i32!(OldId<Pg>);
    wrap::wrap_u64!(AssetId<Pg>);
    wrap::wrap_i64!(NewId<Pg>);
}
