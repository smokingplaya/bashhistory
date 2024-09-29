use std::{fs::File, io::Read};

pub(crate) type BashHistory = Vec<String>;

pub(crate) fn get_bash_history_file_path(
  user: &str
) -> anyhow::Result<String> {
  let path = match user {
    "root" => "/root/.bash_history".to_string(),
    _ => format!("/home/{}/.bash_history", user)
  };

  Ok(path)
}

fn get_bash_history_file(
  user: &str
) -> anyhow::Result<File> {
  let bh_path = get_bash_history_file_path(user)?;

  Ok(File::open(bh_path)?)
}

pub(crate) fn get_bash_history(
  user: &str
) -> anyhow::Result<BashHistory> {
  // bh = bash history
  let mut bh_file = get_bash_history_file(user)?;

  // read contents
  let mut content = String::new();
  bh_file.read_to_string(&mut content)?;

  // parse contents
  let result = content
    .lines()
    .map(|line| line.to_string())
    .collect();

  Ok(result)
}