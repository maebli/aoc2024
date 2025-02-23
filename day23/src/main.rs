use graphrs::{algorithms::cluster, Edge, Graph, GraphSpecs};

fn main() {
    let input = include_str!("input.txt");

    let mut graph_with_t: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    let mut graph_without_t: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());

    input.lines().for_each(|line| {
        let mut parts = line.split("-");
        let n1 = parts.next().unwrap();
        let n2 = parts.next().unwrap();

        let edge = Edge::new(n1, n2);
        graph_with_t.add_edge(edge.clone()).unwrap();

        if !n1.starts_with("t") && !n2.starts_with("t") {
            graph_without_t.add_edge(edge).unwrap();
        }
    });

    let triangles_with_t = cluster::triangles(&graph_with_t, None);
    let triangles_without_t = cluster::triangles(&graph_without_t, None);

    if let (Ok(with_t), Ok(without_t)) = (triangles_with_t, triangles_without_t) {
        println!(
            "result part1: {:?}",
            with_t.values().sum::<usize>() / 3 - without_t.values().sum::<usize>() / 3
        );
    }

    let g = graph_with_t;

    let c = cluster::generalized_degree(&g, None);

    let mut pass = vec![];
    if let Ok(clustering) = c {
        // find the maximum key of any cluster hashmap
        let max = clustering
            .iter()
            .map(|(_node, cluster_map)| {
                cluster_map
                    .iter()
                    .filter(|(key, value)| **key as i32 - 1 == **value as i32)
                    .map(|(key, _value)| key)
                    .max()
                    .unwrap_or(&0)
            })
            .max()
            .unwrap();

        println!("max: {:?}", max);

        for (node, cluster_map) in clustering.iter() {
            for (key, value) in cluster_map.iter() {
                // key and value are values for clustring hashmap where key is max and value is 1+max
                if key == max && value == &(max + 1) {
                    pass.push(node.clone());
                    break;
                }
            }
        }
    }

    pass.sort();
    println!("result part2: {:?}", pass.join(","));
}
