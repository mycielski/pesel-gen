pub mod checksum;
pub mod date;
pub mod generation;
pub mod validation;

pub const KNOWN_CORRECT_PESEL: &str = "02070803628"; // sourced from: https://www.gov.pl/web/gov/czym-jest-numer-pesel#:~:text=pe%C5%82ny%20numer%20PESEL%3A-,02070803628,-Rozwi%C5%84%20tekst
pub const WEIGHTS: [i32; 4] = [1, 3, 7, 9];
