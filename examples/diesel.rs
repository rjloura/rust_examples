#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

// extern crate dotenv;

use crate::evacuateobjects::dsl::*;
use diesel::debug_query;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use libmanta::moray::MantaObject;
use serde_json;
use uuid::Uuid;

use dotenv::dotenv;
use std::env;

#[derive(DbEnum, Debug, PartialEq, Clone)]
pub enum EvacuateObjectStatus {
    Unprocessed,
    Processing,
    Skipped,
    Failed,
    Retrying,
}

pub type ObjectId = String;
pub type AssignmentId = String;

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
#[table_name = "evacuateobjects"]
pub struct EvacuateObjectDB {
    pub id: String,
    pub object: String,
    pub assignment: AssignmentId,
    pub status: EvacuateObjectStatus,
}

table! {
    use diesel::sql_types::{Text};
    use super::EvacuateObjectStatusMapping;
    evacuateobjects (id) {
        id -> Text,
        object -> Text,
        assignment -> Text,
        status -> EvacuateObjectStatusMapping,
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(":memory:"));

    dbg!(&database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_object(conn: &SqliteConnection, toggle_status: bool) -> String {
    let mut manta_object = MantaObject::default();
    let mut stat = EvacuateObjectStatus::Skipped;
    if toggle_status {
        stat = EvacuateObjectStatus::Unprocessed;
    }

    manta_object.object_id = Uuid::new_v4().to_string();
    dbg!(&manta_object);

    // create a function that copies this function?
    let manta_bin = serde_json::to_string(&manta_object).unwrap();
    let obj = EvacuateObjectDB {
        id: manta_object.object_id.clone(),
        object: manta_bin,
        assignment: Uuid::new_v4().to_string(),
        status: stat,
    };
    let query = diesel::insert_into(evacuateobjects::table).values(&obj);

    let sql = debug_query::<Sqlite, _>(&query).to_string();
    dbg!(sql);

    query.execute(conn).expect("Error Inserting object");

    obj.id
}

pub fn create_table(conn: &SqliteConnection) {
    conn.execute(r#"DROP TABLE evacuateobjects"#)
        .unwrap_or_else(|_| {
            println!("table exists?");
            0
        });

    conn.execute(
        r#"
        CREATE TABLE evacuateobjects(
            id TEXT PRIMARY KEY,
            object BLOB,
            assignment TEXT,
            status TEXT CHECK(status IN ('unprocessed', 'processing', 'skipped', 'failed', 'retrying')) NOT NULL
        );
    "#,
    ).unwrap();
}

pub fn get_objects(conn: &SqliteConnection) {
    let items = evacuateobjects.load::<EvacuateObjectDB>(conn).unwrap();
    dbg!(&items);
    for i in items {
        let o = i.object;
        let obj: MantaObject = serde_json::from_str(o.as_str()).unwrap();
        dbg!(obj);
    }
}

pub fn update_objects(conn: &SqliteConnection) {
    diesel::update(evacuateobjects)
        .filter(status.eq(EvacuateObjectStatus::Unprocessed))
        .set(status.eq(EvacuateObjectStatus::Processing))
        .execute(conn)
        .expect("update objects");
}

fn get_filtered_objects(conn: &SqliteConnection, objs: &Vec<String>) {
    println!("Getting: {}", objs[1]);
    let res = evacuateobjects
        .filter(status.eq(EvacuateObjectStatus::Processing))
        .load::<EvacuateObjectDB>(conn)
        .expect("getting filtered objects");

    assert_eq!(res.len(), 2);
    dbg!(res);

    let res = evacuateobjects
        .filter(id.eq(&objs[1]))
        .load::<EvacuateObjectDB>(conn)
        .expect("getting filtered objects");
    assert_eq!(res.len(), 1);
    dbg!(res);
}

fn get_objects_with_status(
    conn: &SqliteConnection,
    check_status: EvacuateObjectStatus,
) -> Vec<EvacuateObjectDB> {
    evacuateobjects
        .filter(status.eq(check_status))
        .load::<EvacuateObjectDB>(conn)
        .expect("getting filtered objects")
}

// When updating records it is similar to executing a filtered query.  See
// get_filtered_objects() and update_objects() above.  But say you only want
// to update a subset of records based on the primary keys.  In this function
// we take a vector of primary keys and update only those records.  You
// should be able to do the same thing with any field, not just the PK.
fn update_subset_of_records(
    conn: &SqliteConnection,
    vec_obj_ids: &Vec<String>,
    to_status: EvacuateObjectStatus,
) -> usize {
    let ret = diesel::update(evacuateobjects)
        .filter(id.eq_any(vec_obj_ids))
        .set(status.eq(to_status))
        .execute(conn)
        .unwrap_or_else(|e| {
            let msg = format!("Error updating {}", e);
            panic!(msg);
        });
    ret
}

fn get_subset_of_records(conn: &SqliteConnection, objs: &Vec<String>) {
    println!("Getting: {:?}", objs);
    let res = evacuateobjects
        .filter(id.eq_any(objs))
        .load::<EvacuateObjectDB>(conn)
        .expect("getting filtered objects");

    assert_eq!(res.len(), objs.len());
    dbg!(res);
}

fn main() {
    let mut objs = vec![];
    let conn = establish_connection();
    let mut stat = false;

    create_table(&conn);
    for i in 0..10 {
        if i == 2 || i == 5 {
            stat = true;
        }
        objs.push(create_object(&conn, stat));
        stat = false;
    }

    update_objects(&conn);
    get_objects(&conn);
    get_filtered_objects(&conn, &objs);

    // Getting a subset of records based on multiple possible values of a field
    let mut vec_obj_ids = vec![];
    for i in 0..2 {
        vec_obj_ids.push(objs[i].clone());
    }
    update_subset_of_records(&conn, &vec_obj_ids, EvacuateObjectStatus::Retrying);
    println!("========");
    get_subset_of_records(&conn, &vec_obj_ids);
    let retry_objs = get_objects_with_status(&conn, EvacuateObjectStatus::Retrying);

    assert_eq!(retry_objs.len(), 2);
}
