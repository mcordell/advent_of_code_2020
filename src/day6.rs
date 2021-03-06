use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type AnsweredYes = char;
type Form = HashSet<AnsweredYes>;
type Group = Vec<Form>;

fn parse_group(group_str: &str) -> Group {
    group_str
        .lines()
        .map(|form_str| form_str.chars().collect())
        .collect()
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|group_str| parse_group(group_str))
        .collect()
}

fn count_any_yes(group: &[Form]) -> usize {
    group
        .iter()
        .fold(HashSet::new(), |acc, form| {
            acc.union(form).cloned().collect()
        })
        .len()
}

#[aoc(day6, part1)]
fn part1(groups: &[Group]) -> usize {
    groups.iter().map(|group| count_any_yes(group)).sum()
}

fn count_all_yes(group: &[Form]) -> usize {
    group
        .iter()
        .skip(1)
        .fold(group[0].clone(), |acc, form| {
            acc.intersection(form).cloned().collect()
        })
        .len()
}

#[aoc(day6, part2)]
fn part2(groups: &[Group]) -> usize {
    groups.iter().map(|group| count_all_yes(group)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_GROUP_1: &str = "abc";
    static TEST_GROUP_2: &str = r"a
b
c";
    static TEST_GROUP_3: &str = r"ab
ac";
    static TEST_GROUP_4: &str = r"a
a
a
a";
    static TEST_GROUP_5: &str = "b";
    static TEST_INPUT: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_example() {
        assert_eq!(
            count_any_yes(&parse_group(
                r"abcx
abcy
abcz"
            )),
            6
        );

        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_1)), 3);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_2)), 3);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_3)), 3);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_4)), 1);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_5)), 1);

        assert_eq!(part1(&parse_input(TEST_INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_1)), 3);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_2)), 0);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_3)), 1);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_4)), 1);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_5)), 1);

        assert_eq!(part2(&parse_input(TEST_INPUT)), 6);
    }
}
