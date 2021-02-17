#[derive(Queryable)]
pub struct  links  {
        id : i32,
        link: String,
        person_fk : i32,
    }


#[derive(Queryable)]
pub struct    mentions {
        entry : i32,
        person : i32,

}

#[derive(Queryable)]
pub struct  mood  {
        id : i32,
        name: String,
    }


#[derive(Queryable)]
pub struct     nicknames {
        id : i32,
        nickname: String,
        person_fk : i32,
    }


#[derive(Queryable)]
pub struct   password {
        id : i32,
        passwordval: String,
        q1: String,
        q2: String,
        q3: String,
        a1: String,
        a2: String,
        a3: String,
        wipe_attempt : i32,

}

#[derive(Queryable)]
pub struct    person  {
        id : i32,
        name: String,
        description :Option< String>,
        image :Option< String>,

}

#[derive(Queryable)]
pub struct   preferences  {
        id : i32,
        autosave :Option< bool>,
        autolock_interval :i32,
        backup :Option< bool>,
        autodelete_interval :Option< i32>,
        online :bool,
        user : i32,
        themes : i32,

}

#[derive(Queryable)]
pub struct    simping  {
        person : i32,
        entry : i32,
        amount : i32,
        currency: String,
        is_worth : bool,

}

#[derive(Queryable)]
pub struct    tags  {
        id : i32,
        name: String,
        person_fk  :Option< i32>,
        entry_fk  :Option< i32>,

}

#[derive(Queryable)]
pub struct   themes  {
        id : i32,
        name: String,
        main_color: String,
        font_family: String,
        serif :bool,
    }



#[derive(Queryable)]
pub struct    users  {
        id : i32,
        name: String,
        gdrive: String,
        password : i32,
    }


