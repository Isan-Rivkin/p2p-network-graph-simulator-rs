extern crate csv;

mod input;
mod policy;
mod optimizer;

use std::env;
use optimizer::{Graph, Optimizer};
use policy::{Policy,print_graph};
use input::*;
use std::error::Error;

fn main() {
    generate();
}
fn generate(){
    
    let config = GeneratorConfig{
        opt : 8 ,
        n : 5 ,
        max : 120,
        edges : Vec::new(),
        dns_nodes : 1
    };

    let res = generate_input(config);
    print_graph(&res);
    graph_to_csv("test_delete.csv",&res);

}
/// run the procss of statistics
fn run() {

    let args: Vec<String> = env::args().collect();
    let mut path = "graph_input.csv";

    if args.len()>1 {
        path = &args[1];
    }

    match csv_to_graph(path){
        Ok(result)=>{ 
            let mut dns_nodes = result.0;
            let mut graph = result.1;
            let mut optimal_outbound = result.2;
            let mut max_inbound = result.3;
            print_graph(&graph);
            println!("dns nodes => {:?}", dns_nodes);
            println!("optimal = {} , max_in = {} ",optimal_outbound, max_inbound );
            
            // optimize 
            
            let mut g = graph.clone();
            let g2 = g.clone();
            let optimizer = Optimizer::new(&mut graph,Policy::new(optimal_outbound,max_inbound, &dns_nodes));

            match optimizer.try_satisfy_graph(&g2){
                Ok(result)=>{
                    println!("------------------------------------------" );
                    print_graph(&result.final_graph);
                    println!("------------------------------------------" );
                    println!("{}",result );
                },
                Err(e)=>{
                    println!("[-] Err optimizing graph! {} ", e);
                }
            }
        },
        Err(e)=> println!("Error parsing csv {}",e )
    };
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_graph_input(){
        let config = GeneratorConfig{
            opt : 8 ,
            n : 5 ,
            max : 120,
            edges : Vec::new(),
            dns_nodes : 3
        };
        let res = generate_input(config);
        print_graph(&res);
        assert_eq!(6,res.len() );
        assert_eq!(1,res[0][0] );
        assert_eq!(2,res[1][1] );
        assert_eq!(120 ,res[5][4] );
    }
    #[test]
    fn test_policy_stuff() {

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
        // 
        println!("graph = {:?}",graph );
        println!("---------------------" );
        // 2 is a satisfied not violating node!! 
        let optimal = 2; 
        let max_in = 2;
        let policy = Policy::new(optimal,max_in, &dns);
        let test_node = 2;
        //println!("is_dns ? {}",policy.is_dns(test_node) );
        assert_eq!(false, policy.is_dns(test_node));
        //println!("inbound edges = {:?}",Policy::get_inbound_edges(test_node, &graph));
        assert_eq!(vec![0,1],Policy::get_inbound_edges(test_node, &graph) );
        //println!("outbound edges = {:?}" , Policy::get_outbound_edges(test_node, &graph) );
        assert_eq!(vec![1,0], Policy::get_outbound_edges(test_node, &graph));
        //println!("is violating max_inbound ? {}", policy.is_violating_max_inbound_node(test_node, &graph));
        assert_eq!(false, policy.is_violating_max_inbound_node(test_node, &graph));
        //println!("is violating optimal_outbound ? {}", policy.is_violating_optimal_outbound_node(test_node, &graph));
        assert_eq!(false,policy.is_violating_optimal_outbound_node(test_node, &graph) );
        //println!("is violating node ? {}",policy.is_vaiolating_node(test_node,&graph));    
        assert_eq!(false,policy.is_vaiolating_node(test_node,&graph) );
        //println!("is violating graph ? {}", policy.is_violating_graph(&graph));
        assert_eq!(true, policy.is_violating_graph(&graph) );
        //println!("is satisfied node ? {}",policy.is_satisfied_node(test_node, &graph) );
        assert_eq!(true, policy.is_satisfied_node(test_node, &graph));
        //println!("is satisfied graph ? {}",policy.is_satisfied_graph(&graph));
        assert_eq!(false, policy.is_satisfied_graph(&graph));
    }

    #[test]
    fn test_optimize_node() {
    let  path = "./samples/g1_test.csv";


    match csv_to_graph(path){
        Ok(result)=>{ 
            let mut dns_nodes = result.0;
            let mut graph = result.1;
            let mut optimal_outbound = result.2;
            let mut max_inbound = result.3;
            print_graph(&graph);
            println!("dns nodes => {:?}", dns_nodes);
            println!("optimal = {} , max_in = {} ",optimal_outbound, max_inbound );
            
            // optimize 
            
            let mut g = graph.clone();
            let optimizer = Optimizer::new(&mut graph,Policy::new(optimal_outbound,max_inbound, &dns_nodes));

            match optimizer.try_satisfy_node(2,&mut g){
                Ok(result)=>{
                    let new_graph = result.0;
                    let stats = result.1;

                    println!("cool satisfies some :) graph => {:?}",new_graph );
                    println!("stats = {:?}",stats );
                    assert_eq!(2,stats.node );
                    assert_eq!(2, stats.added_nodes);
                    assert_eq!(0, stats.missing_nodes);
                },
                Err(e)=>{
                    println!("err Pl0x no satisfaction because: {}",e );
                    assert!(false);
                },
            };
        },
        Err(e)=> assert!(false)
    };
    }

    #[test]
    fn test_optimize_graph(){
        
        let mut path = "g2_test.csv";
        match csv_to_graph(path){
            Ok(result)=>{ 
                let mut dns_nodes = result.0;
                let mut graph = result.1;
                let mut optimal_outbound = result.2;
                let mut max_inbound = result.3;
                print_graph(&graph);
                println!("dns nodes => {:?}", dns_nodes);
                println!("optimal = {} , max_in = {} ",optimal_outbound, max_inbound );
                
                // optimize 
                
                let mut g = graph.clone();
                let g2 = g.clone();
                let optimizer = Optimizer::new(&mut graph,Policy::new(optimal_outbound,max_inbound, &dns_nodes));

                match optimizer.try_satisfy_graph(&g2){
                    Ok(result)=>{
                        println!("{}",result );
                        assert_eq!(120, result.config_max_in);
                        assert_eq!(true,result.is_satisfied_graph );
                        assert_eq!(false, result.is_violating_graph );
                        assert_eq!(1, result.dns_nodes_num );
                        assert_eq!(0 , result.not_satisfied_nodes );
                        assert_eq!(9, result.satisfied_nodes_num );
                    },
                    Err(e)=>{
                        assert!(false);
                        println!("[-] Err optimizing graph! {} ", e);
                    }
                }
            },
            Err(e)=> println!("Error parsing csv {}",e )
        };
    }
}