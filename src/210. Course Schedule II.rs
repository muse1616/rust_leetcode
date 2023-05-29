
struct Graph {
    edges: Vec<Vec<i32>>,
    visited: Vec<i32>,
    topology: Vec<i32>,
    has_cycle: bool,
}

impl Solution {
    fn dfs(u: i32, g: &mut Graph) {
        g.visited[u as usize] = 1;
        let edges = g.edges[u as usize].clone();
        for v in edges.iter() {
            if g.visited[*v as usize] == 0 {
                Self::dfs(*v, g);
                if g.has_cycle {
                    return;
                }
            } else if g.visited[*v as usize] == 1 {
                g.has_cycle = true;
                return;
            }
        }
        g.visited[u as usize] = 2;
        g.topology.push(u);
    }
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g: Graph = Graph {
            edges: vec![],
            visited: vec![],
            topology: vec![],
            has_cycle: false,
        };
        g.edges.resize(num_courses as usize, vec![]);
        g.visited.resize(num_courses as usize, 0);
        for prerequisite in prerequisites {
            g.edges[prerequisite[1] as usize].push(prerequisite[0]);
        }
        for i in 0..num_courses {
            if g.has_cycle {
                return vec![];
            }
            if g.visited[i as usize] == 0 {
                Self::dfs(i, &mut g);
            }
        }
        if g.has_cycle {
            return vec![];
        }
        g.topology.reverse();
        g.topology.clone()
    }
}
