use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Instr {
    Acc,
    Jmp,
    Nop,
}

impl Instr {
    fn fix(&self) -> Option<Self> {
        match self {
            Self::Acc => None,
            Self::Jmp => Some(Self::Nop),
            Self::Nop => Some(Self::Jmp),
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            "nop" => Ok(Self::Nop),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Instr, isize)>, ()> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let instr = parts.next().ok_or(()).and_then(|s| s.parse::<Instr>())?;
            let arg = parts.next().ok_or(())?.parse::<isize>().map_err(|_| ())?;
            Ok((instr, arg))
        })
        .collect()
}

fn eval((instr, arg): &(Instr, isize), acc: &mut isize) -> isize {
    match instr {
        Instr::Acc => {
            *acc += arg;
            1
        }
        Instr::Jmp => *arg,
        Instr::Nop => 1,
    }
}

struct Process {
    acc: isize,
    instr_idx: isize,
    visited_instrs: Vec<bool>,
}

impl Process {
    fn execute(instrs: &[(Instr, isize)]) -> Self {
        let mut acc = 0;
        let mut instr_idx: isize = 0;
        let mut visited_instrs = vec![false; instrs.len()];
        while let Ok(idx) = TryInto::<usize>::try_into(instr_idx) {
            if visited_instrs.get(idx) != Some(&false) {
                break;
            }
            visited_instrs[idx] = true;
            instr_idx += eval(&instrs[idx], &mut acc);
        }
        Self {
            acc,
            instr_idx,
            visited_instrs,
        }
    }

    fn get_acc_at_loop(&self) -> Option<isize> {
        if let Ok(idx) = TryInto::<usize>::try_into(self.instr_idx) {
            self.visited_instrs
                .get(idx)
                .and_then(|looped| if *looped { Some(self.acc) } else { None })
        } else {
            None
        }
    }

    fn get_acc_at_stop(&self) -> Option<isize> {
        if self.instr_idx
            == self
                .visited_instrs
                .len()
                .try_into()
                .expect("instrs should fit")
        {
            Some(self.acc)
        } else {
            None
        }
    }
}

fn fix_and_execute(instrs: &[(Instr, isize)]) -> Option<isize> {
    let initial_run = Process::execute(instrs);
    for (idx, ((code, arg), visited)) in instrs
        .iter()
        .zip(initial_run.visited_instrs.iter())
        .enumerate()
    {
        if *visited {
            if let Some(fixed) = code.fix() {
                let mut fixed_instrs = instrs.to_owned();
                fixed_instrs[idx] = (fixed, *arg);
                if let Some(acc) = Process::execute(&fixed_instrs).get_acc_at_stop() {
                    return Some(acc);
                }
            }
        }
    }
    None
}

fn main() {
    let instrs =
        parse_input(include_str!("../../inputs/day08.txt")).expect("failed to parse input");
    println!(
        "Part 1: {}",
        Process::execute(&instrs)
            .get_acc_at_loop()
            .expect("initial run should enter infinite loop")
    );
    println!(
        "Part 2: {}",
        fix_and_execute(&instrs).expect("should be able to fix")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let instrs = parse_input(include_str!("../../inputs/day08-sample.txt"))
            .expect("failed to parse input");
        assert_eq!(Process::execute(&instrs).get_acc_at_loop(), Some(5));
    }

    #[test]
    fn test_part2_sample() {
        let instrs = parse_input(include_str!("../../inputs/day08-sample.txt"))
            .expect("failed to parse input");
        assert_eq!(fix_and_execute(&instrs), Some(8));
    }
}
