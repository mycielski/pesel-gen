#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use pesel_gen::validation::validate_pesel;
    use pesel_gen::KNOWN_CORRECT_PESEL;

    #[test]
    fn pesel_validation() {
        let mut pesel = String::from(KNOWN_CORRECT_PESEL);
        assert_eq!(true, validate_pesel(pesel.clone()));

        pesel.replace_range(10..11, &"2"); //the correct checksum is 8
        assert_eq!(false, validate_pesel(pesel));
    }
}
