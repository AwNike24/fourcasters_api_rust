use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use chrono::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    #[serde(rename = "_id")] // Use MongoDB's default field name
    pub id: ObjectId,
    pub token_type: TokenType,
    pub token: String,
    pub owner: Option<ObjectId>, // Optional because it's not required for 'admin'
    pub expires: Option<DateTime<Utc>>, // Optional because it's not required for all types
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TokenType {
    Login,
    Withdrawal,
    Api,
    Admin,
}
