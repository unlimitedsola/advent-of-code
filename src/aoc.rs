use std::env;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::{bail, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use time::Month::December;
use time::{Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

macro_rules! input {
    () => {
        $crate::aoc::input_for_path(module_path!())
    };
}

pub(crate) use input;

static PARSE_PATH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^advent_of_code::y(?<year>\d+)::day(?<day>\d+)(::.*)?").unwrap());

fn parse_path(path: &str) -> (u32, u8) {
    let captures = PARSE_PATH.captures(path).expect("invalid path");
    let year = captures["year"]
        .parse::<u32>()
        .expect("year should be a number");
    let day = captures["day"]
        .parse::<u8>()
        .expect("day should be a number");
    (year, day)
}

pub fn input_for_path(path: &str) -> String {
    let (year, day) = parse_path(path);

    let token = env::var("AOC_TOKEN").expect("AOC_TOKEN is not set");
    AoC::new(year, token)
        .expect("failed to create AoC instance")
        .read_input(day)
        .expect("failed to read input")
}

pub struct AoC {
    year: u32,
    token: String,
    inputs: PathBuf,
    http: Client,
}

impl AoC {
    pub fn new(year: u32, token: String) -> Result<Self> {
        let inputs = PathBuf::from(format!("./inputs/{year}"));
        create_dir_all(&inputs)?;
        let http = Client::new();
        Ok(Self {
            year,
            token,
            inputs,
            http,
        })
    }

    pub fn read_input(&self, day: u8) -> Result<String> {
        let path = self.inputs.join(format!("{day}.txt"));
        let input = if !path.exists() {
            let input = self.fetch_input(day)?;
            let mut file = File::create(path)?;
            file.write_all(input.as_bytes())?;
            input
        } else {
            read_to_string(path)?
        };
        Ok(input)
    }

    fn fetch_input(&self, day: u8) -> Result<String> {
        if let 1..=25 = day {
        } else {
            bail!("day must be in range 1..=25")
        }

        let starts = PrimitiveDateTime::new(
            Date::from_calendar_date(self.year as i32, December, day)?,
            time::Time::from_hms(0, 0, 0)?,
        )
        .assume_offset(UtcOffset::from_hms(-5, 0, 0)?);

        let now = OffsetDateTime::now_utc();

        if starts > now {
            bail!(
                "day {} is not started yet, remaining: {}",
                day,
                starts - now
            )
        }

        self.http
            .get(format!(
                "https://adventofcode.com/{year}/day/{day}/input",
                year = self.year
            ))
            .header(COOKIE, format!("session={token}", token = self.token))
            .header(
                USER_AGENT,
                "Rustacean: @unlimitedsola (dev at sola dot love)",
            )
            .send()?
            .text()
            .context("failed to fetch input")
    }
}
