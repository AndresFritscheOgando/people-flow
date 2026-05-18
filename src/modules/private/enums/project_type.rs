use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(Debug, EnumIter, Serialize, Deserialize)]
pub enum ProjectType {
    Hourly,
    FixedPrice
}