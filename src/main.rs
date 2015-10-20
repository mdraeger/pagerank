extern crate time;
extern crate utillib;

use time::PreciseTime;
use utillib::*;

fn main() {
    let start = PreciseTime::now();
    let file_name = "../../web-Google.txt";

    let beta = 0.8;
    let epsilon = 1.0e-5;

    let lines = lines_from_file(file_name);
    println!("input file initiated {}", start.to(PreciseTime::now()));
    
    let dict = dict_of_edges_from_lines(lines, 4);

    let file_read = PreciseTime::now();
    println!("input file read and parsed {}", start.to(file_read));

    let out_deg = out_degrees(&dict);
    let m = m_sparse(&dict, &out_deg);
    let n = max_node(&dict);
    println!("max node: {}", n);
    let n_double = 875713.0;
    let r_init = r_vec(n, 1.0/n_double, &m);

    let initialized = PreciseTime::now();
    println!("vectors and matrices initialized {}", file_read.to(initialized));

    let r_final = iterate(&m, &r_init, beta, epsilon, n_double);
    let nodes = vec![0, 99, 767, 11342, 824020, 856260, 903066];
    for node in nodes {
        println!("node: {}, rank: {:e}", node, r_final.get(node).unwrap());
    }

    let finished = PreciseTime::now();
    println!("converged after {}", initialized.to(finished));
}

#[test]
fn test_something(){
    assert!(true);
}
