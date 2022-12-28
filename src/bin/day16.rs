use lazy_regex::{regex, Lazy, Regex};
use std::{
    collections::{HashMap, HashSet},
    io::Write,
    str::FromStr,
};

const SAMPLE_INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[derive(Debug)]
pub struct ParseInputError;

#[derive(Clone, Debug)]
pub struct Valve {
    pub id: String,
    pub flow_rate: u32,
    pub tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PART1_REGEX: &Lazy<Regex> = regex!(r"Valve ([A-Z]+).+rate=(\d+)");
        static PART2_REGEX: &Lazy<Regex> = regex!(r"([A-Z]+)");

        let parts = s.split(';').collect::<Vec<_>>();
        let valve_info = PART1_REGEX.captures(parts[0]).ok_or(ParseInputError)?;
        let id = valve_info
            .get(1)
            .ok_or(ParseInputError)?
            .as_str()
            .to_string();
        let flow_rate = valve_info
            .get(2)
            .ok_or(ParseInputError)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| ParseInputError)?;

        let mut tunnels = vec![];
        let matches = PART2_REGEX.captures_iter(parts[1]);
        for m in matches {
            tunnels.push(m.get(0).ok_or(ParseInputError)?.as_str().to_string());
        }

        Ok(Self {
            id,
            flow_rate,
            tunnels,
        })
    }
}

// Calculates the best path from the last item in the path provided as parameter to the
// destination.
fn calculate_path<'a>(
    valves: &'a HashMap<String, Valve>,
    destination: &str,
    path: &[&'a str],
) -> Vec<&'a str> {
    let Some(last) = path.last() else {
        panic!("Can't start with an empty path :)");
    };

    let last = *last;
    if last == destination {
        return path.to_vec();
    }

    let valve = valves.get(last).expect("Unknown Valve");

    let mut best_path = vec![];
    for tunnel in &valve.tunnels {
        if !path.contains(&tunnel.as_str()) {
            let mut next_path = path.to_vec();
            next_path.push(tunnel.as_str());
            let new_path = calculate_path(valves, destination, &next_path);
            if best_path.is_empty() || (!new_path.is_empty() && new_path.len() < best_path.len()) {
                best_path = new_path;
            }
        }
    }

    best_path
}

// Implements Part 1.
fn find_best_path<'a>(
    valves: &'a HashMap<String, Valve>,
    closed_valves: &HashSet<&'a str>,
    current_valve: &'a str,
    paths: &HashMap<&str, HashMap<&str, Vec<&str>>>,
    total_pressure: u32,
    minutes_left: u32,
) -> u32 {
    let total_pressure =
        total_pressure + minutes_left * valves.get(current_valve).unwrap().flow_rate;
    let minutes_left = minutes_left - 1;
    let closed_valves = closed_valves
        .iter()
        .filter(|v| **v != current_valve)
        .map(|v| *v)
        .collect::<HashSet<_>>();

    if minutes_left == 0 || closed_valves.is_empty() {
        return total_pressure;
    }

    let from_current_valve = paths.get(current_valve).unwrap();
    let mut highest_pressure = total_pressure;
    for valve in &closed_valves {
        let path = from_current_valve.get(valve).unwrap();
        let minutes_left = minutes_left as i32 - (path.len() as i32 - 1);
        if minutes_left < 1 {
            // Not enough time left to open the valve. Skip it.
            continue;
        };
        let pressure = find_best_path(
            valves,
            &closed_valves,
            valve,
            paths,
            total_pressure,
            minutes_left as u32,
        );

        if pressure > highest_pressure {
            highest_pressure = pressure;
        }
    }

    highest_pressure
}

