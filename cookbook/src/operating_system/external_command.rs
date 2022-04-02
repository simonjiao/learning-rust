use anyhow::Result;
use std::process::Command;
use regex::Regex;

#[derive(Debug, PartialEq, Clone, Default)]
struct Commit {
    hash: String,
    message: String,
}

pub fn run_git_log() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        anyhow::bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(?x)
                                            ([0-9a-fA-F]+) # commit hash
                                            (.*)           # The commit message")?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line|pattern.captures(line))
        .map(|cap| {
            Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            }
        }).take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}