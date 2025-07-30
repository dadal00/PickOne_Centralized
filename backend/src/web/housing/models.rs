use chrono::NaiveDate;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ThumbsPayload {
    pub id: Uuid,
    pub thumbs_up: u64,
    pub thumbs_down: u64,
}

pub type ThumbsDeltaMap = HashMap<Uuid, ThumbsDelta>;

#[derive(EnumString, AsRefStr, PartialEq, Clone, Deserialize)]
pub enum ThumbsDelta {
    #[strum(serialize = "up")]
    Up,

    #[strum(serialize = "down")]
    Down,
}

pub type ReviewRow<'a> = (
    Uuid,
    i8,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    NaiveDate,
    &'a str,
    i64,
    i64,
);

#[derive(Serialize, Deserialize)]
pub struct HousingPayload {
    pub id: HousingID,
    // 1-500 => /100 => 0.00 - 5.00
    pub overall_rating: u16,
    // 1-500 => /100 => 0.00 - 5.00
    pub ratings: RatingsBrokenDown,
    pub review_count: u32,
    pub housing_type: HousingType,
    pub campus_type: CampusType,
    // mins <= 255
    pub walk_time_mins: u8,
    // 1-255 => 1k - 255k per year
    pub cost_min: u8,
    // 1-255 => 1k - 255k per year
    pub cost_max: u8,
    pub cost_symbol: Option<CostSymbol>,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Housing {
    pub id: String,
    // 1-500 => /100 => 0.00 - 5.00
    pub overall_rating: u16,
    // 1-500 => /100 => 0.00 - 5.00
    pub ratings: RatingsBrokenDown,
    pub review_count: u32,
    pub housing_type: String,
    pub campus_type: String,
    // mins <= 255
    pub walk_time_mins: u8,
    // 1-255 => 1k - 255k per year
    pub cost_min: u8,
    // 1-255 => 1k - 255k per year
    pub cost_max: u8,
    pub cost_symbol: String,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReviewPayload {
    pub housing_id: HousingID,
    // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
    pub overall_rating: u16,
    // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
    pub ratings: RatingsBrokenDown,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Review {
    pub id: Uuid,
    pub housing_id: String,
    // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
    pub overall_rating: u16,
    // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
    pub ratings: RatingsBrokenDown,
    // Month Year => May 2025
    pub date: String,
    pub description: String,
    pub thumbs_up: u64,
    pub thumbs_down: u64,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone, Serialize, Deserialize)]
pub enum CostSymbol {
    #[strum(serialize = "$")]
    Cheap,

    #[strum(serialize = "$$")]
    Affordable,

    #[strum(serialize = "$$$")]
    Expensive,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone, Serialize, Deserialize)]
pub enum CampusType {
    #[strum(serialize = "On-Campus")]
    OnCampus,

    #[strum(serialize = "Off-Campus")]
    OffCampus,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone, Serialize, Deserialize)]
pub enum HousingType {
    #[strum(serialize = "Dorm")]
    Dorm,

    #[strum(serialize = "Apartment")]
    Apartment,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RatingsBrokenDown {
    // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
    pub living_conditions: u16,
    pub location: u16,
    pub amenities: u16,
    pub value: u16,
    pub community: u16,
}

#[derive(Default)]
pub struct WeightedRatings {
    pub living_conditions: f64,
    pub location: f64,
    pub amenities: f64,
    pub value: f64,
    pub community: f64,
}

#[derive(Default)]
pub struct WeightedHousingRatings {
    pub count: u32,
    pub weighted_overall: f64,
    pub weighted_ratings: WeightedRatings,
}

#[derive(
    EnumString, TryFromPrimitive, AsRefStr, PartialEq, Clone, Serialize, Deserialize, Eq, Hash,
)]
#[repr(u8)]
pub enum HousingID {
    // On Campus
    #[strum(serialize = "cary-quad")]
    CaryQuad = 0,

    #[strum(serialize = "mc-cutcheon")]
    McCutcheon = 1,

    #[strum(serialize = "tarkington")]
    Tarkington = 2,

    #[strum(serialize = "wiley")]
    Wiley = 3,

    #[strum(serialize = "owen")]
    Owen = 4,

    #[strum(serialize = "shreve")]
    Shreve = 5,

    #[strum(serialize = "earhart")]
    Earhart = 6,

