use chrono::{DateTime, Utc};
use serenity::model::id::UserId;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConstructionSite {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String, // Store as string representation of Discord UserId
}

use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Commodity {
    pub id: i64,
    pub construction_site_id: i64,
    pub name: String,
    pub quantity_needed: i64,
    pub quantity_delivered: i64,
}

impl Commodity {
    pub fn remaining_quantity(&self) -> i64 {
        std::cmp::max(0, self.quantity_needed - self.quantity_delivered)
    }
}

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Delivery {
    pub id: i64,
    pub construction_site_id: i64,
    pub commodity_id: i64,
    pub user_id: String, // Discord UserId as string
    pub user_name: String, // Discord username
    pub quantity: i64,
    pub delivered_at: DateTime<Utc>,
}