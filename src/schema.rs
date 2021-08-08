table! {
    accounts (account_id) {
        account_id -> Int4,
        user_id -> Int4,
        value -> Nullable<Float4>,
    }
}

table! {
    transactions (transaction_id) {
        transaction_id -> Int4,
        account_id -> Int4,
        description -> Nullable<Text>,
        before -> Nullable<Float4>,
        after -> Nullable<Float4>,
        amount -> Nullable<Float4>,
        meta -> Nullable<Json>,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(accounts -> users (user_id));
joinable!(transactions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
    users,
);
