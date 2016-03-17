use std::collections::HashMap;
use std::convert::From;
use permutohedron::LexicalPermutation;

pub fn solve(input: &str) -> (usize, usize) {
    let routes: Vec<_> = input.lines().map(Route::from).collect();
    let mut map = Map::new();
    for r in &routes {
        map.add_distance(r.from, r.to,   r.dist);
        map.add_distance(r.to,   r.from, r.dist);
    }
    let mut cities = map.cities();
    let mut trips = vec![];
    while cities.next_permutation() {
        if let Some(dist) = map.trip_distance(&cities) {
            trips.push(dist);
        }
    }
    trips.sort();
    (trips[0], trips[trips.len() - 1])
}

type Distances<'a> = HashMap<&'a str, usize>;
// type Map<'a> = HashMap<&'a str, Distances<'a>>;

struct Map<'a> {
    data: HashMap<&'a str, Distances<'a>>
}

impl<'a> Map<'a> {
    fn new() -> Map<'a> {
        Map { data: HashMap::<&str, Distances>::new() }
    }

    fn cities(&self) -> Vec<&str> {
        self.data.keys().map(|val| *val).collect()
    }

    fn add_distance(&mut self, from: &'a str, to: &'a str, dist: usize) {
        let mut entry = self.data.entry(from).or_insert(Distances::new());
        entry.insert(to, dist);
    }

    fn trip_distance(&self, trip: &Vec<&str>) -> Option<usize> {
        let mut iter = trip.iter().peekable();
        let mut dist = 0;
        while let Some(city) = iter.next() {
            if let Some(next_city) = iter.peek() {
                if let Some(val) = self.distance(city, next_city) {
                    dist += val;
                } else {
                    return None;
                }
            }
        }
        return Some(dist);
    }

    fn distance(&self, from: &str, to: &str) -> Option<usize> {
        self.data.get(from).and_then(|data| data.get(to).map(|val| *val))
    }
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
