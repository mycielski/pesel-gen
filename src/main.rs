use clap::Parser;
use itertools::iproduct;

use pesel_gen::{date, generation};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// Birthday in format "dd-mm-yyyy". If you wish to use a range of dates then write it in format "dd-mm-yyyy_dd-mm-yyyy", where the second date is the range's end.
    #[clap(short, long)]
    birthday: String,

    /// Use "m" for males, "f" for females and "mf" for both.
    #[clap(short, long)]
    gender: String
}

fn main() {
    let args = Args::parse();

    run(args.gender, args.birthday);
}

fn run(gender: String, range: String) {
    let genders: Vec<char> = gender.chars().collect();
    if genders.len() > 2 {
        panic!("Incorrect gender entered: {}", gender);
    }

    let mut dates = Vec::new();

    if range.chars().any(|c| c == '_') {
        let birthdays: Vec<String> = range.split('_').map(|s| s.to_string()).collect();
        let mut date = date::date_from_arg(&birthdays[0]);
        let end_date = date::date_from_arg(&birthdays[1]);
        while date <= end_date {
            dates.push(date);
            date = date.checked_add(time::Duration::days(1)).unwrap();
        }
    } else {
        let birthday = date::date_from_arg(&range);
        dates.push(birthday);
    }

    for (bday, gender) in iproduct!(dates, genders) {
        generation::generate_pesel_numbers(bday, gender);
    }
}
