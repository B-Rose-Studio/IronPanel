#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeekdayName {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Weekday {
    pub day: WeekdayName,
    pub start_time: String,
    pub end_time: String,
}
