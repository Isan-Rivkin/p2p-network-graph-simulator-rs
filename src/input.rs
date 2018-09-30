use std::default::Default;
use std::error::Error;  
use csv;
use optimizer::Graph;

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

#[derive(Default,Debug)]
pub struct GeneratorConfig {
    pub opt : usize,
    pub n : usize, 
    pub max : usize, 
    pub edges : Graph, 
    pub dns_nodes : usize,
}

pub fn generate_input(config : GeneratorConfig)->Graph{
    let mut the_input : Graph = Vec::new();

    // to be added to the last line in the csv 
    let mut dns_nodes : Vec<usize> = (0..config.dns_nodes).collect();
        
    // initiate 2d matrix 
    for i in 0..config.n{
        
        the_input.push(Vec::new());

        // connect the dns's to each other 
        if i< config.dns_nodes{
            
            let mut edges : Vec<usize> = dns_nodes.clone().into_iter()
            .filter(|&dn| dn != i)
            .collect();
            
            the_input[i].append(&mut edges);

        }    
        // connect to dns  
        if i >= config.dns_nodes{
            let mut c = dns_nodes.clone();
            the_input[i].append(&mut c)
        }
    }

    
    // generate the lat line of the csv 
    let mut last_line : Vec<usize> = Vec::new();
    
    //if no connections type ~;last raw order : dns nodes are first, optimal, max_in // i.e 1,0,8,125
    last_line.append(&mut dns_nodes);
    last_line.push(config.opt);
    last_line.push(config.max);
    
    // append the last line to the 2d matrix
    the_input.push(last_line);
    
    the_input
}