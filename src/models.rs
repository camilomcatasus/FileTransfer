use serde::{Serialize, Deserialize};
use database_macros::Queryable;
#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    pub id: usize,
    pub username: String,
    pub password: String,
    pub admin_access: bool,
    pub write_access: bool,
    pub read_access: bool,
}
