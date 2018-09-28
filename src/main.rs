mod policy;
use policy::Policy;

extern crate csv;

use std::error::Error;

type optimal_out = usize; 
type max_in = usize;

pub fn csv_to_graph(path : &str)->Result<(Vec<usize>,Vec<Vec<usize>>,optimal_out,max_in),Box<Error>> {
    
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new().
        flexible(true).
        from_path(path)?;

    let mut graph : Vec<Vec<usize>> = Vec::new();

    for (idx,result) in rdr.records().enumerate(){

        let line = result?;
        graph.push(Vec::new());

        for element in &line {
            let num =  element.to_string().parse::<usize>()?;
            graph[idx].push(num);
        }
    }
    // get the dns nodes 
    if let Some(mut dns_nodes) = graph.pop(){
        if let Some(max_in) = dns_nodes.pop(){

            if let Some(optimal) = dns_nodes.pop(){
                return Ok((dns_nodes,graph, optimal, max_in))
            }
        }
    }

    Err(From::from("expected at least one record but got none"))
}


fn main() {

    match csv_to_graph("graph_input.csv"){
        Ok(result)=>{ 
            let mut dns_nodes = result.0;
            let mut graph = result.1;
            let mut optimal_outbound = result.2;
            let mut max_inbound = result.3;
            println!("graph => {:?} ", graph);
            println!("dns nodes => {:?}", dns_nodes);
            println!("optimal = {} , max_in = {} ",optimal_outbound, max_inbound );
        },
        Err(e)=> println!("Error parsing csv {}",e )
    };
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

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
    println!("is_dns ? {}",policy.is_dns(test_node) );
    println!("inbound edges = {:?}",policy.get_inbound_edges(test_node, &graph));
    println!("outbound edges = {:?}" , policy.get_outbound_edges(test_node, &graph) );
    println!("is violating max_inbound ? {}", policy.is_violating_max_inbound_node(test_node, &graph));
    println!("is violating optimal_outbound ? {}", policy.is_violating_optimal_outbound_node(test_node, &graph));
    println!("is violating node ? {}",policy.is_vaiolating_node(test_node,&graph));    
    println!("is violating graph ? {}", policy.is_violating_graph(&graph));
    println!("is satisfied node ? {}",policy.is_satisfied_node(test_node, &graph) );
    println!("is satisfied graph ? {}",policy.is_satisfied_graph(&graph));
    assert!(true);
    }
}