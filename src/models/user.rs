use serde::{Serialize, Deserialize};
use bson::Decimal128;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    sportsbook_default: Option<bool>,
    email: Option<String>,
    phone_number: Option<String>,
    telegram_username: Option<String>,
    note: Option<String>,
    user_type: String,
    p2p_code: Option<String>,
    closed: bool,
    captains_code: Option<String>,
    credit_limit: Option<i32>,
    password_revoke_token: Option<String>,
    language: Option<String>,
    default_odds_format: String,
    is_admin: bool,
    has_market_maker_access: bool,
    has_monkey_access: Option<bool>,
    seed_access: bool,
    dgs_seeder: bool,
    dgs_seeder_enabled: bool,
    follow_pinnacle_numbers: bool,
    display_balance: Decimal128,
    liability: Decimal128,
    default_expiry: Option<i32>,
    default_bet: Option<i32>,
    default_offer: Option<i32>,
    max_liability: i32,
    default_follow_pinny: bool,
    default_rotation_numbers: Option<bool>,
    notify_via_sms: bool,
    weekly_carry: Decimal128,
    referral_code: Option<String>,
    code: Option<String>,
    deposit_address: Option<String>,
    referred: Vec<Referral>,
    password_hash: String,
    salt: String,
    username: String,
    sharp_rating: Vec<SharpRating>,
    display_name: Option<String>,
    default_commission: Decimal128,
    commission_charged: Decimal128,
    sign_ups_disabled: Option<bool>,
    is_pro: bool,
    sportsbook_minimum_display: Decimal128,
    default_send_order_message: bool,
    locked: bool,
    custom_commission: Option<HashMap<String, String>>,
    matched_volume: bool,
    open_interest: bool,
    display_rotation_numbers: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Referral {
    id: bson::oid::ObjectId,
    confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharpRating {
    league: String,
    factor: i32,
}
