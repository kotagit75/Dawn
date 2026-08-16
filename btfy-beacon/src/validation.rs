use crate::Beacon;

pub fn is_valid_beacon(own_beacon: &Beacon, target_beacon: &Beacon) -> bool {
    own_beacon
        .values
        .iter()
        .zip(target_beacon.values.iter())
        .all(
            |(a, b)| (a - b).abs() <= 5, /* Allowable error is within 0.5 degrees Celsius.*/
        )
}
