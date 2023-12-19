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
        id -> Nullable<Integer>,
        date -> Timestamp,
        name -> Text,
        manufacturer -> Text,
        issue -> Text,
        picture -> Nullable<Text>,
        type_plate -> Nullable<Text>,
        confirmed -> Bool,
        guest -> Integer,
        state -> Integer,
    }
}

diesel::table! {
    guest (id) {
        id -> Nullable<Integer>,
        name -> Text,
        phone -> Text,
        user -> Integer,
    }
}

diesel::table! {
    meeting (id) {
        id -> Nullable<Integer>,
        time -> Timestamp,
        confirmed -> Bool,
        cafe -> Integer,
        device -> Integer,
        supporter -> Integer,
    }
}

diesel::table! {
    message (id) {
        id -> Nullable<Integer>,
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
    supporter (id) {
        id -> Nullable<Integer>,
        name -> Text,
        user -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        mail -> Text,
        notifications -> Bool,
    }
}

diesel::joinable!(device -> guest (guest));
diesel::joinable!(device -> state (state));
diesel::joinable!(guest -> user (user));
diesel::joinable!(meeting -> cafe (cafe));
diesel::joinable!(meeting -> device (device));
diesel::joinable!(meeting -> supporter (supporter));
diesel::joinable!(message -> device (device));
diesel::joinable!(message -> user (sender));
diesel::joinable!(supporter -> user (user));

diesel::allow_tables_to_appear_in_same_query!(
    cafe, device, guest, meeting, message, state, supporter, user,
);
