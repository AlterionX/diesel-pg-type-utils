use std::{hash::Hash, collections::HashMap};

use diesel::{
    sql_types::{
        Nullable,
        Integer,
        VarChar,
        SingleValue,
        BigInt,
    },
    sql_function, QueryResult,
};

sql_function! {
    #[sql_name = "COALESCE"]
    fn coalesce2<T: SingleValue>(v0: Nullable<T>, base: T) -> T
}

sql_function! {
    #[sql_name = "COALESCE"]
    fn coalesce3<T: SingleValue>(v0: Nullable<T>, v1: Nullable<T>, base: T) -> T
}

sql_function! {
    #[sql_name = "COALESCE"]
    fn coalesce_i(x: Nullable<Integer>, y: Integer) -> Integer
}

pub use coalesce_i::coalesce_i as coalesce_i_module;

sql_function! {
    #[sql_name = "COALESCE"]
    fn coalesce_s(x: Nullable<VarChar>, y: VarChar) -> VarChar
}

sql_function! {
    #[sql_name = "COALESCE"]
    fn coalesce_n(x: Nullable<BigInt>, y: BigInt) -> BigInt
}

sql_function! {
    #[sql_name = "LOWER"]
    fn lower(a: VarChar) -> VarChar
}

sql_function! {
    #[sql_name = "ARRAY_AGG"]
    #[aggregate]
    fn array_agg<T: SingleValue>(a: T) -> Array<T>
}

sql_function! {
    #[sql_name = "RANDOM"]
    fn random() -> Integer
}

pub fn match_only<K: Eq + Hash, V>(result: QueryResult<HashMap<K, V>>, key: &K) -> QueryResult<V> {
    not_opt(result.map(|mut m| m.remove(key)))
}

pub fn first_only<V>(result: QueryResult<Vec<V>>) -> QueryResult<V> {
    not_opt(result.map(|r| r.into_iter().next()))
}

pub fn not_opt<T>(result: QueryResult<Option<T>>) -> QueryResult<T> {
    result.and_then(|a| a.ok_or(diesel::result::Error::NotFound))
}
