fn required_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn required_with_fuel_mass(mass: i32) -> i32 {
    let req = required_fuel(mass);
    let extra = required_fuel(req);
    if extra <= 0 {
        req
    } else {
        req + extra + required_with_fuel_mass(extra)
    }
}

fn required_with_fuel_mass_iter(mass: i32) -> i32 {
    let mut req_fuel = required_fuel(mass);
    let mut additional = required_fuel(req_fuel);
    while additional >= 0 {
        req_fuel += additional;
        additional = required_fuel(additional);
    }
    req_fuel
}

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    apply(input, required_fuel)
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    apply(input, required_with_fuel_mass_iter)
}

fn apply(input: &str, func: fn(i32) -> i32) -> i32 {
    input.lines().filter_map(|s| s.parse().ok()).map(func).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_required_fuel() {
        assert_eq!(2, required_fuel(12));
        assert_eq!(2, required_fuel(14));
        assert_eq!(654, required_fuel(1969));
        assert_eq!(33583, required_fuel(100756));
    }

    #[test]
    fn test_mass_with_fuel() {
        assert_eq!(2, required_with_fuel_mass(14));
        assert_eq!(966, required_with_fuel_mass(1969));
        assert_eq!(50346, required_with_fuel_mass(100756));
    }

    #[test]
    fn test_mass_with_fuel_iter() {
        assert_eq!(2, required_with_fuel_mass_iter(14));
        assert_eq!(966, required_with_fuel_mass_iter(1969));
        assert_eq!(50346, required_with_fuel_mass_iter(100756));
    }
}
