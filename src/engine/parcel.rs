use chrono::prelude::*;
use engine::hydro_event::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Parcel {
    id: i32,
    position: i32,
    current_hydro_level: f32,
    base_hydro_level: f32,
    running_hydro_events: Vec<HydroEvent>,
    next_hydro_end: DateTime<Utc>, 
    plant_id: i32
}