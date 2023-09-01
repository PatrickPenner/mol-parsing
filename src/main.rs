extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::{Parser, RuleType};

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::process::ExitCode;

use flate2::read;

mod pdb;
mod sdf;
mod smiles;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_features<T>(
    parse_element: Pair<T>,
    feature_counts: &mut BTreeMap<T, i32>,
    feature_values: &mut BTreeMap<T, BTreeSet<String>>,
) where
    T: RuleType,
{
    let feature = parse_element.as_rule();
    if !feature_counts.contains_key(&feature) {
        feature_counts.insert(feature, 1);
    } else {
        *feature_counts.get_mut(&feature).unwrap() += 1;
    }
    if feature_values.contains_key(&feature) {
        feature_values
            .get_mut(&feature)
            .expect("Set does not exist")
            .insert(String::from(parse_element.as_str()));
    }
    for sub_element in parse_element.into_inner() {
        count_features(sub_element, feature_counts, feature_values)
    }
}

fn parse_smiles(filename: &str) {
    let mut feature_counts = BTreeMap::new();
    let mut feature_values = smiles::get_feature_value_map();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("smiles") {
                    continue;
                }
                // split off name
                let split_line: Vec<&str> = ip.split(" ").collect();
                let successful_parse =
                    smiles::SmilesParser::parse(smiles::Rule::String, &split_line[0])
                        .expect("Failed to parse SMILES")
                        .next()
                        .unwrap();
                // print_parse_elements(successful_parse);
                if successful_parse.as_span().end() != split_line[0].len() {
                    panic!("Failed to parse {:}", ip);
                }
                count_features(successful_parse, &mut feature_counts, &mut feature_values);
            }
        }
    }
    println!("{:}", serde_json::to_string(&feature_counts).unwrap());
    println!("{:}", serde_json::to_string(&feature_values).unwrap());
}

fn read_gz(filename: &str) -> String {
    let file = File::open(filename);
    let mut gz = read::GzDecoder::new(file.expect("File not found"));
    let mut ungzipped_string = String::new();
    gz.read_to_string(&mut ungzipped_string).unwrap();
    return ungzipped_string;
}

fn parse_sdf(sdf_string: &str) {
    let mut feature_counts = BTreeMap::new();
    let mut feature_values = sdf::get_feature_value_map();
    for sdf_record in sdf_string.split("$$$$") {
        if sdf_record.trim().len() == 0 {
            continue;
        }
        let successful_parse = sdf::SdfParser::parse(sdf::Rule::sdfrecord, &sdf_record.trim())
            .expect("Failed to parse SDF")
            .next()
            .unwrap();
        let parse_end = successful_parse.as_span().end();
        count_features(successful_parse, &mut feature_counts, &mut feature_values);
        if parse_end != sdf_record.trim().len() {
            panic!(
                "Failed to parse SDF at molfile {:}",
                *feature_counts
                    .get(&sdf::Rule::molfile)
                    .expect("Did not find any molfiles")
            );
        }
    }
    println!("{:}", serde_json::to_string(&feature_counts).unwrap());
    println!("{:}", serde_json::to_string(&feature_values).unwrap());
}

fn parse_pdb(pdb_string: &str) {
    let successful_parse = pdb::PdbParser::parse(pdb::Rule::pdbfile, &pdb_string)
        .expect("Failed to parse PDB")
        .next()
        .unwrap();
    let parse_end = successful_parse.as_span().end();
    let mut feature_counts = BTreeMap::new();
    let mut feature_values = pdb::get_feature_value_map();
    count_features(successful_parse, &mut feature_counts, &mut feature_values);
    if parse_end != pdb_string.len() {
        panic!("Failed to parse pdb");
    }
    println!("{:}", serde_json::to_string(&feature_counts).unwrap());
    println!("{:}", serde_json::to_string(&feature_values).unwrap());
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: main <format: smiles | sdf | sdfgz | pdb> <path to file>");
        return ExitCode::FAILURE;
    }

    if args[1] == "smiles" {
        parse_smiles(&args[2]);
    } else if args[1] == "sdfgz" {
        parse_sdf(&read_gz(&args[2]));
    } else if args[1] == "sdf" {
        parse_sdf(&read_to_string(&args[2]).expect("File not found"))
    } else if args[1] == "pdbgz" {
        parse_pdb(&read_gz(&args[2]));
    } else if args[1] == "pdb" {
        parse_pdb(&read_to_string(&args[2]).expect("File not found"))
    }

    return ExitCode::SUCCESS;
}
