use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Debug)]
enum LightCommand {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for LightCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(Self::TurnOn),
            "turn off" => Ok(Self::TurnOff),
            "toggle" => Ok(Self::Toggle),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LightCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::TurnOn => write!(f, "turn on"),
            &Self::TurnOff => write!(f, "turn off"),
            &Self::Toggle => write!(f, "toggle"),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Light {
    on: bool,
}

impl Light {
    fn command_light(&mut self, command: &LightCommand) {
        match command {
            LightCommand::TurnOn => self.on = true,
            LightCommand::TurnOff => self.on = false,
            LightCommand::Toggle => self.on = !self.on,
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    lights: Vec<Vec<Light>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            lights: vec![vec![Light { on: false }; width]; height],
        }
    }

    fn command_lights_in_grid(
        &mut self,
        point_from: Point,
        point_to: Point,
        command: LightCommand,
    ) {
        for y in point_from.y..point_to.y + 1{
            for x in point_from.x..point_to.x + 1 {
                self.lights[x][y].command_light(&command);
            }
        }
    }

    fn get_num_lights_on(self) -> usize {
        let mut num_lights_on: usize = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                num_lights_on += self.lights[x][y].on as usize;
            }
        }

        num_lights_on
    }
}

#[derive(Debug)]
struct Input {
    command: LightCommand,
    point_from: Point,
    point_to: Point,
}

fn parse_input(input: String) -> Input {
    let caps =
        Regex::new(r"(turn on|turn off|toggle)\s([0-9]+),([0-9]+)\sthrough\s([0-9]+),([0-9]+)")
            .unwrap()
            .captures(&input)
            .unwrap();

    Input {
        command: LightCommand::from_str(&caps[1]).unwrap(),
        point_from: Point {
            x: usize::from_str(&caps[2]).unwrap(),
            y:  usize::from_str(&caps[3]).unwrap(),
        },
        point_to: Point {
            x:  usize::from_str(&caps[4]).unwrap(),
            y:  usize::from_str(&caps[5]).unwrap(),
        },
    }
}

#[derive(Debug)]
struct Arguments {
    input: String
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Wrong number of arguments: expected 1, got {}.", args.len());
    }

    Arguments {
        input: args[0].clone(),
    }
}

fn main() {
    let args = parse_args();
    let mut grid = Grid::new(1000, 1000);

    for line in args.input.lines() {
        let input = parse_input(line.to_string());
        grid.command_lights_in_grid(input.point_from, input.point_to, input.command);
    }

    println!("{}", grid.get_num_lights_on())
}
