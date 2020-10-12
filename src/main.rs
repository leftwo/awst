/*
 * Read in the cluster file (output from other scripts).
 * Convert the contents of that file to something more structured.
 *
 */
extern crate csv;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[derive(Debug)]
pub struct Cluster {
    nodes: Vec<Node>,
    vpc: String,
    region: String,
    units: u32,
}
#[derive(Debug)]
pub struct Node {
    instance_id: String,
    name: String,
    pub_ip: String,
    pri_ip: String,
    volume_id: String,
    role: String,
    efs_id: String,
    subnet: String,
    security_group: String,
    unit: u32,
    instance: u32,
}

fn read_cluster(file_path: &OsString) -> Result<Cluster, Box<dyn Error>> {
    let vpc: String;
    let region: String;
    let mut nodes: Vec<Node> = Vec::with_capacity(1);

    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(file);

    if let Some(result) = rdr.records().next() {
        let record = result?;
        vpc = record[9].to_string();
        region = record[10].to_string();
        let n: Node = Node {
            instance_id: record[0].to_string(),
            name: record[1].to_string(),
            pub_ip: record[2].to_string(),
            pri_ip: record[3].to_string(),
            volume_id: record[4].to_string(),
            role: record[5].to_string(),
            efs_id: record[6].to_string(),
            subnet: record[7].to_string(),
            security_group: record[8].to_string(),
            unit: record[11].to_string().parse().unwrap(),
            instance: record[12].to_string().parse().unwrap(),
        };
        // println!("First result {:?}", n);
        nodes.push(n);
    } else {
        return Err(From::from("Found no records in cluster file"))
    }

    for result in rdr.records() {
        let record = result?;
        let n: Node = Node {
            instance_id: record[0].to_string(),
            name: record[1].to_string(),
            pub_ip: record[2].to_string(),
            pri_ip: record[3].to_string(),
            volume_id: record[4].to_string(),
            role: record[5].to_string(),
            efs_id: record[6].to_string(),
            subnet: record[7].to_string(),
            security_group: record[8].to_string(),
            unit: record[11].to_string().parse().unwrap(),
            instance: record[12].to_string().parse().unwrap(),
        };
        // println!("next record: {:?}", n);
        nodes.push(n);
    }

    let my_cluster: Cluster = Cluster {
        vpc: vpc,
        region: region,
        units: 1,
        nodes: nodes,
    };
    Ok(my_cluster)
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    let filename = get_first_arg().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let cluster = read_cluster(&filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("We got: {:?}", cluster);
}

#[test]
fn basic() {
    let filename = OsString::from("./tests/example-0_cluster");
    let _cluster = read_cluster(&filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
}
#[test]
fn big() {
    let filename = OsString::from("./tests/big-0_cluster");
    let _cluster = read_cluster(&filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
}
#[test]
fn one() {
    let filename = OsString::from("./tests/one-0_cluster");
    let _cluster = read_cluster(&filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
}
#[test]
fn empty() {
    let filename = OsString::from("./tests/empty-0_cluster");
    match read_cluster(&filename) {
        Ok(_) => panic!("This should not pass"),
        Err(_) => println!("Expected error"),
    }
}
#[test]
fn partial() {
    let filename = OsString::from("./tests/partial-0_cluster");
    match read_cluster(&filename) {
        Ok(_) => panic!("This should not pass"),
        Err(_) => println!("Expected error"),
    }
}
