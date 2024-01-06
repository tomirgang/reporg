// @generated automatically by Diesel CLI.

diesel::table! {
    cafe (id) {
        id -> Integer,
        location -> Text,
        address -> Text,
        date -> Timestamp,
    }
}

diesel::table! {
    device (id) {
        id -> Integer,
        date -> Timestamp,
        name -> Text,
        manufacturer -> Text,
        issue -> Text,
        picture -> Nullable<Text>,
        type_plate -> Nullable<Text>,
        confirmed -> Bool,
        user -> Integer,
        state -> Integer,
    }
}

diesel::table! {
    meeting (id) {
        id -> Integer,
        time -> Timestamp,
        confirmed -> Bool,
        cafe -> Integer,
        device -> Integer,
        supporter -> Integer,
    }
}

diesel::table! {
    message (id) {
        id -> Integer,
        parent -> Integer,
        content -> Text,
        date -> Timestamp,
        device -> Integer,
        sender -> Integer,
    }
}

diesel::table! {
    state (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        #[sql_name = "final"]
        final_ -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        phone -> Text,
        notifications -> Bool,
        roles -> Integer,
    }
}

diesel::joinable!(device -> state (state));
diesel::joinable!(device -> user (user));
diesel::joinable!(meeting -> cafe (cafe));
diesel::joinable!(meeting -> device (device));
diesel::joinable!(meeting -> user (supporter));
diesel::joinable!(message -> device (device));
diesel::joinable!(message -> user (sender));

diesel::allow_tables_to_appear_in_same_query!(cafe, device, meeting, message, state, user,);
