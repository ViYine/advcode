mod part1;
mod part2;

// 12 red cubes, 13 green cubes, and 14 blue cubes
const RED_NUM: usize = 12;
const GREEN_NUM: usize = 13;
const BLUE_NUM: usize = 14;

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Result;

    #[test]
    fn day2_part1_test_should_work() -> Result<()> {
        let res = part1::solve("src/adv2023/day2/test1.txt".into())?;
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
}
