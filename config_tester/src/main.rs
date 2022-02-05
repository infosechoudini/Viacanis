#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_snake_case)]
use serde_yaml;
use serde_yaml::Value;
use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::YamlLoader;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use walkdir::WalkDir;

#[derive(Default, Debug, Clone)]
pub struct Features{
    condition: String,
    apis: Vec<API>,
}

#[derive(Default, Debug, Clone)]
pub struct API{
    api: String,
}

fn main(){
    get_rules();

}



pub fn get_rules(){
    for entry in WalkDir::new("./rules/")
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| !e.file_type().is_dir()){
        let filename = String::from(entry.file_name().to_string_lossy());
        println!("ENTRY: {:#?}", entry);
        //println!("FILENAME: {:#?}", entry.clone().into_path());
        if filename.contains("yaml"){
            let rule_file = std::fs::read_to_string(entry.clone().into_path()).unwrap();
            parse_rules(rule_file);
        }
    }
}


pub fn parse_rules(rule: String) -> Vec<Features> {
    let rule_parsed = YamlLoader::load_from_str(&rule).unwrap();
    
    let rule = &rule_parsed[0];
    let name = rule["rule"]["meta"]["name"].as_str().unwrap();
    println!("Name: {:#?}", name);
    let mut feature_vec = rule["rule"]["features"].as_vec().unwrap();
    let mut features = feature_vec[0].as_hash().unwrap();
    println!("Name: {:#?}", features);

    let mut all_conditionals: Vec<Features> = Vec::new();
    // We get the Top Level Feature Keys
    for entry in features.keys().next(){
        match entry.as_str().unwrap(){
            "and" => { 
                let mut second_level_conditonals_vec = features[entry].as_vec().unwrap().clone();
                println!("second_level_conditonals_vec: {:#?}", second_level_conditonals_vec);
                let rule_features: Vec<Features> = parse_features(second_level_conditonals_vec);
                println!("RULE FEATURES: {:#?}", rule_features);
                return rule_features;
            },
            "or" =>{
                let mut second_level_conditonals_vec = features[entry].as_vec().unwrap().clone();
                println!("second_level_conditonals_vec: {:#?}", second_level_conditonals_vec);
                let rule_features: Vec<Features> = parse_features(second_level_conditonals_vec);
                println!("RULE FEATURES: {:#?}", rule_features);
                return rule_features;
 
            },

            _ => break,
        }
    }

    println!("All Conditionals: {:#?}", all_conditionals);


}

pub fn parse_features(second_level_conditonals_vec: Vec<Yaml>) -> Vec<Features>{
    let mut feature_vec: Vec<Features> = Vec::new();
    for x in 0 .. second_level_conditonals_vec.len(){
        let mut second_level_conditonals = second_level_conditonals_vec[x].as_hash().unwrap().clone();
        println!("second_level_conditonals: {:#?}", second_level_conditonals);
        for second_level in second_level_conditonals.keys().next(){
            //println!("Name: {:#?}", second_level);
            match second_level.as_str().unwrap(){
                "and" => {
                    let mut third_level_conditionals_len = second_level_conditonals[second_level].as_vec().unwrap().clone().len();
                    println!("Array LEN: {:#?}", third_level_conditionals_len);
                    let mut api_vec: Vec<API> = Vec::new();
                    let mut conditional_feature: Features = Features::default();
                    let mut x = 0;
                    for x in 0..third_level_conditionals_len {
                        let mut third_level_conditional = second_level_conditonals[second_level][x].as_hash().unwrap().clone();
                        for third_entry in third_level_conditional.entries().next(){
                            match third_entry.key().clone().as_str().unwrap(){
                                "match" => {
                                    let value = third_entry.get().clone();
                                    conditional_feature.condition = value.clone().as_str().unwrap().to_string();
                                },
                                "api" => {
                                    let mut api: API = API::default();
                                    let value = third_entry.get().clone();
                                    api.api = value.clone().as_str().unwrap().to_string();
                                    api_vec.push(api.clone());
                                },

                                _ => {

                                    println!("DO NOTHING");
                                },
                            }
                        }
                    }
                    conditional_feature.apis = api_vec;
                    feature_vec.push(conditional_feature);
                },
                "or" => {
                    
                },
                _ => continue,
            }
        }      
    }     

    return feature_vec;
}