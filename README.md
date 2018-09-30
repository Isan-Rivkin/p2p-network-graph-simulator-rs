A module for modeling a P2P network into a graph and testing violation and optimization of a graph.   

* `samples/g_8_100_99_1.csv` demonstrates a problem because MAX = 50 and n = 100 
meaning the graph is violating because the dns has more than 50 in.
=> dns architechture.
 
* adding a fix to the script is needed because according to above, even 10 dns dont solve 
since they currently connect to all the dns's. 
need to seperate connections so the graph will be valid.

### Params
* `N` : Number of nodes in the network. 
* `DNS` : Number of DNS node `DNS` is part of `N`
* `INBOUND_MAX` : The number of maximum allowed connecections
* `OUBTBOUND_OPTIMAL` : The number of wished outbound connections
* `TOTAL_DNS_IN` = `DNS` * `INBOUND_MAX`, the total number of possible inbound connections available using all the DNS nodes.
* `TOATAL_OUT_NON_DNS` = (`N` - `DNS`) * `OUTBOUND_OPTIMAL`, the total number of possible outbound connections from all non DNS Nodes.

### Benchmarks 
| Input              | N   | DNS | MAX_INBOUND | OPTIMAL_OUTBOUND | Optimal? | Satisfied Nodes (Excluding DNS) | Legal State? |
|--------------------|-----|-----|-------------|------------------|----------|---------------------------------|--------------|
| g_o50_n100_m99_d1  | 100 | 1   | 125         | 50               | true     | 99/99                           | true         |
| g_o51_n100_m99_d1  | 100 | 1   | 125         | 51               | false    | 49/99                           | true         |
| g_o55_n100_m125_d1 | 100 | 1   | 125         | 55               | false    | 45/99                           | true         |
| g_o60_n100_m125_d1 | 100 | 1   | 125         | 60               | false    | 40/99                           | true         |
| g_o65_n100_m99_d1  | 100 | 1   | 125         | 65               | false    | 35/99                           | true         |
| g_o70_n100_m125_d1 | 100 | 1   | 125         | 70               | false    | 30/99                           | true         |
| g_o80_n100_m125_d1 | 100 | 1   | 125         | 80               | false    | 20/99                           | true         |
| g_o90_n100_m125_d1 | 100 | 1   | 125         | 90               | false    | 10/99                           | true         |  
### Considerations
Optimal Network will not be possible (In current DNS desgin) if:

1) There are more possible outbound connections than inbound to the dns group..

    * TOTAL_DNS_IN < TOTAL_OUT_NON_DNS
