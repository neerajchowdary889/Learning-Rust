use std::time::{Duration, PrimitiveDateTime as DateTime};
// Returns a DateTime one billion seconds after start.
pub fn main(start: DateTime) -> DateTime {
    start + Duration::seconds(1000000000)
}
