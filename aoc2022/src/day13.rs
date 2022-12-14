use std::{cmp::Ordering, iter::Peekable, str::Bytes};

use crate::*;

pub const SOLUTION: Solution = Solution { day: 13, solve };

// ~1 ms
fn solve(input: &str) -> AnswerSet {
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    let mut div1_count = 0;
    let mut div2_count = 0;

    let p1 = input
        .split("\n\n")
        .map(|group| group.split_once('\n').unwrap())
        .map(|(packet1, packet2)| {
            let packet1 = parse_packet(&mut packet1.bytes().peekable());
            let packet2 = parse_packet(&mut packet2.bytes().peekable());

            if cmp_packets(&packet1, &div1) == Ordering::Less {
                div1_count += 1;
            } else if cmp_packets(&packet1, &div2) == Ordering::Less {
                div2_count += 1;
            }

            if cmp_packets(&packet2, &div1) == Ordering::Less {
                div1_count += 1;
            } else if cmp_packets(&packet2, &div2) == Ordering::Less {
                div2_count += 1;
            }

            cmp_packets(&packet1, &packet2)
        })
        .enumerate()
        .filter(|&(_, ord)| ord == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let p2 = (div1_count + 1) * (div1_count + div2_count + 2);

    AnswerSet {
        p1: Answer::Usize(p1),
        p2: Answer::Usize(p2),
    }
}

fn cmp_packets(packet1: &Packet, packet2: &Packet) -> Ordering {
    match packet1 {
        Packet::Num(num1) => match packet2 {
            Packet::Num(num2) => num1.cmp(&num2),
            Packet::List(list2) => {
                if list2.is_empty() {
                    Ordering::Greater
                } else {
                    let cmp = cmp_packets(packet1, &list2[0]);
                    if cmp != Ordering::Equal {
                        cmp
                    } else if list2.len() == 1 {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
            }
        },
        Packet::List(list1) => match packet2 {
            Packet::Num(_) => {
                if list1.is_empty() {
                    Ordering::Less
                } else {
                    let cmp = cmp_packets(&list1[0], packet2);
                    if cmp != Ordering::Equal {
                        cmp
                    } else if list1.len() == 1 {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                }
            }
            Packet::List(list2) => {
                let len_cmp = list1.len().cmp(&list2.len());
                list1
                    .into_iter()
                    .zip(list2)
                    .map(|(num1, num2)| cmp_packets(num1, num2))
                    .find(|&ord| ord != Ordering::Equal)
                    .unwrap_or_else(|| len_cmp)
            }
        },
    }
}

fn parse_packet(bytes: &mut Peekable<Bytes>) -> Packet {
    let mut packets = Vec::new();

    while bytes.next().unwrap() != b']' {
        let &next = bytes.peek().unwrap();
        match next {
            b'[' => packets.push(parse_packet(bytes)),
            b']' => continue,
            _ => packets.push(Packet::Num(parse_u8_from_peekable_iter(bytes))),
        }
    }

    Packet::List(packets)
}

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Num(u8),
}

fn parse_u8_from_peekable_iter(ascii_iter: &mut Peekable<Bytes>) -> u8 {
    let mut num = 0;

    while let Some(byte) = ascii_iter.peek() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += byte - b'0';

        ascii_iter.next();
    }

    num
}
