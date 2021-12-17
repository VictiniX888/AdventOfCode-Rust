use aoc;

fn main() {
    let input = aoc::read_input(17);
    let input = input.trim_end();
    let input = input
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|s| {
            s.split_once('=')
                .unwrap()
                .1
                .split_once("..")
                .and_then(|(a, b)| Some((aoc::parse_str(a), aoc::parse_str(b))))
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    let (x1, x2) = input[0];
    let (y1, y2) = input[1];

    println!("{}", part_1(x1, x2, y1, y2));
    println!("{}", part_2(x1, x2, y1, y2));
}

// PART 1
fn part_1(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let mut vx_min = -1;
    let mut sum = 0;
    for vx in 0.. {
        sum += vx;
        if sum >= x1 {
            vx_min = vx;
            break;
        }
    }

    let vx_max = x2;

    let vy_min = 0;
    let vy_max = y1.abs();

    let mut highest = 0;
    for vx in vx_min..=vx_max {
        for vy in vy_min..=vy_max {
            let mut vx = vx;
            let mut vy = vy;
            let mut x = 0;
            let mut y = 0;
            let mut new_highest = 0;
            loop {
                x += vx;
                y += vy;

                if y > new_highest {
                    new_highest = y;
                }

                if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
                    if new_highest > highest {
                        highest = new_highest;
                    }

                    break;
                }

                if x > x2 || y < y1 {
                    break;
                }

                vx -= (vx - 0).signum();
                vy -= 1;
            }
        }
    }

    highest
}

// PART 2
fn part_2(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let mut vx_min = -1;
    let mut sum = 0;
    for vx in 0.. {
        sum += vx;
        if sum >= x1 {
            vx_min = vx;
            break;
        }
    }

    let vx_max = x2;

    let vy_min = y1;
    let vy_max = y1.abs();

    let mut sum = 0;
    for vx in vx_min..=vx_max {
        for vy in vy_min..=vy_max {
            let mut vx = vx;
            let mut vy = vy;
            let mut x = 0;
            let mut y = 0;
            loop {
                x += vx;
                y += vy;

                if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
                    sum += 1;
                    break;
                }

                if x > x2 || y < y1 {
                    break;
                }

                vx -= (vx - 0).signum();
                vy -= 1;
            }
        }
    }

    sum
}
