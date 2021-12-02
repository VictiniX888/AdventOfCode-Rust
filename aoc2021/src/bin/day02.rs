use aoc;

fn main() {
    let input = aoc::read_input(2);
    let input = input.lines().map(Command::parse);

    println!("{}", part_1(input.clone()));
    println!("{}", part_2(input));
}

// PART 1
fn part_1<T>(commands: T) -> i32
where
    T: Iterator<Item = Command>,
{
    let final_position = commands.fold(Point { x: 0, y: 0 }, move_point);

    final_position.x * final_position.y
}

struct Point {
    x: i32,
    y: i32,
}

fn move_point(point: Point, command: Command) -> Point {
    match command.movement.as_str() {
        "forward" => move_point_forward(point, command.steps),
        "down" => move_point_deeper(point, command.steps),
        "up" => move_point_deeper(point, -command.steps),
        _ => panic!(
            "Unknown command {} {} found",
            command.movement, command.steps
        ),
    }
}

fn move_point_forward(point: Point, steps: i32) -> Point {
    Point {
        x: point.x + steps,
        y: point.y,
    }
}

fn move_point_deeper(point: Point, steps: i32) -> Point {
    Point {
        x: point.x,
        y: point.y + steps,
    }
}

// PART 2
fn part_2<T>(commands: T) -> i32
where
    T: Iterator<Item = Command>,
{
    let final_position = commands.fold(
        State {
            pos_horizontal: 0,
            depth: 0,
            aim: 0,
        },
        move_submarine,
    );

    final_position.depth * final_position.pos_horizontal
}

struct State {
    pos_horizontal: i32,
    depth: i32,
    aim: i32,
}

fn move_submarine(state: State, command: Command) -> State {
    match command.movement.as_str() {
        "down" => move_aim(state, command.steps),
        "up" => move_aim(state, -command.steps),
        "forward" => move_forward(state, command.steps),
        _ => panic!(
            "Unknown command {} {} found",
            command.movement, command.steps
        ),
    }
}

fn move_aim(state: State, steps: i32) -> State {
    State {
        aim: state.aim + steps,
        ..state
    }
}

fn move_forward(state: State, steps: i32) -> State {
    State {
        pos_horizontal: state.pos_horizontal + steps,
        depth: state.depth + state.aim * steps,
        aim: state.aim,
    }
}

// COMMON
struct Command {
    movement: String,
    steps: i32,
}

impl Command {
    fn parse(s: &str) -> Command {
        let mut command_iter = s.split_whitespace();

        Command {
            movement: command_iter.next().unwrap().to_string(),
            steps: command_iter.next().unwrap().parse().unwrap(),
        }
    }
}
