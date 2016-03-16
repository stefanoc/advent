use std::collections::HashMap;
use std::convert::From;
use permutohedron::LexicalPermutation;

pub fn solve(input: &str) -> usize {
    let routes: Vec<_> = input.lines().map(Route::from).collect();
    let mut map = Map::new();
    for r in &routes {
        {
            let mut data = map.entry(r.from).or_insert(Distances::new());
            data.insert(r.to, r.dist);
        }
        {
            let mut data = map.entry(r.to).or_insert(Distances::new());
            data.insert(r.from, r.dist);
        }
    }
    let mut cities: Vec<_> = map.keys().map(|val| *val).collect();
    let mut trips = vec![];
    while cities.next_permutation() {
        if let Some(dist) = trip_distance(&map, &cities) {
            trips.push(dist);
        }
    }
    trips.sort();
    trips[0]
}

type Distances<'a> = HashMap<&'a str, usize>;
type Map<'a> = HashMap<&'a str, Distances<'a>>;

fn trip_distance(map: &Map, trip: &Vec<&str>) -> Option<usize> {
    let mut iter = trip.iter().peekable();
    let mut dist = 0;
    while let Some(city) = iter.next() {
        if let Some(next_city) = iter.peek() {
            if let Some(val) = distance(&map, city, next_city) {
                dist += val;
            } else {
                return None;
            }
        }
    }
    return Some(dist);
}

fn distance(map: &Map, from: &str, to: &str) -> Option<usize> {
    map.get(from).and_then(|data| data.get(to).map(|val| *val))
}

#[derive(Debug, Clone)]
struct Route<'a> {
    from: &'a str,
    to: &'a str,
    dist: usize
}

impl<'a> From<&'a str> for Route<'a> {
    fn from(line: &'a str) -> Self {
        let mut p1  = line.split(" = ");
        let mut p2  = p1.next().map(|p| p.split(" to ")).unwrap_or_else(|| panic!("Bad route: {} (1)", line));
        let from    = p2.next().unwrap_or_else(|| panic!("Bad route: {} (2)", line));
        let to      = p2.next().unwrap_or_else(|| panic!("Bad route: {} (3)", line));
        let dist    = p1.next().map(|p| p.parse::<usize>()).unwrap_or_else(|| panic!("Bad route: {} (4)", line)).unwrap_or_else(|_| panic!("Bad route: {} (5)", line));
        Route { from: from, to: to, dist: dist }
    }
}
