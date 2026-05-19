use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, EnumIter)]
pub enum Status {
    Applied,
    Interview,
    Hired,
    Rejected
}