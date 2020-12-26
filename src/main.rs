use ruzx::graph::{Graph,VType,EType};

fn main() {
    let mut g = Graph::new();
    g.add_vertex(VType::Z);
    g.add_vertex(VType::X);
    g.add_vertex(VType::X);
    g.add_edge(0,1,EType::N);
    g.add_edge(1,2,EType::H);
    println!("Graph has {} vertices and {} edges.",
             g.num_vertices(), g.num_edges());
    println!("{:?}", g);
}
