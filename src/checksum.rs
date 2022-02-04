use crate::WEIGHTS;

pub fn calculate_checksum(pesel_candidate: String) -> i32 {
    if pesel_candidate.len() < 10 {
        panic!(
            "PESEL without its checksum must be 10 digits long. Given: {}, of length {}",
            pesel_candidate,
            pesel_candidate.len()
        );
    }

    (10 - (pesel_candidate
        .chars()
        .take(10)
        .map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, d)| (i32::try_from(d.unwrap()).unwrap() * WEIGHTS[i % 4]) % 10)
        .sum::<i32>()
        % 10))
        % 10
}
