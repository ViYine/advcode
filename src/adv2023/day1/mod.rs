mod part1;
mod part2;
// 问题描述： https://adventofcode.com/2023/day/1

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Result;

    #[test]
    fn part1_test_should_work() -> Result<()> {
        let res = part1::solve("src/adv2023/day1/test1.txt".into())?;
        println!("part1 res: {}", res);
        assert_eq!(res, 142);
        Ok(())
    }

    #[test]
    fn part1_should_work() -> Result<()> {
        let res = part1::solve("src/adv2023/day1/input.txt".into())?;
        println!("part1 res: {}", res);
        assert_eq!(res, 54630);
        Ok(())
    }

    #[test]
    fn part2_test_should_work() -> Result<()> {
        let res = part2::solve("src/adv2023/day1/test2.txt".into())?;
        println!("part2 res: {}", res);
        assert_eq!(res, 281);
        Ok(())
    }

    #[test]
    fn part2_should_work() -> Result<()> {
        let res = part2::solve("src/adv2023/day1/input.txt".into())?;
        println!("part2 res: {}", res);
        assert_eq!(res, 54770);
        Ok(())
    }
}
