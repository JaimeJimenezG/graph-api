// @generated automatically by Diesel CLI.

diesel::table! {
    charts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
    }
}

diesel::table! {
    components (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    group_navigations (id) {
        id -> Int4,
        group_id -> Int4,
        navigation_id -> Int4,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    group_users (id) {
        id -> Int4,
        group_id -> Int4,
        user_id -> Int4,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    navigations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url -> Varchar,
        public -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        component_id -> Int4,
        #[max_length = 255]
        icon -> Varchar,
        order -> Int4,
    }
}

diesel::table! {
    user_navigations (id) {
        id -> Int4,
        user_id -> Int4,
        navigation_id -> Int4,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        second_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        nickname -> Varchar,
    }
}

diesel::joinable!(charts -> users (user_id));
diesel::joinable!(group_navigations -> groups (group_id));
diesel::joinable!(group_navigations -> navigations (navigation_id));
diesel::joinable!(group_users -> groups (group_id));
diesel::joinable!(group_users -> users (user_id));
diesel::joinable!(navigations -> components (component_id));
diesel::joinable!(user_navigations -> navigations (navigation_id));
diesel::joinable!(user_navigations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    charts,
    components,
    group_navigations,
    group_users,
    groups,
    navigations,
    user_navigations,
    users,
);