// Implements Part 2. Information for me is prefixed with "m". Information for the elephant is
// prefixed with "e".
fn find_best_path_with_elephant<'a>(
    valves: &'a HashMap<String, Valve>,
    mut closed_valves: HashSet<&'a str>,
    m_current_valve: &'a str,
    e_current_valve: &'a str,
    paths: &HashMap<&str, HashMap<&str, Vec<&str>>>,
    mut m_total_pressure: u32,
    mut e_total_pressure: u32,
    mut m_minutes_left: u32,
    mut e_minutes_left: u32,
    depth: u32,
) -> u32 {
    // Open valves if there's enough time left.
    if m_minutes_left > 0 && closed_valves.contains(&m_current_valve) {
        m_minutes_left = m_minutes_left - 1;
        m_total_pressure =
            m_total_pressure + m_minutes_left * valves.get(m_current_valve).unwrap().flow_rate;
        closed_valves.remove(&m_current_valve);
    }

    if e_minutes_left > 0 && closed_valves.contains(&e_current_valve) {
        e_minutes_left = e_minutes_left - 1;
        e_total_pressure =
            e_total_pressure + e_minutes_left * valves.get(e_current_valve).unwrap().flow_rate;
        closed_valves.remove(&e_current_valve);
    }

    // Total pressure released so far.
    let mut overall_total_pressure = m_total_pressure + e_total_pressure;

    // At least 2 minutes are needed to continue - at least 1 to move to the next closed valve and
    // another to open it. If we don't have this time for myself and the elephant, the search is over.
    // It's also over if there are no more closed valves.
    if (m_minutes_left < 2 && e_minutes_left < 2) || closed_valves.is_empty() {
        return overall_total_pressure;
    }

    let from_m_current_valve = paths.get(m_current_valve).unwrap();
    let from_e_current_valve = paths.get(e_current_valve).unwrap();

    let total = closed_valves.len() * closed_valves.len();
    let mut progress = 0;
    for m_valve in &closed_valves {
        let m_path = from_m_current_valve.get(m_valve).unwrap();
        let m_minutes_left = (m_minutes_left as i32 - (m_path.len() as i32 - 1)).max(0) as u32;

        for e_valve in &closed_valves {
            progress += 1;
            if depth == 0 {
                print!(
                    "\r{} of {} => {}% - {}        ",
                    progress,
                    total,
                    progress * 100 / total,
                    overall_total_pressure
                );
                std::io::stdout().flush().unwrap();
            }

            // Technically, this should be fine, but we can avoid one iteration.
            if m_valve == e_valve {
                continue;
            }

            let e_path = from_e_current_valve.get(e_valve).unwrap();
            let e_minutes_left = (e_minutes_left as i32 - (e_path.len() as i32 - 1)).max(0) as u32;

            let pressure = find_best_path_with_elephant(
                valves,
                closed_valves.clone(),
                m_valve,
                e_valve,
                paths,
                m_total_pressure,
                e_total_pressure,
                m_minutes_left as u32,
                e_minutes_left as u32,
                depth + 1,
            );

            // Store the new best pressure, if this path is better.s
            if pressure > overall_total_pressure {
                overall_total_pressure = pressure;
            }
        }
    }

    overall_total_pressure
}

fn main() {
    // let input = SAMPLE_INPUT;
    let input = include_str!("day16.txt");

    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in input.lines() {
        let valve = line.parse::<Valve>().unwrap();
        valves.insert(valve.id.clone(), valve);
    }

    // Valves that can be opened.
    let openable_valves = valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id.as_str())
        .collect::<HashSet<_>>();

    println!("Found {} openable valves", openable_valves.len());

    // Pre-compute the paths between valves. The only valve that's not openable that we need as an
    // origin is point is "AA", but including all of them. Only using openable valves as destinations.
    let mut paths: HashMap<&str, HashMap<&str, Vec<&str>>> = HashMap::new();
    for a in valves.keys() {
        for b in valves.keys() {
            let valve = valves.get(b).unwrap();
            if valve.flow_rate == 0 {
                continue;
            }

            let best_path = calculate_path(&valves, b, &[a]);
            paths
                .entry(a)
                .or_insert(HashMap::default())
                .insert(b, best_path);
        }
    }

    // Filter the list of valves with the ones that are openable.
    let closed_valves = valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id.as_str())
        .collect::<HashSet<_>>();

    let result = find_best_path(&valves, &closed_valves, "AA", &paths, 0, 30);
    println!("Part1: {:?}", result);

    let result = find_best_path_with_elephant(
        &valves,
        closed_valves.clone(),
        "AA",
        "AA",
        &paths,
        0,
        0,
        26,
        26,
        0,
    );

    println!("\nPart 2: {}", result); // 2705 is the correct output.
}
