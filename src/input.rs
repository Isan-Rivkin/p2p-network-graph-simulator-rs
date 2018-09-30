use std::error::Error;  
use csv;

pub type OptimalOut = usize; 
pub type MaxIn = usize;

pub fn csv_to_graph(path : &str)->Result<(Vec<usize>,Vec<Vec<usize>>,OptimalOut,MaxIn),Box<Error>> {
    
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new().
        flexible(true).
        from_path(path)?;

    let mut graph : Vec<Vec<usize>> = Vec::new();

    for (idx,result) in rdr.records().enumerate(){

        let line = result?;
        graph.push(Vec::new());

        for element in &line {
            if element.to_string() != "~"{
                let num =  element.to_string().parse::<usize>()?;
                graph[idx].push(num);
            }
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