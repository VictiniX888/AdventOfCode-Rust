use std::collections::{BTreeMap, VecDeque};

use crate::*;

pub const SOLUTION: Solution = Solution { day: 16, solve };

const N_ALL: usize = 54;
const N: usize = 16;
const SPACE: usize = 2_usize.pow(25);
const LIMIT: u32 = 30;

// ~27 ms
fn solve(input: &str) -> AnswerSet {
    let map = generate_map(input);
    let dist = get_pairwise_distances(&map);

    let map: Vec<u32> = map
        .values()
        .enumerate()
        .filter(|(i, valve)| valve.flow_rate > 0 || i == &0)
        .map(|(_, valve)| valve.flow_rate)
        .collect();

    let (p1, _) = bfs(&map, &dist, 0);
    let p2 = bfs_double(&map, &dist, 4);

    AnswerSet {
        p1: Answer::U32(p1),
        p2: Answer::U32(p2),
    }
}

// Doesn't work for the example input (yet) because it assumes very specifically that we have 16 "interesting" valves
fn bfs_double(flow_map: &[u32], dist: &[[u32; N]; N], start_time: u32) -> u32 {
    let (_, cache) = bfs(flow_map, dist, start_time);

    let mut max = 0;

    // Our end states exist at the region where timestep = LIMIT - 1 and loc = 0
    // We only have to check the first half of the range because the other half will necessarily be covered by the elephant
    for state1 in (LIMIT - 1) << 20..(((LIMIT - 1) << 20) + (1 << (16 - 1))) {
        let flow_rate_1 = cache[state1 as usize];

        // Check all disjoint sets of *opened* valves
        // i.e. no time is wasted by having both agents open the same valve
        // This makes it easy to calculate the total flow

        // We generate disjoint sets by iterating over the submasks of the bitmask
        // See: https://cp-algorithms.com/algebra/all-submasks.html#enumerating-all-submasks-of-a-given-mask

        // state2 is a misnomer, it is the bitmask that represents the set of valves the elephant could open
        let state2_max = state1 & 0xFFFF;
        let mut state2 = state2_max;

        while state2 != 0 {
            // We get the actual state here with that terrifying bitfiddling sequence
            let total_flow =
                flow_rate_1 + cache[(((LIMIT - 1) << 20) + (!state2 & 0xFFFF) - 1) as usize];

            if total_flow > max {
                max = total_flow;
            }

            // This generates the next submask
            // s - 1 clears the rightmost set bit and sets all the bits to its right
            // We then clear any extra bits that are not in the original bitmask
            state2 = (state2 - 1) & state2_max;
        }
    }

    max
}

fn bfs(flow_map: &[u32], dist: &[[u32; N]; N], start_time: u32) -> (u32, Vec<u32>) {
    // BFS
    // Adapted from https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j0fep3h/

    // We store state as a bitset (u32) instead of as a struct
    // From LSB to MSB
    // Bit 0-15 = unopened valves
    // Bit 16-19 = current location
    // Bit 20-24 = current timestep
    // Bit 25-31 = empty (0)
    // Thus we only need a 1D array of ~33 million entries
    // Each entry represents the flow rate at that state

    let mut cache = vec![0; SPACE];

    // Max flow rate so far
    let mut max = 0;

    // Outstanding states that need to be accounted for
    let mut queue: VecDeque<u32> = VecDeque::new();

    // Start at AA
    let start_state = (start_time << 20) + 0xFFFE;
    queue.push_back(start_state);

    // Iterate until queue is empty
    while let Some(state) = queue.pop_front() {
        visit_all(&flow_map, &dist, &mut cache, state, &mut queue, &mut max);
    }

    (max, cache)
}

fn visit_all(
    flow_map: &[u32],
    dist: &[[u32; N]; N],
    cache: &mut [u32],
    state: u32,
    queue: &mut VecDeque<u32>,
    max: &mut u32,
) {
    let flow_rate = cache[state as usize];

    // Get current time
    let time = (state >> 20) & 0x1F;

    // Get current location
    let loc = (state >> 16) & 0xF;

    // Get unopened valves
    let unopened = state & 0xFFFF;

    let mut unopened_iter = unopened;

    while unopened_iter != 0 {
        // Get next unopened valve
        let next = unopened_iter.trailing_zeros();

        if dist[loc as usize][next as usize] < u32::MAX {
            // Get next timestamp
            let time_next = time + dist[loc as usize][next as usize] + 1;

            if time_next < LIMIT - 1 {
                // Get next flow rate
                let flow_next = flow_rate + flow_map[next as usize] * (30 - time_next);

                // Get new unopened valves
                let unopened_next = unopened ^ (1 << next);

                let state_next = (time_next << 20) + (next << 16) + unopened_next;

                if cache[state_next as usize] == 0 {
                    // Ensure we only check each state once
                    queue.push_back(state_next);
                }

                if cache[state_next as usize] < flow_next {
                    // Ensure we do not overwrite flow rate with a lower one
                    cache[state_next as usize] = flow_next;
                }

                cache_end_state(cache, unopened_next, flow_next, max);

                // if flow_next > *max {
                //     *max = flow_next;
                // }
            } else {
                // This is an end state
                cache_end_state(cache, unopened, flow_rate, max);
            }
        } else {
            cache_end_state(cache, unopened, flow_rate, max);
            break;
        }

        // Clear the unopened bit (i.e. opens it)
        unopened_iter ^= 1 << next;
    }
}

