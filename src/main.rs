#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use] extern crate clap;

use std::str::FromStr;
use clap::{App, AppSettings};

use tokio::task;
use futures::join;
use ipfs::{make_ipld, Ipfs, IpfsPath, Ipld, Types, UninitializedIpfs};

use futures::TryStreamExt;
use ipfs_api::IpfsClient;
use std::io::{self, Write, Read};


#[tokio::main]
async fn main() {
	let client = IpfsClient::default();
	env_logger::Builder::new().filter_level(log::LevelFilter::Info).init(); // Init Logger
	
	let yaml = load_yaml!("app.yml");
	let app = App::from_yaml(yaml);
	let app = app.setting(AppSettings::ArgRequiredElseHelp);
	let matches = app.get_matches();
	
	match matches.subcommand() {
		("add", Some(sub_cmd)) => {
			println!("Adding File: {}", "smthn");
		},
		("rm", Some(sub_cmd)) => {
			println!("Removing CIDs: {}", "something here wip");
		},
		("block", Some(sub_cmd)) => {
			match sub_cmd.subcommand() {
				("get", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_get(cid_str).map_ok(|chunk| chunk.to_vec()).try_concat().await {
						Ok(data) => println!("Received block with contents: {:?}", data),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				("put", Some(sub_cmd)) => {
					if let Some(file) = sub_cmd.value_of("DATA") {
						let data = std::fs::File::open(file).expect("Failed to open file");
						match client.block_put(data).await {
							Ok(hash) => println!("Block Hash: {:?}", hash),
							Err(err) => println!("Error Occured Putting Data: {:?}", err),
						}
					} else {
						println!("Reading from stdin...");
						let data = std::io::stdin();
						match client.block_put(data).await {
							Ok(hash) => println!("Block Hash: {:?}", hash),
							Err(err) => println!("Error Occured Putting Data: {:?}", err),
						}
					}
					
					
				},
				("rm", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_stat(cid_str).await {
						Ok(resp) => println!("Deleted file: {:?}", resp),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				("stat", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_stat(cid_str).await {
						Ok(resp) => println!("Block stat data: {:?}", resp),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				_ => println!("ipfs: block: unknown subcommand"),
			}
		}
		
		("daemon", Some(sub_cmd)) => {
			println!("Starting IPFS Daemon");
			/*let (ipfs, fut): (Ipfs<Types>, _) = UninitializedIpfs::default().await.start().await.expect("Failed to start IPFS Daemon");
			task::spawn(fut);
			
			ctrlc::set_handler(move || {
				println!("Stopping Daemon...");
				ipfs.exit_daemon();
			});*/
		},
		
		("dht", Some(sub_cmd)) => {
			match sub_cmd.subcommand() {
				("findpeer", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_get(cid_str).map_ok(|chunk| chunk.to_vec()).try_concat().await {
						Ok(data) => println!("Received block with contents: {:?}", data),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				("findprovs", Some(sub_cmd)) => {
					if let Some(file) = sub_cmd.value_of("DATA") {
						let data = std::fs::File::open(file).expect("Failed to open file");
						match client.block_put(data).await {
							Ok(hash) => println!("Block Hash: {:?}", hash),
							Err(err) => println!("Error Occured Putting Data: {:?}", err),
						}
					} else {
						println!("Reading from stdin...");
						let data = std::io::stdin();
						match client.block_put(data).await {
							Ok(hash) => println!("Block Hash: {:?}", hash),
							Err(err) => println!("Error Occured Putting Data: {:?}", err),
						}
					}
					
					
				},
				("get", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_stat(cid_str).await {
						Ok(resp) => println!("Deleted file: {:?}", resp),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				("provide", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_stat(cid_str).await {
						Ok(resp) => println!("Block stat data: {:?}", resp),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				("put", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_stat(cid_str).await {
						Ok(resp) => println!("Block stat data: {:?}", resp),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				("query", Some(sub_cmd)) => {
					let cid_str = sub_cmd.value_of("CID").expect("ipfs: block: get: Expected Content ID (CID) Hash");
					
					match client.block_stat(cid_str).await {
						Ok(resp) => println!("Block stat data: {:?}", resp),
						Err(err) => println!("Error Occured Fetching Data: {:?}", err),
					}
				},
				_ => println!("ipfs: block: unknown subcommand"),
			}
		}
		cmd => println!("ipfs: unknown subcommand: {:?}", cmd)
	}
	
	
	/*
	// Create a DAG
	let f1 = ipfs.put_dag(make_ipld!("block1"));
	let f2 = ipfs.put_dag(make_ipld!("block2"));
	let (res1, res2) = join!(f1, f2);
	let root = make_ipld!([res1.unwrap(), res2.unwrap()]);
	let cid = ipfs.put_dag(root).await.unwrap();
	let path = IpfsPath::from(cid);

	// Query the DAG
	let path1 = path.sub_path("0").unwrap();
	let path2 = path.sub_path("1").unwrap();
	let f1 = ipfs.get_dag(path1);
	let f2 = ipfs.get_dag(path2);
	let (res1, res2) = join!(f1, f2);
	println!("Received block with contents: {:?}", res1.unwrap());
	println!("Received block with contents: {:?}", res2.unwrap());
	*/
	// Exit
}