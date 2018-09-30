use std::error::Error;
use policy::{Policy,print_graph};
use std::cmp;
use std::fmt;
use std::default::Default;

pub type Graph = Vec<Vec<usize>>; 
pub type DeltaState = Vec<Vec<usize>>;

#[derive(Default,Debug)]
pub struct NodeStat{
    pub node : usize,
    pub added_nodes : usize, 
    pub missing_nodes : usize, 
}    
#[derive(Default,Debug)]
pub struct GraphStat{
    pub initial_graph : Graph,
    pub final_graph : Graph, 
    pub node_stats : Vec<(NodeStat,DeltaState)>,
    pub is_satisfied_graph : bool, 
    pub is_violating_graph : bool,
    pub changed_nodes : usize, 
    pub unchanged_nodes : usize, 
    pub satisfied_nodes_num : usize, 
    pub dns_nodes_num : usize,
    pub not_satisfied_nodes : usize,
    pub config_optimal_out : usize , 
    pub config_max_in : usize, 
}

impl fmt::Display for GraphStat {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"\n --- START Optimization Stats --- \n\n");
        
        write!(f,"\t1) Config: Optimal = {} , Max inbound = {} \n", self.config_optimal_out, self.config_max_in);
        write!(f,"\t2) Satisfied Graph : {} \n" , self.is_satisfied_graph);
        write!(f,"\t3) Violated graph : {} \n", self.is_violating_graph);
        write!(f, "\t4) Total Nodes (with dns) : {} \n", self.final_graph.len());
        write!(f,"\t5) Satisfied Nodes (exclude dns): {}/{} \n", self.satisfied_nodes_num , self.final_graph.len()- self.dns_nodes_num);
        write!(f,"\t6) Dns Nodes : {} \n" , self.dns_nodes_num);
        write!(f,"\t7) Not Satisfied Nodes : {} \n" , self.not_satisfied_nodes);

        write!(f,"\n --- END Optimization Stats --- \n\n")
    }
}
pub struct Optimizer<'a,'b>{
    policy : Policy<'b>,
    initial_graph :&'a Graph
}
impl<'a,'b> Optimizer<'a,'b>{
    pub fn new(graph : &'a mut Graph, policy : Policy<'b>)->Optimizer<'a,'b>{
        
        Optimizer {
            policy : policy,
            initial_graph : graph,
        }
    }
    fn add_edge(&self, from : usize, to : usize , graph : &mut Graph)->bool{
        graph[from].push(to);
        true
    }
    pub fn try_satisfy_graph(&self, graph : &Graph)->Result<GraphStat,Box<Error>>{
        let mut initial_graph = graph.clone();
        
        let mut node_stats : Vec<(NodeStat, DeltaState)> = Vec::new();

        let mut changed_nodes = 0;
        let mut satisfied_nodes = 0;
        // try optimize each node         
        for id in 0..initial_graph.len(){
            match self.try_satisfy_node(id, &mut initial_graph){
                
                Ok(result)=>{

                    let new_graph = result.0;
                    let stat = result.1;
                    if stat.added_nodes > 0 {
                        changed_nodes += 1;
                    }
                    if self.policy.is_satisfied_node(id, &initial_graph){
                        if !self.policy.is_dns(id){
                            satisfied_nodes += 1;
                        }
                    }
                    node_stats.push((stat, new_graph));
                },
                
                Err(e)=>{
                    println!("[-] {} not satisfied: {} ",id, e );
                }
            };
        }
        
        // make graph level stats 
        let mut graph_stats = GraphStat::default();
        graph_stats.initial_graph = graph.clone();
        graph_stats.final_graph = initial_graph.clone(); 
        graph_stats.node_stats = node_stats;
        graph_stats.is_satisfied_graph = self.policy.is_satisfied_graph(&initial_graph);
        graph_stats.is_violating_graph = self.policy.is_violating_graph(&initial_graph);
        graph_stats.changed_nodes = changed_nodes;
        graph_stats.unchanged_nodes = initial_graph.len() - changed_nodes; 
        graph_stats.satisfied_nodes_num = satisfied_nodes; 
        graph_stats.dns_nodes_num  = self.policy.dns_nodes_size();
        graph_stats.not_satisfied_nodes = initial_graph.len() - (self.policy.dns_nodes_size() + satisfied_nodes);
        graph_stats.config_max_in = self.policy.get_inbound_max();
        graph_stats.config_optimal_out = self.policy.get_outbound_optimal();
        Ok(graph_stats)
    }
    pub fn try_satisfy_node(&self, node : usize,  graph: &mut Graph)->Result<(Graph,NodeStat), Box<Error>>{

        let mut test_graph = graph.clone();
        
        // verify that the graph is not violating the policy and the node not satisfied already or if its dns, or violating node
        // or if a node outbound >= optimal  
        if self.policy.is_violating_graph(graph){

            return Err(From::from("violating graph"));

        }else if self.policy.is_satisfied_node(node, graph) || self.policy.is_dns(node){
                
                let stats = NodeStat{
                    node : node, 
                    added_nodes : 0, 
                    missing_nodes : 0,
                };
            return Ok((test_graph,stats));

        }else if self.policy.is_vaiolating_node(node, graph){
            return Err(From::from("violating node"));
        }

        // try satisfy the node 
        
        let delta_out = self.policy.get_outbound_optimal()- Policy::get_outbound_edges(node, graph).len();

        match self.policy.get_all_connectables(node, graph){
            Some(connectables)=>{
                
                let try_num = cmp::min(delta_out, connectables.len());
                // iterate all try_out # and connect to the nodes
                let mut counter = 0;
                for idx in 0..try_num{
                    let n = connectables[idx]; 
                    // make edge 
                    self.add_edge(node, n, graph);
                    counter += 1;
                }

                let stats = NodeStat{
                    node : node, 
                    added_nodes : counter, 
                    missing_nodes : delta_out - counter,
                };

                return Ok((test_graph,stats));
            },
            None =>{
                return Err(From::from("no connectables"));
            }
        }
    }
}

