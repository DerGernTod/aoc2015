use std::{fs, collections::{HashSet, HashMap}};
#[derive(Hash, PartialEq, Eq)]
struct Coords(usize, usize);

enum Command {
    On,
    Off,
    Toggle
}

#[allow(dead_code)]
pub fn day_06() {
    let commands = parse_into_commands("./input/day_06.txt");
    let lights = apply_commands(&commands);
    println!("{} lights are on,", lights.len());
    let commands = parse_into_commands("./input/day_06.txt");
    let lights = apply_commands_brightness(&commands);
    println!("with a brightness of {}!", calc_total_brightness(&lights));
}

fn parse_into_commands(path: &str) -> Vec<(Command, Coords, Coords)> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| {
            let cleaned = line.replace("turn ", "").replace("through ", "");
            let mut split = cleaned.split(' ');
            let cmd = split.next().unwrap();
            let mut from = split.next().unwrap().split(',');
            let mut to = split.next().unwrap().split(',');
            let from = Coords(from.next().unwrap().parse::<usize>().unwrap(), from.next().unwrap().parse::<usize>().unwrap());
            let to = Coords(to.next().unwrap().parse::<usize>().unwrap(), to.next().unwrap().parse::<usize>().unwrap());
            match cmd {
                "toggle" => (Command::Toggle, from, to),
                "on" => (Command::On, from, to),
                "off" => (Command::Off, from, to),
                _ => panic!("Invalid command: {}", cmd)
            }
        })
        .collect()
}

fn apply_commands(commands: &[(Command, Coords, Coords)]) -> HashSet<Coords> {
    let mut lights = HashSet::with_capacity(1_000_000);

    for command in commands {
        apply_command(command, &mut lights);
    }
    lights
}

fn apply_command(command: &(Command, Coords, Coords), lights: &mut HashSet<Coords>) {
    for x in command.1.0..=command.2.0 {
        for y in command.1.1..=command.2.1 {

            match command.0 {
                Command::On => { lights.insert(Coords(x, y)); },
                Command::Off => { lights.remove(&Coords(x, y)); },
                Command::Toggle => {
                    let target = Coords(x, y);
                    if lights.contains(&target) {
                        lights.remove(&target);
                    } else {
                        lights.insert(target);
                    }
                },
            };
        }
    }
}

fn apply_commands_brightness(commands: &[(Command, Coords, Coords)]) -> HashMap<Coords, usize> {
    let mut lights = HashMap::with_capacity(1_000_000);

    for command in commands {
        apply_command_brightness(command, &mut lights);
    }
    lights
}

fn apply_command_brightness(command: &(Command, Coords, Coords), lights: &mut HashMap<Coords, usize>) {
    for x in command.1.0..=command.2.0 {
        for y in command.1.1..=command.2.1 {
            match command.0 {
                Command::On => { lights.entry(Coords(x, y)).and_modify(|e| *e += 1).or_insert(1); },
                Command::Off => { lights.entry(Coords(x, y)).and_modify(|e| *e = e.saturating_sub(1)).or_insert(0); },
                Command::Toggle => { lights.entry(Coords(x, y)).and_modify(|e| *e += 2).or_insert(2); },
            };
        }
    }
}

fn calc_total_brightness(lights: &HashMap<Coords, usize>) -> usize {
    lights.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_06::{parse_into_commands, apply_commands, calc_total_brightness, apply_commands_brightness};

    #[test]
    fn test_part_1() {
        let commands = parse_into_commands("./input/day_06.test.txt");
        assert_eq!(commands.len(), 1);
        let lights = apply_commands(&commands);
        assert_eq!(lights.len(), 1_000_000);
        let commands = parse_into_commands("./input/day_06.test.2.txt");
        assert_eq!(commands.len(), 1);
        let lights = apply_commands(&commands);
        assert_eq!(lights.len(), 1_000);
    }

    #[test]
    fn test_part_2() {
        let num = 2usize;
        assert_eq!(num.saturating_sub(4), 0);
        let commands = parse_into_commands("./input/day_06.test.2.txt");
        let lights = apply_commands_brightness(&commands);
        assert_eq!(calc_total_brightness(&lights), 2000);
    }
}