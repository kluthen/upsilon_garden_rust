use engine::garden::*;
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn index() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "gardens": Garden::all_short()}))
}

#[get("/<gid>")]
pub fn show(gid: i32) -> Json<Value> {
    match Garden::by_id(&gid) {
        Some(g) => {
            Json(json!({
            "status": "ok",
            "garden": g
            }))
        },
        None => {
            Json(json!({
                "status" : "error"
            }))
        }
    }
}

// Only accept name specification ATM
#[derive(Deserialize, Debug)]
pub struct GardenParam {
    name: String
}

#[post("/", data="<gparam>")]
pub fn create(gparam: Json<GardenParam>) -> Json<Value> {

    let mut g = Garden::new();
    g.name = gparam.name.clone();
    g.repsert();

    Json(json!({
        "status": "ok",
        "garden": g}))
}



#[put("/<gid>", data="<gparam>")]
pub fn update(gid: i32, gparam: Json<GardenParam>) -> Json<Value> {
    match Garden::by_id(&gid).as_mut() {
        Some(g) => {

            g.name = gparam.name.clone();
            g.repsert();

            Json(json!({
            "status": "ok",
            "garden": g
            }))
        },
        None => {
            Json(json!({
                "status" : "error"
            }))
        }
    }
}

#[delete("/<gid>")]
pub fn delete(gid: i32) -> Json<Value> {
    Garden::drop(&gid);
    Json(json!({"status": "ok"}))
}