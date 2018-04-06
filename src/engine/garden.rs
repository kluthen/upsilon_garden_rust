use chrono::prelude::*;
use engine::parcel::*;
use engine::plant::*;
use db;
use serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct Garden {
    pub id: i32,
    pub name: String,
    pub parcels: Vec<Parcel>,
    pub last_update:  DateTime<Utc>,
    pub next_update:  DateTime<Utc>,
    pub plants: Vec<Plant>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ShortGarden {
    id: i32,
    name: String,
}

impl Garden {
    pub fn by_id(id: &i32) -> Option<Garden> {
        let handle = db::new_handle();
        for row in &handle.query("select garden_id, name, last_update, next_update, parcels, plants from garden where garden_id=$1",&[id]).unwrap() {
            let garden = Garden {
                id: row.get(0),
                name: row.get(1),
                last_update: row.get(2),
                next_update: row.get(3),
                parcels: serde_json::from_value(row.get(4)).unwrap(),
                plants: serde_json::from_value(row.get(5)).unwrap()
            }; 

            return Some(garden);
        }
        return None;
    }

    pub fn all_short() -> Vec<ShortGarden> {
        let handle = db::new_handle();
        let mut res = Vec::<ShortGarden>::new();
        for row in &handle.query("select garden_id, name from garden ",&[]).unwrap() {
            let garden = ShortGarden {
                id: row.get(0),
                name: row.get(1)
            };

            res.push(garden);
        }
        res
    }

    pub fn repsert(&mut self) {
        let handle = db::new_handle();
        if self.id < 0 {
            for row in &handle.query("insert into garden(name) values ('') returning garden_id", &[]).unwrap() {
                self.id = row.get(0);
            }
        }

        handle.query("update garden set 
            name=$1, 
            last_update=$2,
            next_update=$3,
            parcels=$4,
            plants=$5
            where garden_id=$6", &[
                &self.name,
                &self.last_update,
                &self.next_update,
                &json!(self.parcels),
                &json!(self.plants),
                &self.id
            ]).unwrap();
    }

    pub fn new() -> Garden {
        Garden {
            id: -1,
            name: "".to_string(),
            last_update: Utc::now(),
            next_update: Utc::now(),
            parcels: Vec::<Parcel>::new(),
            plants: Vec::<Plant>::new(),
        }
    }

    pub fn drop(id: &i32) {
        let handle = db::new_handle();
        if *id > 0 {
            handle.execute("delete from garden where garden_id=$1", &[&id]).unwrap();
        }
    }
}