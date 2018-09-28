pub struct Policy<'a> {
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
    pub fn is_satisfied_graph(&self, graph : &Vec<Vec<usize>>)->bool{
        
        let mut satisfied_graph = true;

        for (idx,vertex) in graph.into_iter().enumerate(){
            
            if !self.is_dns(idx){

                satisfied_graph = vertex.iter().any(|&v| self.is_satisfied_node(v, graph));
                if !satisfied_graph{
                    break;
                }    

            }
        }
        
        satisfied_graph
    }
}
//fn get_nodes(graph : &mut Vec< Vec<usize>>){



fn get_nodes(graph :& Vec< Vec<usize>>)->Vec<usize>{
     let size = graph.len();
     let v: Vec<usize> = (0..size).collect();
     v
}

fn get_edges(node : usize , graph :& Vec< Vec<usize>>)->Vec<usize>{
    let edges_of_node :Vec<usize> = graph[node].clone();
    edges_of_node
}

