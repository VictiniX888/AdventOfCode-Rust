use aoc;

fn main() {
    let input = aoc::read_input(20);
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm: Vec<bool> = algorithm.chars().map(parse_char).collect();
    let image: Vec<Vec<bool>> = image
        .lines()
        .map(|s| s.chars().map(parse_char).collect())
        .collect();

    println!("{}", solve(&algorithm, image.clone(), 2));
    println!("{}", solve(&algorithm, image.clone(), 50));
}

// COMMON
// assumes first bit of algorithm is 1 and last bit is 0
fn solve(algorithm: &Vec<bool>, input_image: Vec<Vec<bool>>, times: usize) -> usize {
    let mut image = input_image;

    for i in 0..times {
        image = enhance(algorithm, image, if i % 2 == 0 { false } else { true });
    }

    image.iter().flatten().filter(|&&b| b).count()
}

fn enhance(
    algorithm: &Vec<bool>,
    input_image: Vec<Vec<bool>>,
    outside_bit: bool,
) -> Vec<Vec<bool>> {
    let mut image: Vec<Vec<bool>> = vec![];

    for r in -1..=input_image.len() as i32 {
        let mut row: Vec<bool> = vec![];
        for c in -1..=input_image[0].len() as i32 {
            // get binary number
            let index = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .into_iter()
            .map(|(dr, dc)| {
                if r + dr < 0
                    || c + dc < 0
                    || r + dr >= input_image.len() as i32
                    || c + dc >= input_image[0].len() as i32
                {
                    outside_bit
                } else {
                    input_image[(r + dr) as usize][(c + dc) as usize]
                }
            })
            .fold(0, |acc, b| acc * 2 + if b { 1 } else { 0 });

            let b = algorithm[index];

            row.push(b);
        }

        image.push(row);
    }

    image
}

#[allow(dead_code)]
fn pretty_print(grid: &Vec<Vec<bool>>) {
    let str: String = grid
        .iter()
        .map(|row| row.iter().map(|&b| if b { '#' } else { '.' }).collect())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", str);
}

fn parse_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Failed to parse char!"),
    }
}
