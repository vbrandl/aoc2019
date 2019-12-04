use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[aoc_generator(day2)]
fn parser(input: &str) -> Program {
    input.parse().unwrap()
}

#[aoc(day2, part1)]
fn execute(prg: &Program) -> i32 {
    let mut prg = prg.clone();
    prg[1] = 12;
    prg[2] = 2;
    prg.execute().parse::<Program>().unwrap()[0] as i32
}

#[aoc(day2, part2)]
fn find(prg_inp: &Program) -> i32 {
    let mut prg;
    let mut noun: i32 = 0;
    let mut verb: i32 = 0;
    loop {
        prg = prg_inp.clone();
        prg[1] = noun as usize;
        prg[2] = verb as usize;
        let res = prg.execute().parse::<Program>().unwrap()[0] as i32;
        if res == 19690720 {
            return (100 * noun + verb) as i32;
        }
        if verb == std::i16::MAX as i32 {
            verb = 0;
            noun += 1;
        } else {
            verb += 1;
        }
    }
}

#[derive(Clone)]
struct Program(Vec<usize>);

impl FromStr for Program {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Program(
            input.split(',').filter_map(|s| s.parse().ok()).collect(),
        ))
    }
}

impl Program {
    fn can_use(&self, idx: usize) -> bool {
        idx < self.len()
    }

    fn execute(&mut self) -> String {
        let mut ip = 0;
        loop {
            let op = self[ip];
            match op {
                1 => {
                    if !self.can_use(ip + 2) {
                        return "0".to_string();
                    }
                    let a = self[ip + 1];
                    let b = self[ip + 2];
                    let target = self[ip + 3];
                    if !self.can_use(target) || !self.can_use(a) || !self.can_use(b) {
                        return "0".to_string();
                    }
                    self[target] = self[a] + self[b];
                    ip += 4;
                }
                2 => {
                    if !self.can_use(ip + 2) {
                        return "0".to_string();
                    }
                    let a = self[ip + 1];
                    let b = self[ip + 2];
                    let target = self[ip + 3];
                    if !self.can_use(target) || !self.can_use(a) || !self.can_use(b) {
                        return "0".to_string();
                    }
                    self[target] = self[a] * self[b];
                    ip += 4;
                }
                _ => break,
            }
        }
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Deref for Program {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Program {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_and_execute() {
        let data = [
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ];
        for date in data.into_iter() {
            assert_eq!(date.1, &date.0.parse::<Program>().unwrap().execute())
        }
    }
}
