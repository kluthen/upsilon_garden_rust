use chrono::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
struct HydroRange {
    min_not_dead: f32,
    max_not_dead: f32,
    min_ok: f32,
    max_ok: f32,
    min_super: f32,
    max_super: f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Plant {
    id: i32, 
    level: i32, 
    name: String,
    plant_type: String,
    target_hydro: HydroRange, 
    next_update: DateTime<Utc>,
    sp_per_level: i32, 
    sp_max: i32, 
    sp_current: i32
}