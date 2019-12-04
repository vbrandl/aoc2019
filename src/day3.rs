use std::{collections::HashSet, str::FromStr};

type Pos = (i16, i16);

#[aoc_generator(day3)]
fn parser(input: &str) -> Vec<Vec<Movement>> {
    input
        .lines()
        .map(|s| {
            s.split(',')
                .map(|x| x.parse::<Movement>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn wires(input: &[Vec<Movement>]) -> i32 {
    assert!(input.len() == 2);
    let r1: HashSet<_> = visited_points(&input[0]).into_iter().collect();
    let r2 = visited_points(&input[1]).into_iter().collect();

    r1.intersection(&r2)
        .into_iter()
        .map(|p| manhattan_distance(&(0, 0), p))
        .min()
        .unwrap() as i32
}

#[aoc(day3, part2)]
fn shortest(input: &[Vec<Movement>]) -> i32 {
    assert!(input.len() == 2);
    let r1 = visited_points(&input[0]);
    let r2 = visited_points(&input[1]);
    r1.iter()
        .collect::<HashSet<_>>()
        .intersection(&r2.iter().collect())
        .map(|inte| pos_in_route(&r1, inte) + pos_in_route(&r2, inte) + 2)
        .min()
        .unwrap() as i32
}

fn pos_in_route(route: &[Pos], pos: &Pos) -> usize {
    for i in 0..route.len() {
        if route[i] == *pos {
            return i;
        }
    }
    unreachable!()
}

fn manhattan_distance(a: &Pos, b: &Pos) -> i16 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn visited_points(route: &[Movement]) -> Vec<Pos> {
    let mut pos = (0, 0);
    let mut res = Vec::with_capacity(route.len());
    for mov in route {
        for _ in 1..=mov.1 {
            match mov.0 {
                Dir::Left => pos.0 -= 1,
                Dir::Right => pos.0 += 1,
                Dir::Up => pos.1 += 1,
                Dir::Down => pos.1 -= 1,
                // Dir::Left => pos = (pos.0 - 1, pos.1),
                // Dir::Right => pos = (pos.0 + 1, pos.1),
                // Dir::Up => pos = (pos.0, pos.1 + 1),
                // Dir::Down => pos = (pos.0, pos.1 - 1),
            }
            res.push(pos);
        }
    }
    res
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        match inp {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Movement(Dir, i16);

impl FromStr for Movement {
    type Err = ();

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        if inp.len() < 2 {
            Err(())
        } else {
            let dir = Dir::from_str(&inp[0..1])?;
            let amount = i16::from_str(&inp[1..]).map_err(|_| ())?;
            Ok(Movement(dir, amount))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_movement() {
        assert_eq!(Movement(Dir::Left, 5), "L5".parse().unwrap());
    }

    #[test]
    fn test_example_input() {
        let data = [
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ];
        for (input, should) in data.iter() {
            assert_eq!(*should, wires(&parser(input)));
        }
    }

    #[test]
    fn test_example_input_part2() {
        let data = [
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                610,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            ),
        ];
        for (input, should) in data.iter() {
            assert_eq!(*should, shortest(&parser(input)));
        }
    }
}
