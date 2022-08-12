fn main() {
    let args = parse_args();

    let mut santa = Santa {
        current_location: Location { x: 0, y: 0 },
        locations: vec![],
    };
    let mut robo_santa = Santa {
        current_location: Location { x: 0, y: 0 },
        locations: vec![],
    };

    let santa_directions: String = args.input.chars().step_by(2).collect();
    let robo_santa_directions: String = (&args.input as &str)[1..].chars().step_by(2).collect();

    santa.add_location(santa.current_location.clone());
    robo_santa.add_location(santa.current_location.clone());

    give_santa_directions(santa_directions, &mut santa);
    give_santa_directions(robo_santa_directions, &mut robo_santa);

    println!("Number of houses Santa visted: {}", santa.locations.len());
    println!(
        "Number of houses Robo Santa visted: {}",
        robo_santa.locations.len()
    );

    {
        let mut combined_locations =
            [santa.locations.clone(), robo_santa.locations.clone()].concat();
        combined_locations.sort_by_key(|l| l.x);
        combined_locations.sort_by_key(|l| l.y);
        combined_locations.dedup();
        println!(
            "Number of houses Santa visted: {}",
            combined_locations.len()
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Santa {
    current_location: Location,
    locations: Vec<Location>,
}

#[derive(Debug)]
struct Arguments {
    input: String,
}

impl Santa {
    fn up(&mut self) {
        self.current_location.y += 1
    }

    fn down(&mut self) {
        self.current_location.y -= 1
    }

    fn left(&mut self) {
        self.current_location.x -= 1
    }

    fn right(&mut self) {
        self.current_location.x += 1
    }

    fn add_location(&mut self, location: Location) {
        if !self.locations.contains(&location) {
            self.locations.push(location);
        }
    }

    fn move_location(&mut self, movement: char) {
        match movement {
            '^' => self.up(),
            'v' => self.down(),
            '>' => self.right(),
            '<' => self.left(),
            _ => (),
        }
    }
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

fn give_santa_directions(input: String, santa: &mut Santa) {
    for movement in input.chars() {
        santa.move_location(movement);
        santa.add_location(santa.current_location.clone());
    }
}
