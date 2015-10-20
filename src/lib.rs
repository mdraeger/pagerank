use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Edge = (usize, usize);
pub type CountMap = BTreeMap<usize, usize>;
pub type SparseDoubleMatrix = BTreeMap<usize, BTreeMap<usize, f64>>;
pub type EdgeMatrix = BTreeMap<usize, Vec<usize>>;
pub type DoubleVector = Vec<f64>;

pub fn iterate(m: &SparseDoubleMatrix, r_vec: &DoubleVector, beta: f64, epsilon: f64, n: f64) -> DoubleVector{
    let mut rcopy = r_vec.clone();
    let mut dist = 1000.0 * epsilon;
    let mut i=0;
    while dist > epsilon {
        println!("iteration: {}, distance: {:e}", i, dist);
        i+=1;
        let rnew = mult(m, &rcopy, beta, n);
        dist = norm(&abs_minus(&rcopy, &rnew));
        rcopy = rnew;
    }
    rcopy
}

fn sum(r_vec: &DoubleVector) -> f64{
    let mut sum = 0.0;
    for v in r_vec{
        sum += *v;
    }
    sum
}

fn norm(r_vec: &DoubleVector) -> f64{
    let mut norm = 0.0;
    for v in r_vec{
        norm += v.powi(2);
    }
    norm.sqrt()
}

fn abs_minus(rnew: &DoubleVector, rold: &DoubleVector) -> DoubleVector{
    assert_eq!(rnew.len(), rold.len());
    let mut result = Vec::with_capacity(rnew.len());
    for i in 0..rnew.len(){
        result.push((rnew.get(i).unwrap() - rold.get(i).unwrap()).abs());
    }
    result
}

fn mult(m_matrix: &SparseDoubleMatrix, r_vec: &DoubleVector, beta: f64, n: f64) -> DoubleVector{
    let mut new_r = Vec::with_capacity(r_vec.len() +1);
    for i in 0..r_vec.len(){
        let mut val = 0.0;
        if m_matrix.contains_key(&i){
            let ref inner_map = m_matrix[&i];
            for (&k,v) in inner_map {
                val += r_vec.get(k).unwrap() * v * beta;
            }
        }
        new_r.push(val);
    }
    
    let sum = sum(&new_r);
    let mut final_r = Vec::with_capacity(r_vec.len() + 1);
    for v in new_r {
        final_r.push(v + (1.0 - sum)/n);
    }

    final_r
}

pub fn r_vec(number_of_nodes: usize, init_val: f64, m: &SparseDoubleMatrix ) -> DoubleVector{
    let mut vec = Vec::with_capacity(number_of_nodes + 1);
    for i in 0..(number_of_nodes+1){
        if m.contains_key(&i) {
            vec.push(init_val);
        } else {
            vec.push(0.0);
        }
    }
    vec
}

pub fn m_sparse(edges: &EdgeMatrix, out_deg: &CountMap) -> SparseDoubleMatrix {
    let mut m_sp = BTreeMap::new();
    for (&from_node, to_nodes) in edges {
        for &to_node in to_nodes {
            let out = out_deg[&from_node] as f64;
            let mut inner_map = m_sp.entry(to_node).or_insert(BTreeMap::new());
            inner_map.insert(from_node, 1.0/out);
        }
    }
    m_sp
}

pub fn out_degrees(edges: &EdgeMatrix) -> CountMap{
    let mut out_deg = BTreeMap::new();
    for (&k,v) in edges {
        out_deg.insert(k, v.len());
    }
    out_deg
}

pub fn max_node(edges: &EdgeMatrix) -> usize {
    let mut max_node = 0 as usize;
    for (&k, vec) in edges {
        if k > max_node {
            max_node = k;
        }
        for v in vec {
            if *v > max_node {
                max_node = *v
            }
        }
    }
    max_node
}

