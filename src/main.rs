use std::collections::HashMap;
use std::collections::BTreeMap;
use rand::Rng;

struct Edge {
    src: String,
    dest: String
}

impl Edge {
    fn new(src: &str, dest: &str) -> Self{
        return Self { src: src.into(), dest: dest.into() }
    }
}

struct Graph {
    N: usize,
    adj_list: BTreeMap<String, Vec<String>>,
}

impl Graph {
    fn new(edges: Vec<Edge>) -> Self {
        let mut adj_list: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for edge in edges.iter() {
            if !adj_list.contains_key(&edge.src){
               adj_list.insert(edge.src.clone(), vec![edge.dest.clone()]);
            } else {
                let neighbors: &mut Vec<String> = adj_list.get_mut(&edge.src).unwrap();
                // Don't add duplicates to neighbor list
                if !neighbors.contains(&edge.dest) && (&edge.dest != &edge.src) {
                    neighbors.push(edge.dest.clone());
                }
            }
            // Ensure that vertices that do not have a destination node get added 
            if !adj_list.contains_key(&edge.dest) {
                adj_list.insert(edge.dest.clone(), vec![]);
            }
        } 
        let N: usize = adj_list.len();
        return Self{ N, adj_list }
    }

    fn add_edge(&mut self, src: &str, dest: &str){
        let edge: Edge = Edge::new(src, dest);
        if !self.adj_list.contains_key(&edge.src){
            if !edge.dest.eq(&edge.src){
                self.adj_list.insert(edge.src.clone(), vec![edge.dest.clone()]);
            } else {
                self.adj_list.insert(edge.src.clone(), vec![]);
            }
        } else {
            let neighbors: &mut Vec<String> = self.adj_list.get_mut(&edge.src).unwrap();
            if !neighbors.contains(&edge.dest) && !edge.dest.eq(&edge.src){
                neighbors.push(edge.dest.clone()); 
            }
        }
        // Ensure that vertices that do not have a destination node get added 
        if !self.adj_list.contains_key(&edge.dest) {
            self.adj_list.insert(edge.dest.clone(), vec![]);
        }
        self.N = self.adj_list.len();
    }

    fn print_adj_list(&self){
        println!("Total vertices: {}", self.N);
        for (key, neighbors) in self.adj_list.iter() {
            print!("{}: [", key);
            if neighbors.len() > 0 {
                for neighbor in neighbors.split_last().unwrap().1 {
                    print!("{},", neighbor);
                }
                print!("{}", neighbors.last().unwrap());
            }
            println!("]");
        } 
        println!();
    }

    fn get_path_matrix(&self) -> BTreeMap<String, Vec<(String,bool)>>{
        let mut path_matrix: BTreeMap<String, Vec<(String, bool)>> = BTreeMap::new();
        // Populate matrix with empty values
        for (src, _neighbors) in self.adj_list.iter() {
            path_matrix.insert(src.clone(), vec![]);
            for (dest, _n) in self.adj_list.iter() {
                path_matrix.get_mut(src).unwrap().push((dest.clone(), false));
            }
        }

        for (src, _neighbors) in self.adj_list.iter() {
            for (dest, _n) in self.adj_list.iter(){
                if let Some(v) = path_matrix.get_mut(src){
                    let d: &mut (String, bool) = v
                        .into_iter()
                        .find(|x| x.0 == dest.clone()) 
                        .unwrap();

                    d.1 = self.has_path(src.clone(), dest.clone());
                    // if d.1 { println!("{} has path to {}", src, dest); } 
                    // else { println!("{} has no path to {}", src, dest); };
                }
    
            }
        }
        return path_matrix;
    }

    fn get_adj_matrix(&self) -> BTreeMap<String, Vec<(String, bool)>>{
        let mut adj_matrix: BTreeMap<String, Vec<(String, bool)>> = BTreeMap::new();
        for (src, _neighbors) in self.adj_list.iter(){
            adj_matrix.insert(src.clone(), vec![]);
            // Populate the map with false, and true when the source and destination are the same
            for (dest, _n) in self.adj_list.iter(){
                if dest.eq(src){
                    adj_matrix.get_mut(src)
                        .unwrap()
                        .push((dest.clone(), true)); 
                } else {
                    adj_matrix.get_mut(src)
                        .unwrap()
                        .push((dest.clone(), false)); 
                }
            }
        }

        for (src, neighbors) in self.adj_list.iter(){
            for neighbor in neighbors {
                if let Some(v) = adj_matrix.get_mut(src){
                    let d: &mut (String, bool) = v
                        .into_iter()
                        .find(|x| x.0 == neighbor.clone())
                        .unwrap();
                    d.1 = true;
                }
            }
        }
        return adj_matrix;
    }

    fn has_path(&self, src: String, dest: String) -> bool {
        let mut path_found: bool = false;
        let mut visited: BTreeMap<String, bool> = BTreeMap::new(); 
        if src.eq(&dest) {
            return true;
        }
        for (key, _neighbors) in self.adj_list.iter() {
            visited.insert(key.clone(), false);
        }
        self.DFS(src, dest, &mut path_found, &mut visited);
        return path_found;
    }

    fn DFS(&self, val: String, dest: String, path_found: &mut bool, 
        visited: &mut BTreeMap<String, bool>){
        // Mark as visited
        *visited.get_mut(&val).unwrap() = true;
        if val.eq(&dest){
            *path_found = true;
        }
        // print!("{}->", val);
        let neighbors = self.adj_list.get(&val).unwrap();
        for neighbor in neighbors {
            if !(*visited.get_mut(neighbor).unwrap()) {
                self.DFS(neighbor.clone(), dest.clone(), path_found, visited);
            }
        }
    }

    fn gen_rand_nodes(&mut self, num_nodes: usize, num_alphas: usize){
        let mut N: usize;
        let mut rng = rand::thread_rng();
        if num_alphas > 26 {
            N = 26;
        } else {
            N = num_alphas;
        }

        for _i in 0..num_nodes {
            let src = rng.gen_range('A'..(('A' as u8) + (N as u8)) as char).to_string();
            let dest = rng.gen_range('A'..(('A' as u8) + (N as u8)) as char).to_string();
            self.add_edge(src.as_str(), dest.as_str());
        } 
    }
}

fn print_matrix(path_matrix: BTreeMap<String, Vec<(String,bool)>>){
    let itr = path_matrix.iter();
    print!(" |");
    for (key, _values) in itr.clone() {
        print!("{}|", key);
    }
    println!();
    for (key, values) in itr.clone() {
        print!("{}[", key); 
        for value in values.split_last().unwrap().1 {
            print!("{},", value.1 as u8);
        }
        print!("{}", values.last().unwrap().1 as u8);
        println!("]");
    }
    println!();
}


fn main() {
    let edges: Vec<Edge> = Vec::new();
    let N: usize = 5;
    let mut graph: Graph = Graph::new(edges);
    graph.gen_rand_nodes(25, 15);
    // graph.add_edge("E", "A");
    // graph.add_edge("A", "B");
    // graph.add_edge("B", "C");
    // graph.add_edge("C", "D");
    // graph.add_edge("D", "A");
    graph.print_adj_list();
    let path_matrix = graph.get_path_matrix();
    println!("PATH MATRIX");
    print_matrix(path_matrix);

    let adj_matrix = graph.get_adj_matrix();
    println!("ADJACENCY MATRIX");
    print_matrix(adj_matrix);
}
