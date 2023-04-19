use ::reqwest::Client;
use chrono::prelude::*;
use clap::{Args, Parser, Subcommand};
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use std::error::Error;
use user_timelogs::UserTimelogsCurrentUserTimelogsNodesProject;

type Time = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct UserTimelogs;

#[derive(Debug)]
pub struct Record {
    pub spent_at: chrono::NaiveDate,
    pub time_spent_seconds: i64,
    pub project: UserTimelogsCurrentUserTimelogsNodesProject,
    pub issue_id: String,
    pub issue_title: String,
}

async fn query_user_timelog(
    variables: user_timelogs::Variables,
) -> Result<Vec<Record>, Box<dyn Error>> {
    let gitlab_token = std::env::var("GITLAB_TOKEN").expect("Missing GITLAB_TOKEN env var");
    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", gitlab_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;
    let response_body =
        post_graphql::<UserTimelogs, _>(&client, "https://gitlab.com/api/graphql", variables)
            .await?;
    let data = response_body.data.unwrap();
    let mut records = Vec::new();
    for x in data.current_user.unwrap().timelogs.unwrap().nodes.unwrap() {
        let unwrapped = x.unwrap();
        let issue = unwrapped.issue.unwrap();
        let spent_at = unwrapped.spent_at.unwrap();
        let record = Record {
            time_spent_seconds: unwrapped.time_spent,
            spent_at: chrono::NaiveDate::parse_from_str(&spent_at[..10], "%Y-%m-%d")?,
            project: unwrapped.project,
            issue_id: issue.id,
            issue_title: issue.title,
        };
        records.push(record);
    }
    Ok(records)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Report(Report),
}

#[derive(Args)]
#[command(
    about = "Generate a report of time spent on projects",
    long_about = r#" 
Generate a report of time spent on projects.

Pre-requisites:
    - Set the GITLAB_TOKEN environment variable to a Gitlab personal access token
    - The token must have the api scope

The report is generated from the current user's timelogs in Gitlab.
The timelogs are filtered by date, and the time spent on each project is
summarized.

The report is printed to stdout.

Examples:
    gitlab-report report
    gitlab-report report --from-beginning
    gitlab-report report --start-date 2021-01-01 --end-date 2021-01-31

>> git3t report

From 2021-01-04
To 2021-01-10

0d 0h 30m on gitlab-report
0d 4h 20m on important-issues
0d 7h 0m on random-project

Total: 0d 11h 50m
Total mandays: 1d 3h 50m
"#
)]
pub struct Report {
    #[arg(short, long, default_value = None, help = "Start date of the report. Defaults to Monday of this week")]
    start_date: Option<chrono::NaiveDate>,
    #[arg(short, long, default_value = None, help = "End date of the report. Defaults to today")]
    end_date: Option<chrono::NaiveDate>,
    #[arg(short, long, help = "Start from the beginning of time")]
    from_beginning: bool,
}

impl Report {
    pub fn start_date(&self) -> chrono::NaiveDate {
        if self.from_beginning {
            return chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        }
        self.start_date.unwrap_or_else(|| {
            let now = chrono::Local::now().naive_local();
            let days = now.weekday().num_days_from_monday();
            now.date() - chrono::Duration::days(days as i64)
        })
    }

    pub fn end_date(&self) -> chrono::NaiveDate {
        self.end_date
            .unwrap_or_else(|| chrono::Local::now().naive_local().date())
    }

    pub async fn print_report(&self) {
        let variables = user_timelogs::Variables {};
        let records = query_user_timelog(variables).await.unwrap();
        let mut total_seconds = 0;
        // filter records by date
        let records = records
            .into_iter()
            .filter(|record| {
                record.spent_at >= self.start_date() && record.spent_at <= self.end_date()
            })
            .collect::<Vec<_>>();

        let mut projects = std::collections::HashMap::new();
        for record in records {
            total_seconds += record.time_spent_seconds;
            let project = record.project.name;
            let seconds = projects.entry(project).or_insert(0);
            *seconds += record.time_spent_seconds;
        }

        println!("From {}", self.start_date());
        println!("To {}", self.end_date());
        println!();
        for (project, seconds) in projects {
            let time = chrono::Duration::seconds(seconds);
            let days = time.num_days();
            let hours = time.num_hours() - days * 24;
            let minutes = time.num_minutes() - days * 24 * 60 - hours * 60;
            println!("{}d {}h {}m on {}", days, hours, minutes, project);
        }

        println!();
        let total_time = chrono::Duration::seconds(total_seconds);
        let total_days = total_time.num_days();
        let total_hours = total_time.num_hours() - total_days * 24;
        let total_minutes = total_time.num_minutes() - total_days * 24 * 60 - total_hours * 60;
        println!("Total: {}d {}h {}m", total_days, total_hours, total_minutes);
        // Mandays are calculated as 8 hours per day
        let mandays = total_seconds as f64 / 60.0 / 60.0 / 8.0;
        let hours = mandays.fract() * 8.0;
        let minutes = hours.fract() * 60.0;
        println!(
            "Total mandays: {}d {}h {}m",
            mandays.trunc(),
            hours.trunc(),
            minutes
        );
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Report(report)) => {
            report.print_report().await;
        }
        None => {}
    }
}
