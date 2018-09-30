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

    pub fn dns_nodes_size(&self)->usize{
        self.dns_nodes.len()
    }
    fn get_nodes(graph :& Vec< Vec<usize>>)->Vec<usize>{
        let size = graph.len();
        let v: Vec<usize> = (0..size).collect();
        v
    }
    fn is_connected(node1 : usize, node2 : usize ,graph: &Vec<Vec<usize>>)->bool{
        let node1_conns = Policy::get_outbound_edges(node1, graph);
        let node2_conns = Policy::get_outbound_edges(node2, graph);   

        node1_conns.contains(&node2) || node2_conns.contains(&node1)
    }
    // get all the nodes that node can connect to them without violating the policy
    // connectable n is if: 
    // no_connection(node,n) == true  
    // can_accept_in(n) == true 

    pub fn get_all_connectables(&self,node : usize , graph : &Vec<Vec<usize>>)->Option<Vec<usize>>{
        let mut connectable_nodes :Vec<usize>  =  Vec::new();
        
        let all_nodes = Policy::get_nodes(graph);
        
        for v in all_nodes {
            if v != node && !Policy::is_connected(node, v, graph){
                let inbound_size = Policy::get_inbound_edges(v, graph).len();
                if inbound_size < self.inbound_max {
                    connectable_nodes.push(v);
                }
            }
        }
        
        if connectable_nodes.len() > 0{
            Some(connectable_nodes)
        }else{
            None
        }
    }
    pub fn get_outbound_optimal(&self)->usize{
        self.outbound_optimal
    }

    pub fn get_inbound_max(&self)->usize{
        self.inbound_max
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
        // if self.is_dns(node){
        //     return false;
        // }
        let outbound_size = graph[node].len();
        outbound_size > self.outbound_optimal
    }
    pub fn is_violating_max_inbound_node(&self, node : usize, graph :& Vec< Vec<usize>>)->bool{
        let inbound_size = Policy::get_inbound_edges(node, graph).len();
        inbound_size > self.inbound_max
    }
    

    pub fn get_inbound_edges(node : usize, graph :& Vec< Vec<usize>>)->Vec<usize>{
        
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

    pub fn get_outbound_edges( node : usize, graph :& Vec< Vec<usize>>)->Vec<usize>{
        graph[node].clone()
    }

    pub fn is_satisfied_node(&self, node : usize, graph :& Vec< Vec<usize>>)->bool{
        let is_violating = self.is_vaiolating_node(node, graph);
        if self.is_dns(node){
            
            !is_violating

        }else{

            let outbound = Policy::get_outbound_edges(node, graph).len();
            !is_violating && outbound == self.outbound_optimal
        }
    }   
    pub fn is_satisfied_graph(&self, graph : &Vec<Vec<usize>>)->bool{
        
        let mut satisfied_graph = true;
        for (idx,_) in graph.into_iter().enumerate(){
            
            if !self.is_dns(idx){
                satisfied_graph = self.is_satisfied_node(idx, graph);
                // satisfied_graph = vertex.iter().any(|&v| self.is_satisfied_node(v, graph));
                if !satisfied_graph{
                    break;
                }    

            }
        }
        
        satisfied_graph
    }
}


/// id -> v1,v2...
pub fn print_graph(g : & Vec<Vec<usize>>){
    for (id,line) in g.into_iter().enumerate() {
        println!("{} => {:?} ",id, line);
    }
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