    #[strum(serialize = "harrison")]
    Harrison = 7,

    #[strum(serialize = "hillenbrand")]
    Hillenbrand = 8,

    #[strum(serialize = "meredith")]
    Meredith = 9,

    #[strum(serialize = "meredith-south")]
    MeredithSouth = 10,

    #[strum(serialize = "windsor")]
    Windsor = 11,

    #[strum(serialize = "first-street")]
    FirstStreet = 12,

    #[strum(serialize = "hilltop")]
    Hilltop = 13,

    #[strum(serialize = "winifred")]
    Winifred = 14,

    #[strum(serialize = "frieda")]
    Frieda = 15,

    #[strum(serialize = "hawkins")]
    Hawkins = 16,

    #[strum(serialize = "fuse")]
    Fuse = 17,

    #[strum(serialize = "aspire")]
    Aspire = 18,

    #[strum(serialize = "3rd-and-west")]
    ThirdAndWest = 29,

    #[strum(serialize = "benchmark-ii")]
    BenchmarkII = 30,

    #[strum(serialize = "grant-333")]
    Grant333 = 31,

    #[strum(serialize = "provenance")]
    Provenance = 32,

    #[strum(serialize = "russell-414")]
    Russell414 = 33,

    #[strum(serialize = "steely-410")]
    Steely410 = 34,

    #[strum(serialize = "waldron-125")]
    Waldron125 = 35,

    #[strum(serialize = "waldron-19")]
    Waldron19 = 36,

    #[strum(serialize = "waldron-square")]
    WaldronSquare = 37,

    #[strum(serialize = "honors-college-residences")]
    HonorsCollegeResidences = 38,

    // Off Campus
    #[strum(serialize = "hub")]
    Hub = 19,

    #[strum(serialize = "rise")]
    Rise = 20,

    #[strum(serialize = "chauncey")]
    Chauncey = 21,

    #[strum(serialize = "lark")]
    Lark = 22,

    #[strum(serialize = "allight")]
    Allight = 23,

    #[strum(serialize = "redpoint")]
    Redpoint = 24,

    #[strum(serialize = "verve")]
    Verve = 26,

    #[strum(serialize = "river")]
    River = 27,

    #[strum(serialize = "morris")]
    Morris = 28,
}

// Search indexes are named by HousingID
// User search are matches by HousingLabels
impl HousingID {
    pub fn display_name(&self) -> &'static str {
        match self.as_ref() {
            // On Campus
            "cary-quad" => "Cary Quadrangle",
            "mc-cutcheon" => "McCutcheon Hall",
            "tarkington" => "Tarkington Hall",
            "wiley" => "Wiley Hall",
            "owen" => "Owen Hall",
            "shreve" => "Shreve Hall",
            "earhart" => "Earhart Hall",
            "harrison" => "Harrison Hall",
            "hillenbrand" => "Hillenbrand Hall",
            "meredith" => "Meredith Hall",
            "meredith-south" => "Meredith South",
            "windsor" => "Windsor Halls",
            "first-street" => "First Street Towers",
            "hilltop" => "Hilltop Apartments",
            "winifred" => "Winifred Parker Hall",
            "frieda" => "Frieda Parker Hall",
            "hawkins" => "Hawkins Hall",
            "fuse" => "Fuse Apartments",
            "aspire" => "Aspire at Discovery Park",
            "3rd-and-west" => "3rd and West",
            "benchmark-ii" => "Benchmark II",
            "grant-333" => "Grant Street Station 333",
            "provenance" => "Provenance Apartments",
            "russell-414" => "414 Russell Street",
            "steely-410" => "410 Steely Street",
            "waldron-125" => "125 Waldron Street",
            "waldron-19" => "19 Waldron Street",
            "waldron-square" => "Waldron Square",
            "honors-college-residences" => "Honors College Residences",

            // Off Campus
            "hub" => "Hub on Campus",
            "rise" => "Rise on Chauncey",
            "chauncey" => "Chauncey Square Apartments",
            "lark" => "Lark West Lafayette",
            "allight" => "Alight West Lafayette",
            "redpoint" => "Redpoint West Lafayette",
            "verve" => "Verve West Lafayette",
            "river" => "River Market Apartments",
            "morris" => "Morris Rentals",

            _ => panic!("Housing is misconfigured!"),
        }
    }
}
