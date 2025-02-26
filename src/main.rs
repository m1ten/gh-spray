use alphabet::Font;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use clap::Parser;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::process::exit;
use std::{fs, process::Command};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

mod alphabet;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    text: String,

    #[arg(short, long)]
    startdate: Option<String>,

    #[arg(short, long, default_value = "1")]
    multiplier: u32,

    #[arg(short, long, value_parser = clap::builder::PossibleValuesParser::new(&["default"]))]
    font: Option<String>,

    #[arg(short, long)]
    dryrun: bool,
}

fn main() {
    let args = Args::parse();
    let font = Font::new(args.font.as_deref().unwrap_or("default"));
    let pattern = generate_pattern_from_text(&args.text, font);
    let start_date = parse_start_date(args.startdate);

    if args.dryrun {
        show_preview(&pattern);
        println!("Dry run preview shown above.");
        return;
    }

    let folder = format!(
        "spray-{}",
        rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect::<String>()
    );

    fs::create_dir(&folder).expect("Failed to create directory");

    let _ = Command::new("git")
        .args(&["init"])
        .current_dir(&folder)
        .output()
        .expect("Failed to initialize git repo");

    let commits = generate_commits(&folder, &pattern, start_date, args.multiplier);

    println!("Start date: {}", start_date);
    println!("Number of commits: {}", commits);
    println!("Done! Check the folder {}", folder);
}

fn show_preview(pattern: &[Vec<u32>]) {
    let mut stdout = stdout();
    stdout.execute(Hide).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();

    let max_weeks = pattern[0].len();
    for week in 0..max_weeks {
        for day in 0..alphabet::WEEK_DAYS {
            stdout.execute(MoveTo(week as u16, day as u16)).unwrap();
            print!(
                "{} ",
                alphabet::CHARS[if pattern[day][week] > 0 { 3 } else { 0 }]
            );
            stdout.flush().unwrap();
        }
    }

    stdout.execute(Show).unwrap();
    stdout
        .execute(MoveTo(0, alphabet::WEEK_DAYS as u16))
        .unwrap();
    stdout.flush().unwrap();
}

fn generate_pattern_from_text(text: &str, font: Font) -> Vec<Vec<u32>> {
    let alphabet = font.get_patterns();
    let mut pattern = vec![String::new(); alphabet::WEEK_DAYS];

    for c in text.chars() {
        if let Some(letter) = alphabet.iter().find(|(ch, _)| *ch == c) {
            for (j, row) in letter.1.iter().enumerate() {
                pattern[j].push_str(row);
                if c != ' ' && c != text.chars().last().unwrap() {
                    pattern[j].push(' ');
                }
            }
        } else {
            eprintln!("Warning: character '{}' not supported", c);
            exit(1);
        }
    }

    pattern
        .iter()
        .map(|row| row.chars().map(|c| (c == '1') as u32).collect())
        .collect()
}

fn parse_start_date(date_str: Option<String>) -> DateTime<Utc> {
    match date_str {
        Some(date) => {
            let dt = DateTime::parse_from_rfc3339(&date)
                .or_else(|_| DateTime::parse_from_str(&date, "%Y-%m-%d"))
                .expect("Invalid date format. Use YYYY-MM-DD or RFC3339")
                .with_timezone(&Utc);

            let days_since_sunday = dt.weekday().num_days_from_sunday();
            dt - Duration::days(days_since_sunday as i64)
        }
        None => {
            // 53 weeks ago, starting from next sunday
            let now = Utc::now();
            let weeks_ago = now - Duration::weeks(53);
            let days_to_next_sunday = 7 - weeks_ago.weekday().num_days_from_sunday();
            weeks_ago + Duration::days(days_to_next_sunday as i64)
        }
    }
    .with_hour(0)
    .unwrap()
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap()
    .with_nanosecond(0)
    .unwrap()
}

fn get_git_credentials() -> (String, String) {
    let config = git2::Config::open_default().ok();
    let name = config
        .as_ref()
        .and_then(|c| c.get_string("user.name").ok())
        .unwrap_or_else(|| {
            eprintln!("Warning: please set `git config --global user.name 'NAME'`");
            exit(1);
        });
    let email = config
        .as_ref()
        .and_then(|c| c.get_string("user.email").ok())
        .unwrap_or_else(|| {
            eprintln!("Warning: please set `git config --global user.email 'EMAIL'`");
            exit(1);
        });
    (name, email)
}

fn generate_commits(
    folder: &str,
    pattern: &[Vec<u32>],
    start_date: DateTime<Utc>,
    multiplier: u32,
) -> usize {
    let repo = git2::Repository::open(folder).expect("Failed to open repository");
    let (name, email) = get_git_credentials();

    // Create and commit initial README.md
    let readme = "# GitHub Contribution Graph Art\n\nCreated with [gh-spray](https://github.com/m1ten/gh-spray)";
    fs::write(format!("{}/README.md", folder), readme).expect("Failed to write README");

    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("README.md")).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = git2::Signature::now(&name, &email).unwrap();

    // Create initial commit
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .unwrap();

    let mut commits = 1;

    let max_weeks = pattern[0].len();
    let mut stdout = stdout();
    stdout.execute(Hide).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();

    // For each position in the pattern
    for week in 0..max_weeks {
        for day in 0..alphabet::WEEK_DAYS {
            let current_date = start_date + Duration::days((week * 7 + day) as i64);
            let commits_count = pattern[day][week] * multiplier;

            stdout.execute(MoveTo(week as u16, day as u16)).unwrap();
            print!(
                "{} ",
                alphabet::CHARS[if commits_count > 0 { 3 } else { 0 }]
            );
            stdout.flush().unwrap();

            // Generate commits
            for _ in 0..commits_count {
                let tree_id = repo.index().unwrap().write_tree().unwrap();
                let tree = repo.find_tree(tree_id).unwrap();
                let sig = git2::Signature::new(
                    &name,
                    &email,
                    &git2::Time::new(current_date.timestamp(), 0),
                )
                .unwrap();

                if let Ok(head) = repo.head() {
                    let parent = repo.find_commit(head.target().unwrap()).unwrap();
                    repo.commit(Some("HEAD"), &sig, &sig, "Spray commit", &tree, &[&parent])
                        .unwrap();
                    commits += 1;
                }
            }
        }
    }

    stdout.execute(Show).unwrap();
    stdout
        .execute(MoveTo(0, alphabet::WEEK_DAYS as u16))
        .unwrap();
    stdout.flush().unwrap();

    commits
}
