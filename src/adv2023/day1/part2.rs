use std::path::PathBuf;

use color_eyre::Result;

#[allow(dead_code)]
pub(crate) fn solve(input_path: PathBuf) -> Result<i32> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let input = std::fs::read_to_string(input_path)?;
    let res = input
        .split('\n')
        // not contains number
        // replace all word to number: one, two, three, four, five, six, seven, eight, nine
        .map(|s| {
            let mut rs = s.to_string();
            let mut replace_words = vec![];
            for (i, word) in words.iter().enumerate() {
                if rs.contains(word) {
                    let ix = rs.find(*word).unwrap();
                    replace_words.push((*word, i + 1, ix));
                }
            }
            // replace word to number use replace_word
            // replace_words sort by ix
            replace_words.sort_by(|a, b| a.2.cmp(&b.2));
            // replace
            for (word, num, _ix) in replace_words {
                if rs.contains(word) {
                    rs = rs.replace(word, &num.to_string());
                }
            }
            rs
        })
        .filter(|x| x.chars().any(|c| c.is_numeric()))
        .map(|x| {
            println!("x: {}", x);
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

    println!("part2 res: {}", res);
    Ok(res)
}
