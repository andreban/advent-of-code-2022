use std::collections::{HashSet, HashMap};

use aoc22::Point;
use lazy_regex::{regex, Lazy, Regex};

const SAMPLE_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[derive(Debug)]
pub struct Sensor {
    pub point: Point,
    pub range: u64,
}

impl Sensor {
    pub fn contains(&self, other: &Point) -> bool {
        self.point.manhattan_distance(other) <= self.range
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.end && self.start >= other.start)
            || (other.start <= self.end && other.start >= self.start)
    }

    pub fn adjacent(&self, other: &Range) -> bool {
        other.start - self.end == 1 || self.start - other.end == 1
    }

    pub fn merge(&self, other: &Range) -> Option<Range> {
        // Ranges are equal, return self.
        if self == other {
            return Some(*self);
        }

        // self contains other, return self.
        if self.contains(other) {
            return Some(*self);
        }

        // other contains self, return other.
        if other.contains(self) {
            return Some(*other);
        }

        // self and other overlap.
        if self.overlaps(other) || other.overlaps(self){
            let start = i64::min(self.start, other.start);
            let end = i64::max(self.end, other.end);
            return Some(Range::new(start, end));
        }

        // other is adjacent to self.
        if self.adjacent(other) {
            let start = i64::min(self.start, other.start);
            let end = i64::max(self.end, other.end);
            return Some(Range::new(start, end));      
        }

        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ranges {
    pub ranges: Vec<Range>
}

impl Ranges {
    pub fn merge_ranges(&mut self) {
        // Nothing to do here.
        if self.ranges.len() == 1 {
            return;
        }

        loop {
            let mut changed = false;
            self.ranges.sort();
            let mut pos = 0;
            let mut result = vec![];
            while pos < self.ranges.len() {
                if pos == self.ranges.len() - 1 {
                    result.push(self.ranges[pos]);
                    break;
                }

                match self.ranges[pos].merge(&self.ranges[pos + 1]) {
                    Some(r) => {
                        result.push(r);
                        changed = true;
                        pos += 2;
                    },
                    None => {
                        result.push(self.ranges[pos]);
                        pos += 1;
                    },
                }
            }
            self.ranges = result;
            if !changed {
                break;
            }
        }       
    }

    pub fn extend(&mut self, new: Range) {
        self.ranges.push(new);
        self.merge_ranges();
    }
}

fn compute_valid_points(sensor_beacon_map: &HashMap<Point, Point>, beacons: &HashSet<Point>, target_y: i64) -> HashSet<Point> {
    let mut valid_points = HashSet::new();
    for (sensor, beacon) in sensor_beacon_map {
        let beacon_distance = sensor.manhattan_distance(beacon);
        let middle = Point::new(sensor.x, target_y);
        let middle_distance = sensor.manhattan_distance(&middle);

        // Too far away...
        if sensor.manhattan_distance(&middle) > beacon_distance {
            continue;
        }

        let diff = middle_distance.abs_diff(beacon_distance) as i64;
        for x in (middle.x - diff)..=(middle.x + diff) {
            let point = Point::new(x, target_y);
            if !beacons.contains(&point) {
                valid_points.insert(point);
            }
        }
    }
    valid_points
}

fn main() {
    // let (input, target_y, part_2_limit) = (SAMPLE_INPUT, 10, 20);
    let (input, target_y, part_2_limit) = (include_str!("day15.txt"), 2000000, 4000000);
    static REGEX: &Lazy<Regex> = regex!(r"x=(-?\d+), y=(-?\d+).+? x=(-?\d+), y=(-?\d+)");
    let mut sensor_beacon_map = HashMap::new();
    let mut sensors = vec![];
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let result = REGEX.captures(line).unwrap();
        let (sensor_x, sensor_y) = (
            result.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            result.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );

        let (beacon_x, beacon_y) = (
            result.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            result.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        );

        let sensor = Point::new(sensor_x, sensor_y);
        let beacon = Point::new(beacon_x, beacon_y);
        beacons.insert(beacon.clone());
        sensors.push(Sensor {
            point: sensor,
            range: sensor.manhattan_distance(&beacon),
        });
        sensor_beacon_map.insert(sensor, beacon);
    }

    let valid_points = compute_valid_points(&sensor_beacon_map, &beacons, target_y);
    println!("Part 1: {}", valid_points.len());

    let mut all_ranges = vec![Ranges::default(); part_2_limit + 1];
    for sensor in &sensors {
        let start_y = i64::max(0, sensor.point.y - sensor.range as i64);
        let end_y = i64::min(sensor.point.y + sensor.range as i64, part_2_limit as i64);
        // println!("{:?} => start: {}, end: {}", sensor, start_y, end_y);
        for y in start_y..=end_y {
            // Difference between current y and sensor y.
            let y_distance = y.abs_diff(sensor.point.y);
            // How much of the sensor range was spent on the Y axis.
            let diff = sensor.range.abs_diff(y_distance) as i64;

            let start_x = i64::max(0, sensor.point.x - diff);
            let end_x = i64::min(part_2_limit as i64, sensor.point.x + diff);
            let range = Range::new(start_x, end_x);
            all_ranges[y as usize].extend(range);
        }
    }

    let (y, r) = all_ranges
        .iter()
        .enumerate()
        .find(|(_, r)| r.ranges.len() > 1)
        .unwrap();
    println!("range: {:?}, y: {}", r, y);
    let freq = (r.ranges[0].end + 1) * 4000000 + y as i64;
    println!("Part 2: {}", freq);
    

    // println!("{}", sensors.len());
    // for y in 0..=part_2_limit {
    //     if y % 100 == 0 {
    //        println!("Y: {}", y);
    //     }

    //     for x in 0..=part_2_limit {
    //         let point = Point::new(x, y);

    //         let mut in_range = false;
    //         for sensor in &sensors {    
    //             if sensor.contains(&point) {
    //                 in_range = true;
    //                 // println!("{:?} contains {:?}", sensor, point);
    //                 break;
    //             }
    //         }
    //         if !in_range {
    //             println!("{:?} => {}", point, point.x * 4000000 + y);
    //             break;
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_one_range() {
        let mut ranges = Ranges::default();
        ranges.extend(Range::new(-1, 10));
        assert_eq!(ranges.ranges.len(), 1); 
    }

    #[test]
    fn adds_overlapping_ranges_start() {
        let mut ranges = Ranges::default();
        ranges.extend(Range::new(-1, 10));
        ranges.extend(Range::new(5, 15));
        println!("{:?}", ranges);
        assert_eq!(ranges.ranges.len(), 1);
        assert_eq!(ranges.ranges[0].start, -1);
        assert_eq!(ranges.ranges[0].end, 15);
    }

    #[test]
    fn adds_overlapping_ranges_end() {
        let mut ranges = Ranges::default();
        ranges.extend(Range::new(5, 15));
        ranges.extend(Range::new(-1, 10));
        println!("{:?}", ranges);
        assert_eq!(ranges.ranges.len(), 1);
        assert_eq!(ranges.ranges[0].start, -1);
        assert_eq!(ranges.ranges[0].end, 15);
    }

    #[test]
    fn adds_containing_ranges_end() {
        let mut ranges = Ranges::default();
        ranges.extend(Range::new(-1, 15));
        ranges.extend(Range::new(5, 10));
        println!("{:?}", ranges);
        assert_eq!(ranges.ranges.len(), 1);
        assert_eq!(ranges.ranges[0].start, -1);
        assert_eq!(ranges.ranges[0].end, 15);
    }

    #[test]
    fn adds_containing_ranges_start() {
        let mut ranges = Ranges::default();
        ranges.extend(Range::new(5, 10));
        ranges.extend(Range::new(-1, 15));
        println!("{:?}", ranges);
        assert_eq!(ranges.ranges.len(), 1);
        assert_eq!(ranges.ranges[0].start, -1);
        assert_eq!(ranges.ranges[0].end, 15);
    }

    #[test]
    fn adds_case_1() {
        let mut ranges = Ranges::default();
        ranges.extend(Range::new(12, 14));
        ranges.extend(Range::new(6, 10));
        ranges.extend(Range::new(0, 12));
        ranges.extend(Range::new(14, 20));
        println!("{:?}", ranges);
        assert_eq!(ranges.ranges.len(), 1);
        assert_eq!(ranges.ranges[0].start, 0);
        assert_eq!(ranges.ranges[0].end, 20);
    }
}