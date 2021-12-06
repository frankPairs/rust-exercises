const CARS_PRODUCE_PER_HOUR: f64 = 221.0;

/// It returns the success rate based on the speed range.
///
/// Here the success rate scale based on the speed level:
///
/// `1` to `4`: 100% success rate.
/// `5` to `8`: 90% success rate.
/// `9` and `10`: 77% success rate.
///
/// IF the speed value is not between 0 and 10 so the number, the method just panics.
///
fn get_success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 100.0,
        5..=8 => 90.0,
        9..=10 => 77.0,
        _ => panic!("Speed out of range"),
    }
}

/// It calculates the numbers of cars produced per hour.
///
/// It takes into account the success rate value. As higher is the production speed, as lower
/// will be the number of cars build successfully.
///
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate_per_hour = speed as f64 * CARS_PRODUCE_PER_HOUR;
    let success_rate = get_success_rate(speed) / 100.0;

    rate_per_hour * success_rate
}

/// It calculates the numbers of cars produced per minute.
///
/// It takes into account the success rate value. As higher is the production speed, as lower
/// will be the number of cars build successfully.
///
pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);

    rate_per_hour as u32 / 60
}
