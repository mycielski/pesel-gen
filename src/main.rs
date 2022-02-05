use clap::Parser;

use pesel_gen::{date, generation};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// Birthday in format "dd-mm-yyyy".
    #[clap(short, long)]
    birthday: String,

    /// Use "m" for males, "f" for females and "b" for both.
    #[clap(short, long)]
    gender: char,
}

fn main() {
    let args = Args::parse();

    let birthday = date::date_from_arg(args.birthday);

    if args.gender == 'm' || args.gender == 'b' {
        generation::generate_pesel_numbers(birthday, 'm');
    }
    if args.gender == 'f' || args.gender == 'b' {
        generation::generate_pesel_numbers(birthday, 'f');
    }
}
