use std::{path::PathBuf, str::FromStr};

use super::GameInfo;
#[allow(dead_code)]
pub(crate) fn solve(input_path: PathBuf) -> color_eyre::Result<usize> {
    let input = std::fs::read_to_string(input_path)?;
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let res: usize = input
        .lines()
        .map(|x| GameInfo::from_str(x).unwrap())
        .map(|x| x.blue * x.red * x.green)
        // .map(|x| x.game_id)
        .sum();
    Ok(res)
}
