use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims { // data stored inside token is called claims
    pub user_id: i32,
    pub role_id: i32,
    pub organization_id: i32,
    pub exp: usize,
}
