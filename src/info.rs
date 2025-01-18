use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    pub username: String,
    pub point_list: Vec<Vec<u32>>,
    pub line_list: Vec<Vec<u32>>,
}

impl Info {
    pub fn build(username: String, point_list: Vec<Vec<u32>>, line_list: Vec<Vec<u32>>) -> Info {
        Info { username: username, point_list: point_list, line_list: line_list }
    }
}