fn cache_end_state(cache: &mut [u32], unopened: u32, flow_rate: u32, max: &mut u32) {
    // We store end states with timestep = LIMIT - 1 (29) and location = 0 for easy lookup
    let state_end = ((LIMIT - 1) << 20) + unopened;

    if cache[state_end as usize] < flow_rate {
        cache[state_end as usize] = flow_rate;
    }

    if flow_rate > *max {
        *max = flow_rate;
    }
}

fn get_pairwise_distances(map: &BTreeMap<u32, Valve>) -> [[u32; N]; N] {
    // Adjacency matrix between valves of interest
    let mut dist = [[u32::MAX; N_ALL]; N_ALL];

    // Floyd-Warshall on unweighted graph to compute all pairwise distances
    // Initialize edges and self-cycles
    for (i, vertex) in map.iter().enumerate() {
        for &j in vertex.1.tunnels.iter() {
            dist[i][j as usize] = 1;
        }

        dist[i][i] = 0;
    }

    // Iterate
    for k in 0..map.len() {
        for i in 0..map.len() {
            for j in i + 1..map.len() {
                // Calculate distance from i to j through k
                if dist[i][k] < u32::MAX && dist[k][j] < u32::MAX {
                    let dist_through_k = dist[i][k] + dist[k][j];
                    if dist[i][j] > dist_through_k {
                        dist[i][j] = dist_through_k;
                        dist[j][i] = dist_through_k;
                    }
                }
            }
        }
    }

    let mut dist_filtered = [[u32::MAX; N]; N];

    // Filter only valves of interest (start or flow rate > 0)
    // The starting valve will  always be the first entry in the sorted map
    let valves_interest: Vec<&u32> = map
        .iter()
        .filter(|(name, valve)| valve.flow_rate > 0 || *name == map.keys().next().unwrap())
        .map(|(name, _)| name)
        .collect();

    for i in 0..valves_interest.len() {
        for j in 0..valves_interest.len() {
            dist_filtered[i][j] = dist[*valves_interest[i] as usize][*valves_interest[j] as usize];
        }
    }

    dist_filtered
}

fn generate_map(input: &str) -> BTreeMap<u32, Valve> {
    // Encode vertices as integers
    let mut valves: BTreeMap<String, u32> = BTreeMap::new();
    let mut i = 1;

    let mut map = BTreeMap::new();

    for line in input.lines() {
        let mut iter = line.bytes().skip(6);
        let mut name = String::with_capacity(2);
        name.push(iter.next().unwrap() as char);
        name.push(iter.next().unwrap() as char);

        let vertex = get_or_insert_valve(&mut valves, name, &mut i);

        iter.nth(14);
        let flow_rate = parse_u32_from_iter(&mut iter);

        iter.nth(21);
        let mut tunnels = Vec::new();
        if iter.next().unwrap() != b' ' {
            iter.next();
        }
        while let Some(char) = iter.next() {
            let mut tunnel = String::with_capacity(2);
            tunnel.push(char as char);
            tunnel.push(iter.next().unwrap() as char);
            let edge = get_or_insert_valve(&mut valves, tunnel, &mut i);
            tunnels.push(edge);
            iter.nth(1);
        }

        map.insert(vertex, Valve { flow_rate, tunnels });
    }

    map
}

fn get_or_insert_valve(valves: &mut BTreeMap<String, u32>, name: String, i: &mut u32) -> u32 {
    *valves.entry(name.clone()).or_insert_with(|| {
        if name == "AA" {
            0
        } else {
            // Reinventing post-increment
            let pre = *i;
            *i += 1;
            pre
        }
    })
}

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<u32>,
}

fn parse_u32_from_iter(ascii_iter: &mut impl Iterator<Item = u8>) -> u32 {
    let mut num = 0;

    while let Some(byte) = ascii_iter.next() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += (byte - b'0') as u32;
    }

    num
}
