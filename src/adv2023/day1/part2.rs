use std::{collections::HashMap, path::PathBuf};

use color_eyre::Result;

#[allow(dead_code)]
pub(crate) fn solve(input_path: PathBuf) -> Result<i32> {
    let words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let input = std::fs::read_to_string(input_path)?;
    let res = input
        .split('\n')
        // not contains number
        // replace all word to number: one, two, three, four, five, six, seven, eight, nine
        // .map(|s| {
        //     let mut rs = s.to_string();
        //     let mut replace_words = vec![];
        //     for (i, word) in words.iter().enumerate() {
        //         if rs.contains(word) {
        //             let ix = rs.find(*word).unwrap();
        //             replace_words.push((*word, i + 1, ix));
        //         }
        //     }
        //     // replace word to number use replace_word
        //     // replace_words sort by ix
        //     replace_words.sort_by(|a, b| a.2.cmp(&b.2));
        //     // replace
        //     for (word, num, _ix) in replace_words {
        //         if rs.contains(word) {
        //             rs = rs.replace(word, &num.to_string());
        //         }
        //     }
        //     rs
        // })
        // .filter(|x| x.chars().any(|c| c.is_numeric()))
        .map(|x| {
            // println!("x: {}", x);
            // find first number
            let mut first_num = 0;
            let mut last_num = 0;
            'outer: for i in 0..x.len() {
                let sx = &x[i..];
                for w in words.iter() {
                    if sx.starts_with(w.0) {
                        first_num = *w.1;
                        break 'outer;
                    }
                }
            }
            // reverse
            'outer1: for i in (0..x.len()).rev() {
                let sx = &x[i..];
                for w in words.iter() {
                    if sx.starts_with(w.0) {
                        last_num = *w.1;
                        break 'outer1;
                    }
                }
            }
            first_num * 10 + last_num
        })
        .sum::<i32>();

    println!("part2 res: {}", res);
    Ok(res)
}
