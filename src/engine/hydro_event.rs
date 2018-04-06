use chrono::prelude::*;



#[derive(Serialize, Deserialize, Debug)]
pub struct HydroEvent {
    begin_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    power: f32
}