table! {
    links (id) {
        id -> Integer,
        link -> Text,
        person_fk -> Integer,
    }
}

table! {
    mentions (entry, person) {
        entry -> Integer,
        person -> Integer,
    }
}

table! {
    mood (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    nicknames (id) {
        id -> Integer,
        nickname -> Text,
        person_fk -> Integer,
    }
}

table! {
    password (id) {
        id -> Integer,
        passwordval -> Text,
        q1 -> Text,
        q2 -> Text,
        q3 -> Text,
        a1 -> Text,
        a2 -> Text,
        a3 -> Text,
        wipe_attempt -> Integer,
    }
}

table! {
    person (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

table! {
    preferences (id) {
        id -> Integer,
        autosave -> Nullable<Integer>,
        autolock_interval -> Integer,
        backup -> Nullable<Integer>,
        autodelete_interval -> Nullable<Integer>,
        online -> Integer,
        user -> Integer,
        themes -> Integer,
    }
}

table! {
    simping (person, entry) {
        person -> Integer,
        entry -> Integer,
        amount -> Integer,
        currency -> Text,
        is_worth -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        person_fk -> Nullable<Integer>,
        entry_fk -> Nullable<Integer>,
    }
}

table! {
    themes (id) {
        id -> Integer,
        name -> Text,
        main_color -> Text,
        font_family -> Text,
        serif -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        gdrive -> Text,
        password -> Integer,
    }
}

joinable!(links -> person (person_fk));
joinable!(mentions -> person (person));
joinable!(nicknames -> person (person_fk));
joinable!(preferences -> themes (themes));
joinable!(preferences -> users (user));
joinable!(simping -> person (person));
joinable!(tags -> person (person_fk));
joinable!(users -> password (password));

allow_tables_to_appear_in_same_query!(
    links,
    mentions,
    mood,
    nicknames,
    password,
    person,
    preferences,
    simping,
    tags,
    themes,
    users,
);
