/*
 * Read in the cluster file (output from other scripts).
 * Convert this to something more structured.
 *
 * This version expects the first line to be the header.
 * Below is the format of the cluster file that I need to import

# Cluster created on Fri Oct  2 08:26:03 PDT 2020 by ubuntu on host ohiokube
#instance_id,name,public_ip,private_ip,volume_id,role,efsid,subnet,security-group,vpc,region,unit,instance
# VPC vpc-0c6171d6bc60ffcbb
# AWS_REGION us-east-2
# MS_VPC_INFO sg-0a697e667b65da583 subnet-03137f7b1dca57f48, rtb-022b10b7021c0301d igw-0ff13bdd1223675e6 vpc-0c6171d6bc60ffcbb
# VPC_INFO sg-0ae887d1039187111 subnet-0f7a5e07b5e1ed597, rtb-022b10b7021c0301d igw-0ff13bdd1223675e6 vpc-0c6171d6bc60ffcbb
i-073b92a9037913280,ueb-BASE-0-0-0,18.216.95.92,10.0.16.70, vol-0682b627cda5807a7 vol-0ad146c89478e76b1,BASE,NONE,subnet-0f7a5e07b5e1ed597,sg-0ae887d1039187111,vpc-0c6171d6bc60ffcbb,us-east-2,0,0
i-0e635b5061b10ab4e,ueb-EDGE-0-0-0,52.15.209.56,10.0.16.60, vol-03fd6cb31a6b3f760 vol-00bd82f4b3718499c,EDGE,NONE,subnet-0f7a5e07b5e1ed597,sg-0ae887d1039187111,vpc-0c6171d6bc60ffcbb,us-east-2,0,0

 * The next steps are going to be to convert this into some json formatted file and
 * then output that file.  Future program should make use of importing the json
 * file.
 */

extern crate csv;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[derive(Debug)]
struct Cluster {
    nodes: Vec<Node>,
    vpc: String,
    region: String,
    units: u32,
}
#[derive(Debug)]
struct Node {
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

fn read_cluster(file_path: &OsString) -> Result<(), Box<dyn Error>> {
    // let file_path = get_first_arg()?;
    let vpc: String;
    let region: String;
    let mut nodes: Vec<Node>;

    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(file);

    if let Some(result) = rdr.records().next() {
        let record = result?;
        println!("First result {:?}", result);
    } else {
        return Err(From::from("expected at least one record but got none"))
    } 

/*
    let record = match rdr.records().next() {
        Some(result) => result
    if result == None {

    let record = result;

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
    let my_cluster: Cluster = Cluster {
        vpc: record[9].to_string(),
        region: record[10].to_string(),
        units: 1,
        nodes: vec![n],
    };

    println!("we got: {:?}", my_cluster);
*/
    for result in rdr.records() {
        let record = result?;
        let tstr = record[11].to_string();
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
            unit: tstr.parse().unwrap(),
            instance: record[12].to_string().parse().unwrap(),
        };
        println!("we got: {:?}", n);
        // my_cluster.nodes.push(n);
    }
    Ok(())
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

    if let Err(err) = read_cluster(&filename) {
        println!("{}", err);
        process::exit(1);
    }
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
