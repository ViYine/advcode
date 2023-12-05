use std::path::PathBuf;

use color_eyre::Result;

#[allow(dead_code)]
pub(crate) fn solve(input_path: PathBuf) -> Result<i32> {
    let input = std::fs::read_to_string(input_path)?;
    let res = input
        .split('\n')
        // not contains number
        .filter(|x| x.chars().any(|c| c.is_numeric()))
        .map(|x| {
            // find first number
            let mut first_num = 0;
            let mut last_num = 0;
            for c in x.chars() {
                if c.is_numeric() {
                    // to i32
                    first_num = c.to_digit(10).unwrap() as i32;
                    break;
                }
            }
            // reverse
            for c in x.chars().rev() {
                if c.is_numeric() {
                    // to i32
                    last_num = c.to_digit(10).unwrap() as i32;
                    break;
                }
            }
            first_num * 10 + last_num
        })
        .sum::<i32>();

    println!("part1 res: {}", res);
    Ok(res)
}
