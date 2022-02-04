#![feature(test)]
extern crate test;

#[cfg(test)]
mod benches {
    use pesel_gen::checksum::calculate_checksum;
    use pesel_gen::date::{generate_date, rrmmdd_from_date};
    use pesel_gen::generation::generate_pesel_numbers;
    use pesel_gen::validation::validate_pesel;
    use pesel_gen::KNOWN_CORRECT_PESEL;
    use test::Bencher;

    #[bench]
    fn pesel_validation(b: &mut Bencher) {
        b.iter(|| validate_pesel(String::from(KNOWN_CORRECT_PESEL)));
    }

    #[bench]
    fn checksum(b: &mut Bencher) {
        b.iter(|| calculate_checksum(String::from(KNOWN_CORRECT_PESEL)));
    }

    #[bench]
    fn one_persons_pesel_generation(b: &mut Bencher) {
        let birthday = generate_date(1, 1, 1970);
        b.iter(|| generate_pesel_numbers(birthday, 'm'));
    }

    #[bench]
    fn date_creation(b: &mut Bencher) {
        b.iter(|| generate_date(1, 1, 1970));
    }

    #[bench]
    fn date_to_rrmmdd(b: &mut Bencher) {
        let birthday = generate_date(1, 1, 1970);
        b.iter(|| rrmmdd_from_date(birthday));
    }
}
