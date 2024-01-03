
use colored::Colorize;
use chrono::{offset::Local, Datelike, NaiveDate, Months};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const DAYS: [&str; 7] = [
    "Mo",
    "Tu",
    "We",
    "Th",
    "Fr",
    "Sa",
    "Su"
];

fn pad(string: String, padding: usize) -> String {
    format!(
        "{}{}",
        " ".repeat(padding - string.chars().count()),
        string
    )
}

fn month_to_string(month: u32) -> String {
    MONTHS[month as usize].to_string()
}

fn normalize_month(date: &NaiveDate) -> Option<NaiveDate> {
    Some(
        NaiveDate::from_ymd_opt(date.year(), date.month(), 1)?
        )
}

fn get_days_from_month(date: &NaiveDate) -> Option<usize> {
    let date = normalize_month(date)?;
    Some(
        date.checked_add_months(Months::new(1))?
            .signed_duration_since(date)
            .num_days() as usize
        )
}

fn get_offset_of_month(date: &NaiveDate) -> Option<usize> {
    Some(
        (normalize_month(date)?
        .weekday().number_from_monday() - 1) as usize
        )
}

fn generate_matrix(length: usize, offset: usize, colon: usize) -> Vec<Vec<Option<usize>>> {
    (0..(length + offset))
        .map(|x| x.checked_sub(offset).map(|x| x + 1))
        .enumerate()
        .fold(
            // init Vec<Vec<Option<usize>>>
            (0..colon).fold(
                Vec::<Vec::<Option<usize>>>::with_capacity(colon),
                |mut acc, _| {
                    acc.push(Vec::<Option<usize>>::new());
                    acc
                }
            ),
            // function
            |mut acc, (i, x)| {
                acc[i % colon as usize].push(x);
                acc
            }
        )
}

fn print_month(date: &NaiveDate) {

    println!("     {} {}",
             month_to_string(date.month0()),
             date.year()
            );


    let result = generate_matrix(
        get_days_from_month(date).unwrap(),
        get_offset_of_month(date).unwrap(),
        7);

    let current_date = Local::now().date_naive();

    for (v1, day) in result.iter().zip(DAYS.iter()) {
        let line = v1.iter()
            .fold(
                "".to_string(),
                |acc, &s| {
                    format!("{} {}", acc,
                        match s {
                            Some(x) => {
                                if current_date.day() as usize == x && &current_date == date {
                                    pad(x.to_string(), 2).black().on_bright_white().to_string()
                                } else {
                                    pad(x.to_string(), 2)
                                }
                            },
                            None => {
                                pad(" ".to_string(), 2)
                            }
                        }
                    )
                }
            );
        println!("{} {}", day, line);
    }
    println!();

}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let current_date = Local::now().date_naive();
    print_month(&current_date);


    let current_date = current_date.checked_add_months(Months::new(1)).unwrap();
    print_month(&current_date);

}

