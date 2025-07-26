use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum_macros::{AsRefStr, EnumString};

#[derive(Serialize)]
pub struct Housing {
    pub id: HousingID,
    pub overall_rating: u16,        // 1-500 => /100 => 0.00 - 5.00
    pub ratings: RatingsBrokenDown, // 1-500 => /100 => 0.00 - 5.00
    pub review_count: u32,
    pub housing_type: HousingType,
    pub campus_type: CampusType,
    pub walk_time_mins: u8, // mins <= 255
    pub cost_min: u8,       // 1-255 => 1k - 255k per year
    pub cost_max: u8,       // 1-255 => 1k - 255k per year
    pub cost_symbol: Option<CostSymbol>,
    pub address: String,
    pub amenities: HashSet<Amenity>,
}

#[derive(Serialize, Deserialize)]
pub struct ReviewPayload {
    pub id: HousingID,
    pub overall_rating: u16, // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
    pub ratings: RatingsBrokenDown, // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
    pub semester_season: SemesterSeason,
    pub semester_year: u8, // year <= 255 + 2000
    pub description: String,
    pub thumbs_up: u32,
    pub thumbs_down: u32,
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
#[repr(u8)]
pub enum SemesterSeason {
    #[strum(serialize = "Fall")]
    Fall = 0,

    #[strum(serialize = "Spring")]
    Spring = 1,

    #[strum(serialize = "Summer")]
    Summer = 2,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone, Serialize)]
pub enum Amenity {
    #[strum(serialize = "Study Lounges")]
    StudyLounges,

    #[strum(serialize = "24/7 Front Desk")]
    FrontDesk,

    #[strum(serialize = "Community Kitchen")]
    CommunityKitchen,

    #[strum(serialize = "Lounge Areas")]
    LoungeAreas,

    #[strum(serialize = "In-Hall Dining Court")]
    InHallDiningCourt,

    #[strum(serialize = "Fitness Room")]
    FitnessRoom,

    #[strum(serialize = "Study Rooms")]
    StudyRooms,

    #[strum(serialize = "Vending Areas")]
    VendingAreas,

    #[strum(serialize = "Private Bathrooms")]
    PrivateBathrooms,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone, Serialize)]
pub enum CampusType {
    #[strum(serialize = "On-Campus")]
    OnCampus,

    #[strum(serialize = "Off-Campus")]
    OffCampus,
}

#[derive(EnumString, AsRefStr, PartialEq, Clone, Serialize)]
pub enum HousingType {
    #[strum(serialize = "Dorm")]
    Dorm,

    #[strum(serialize = "Apartment")]
    Apartment,
}

#[derive(Serialize, Deserialize)]
pub struct RatingsBrokenDown {
    pub living_conditions: u16, // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
    pub location: u16, // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
    pub amenities: u16, // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
    pub value: u16,    // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
    pub community: u16, // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
}

#[derive(EnumString, TryFromPrimitive, AsRefStr, PartialEq, Clone, Serialize, Deserialize)]
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