pub fn dict_of_edges_from_lines(lines: Result<io::Lines<BufReader<File>>, io::Error>, skip: usize) 
    -> EdgeMatrix{
    let mut edges = BTreeMap::new();
    match lines {
        Ok(l) => {
            for line in l.skip(skip) {
                let (from, to) = split_from_to(&line.unwrap());
                if to == 903066 {
                    println!("\t{} -> {}", from, to);
                }
                edges.entry(from).or_insert(Vec::new()).push(to);
            }
        },
        Err(e) => print!("Error {}", e)
    }
    edges
}

fn split_from_to(line: &str) -> Edge {
    let split_list = line.split_whitespace().collect::<Vec<_>>();
    let v1 = split_list.first().unwrap().parse::<usize>().unwrap();
    let v2 = split_list.last().unwrap().parse::<usize>().unwrap();
    (v1, v2)
}

pub fn lines_from_file<P>(filename: P) -> Result<io::Lines<BufReader<File>>, io::Error>
where P: AsRef<Path> {
    let file = try!(File::open(filename));
    Ok(BufReader::new(file).lines())
}

#[test]
fn test_lines_from_file() {
    let file_name = "../../web-short.txt";
    let lines = lines_from_file(file_name);
    match lines {
        Ok(l) => assert_eq!(5000, l.count()),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_split_into_from_to(){
    let str1 = "0       11342";
    let str2 = "0       824020";
    let str3 = "11342   0";
    
    assert_eq!((0,11342), split_from_to(str1));
    assert_eq!((0,824020), split_from_to(str2));
    assert_eq!((11342,0), split_from_to(str3));
}

#[test]
fn test_dict_from_file(){
    let lines = lines_from_file("../../web-short.txt");
    assert_eq!(416, dict_of_edges_from_lines(lines, 4).keys().count());
}

#[test]
fn test_max_node(){
    let lines = lines_from_file("../../web-short.txt");
    let dict = dict_of_edges_from_lines(lines, 4);
    assert_eq!(914842, max_node(&dict));
}

#[test]
fn test_out_degree() {
    let lines = lines_from_file("../../web-short.txt");
    let dict = dict_of_edges_from_lines(lines, 4);
    assert_eq!(4, out_degrees(&dict)[&0]);
}

#[test]
fn test_m_sparse(){
    let lines = lines_from_file("../../web-short.txt");
    let dict = dict_of_edges_from_lines(lines, 4);
    let out_deg = out_degrees(&dict);
    assert_eq!(1.0/4.0,  m_sparse(&dict,&out_deg)[&11342][&0]);
}

#[test]
fn test_r_vec(){
    let lines = lines_from_file("../../web-short.txt");
    let dict = dict_of_edges_from_lines(lines, 4);
    let out_deg = out_degrees(&dict);
    let m = m_sparse(&dict, &out_deg);
    let max_node = out_deg.keys().max().unwrap();
    let r_v = r_vec(*max_node, 1.0/ 5.000, &m);
    assert_eq!(*max_node+1, r_v.len());
}

#[test]
fn test_mult(){
    let mut m = BTreeMap::new();
    let mut i0 = BTreeMap::new();
    i0.insert(0 as usize, 1.0);
    let mut i1 = BTreeMap::new();
    i1.insert(1 as usize, 1.0);
    let mut i2 = BTreeMap::new();
    i2.insert(2 as usize, 1.0);
    m.insert(0 as usize, i0);
    m.insert(1 as usize, i1);
    m.insert(2 as usize, i2);

    let v = vec![1.0/6.0, 2.0/6.0, 3.0/6.0];
    assert_eq!(1.0, sum(&mult(&m, &v, 1.0, 3.0)));
}

#[test]
fn test_abs_minus(){
    let v1 = vec![1.0,1.0,1.0];
    let v2 = vec![1.0,1.0,0.0];
    let vres = vec![0.0, 0.0, 0.0];
    assert_eq!(vres, abs_minus(&v1, &v1));
    assert_eq!(vec![0.0,0.0,1.0], abs_minus(&v1, &v2));
}

#[test]
fn test_norm(){
    let v = vec![1.0,2.0,3.0];
    assert_eq!((14.0 as f64).sqrt(), norm(&v));
}
