use fantoccini::elements::Element;
use sqlx::types::chrono::{DateTime, Utc};

pub struct JobCardData {
    // there's also random dot things in some of the data
    // ·
    // whatever that is... we need to strip those out
    pub idx:                   usize,
    pub card_title:            String, // invisible // Full Stack Engineer (Verified job)
    pub job_title:             String, // Full Stack Engineer
    pub company_name:          String, // Nityo Infotech
    pub location:              Location, // { state: New South Wales, contry:  Australia }
    pub work_type:             WorkType, // (Hybrid)
    pub is_actively_reviewing: bool,   // Actively reviewing applicants
    pub already_viewed:        bool,   // Viewed
    pub posted_date:           DateInfo, //  Posted on January 19, 2026, 10:43 AM
    pub has_easy_apply:        bool,   // the icon for it //Easy Apply
}

// TODO: [same_fn_1] :
// Prefer to use From<Element> here for parsing job card data
// instead of standalone functions.
//
// PROBLEM:
// many components of parsing itself is async
//
// SOLUTION:
// We could do what we do when creating default *Config structs,
// tokio::task::block_in_place -> runtime::Handle::current().block_on(async { ... })
// This is ugly as hell, but fantoccini's Element methods are all async.
//
impl From<Element> for JobCardData {
    fn from(_element: Element) -> Self {
        //

        todo!()
    }
}

pub struct Location {
    pub state:   String, // New South Wales
    pub country: String, // Australia
}

impl From<&str> for Location {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').map(|part| part.trim()).collect();
        let state = parts.first().unwrap_or(&"").to_string();
        let country = parts.get(1).unwrap_or(&"").to_string();

        Location { state, country }
    }
}

impl From<Location> for String {
    fn from(location: Location) -> Self {
        format!("{}, {}", location.state, location.country)
    }
}

pub enum WorkType {
    OnSite,
    Remote,
    Hybrid,
}

impl From<&str> for WorkType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "on-site" => WorkType::OnSite,
            "remote" => WorkType::Remote,
            "hybrid" => WorkType::Hybrid,
            _ => {
                unimplemented!(
                    "You've attempted to turn an unrecognized work kind string into a WorkKind enum variant: {}",
                    s
                )
            }
        }
    }
}

impl From<WorkType> for &str {
    fn from(work_kind: WorkType) -> Self {
        match work_kind {
            WorkType::OnSite => "on-site",
            WorkType::Remote => "remote",
            WorkType::Hybrid => "hybrid",
        }
    }
}

pub struct DateInfo {
    // We'd want to actually strip the 'Posted on' part, and use a proper DT type
    pub full_date: String, // Posted on January 19, 2026, 10:43 AM
    pub relative:  String, // 1 week ago
}

pub fn parse_date(full: &str, relative: &str) -> DateInfo {
    let full_clean = full.replace("Posted on ", "").trim().to_string();
    let _parsed_dt: DateTime<Utc> = full_clean.parse().unwrap_or_default();
    DateInfo {
        full_date: full_clean,
        relative:  relative.trim().to_string(),
    }
}

pub enum StatusType {
    Pending,
    InProgress,
    Completed,
    Failed,
}

pub struct Status {
    id:     i64,
    status: StatusType,
}

impl From<&str> for StatusType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => StatusType::Pending,
            "in_progress" => StatusType::InProgress,
            "completed" => StatusType::Completed,
            "failed" => StatusType::Failed,
            _ => {
                unimplemented!(
                    "You've attempted to turn an unrecognized status string into a StatusTypes enum variant: {}",
                    s
                )
            }
        }
    }
}

impl From<StatusType> for &str {
    fn from(status: StatusType) -> Self {
        match status {
            StatusType::Pending => "pending",
            StatusType::InProgress => "in_progress",
            StatusType::Completed => "completed",
            StatusType::Failed => "failed",
        }
    }
}

// pub enum EmploymentType {
//     FullTime,
//     PartTime,
//     Contract,
//     Temporary,
//     Internship,
//     Volunteer,
//     Other,
// }
