use std::{path::PathBuf, str::FromStr};

use super::{GameInfo, BLUE_NUM, GREEN_NUM, RED_NUM};

#[allow(dead_code)]
pub(crate) fn solve(input_path: PathBuf) -> color_eyre::Result<usize> {
    let input = std::fs::read_to_string(input_path)?;
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let res: usize = input
        .lines()
        .map(|x| GameInfo::from_str(x).unwrap())
        .filter(|x| {
            if x.blue > BLUE_NUM {
                return false;
            }
            if x.red > RED_NUM {
                return false;
            }
            if x.green > GREEN_NUM {
                return false;
            }
            true
        })
        .map(|x| x.game_id)
        .sum();
    Ok(res)
}
