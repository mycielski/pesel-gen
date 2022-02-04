use clap::Parser;

use pesel_gen::{date, generation};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// Either "m" or "f".
    #[clap(short, long)]
    gender: char,

    /// Birthday in format "dd-mm-yyyy".
    #[clap(short, long)]
    birthday: String,
}

fn main() {
    let args = Args::parse();

    let birthday = date::date_from_arg(args.birthday);
    generation::generate_pesel_numbers(birthday, args.gender);
}
