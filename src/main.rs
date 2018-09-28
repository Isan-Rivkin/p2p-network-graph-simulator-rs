struct Policy<'a> {
    outbound_optimal : usize, 
    inbound_max : usize,
    dns_nodes : &'a Vec<usize>,
}

impl<'a> Policy<'a>{
    pub fn new( outbound_optimal:usize,inbound_max : usize,dns_nodes : &'a Vec<usize>)->Policy{
        Policy {
            outbound_optimal : outbound_optimal,
            inbound_max : inbound_max, 
            dns_nodes :dns_nodes,
        }
    }
    pub fn is_dns(&self , node : usize)->bool{
        self.dns_nodes.contains(&node)
    }
    pub fn is_violating_graph(&self,graph :& Vec< Vec<usize>>)->bool{
        
        let mut violate = false; 

        for vertex in graph{
            violate = vertex.iter().any(|&v| self.is_vaiolating_node(v, graph));
            if violate{
                break;
            }
        }
        
        violate
    }
    pub fn is_vaiolating_node(&self, node : usize, graph :& Vec< Vec<usize>>)->bool{
        self.is_violating_optimal_outbound_node(node, graph) ||
            self.is_violating_max_inbound_node(node, graph)
    }
    pub fn is_violating_optimal_outbound_node(&self, node : usize, graph :& Vec< Vec<usize>>)->bool{
        let outbound_size = graph[node].len();
        outbound_size > self.outbound_optimal
    }
    pub fn is_violating_max_inbound_node(&self, node : usize, graph :& Vec< Vec<usize>>)->bool{
        let inbound_size = self.get_inbound_edges(node, graph).len();
        inbound_size > self.inbound_max
    }

    pub fn get_inbound_edges(&self, node : usize, graph :& Vec< Vec<usize>>)->Vec<usize>{
        
        let mut inbound : Vec<usize> = Vec::new();

        for (idx, vertex) in graph.into_iter().enumerate(){
            if idx != node {
                for v in vertex {
                    if *v == node {
                        inbound.push(idx);
                    }
                }        
            }
        }
        inbound
    }

    pub fn get_outbound_edges(&self, node : usize, graph :& Vec< Vec<usize>>)->Vec<usize>{
        graph[node].clone()
    }

    pub fn is_satisfied_node(&self, node : usize, graph :& Vec< Vec<usize>>)->bool{
        let is_violating = self.is_vaiolating_node(node, graph);
        let outbound = self.get_outbound_edges(node, graph).len();
        !is_violating && outbound == self.outbound_optimal
    }   
}
//fn get_nodes(graph : &mut Vec< Vec<usize>>){

fn print_graph(graph : &Vec< Vec<usize>>){
    ///    graph[0].push(2);// = 22;
    println!("graph = {:?}",graph );
}

fn get_nodes(graph :& Vec< Vec<usize>>)->Vec<usize>{
     let size = graph.len();
     let v: Vec<usize> = (0..size).collect();
     v
}

fn get_edges(node : usize , graph :& Vec< Vec<usize>>)->Vec<usize>{
    let edges_of_node :Vec<usize> = graph[node].clone();
    edges_of_node
}



fn main() {
    let mut graph : Vec<Vec<usize>> = Vec::new();
    let mut dns : Vec<usize> = vec![1,3];
    // node 0,1,2,3
    graph.push(Vec::new());
    graph.push(Vec::new());
    graph.push(Vec::new());
    graph.push(Vec::new());
    // node 0 edges 1,2,3
    graph[0].push(1);
    graph[0].push(2);
    graph[0].push(3);
    // node 1 edges 2,3
    graph[1].push(2);
    graph[1].push(3);
    // node 2 edges 1,0
    graph[2].push(1);
    graph[2].push(0);
    // node 3 edges 0 
    graph[3].push(0);

    print_graph(&graph);
    println!("nodes = {:?}",get_nodes(&graph));
    println!("edges of 2 = {:?}",get_edges(2,&graph));
    println!("---------------------" );
    // 2 is a satisfied not violating node!! 
    let optimal = 2; 
    let max_in = 2;
    let policy = Policy::new(optimal,max_in, &dns);
    let test_node = 2;
    println!("is_dns ? {}",policy.is_dns(test_node) );
    println!("inbound edges = {:?}",policy.get_inbound_edges(test_node, &graph));
    println!("outbound edges = {:?}" , policy.get_outbound_edges(test_node, &graph) );
    println!("is violating max_inbound ? {}", policy.is_violating_max_inbound_node(test_node, &graph));
    println!("is violating optimal_outbound ? {}", policy.is_violating_optimal_outbound_node(test_node, &graph));
    println!("is violating node ? {}",policy.is_vaiolating_node(test_node,&graph));    
    println!("is violating graph ? {}", policy.is_violating_graph(&graph));
    println!("is satisfied node ? {}",policy.is_satisfied_node(test_node, &graph) );
}
