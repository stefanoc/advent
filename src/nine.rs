use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut routes  = HashMap::<_, _>::new();
    for (route1, route2) in input.lines().map(|line| parse(line)) {
        {
            let mut e = routes.entry(route1.from).or_insert_with(|| vec![]);
            e.push(route1);
        }
        {
            let mut e = routes.entry(route2.from).or_insert_with(|| vec![]);
            e.push(route2);
        }
    }
    for start in routes.keys() {
        let mut nav = Navigator::new(start, &routes);
        let trips = nav.trips();
        println!("Starting from: {}", start);
        println!("-> {:?}", trips);
    }
    0
}

struct Navigator<'a> {
    start:      &'a str,
    visited:    Vec<&'a str>,
    routes:     &'a HashMap<&'a str, Vec<Route<'a>>>
}

impl<'a> Navigator<'a> {
    fn new(start: &'a str, routes: &'a HashMap<&'a str, Vec<Route<'a>>>) -> Navigator<'a> {
        Navigator { start: start, visited: vec![], routes: &routes }
    }

    fn trips(&'a mut self) -> Vec<Vec<&'a Route>> {
        let mut trips = vec![];
        
    }
}

#[derive(Debug,Clone)]
struct Route<'a> {
    from: &'a str,
    to: &'a str,
    dist: usize
}

impl<'a> Route<'a> {
    fn inverse(&'a self) -> Route<'a> {
        Route { from: self.to, to: self.from, dist: self.dist }
    }
}

fn parse<'a>(line: &'a str) -> (Route<'a>, Route<'a>) {
    let mut p1  = line.split(" = ");
    let mut p2  = p1.next().map(|p| p.split(" to ")).unwrap_or_else(|| panic!("Bad route: {} (1)", line));
    let from    = p2.next().unwrap_or_else(|| panic!("Bad route: {} (2)", line));
    let to      = p2.next().unwrap_or_else(|| panic!("Bad route: {} (3)", line));
    let dist    = p1.next().map(|p| p.parse::<usize>()).unwrap_or_else(|| panic!("Bad route: {} (4)", line)).unwrap_or_else(|_| panic!("Bad route: {} (5)", line));
    (
        Route { from: from, to: to, dist: dist },
        Route { from: to, to: from, dist: dist }
    )
}
