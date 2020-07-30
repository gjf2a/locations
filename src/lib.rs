use immutable_map::TreeMap;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct LocationGraph<L: Copy+Clone+Ord+PartialOrd+Eq+PartialEq, D:Copy+Clone+Ord+PartialOrd+Eq+PartialEq> {
    distances: TreeMap<L,TreeMap<L,D>>
}

impl <L:Copy+Clone+Ord+PartialOrd+Eq+PartialEq, D:Copy+Clone+Ord+PartialOrd+Eq+PartialEq>
    LocationGraph<L,D> {
    pub fn new(distances: Vec<(L,L,D)>) -> Self {
        let mut map_graph = LocationGraph {distances: TreeMap::new()};
        for distance in distances.iter() {
            map_graph.add(distance.0, distance.1, distance.2);
        }
        map_graph
    }

    pub fn get(&self, start: L, end: L) -> Option<D> {
        self.distances.get(&start)
            .and_then(|map| map.get(&end))
            .map(|d| *d)
    }

    pub fn add(&mut self, m1: L, m2: L, distance: D) {
        self.add_one_way(m1, m2, distance);
        self.add_one_way(m2, m1, distance);
    }

    fn add_one_way(&mut self, start: L, end: L, distance: D) {
        let updated = self.distances.get(&start)
            .unwrap_or(&TreeMap::new())
            .insert(end, distance);
        self.distances = self.distances.insert(start, updated);
    }
}

#[cfg(test)]
mod tests {
    use crate::LocationGraph;

    #[test]
    fn it_works() {
        let graph = LocationGraph::new(vec![('a','b',5), ('b', 'c', 10)]);
        assert_eq!(graph.get('a', 'b').unwrap(), 5);
        assert_eq!(graph.get('b', 'a').unwrap(), 5);
        assert_eq!(graph.get('b', 'c').unwrap(), 10);
        assert_eq!(graph.get('c', 'b').unwrap(), 10);
        assert_eq!(graph.get('a', 'c'), None);
    }
}
