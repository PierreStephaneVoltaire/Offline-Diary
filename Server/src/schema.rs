table! {
    links (id) {
        id -> Double,
        link -> Text,
        person_fk -> Double,
    }
}

table! {
    mentions (entry, person) {
        entry -> Double,
        person -> Double,
    }
}

table! {
    mood (id) {
        id -> Double,
        name -> Text,
    }
}

table! {
    nicknames (id) {
        id -> Double,
        nickname -> Text,
        person_fk -> Double,
    }
}

table! {
    password (id) {
        id -> Double,
        passwordval -> Text,
        q1 -> Text,
        q2 -> Text,
        q3 -> Text,
        a1 -> Text,
        a2 -> Text,
        a3 -> Text,
        wipe_attempt -> Double,
    }
}

table! {
    person (id) {
        id -> Double,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

table! {
    preferences (id) {
        id -> Double,
        autosave -> Nullable<Integer>,
        autolock_interval -> Integer,
        backup -> Nullable<Integer>,
        autodelete_interval -> Nullable<Integer>,
        online -> Integer,
        user -> Double,
        themes -> Double,
    }
}

table! {
    simping (person, entry) {
        person -> Double,
        entry -> Double,
        amount -> Double,
        currency -> Text,
        is_worth -> Double,
    }
}

table! {
    tags (id) {
        id -> Double,
        name -> Text,
        person_fk -> Nullable<Double>,
        entry_fk -> Nullable<Double>,
    }
}

table! {
    themes (id) {
        id -> Double,
        name -> Text,
        main_color -> Text,
        font_family -> Text,
        serif -> Integer,
    }
}

table! {
    users (id) {
        id -> Double,
        name -> Text,
        gdrive -> Text,
        password -> Double,
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
