use clap::{Parser, Subcommand};
use reqwest::blocking::Client;

use std::{
    cell::OnceCell,
    fmt::Display,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

mod days;

fn exit_with(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1)
}

// This struct is used when I've implemented part 1 of a day, but not part 2.
// I still want to be able to print part 1.
#[allow(unused)]
pub struct UnimplementedPartTwo;

impl Display for UnimplementedPartTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not yet implemented")
    }
}

type Solver = Box<dyn Fn(&str) -> (Box<dyn Display>, Box<dyn Display>)>;

fn box_solver<F, A, B>(f: F) -> Option<Solver>
where
    A: Display + 'static,
    B: Display + 'static,
    F: Fn(&str) -> (A, B) + 'static,
{
    Some(Box::new(move |s| {
        let (a, b) = f(s);
        (Box::new(a), Box::new(b))
    }))
}

fn get_solver(day: Day) -> Option<Solver> {
    match day.0 {
        1 => box_solver(days::day01::solve),
        2 => box_solver(days::day02::solve),
        3 => box_solver(days::day03::solve),
        4 => box_solver(days::day04::solve),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Day(u8);

impl Day {
    fn new_or_exit(u: u8) -> Self {
        if !(1..=25).contains(&u) {
            exit_with(&format!("Day {} must be in 1-25", u))
        };
        Day(u)
    }
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u: u8 = s
            .parse::<_>()
            .map_err(|_| format!("Could not parse as day integer: {}", s))?;
        Ok(Day::new_or_exit(u))
    }
}

enum Days {
    All,
    Explicit(Vec<Day>),
}

impl Days {
    fn new(days: Option<Vec<Day>>, all: bool) -> Self {
        if all && days.is_some() {
            exit_with("If --all is set, days cannot be passed explicitly")
        } else if !all && days.is_none() {
            return Days::Explicit(Vec::new());
        }
        match days {
            None => Days::All,
            Some(mut v) => {
                v.sort_unstable();
                v.dedup();
                Days::Explicit(v)
            }
        }
    }

    fn as_vec(&self) -> Vec<Day> {
        match self {
            Self::All => (1..=25).map(Day::new_or_exit).collect(),
            Self::Explicit(v) => v.clone(),
        }
    }
}

#[derive(Subcommand)]
enum SubCommand {
    /// Solve AoC days
    Solve {
        /// Directory with input data. Each file must be named e.g. "day01.txt"
        data_dir: PathBuf,
        /// List of days to solve (incompatible with --all)
        days: Option<Vec<Day>>,
        /// Solve all implemented days
        #[arg(long)]
        all: bool,
    },
    /// Download input data files for AoC
    Download {
        /// Advent of code session key, 128-character hexadecimal
        session_key: String,
        /// Directory to download data to, creating it if necessary
        data_dir: PathBuf,
        /// List of days to download (incompatible with --all)
        days: Option<Vec<Day>>,
        /// Download all released data
        #[arg(long)]
        all: bool,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

fn solve(data_dir: &Path, days: Option<Vec<Day>>, all: bool) {
    let days = Days::new(days, all);
    let solvers: Vec<(Day, Option<Solver>)> = match days {
        Days::All => days
            .as_vec()
            .iter()
            .filter_map(|&day| get_solver(day).map(|s| (day, Some(s))))
            .collect(),
        Days::Explicit(v) => v.iter().map(|&day| (day, get_solver(day))).collect(),
    };
    // If nothing to do, exit the program without an error
    if solvers.iter().all(|(_, s)| s.is_none()) {
        std::process::exit(0)
    }
    // Read input files
    if !data_dir.is_dir() {
        exit_with(&format!(
            "Data directory \"{:?}\" is not an existing directory",
            data_dir
        ));
    }
    // Load the data in as String, for all requested days that have a solver
    let data: Vec<(Day, Option<(String, Solver)>)> = solvers
        .into_iter()
        .map(|(day, maybe_solver)| {
            if let Some(solver) = maybe_solver {
                let path = data_dir.join(format!("day{:02}.txt", day.0));
                let string = std::fs::read_to_string(&path).unwrap_or_else(|_| {
                    exit_with(&format!(
                        "Could not read file \"{:?}\" into UTF-8 string",
                        path
                    ))
                });
                (day, Some((string, solver)))
            } else {
                (day, None)
            }
        })
        .collect();
    // Execute and time each
    for (day, maybe_data_solver) in data {
        if let Some((data, solver)) = maybe_data_solver {
            let begin = Instant::now();
            let (a, b) = solver(&data);
            let duration = Instant::duration_since(&Instant::now(), begin);
            println!(
                "Day {:02} [{:.2?}]:\n  Part 1: {}\n  Part 2: {}\n",
                day.0, duration, a, b
            );
        } else {
            println!("Day {:02}: Unimplemented!\n", day.0);
        }
    }
}

enum Downloaded {
    NotYetReleased,
    Data(String),
}

fn download(data_dir: &Path, session_key: &str, days: Option<Vec<Day>>, all: bool) {
    let days = Days::new(days, all);
    // If we need to download no days, exit early
    if let Days::Explicit(ref v) = days {
        if v.is_empty() {
            std::process::exit(0)
        }
    }
    // Create data dir if it does not aleady exists
    if !data_dir.is_dir() {
        std::fs::create_dir_all(data_dir).unwrap_or_else(|_| {
            exit_with(&format!(
                "Could not create data directory, and is not an existing directory: \"{:?}\"",
                data_dir
            ))
        })
    }
    // This allows us to lazily construct the client, only if we need it
    let client = OnceCell::new();
    for day in days.as_vec() {
        let path = data_dir.join(format!("day{:02}.txt", day.0));
        // If the path exist, no need to download anything
        if path.is_file() {
            println!("Day {:02} is already downloaded", day.0);
            continue;
        } else {
            let data = download_input(client.get_or_init(|| make_client(session_key)), day);
            match data {
                Downloaded::NotYetReleased => match days {
                    // Since we know that the days returned from days.as_vec() are in sorted order,
                    // the first time we encounter a day not yet released, we know none of the next
                    // ones are not released either, and we can stop
                    Days::All => break,
                    // If the user explicitly requested an unreleased day, we can't fulfill the
                    // request and need to exit
                    Days::Explicit(_) => exit_with(&format!(
                        "Explicitly requested day {:02}, but this day is not yet released.",
                        day.0
                    )),
                },
                // If downloaded a file, save it
                Downloaded::Data(s) => {
                    let mut file = File::create_new(&path).unwrap_or_else(|_| {
                        exit_with(&format!("Could not create new file at path \"{:?}\"", path))
                    });
                    file.write_all(s.as_bytes()).unwrap_or_else(|_| {
                        exit_with(&format!(
                            "Error when writing data to file at \"{:?}\"",
                            path
                        ))
                    });
                    println!("Downloaded day {:02}", day.0)
                }
            }
        }
    }
}

fn make_client(session: &str) -> Client {
    // Verify it's formatted correctly
    let bytes = session.as_bytes();
    if bytes.len() != 128 || !bytes.iter().all(|&b| b.is_ascii_hexdigit()) {
        exit_with("Session key not a 128-character hexadecimal string")
    };
    let mut headers = reqwest::header::HeaderMap::default();
    let cookie =
        reqwest::header::HeaderValue::from_str(format!("session={}", session).as_str()).unwrap();
    headers.insert("Cookie", cookie);
    Client::builder().default_headers(headers).build().unwrap()
}

fn download_input(client: &Client, day: Day) -> Downloaded {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day.0);
    let resp = client.get(url.as_str()).send().unwrap();
    if !resp.status().is_success() {
        let text = resp.text().unwrap();
        // The response will begin with this string if the day is not yet released
        if text.contains("Please don't repeatedly request this endpoint before it unlocks") {
            return Downloaded::NotYetReleased;
        } else {
            exit_with(&format!("Error when processing request:\n{}", text))
        }
    }
    Downloaded::Data(resp.text().unwrap())
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        SubCommand::Solve {
            data_dir,
            days,
            all,
        } => solve(&data_dir, days, all),
        SubCommand::Download {
            session_key,
            data_dir,
            days,
            all,
        } => download(&data_dir, session_key.trim(), days, all),
    }
}
