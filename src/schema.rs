table! {
    tasks (task_id) {
        task_id -> Int4,
        detail -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_uuid -> Uuid,
        name -> Varchar,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
