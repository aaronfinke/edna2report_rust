#![allow(non_snake_case)]
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_xml_rs::*;
use itertools::Itertools;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ReflectionData {
    ResolutionHigh: f64,
    NumberReflections: i32,
    NumberObservations: i32,
    NumberParts: i32,
    NumberLattices: i32,
    NumberBatches: i32,
    NumberDatasets: i32,
    Dataset: Dataset,
    data: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Dataset {
    Wavelength : f64,
    Run: Run,
    name: String
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Run {
    number: i32,
    BatchOffset: i32,
    BatchRange: String,
    FileStream: String,
}
pub fn convert_to_tuple(reflection_data:ReflectionData) -> Vec<(f64,f64)> {
    let data = reflection_data.data;
    let moo2: Vec<&str> = data.split('\n').collect();
    let moo3: Vec<&str> = moo2.into_iter().map(|f| f.trim()).collect();
    let moo4: Vec<(&str,&str)> = moo3.into_iter().map(|f| f.split_whitespace().collect_tuple().unwrap()).collect();
    let moo5: Vec<(f64,f64)> = moo4.into_iter().map( |f| (f.0.parse::<f64>().unwrap(),f.1.parse::<f64>().unwrap())).collect();
    return moo5;
}
