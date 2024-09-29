use std::env;
use bash_history::get_bash_history;
use colored::Colorize;
use tiny_gradient::{Gradient, GradientStr};
use users::get_current_username;
use hashbrown::HashMap;

pub(crate) mod bash_history;

pub type SortedHistory = Vec<(String, f32)>;

const MAX_COMMANDS: usize = 10;

fn get_user() -> anyhow::Result<String> {
  let user_binding = get_current_username()
    .ok_or_else(|| anyhow::anyhow!("Unable to get current user"))?;

  Ok(user_binding
    .to_str()
    .ok_or_else(|| anyhow::anyhow!("Unable to convert user OsString to &str"))?
    .to_string()
  )
}

fn calculate_usage(history: &Vec<String>) -> HashMap<&String, f32> {
  let total_count = history.len() as f32;

  let mut word_counts = HashMap::new();

  for word in history {
    *word_counts.entry(word).or_insert(0.0) += 1.0;
  }

  let percentages: HashMap<&String, f32> = word_counts.iter()
    .map(|(&word, &count)| (word, (count / total_count) * 100.0))
    .collect();

  percentages
}

fn get_sorted_usage(
  user: &str
) -> anyhow::Result<SortedHistory> {
  let history = get_bash_history(user)?;
  let commands_usage = calculate_usage(&history);

  let mut sorted_percentages: SortedHistory = commands_usage
    .iter()
    .map(|(&word, &percentage)| (word.to_string(), percentage))
    .collect();

  sorted_percentages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

  Ok(sorted_percentages)
}

fn main() -> anyhow::Result<()> {
  let max_commands = env::args()
    .nth(1)
    .and_then(|arg| arg.parse::<usize>().ok())
    .unwrap_or(MAX_COMMANDS);

  let current_user = get_user()?;
  let history = get_sorted_usage(&current_user)?;

  println!(
    "{} by {}",
    format!("bash history analyzer {}", env!("CARGO_PKG_VERSION"))
      .gradient(Gradient::Atlast)
      .to_string()
      .bold(),
    "https://github.com/smokingplaya"
      .gradient(Gradient::Passion)
      .to_string()
      .bold()
  );

  println!(
    "{}",
    format!("showing {} commands", max_commands)
      .gradient(Gradient::Teen)
      .to_string()
      .bold()
  );

  history.iter()
    .take(max_commands)
    .for_each(|(command, usage)| {
      println!(
        "{}: {:.1}%",
        command.gradient(Gradient::Morning).to_string().bold(),
        usage
      );
    });

  Ok(())
}