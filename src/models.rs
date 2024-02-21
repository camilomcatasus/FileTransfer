use serde::{Serialize, Deserialize};
use database_macros::Queryable;
#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    id: usize,
    username: String,
    password: String,
    admin_access: bool,
    write_access: bool,
    read_access: bool,
}
