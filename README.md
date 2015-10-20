# Pagerank in Rust

This is my first poor attempt to write something useful in Rust.

It expects a file with edges (@from    @to) to be present two directories up. 
When reading the file, the first 4 lines will be skipped, because in my particular task, 
these contained only comments about the file.

Also, the number of distinct nodes is provided in main.rs, because there are gaps in the
data file. I didn't worry about robustness and/or flexibility.

Here you can see the first ten lines of that file:
```
# Directed graph (each unordered pair of nodes is saved once): web-Google.txt 
# Webgraph from the Google programming contest, 2002
# Nodes: 875713 Edges: 5105039
# FromNodeId	ToNodeId
0	11342
0	824020
0	867923
0	891835
11342	0
11342	27469
```

With 875,713 nodes and 5,105,039 edges, it takes less than 10 s to converge at an error of 10^-6,
46 s with an error of 10^-16.


