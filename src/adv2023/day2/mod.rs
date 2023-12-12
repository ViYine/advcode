use std::str::FromStr;

mod part1;
mod part2;

// 12 red cubes, 13 green cubes, and 14 blue cubes
const RED_NUM: usize = 12;
const GREEN_NUM: usize = 13;
const BLUE_NUM: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GameInfo {
    game_id: usize,
    blue: usize,
    red: usize,
    green: usize,
}

impl FromStr for GameInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("input: {}", s);
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let parts = s.split(':').collect::<Vec<&str>>();
        // Game 1
        // println!("{:?}", parts);
        let game_id_strs = parts[0].trim().split(' ').collect::<Vec<&str>>();

        let game_id = game_id_strs[1].trim().parse::<usize>().unwrap();
        // println!("game_id: {}", game_id);

        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let balls = parts[1].trim();
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        // println!("balls: {}", balls);
        let _ = balls
            .split(';')
            .map(|s| {
                // println!("s: {}", s);
                let _ = s
                    .trim()
                    .split(',')
                    .map(|x| {
                        // println!("x: {}", x);
                        let color_nums = x.trim().split(' ').collect::<Vec<&str>>();
                        let color = color_nums[1].trim();
                        let num = color_nums[0].trim().parse::<usize>().unwrap();
                        match color {
                            "blue" => {
                                if num > blue {
                                    blue = num
                                }
                            }
                            "red" => {
                                if num > red {
                                    red = num
                                }
                            }
                            "green" => {
                                if num > green {
                                    green = num
                                }
                            }
                            _ => {}
                        }
                    })
                    .collect::<Vec<()>>();
            })
            .collect::<Vec<()>>();

        Ok(Self {
            game_id,
            blue,
            red,
            green,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Result;

    #[test]
    fn test_game_info_from_str() -> color_eyre::Result<()> {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let res = GameInfo::from_str(s).unwrap();
        assert_eq!(res.game_id, 1);
        assert_eq!(res.blue, 6);
        assert_eq!(res.red, 4);
        assert_eq!(res.green, 2);
        Ok(())
    }

    #[test]
    fn day2_part1_test_should_work() -> Result<()> {
        let res = part1::solve("src/adv2023/day2/test.txt".into())?;
        println!("part1 res: {}", res);
        assert_eq!(res, 8);
        Ok(())
    }

    #[test]
    fn day2_part1_should_work() -> Result<()> {
        let res = part1::solve("src/adv2023/day2/input.txt".into())?;
        println!("part1 res: {}", res);
        assert_eq!(res, 1853);
        Ok(())
    }

    #[test]
    fn day2_part2_test_should_work() -> Result<()> {
        let res = part2::solve("src/adv2023/day2/test.txt".into())?;
        println!("part2 res: {}", res);
        assert_eq!(res, 2286);
        Ok(())
    }

    #[test]
    fn day2_part2_should_work() -> Result<()> {
        let res = part2::solve("src/adv2023/day2/input.txt".into())?;
        println!("part2 res: {}", res);
        assert_eq!(res, 72706);
        Ok(())
    }
}
