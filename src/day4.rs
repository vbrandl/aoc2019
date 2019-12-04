use std::collections::HashMap;

#[aoc_generator(day4)]
fn range_parser(input: &str) -> (Vec<i32>, i32, i32) {
    let mut parts = input.split('-');
    let start = parts.next().unwrap().parse::<i32>().unwrap();
    let end = parts.next().unwrap().parse::<i32>().unwrap();
    ((start..=end).collect(), start, end)
}

#[aoc(day4, part1)]
fn password((input, start, end): &(Vec<i32>, i32, i32)) -> i32 {
    let filter = |val| {
        in_range(*start, *end, val)
            && six_digits(val)
            && adjacend_digits(val)
            && never_decreasing(val)
    };
    input.into_iter().filter(|val| filter(**val)).count() as i32
}

#[aoc(day4, part2)]
fn password_more_criteria((input, start, end): &(Vec<i32>, i32, i32)) -> i32 {
    let filter = |val| {
        in_range(*start, *end, val)
            && six_digits(val)
            && adjacend_digits(val)
            && never_decreasing(val)
            && group_of_exactly_2(val)
    };
    input.into_iter().filter(|val| filter(**val)).count() as i32
}

fn six_digits(val: i32) -> bool {
    val >= 100000 && val <= 999999
}

fn in_range(start: i32, end: i32, val: i32) -> bool {
    val >= start && val <= end
}

fn adjacend_digits(val: i32) -> bool {
    digits(val)
        .into_iter()
        .fold((false, -1), |(res, last), next| (res || last == next, next))
        .0
}

fn never_decreasing(val: i32) -> bool {
    digits(val)
        .into_iter()
        .fold((true, -1), |(res, last), next| (res && last <= next, next))
        .0
}

fn group_of_exactly_2(val: i32) -> bool {
    let mut map = HashMap::new();
    for digit in digits(val) {
        *map.entry(digit).or_insert(0) += 1;
    }
    map.into_iter().any(|(_, v)| v == 2)
}

fn digits(inp: i32) -> Vec<i32> {
    inp.to_string()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_examples() {
        let filter = |val| six_digits(val) && adjacend_digits(val) && never_decreasing(val);
        let data = [(111111, true), (223450, false), (123789, false)];
        for (inp, should) in data.into_iter() {
            assert_eq!(*should, filter(*inp));
        }
    }

    #[test]
    fn test_examples2() {
        let filter = |val| {
            six_digits(val)
                && adjacend_digits(val)
                && never_decreasing(val)
                && group_of_exactly_2(val)
        };
        let data = [(112233, true), (123444, false), (111122, true)];
        for (inp, should) in data.into_iter() {
            assert_eq!(*should, filter(*inp));
        }
    }
}
