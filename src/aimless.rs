#![allow(non_snake_case)]
#![allow(dead_code)]

use std::num::ParseIntError;

use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Debug)]
pub struct Aimless {
	#[serde(rename = "@version")]
	version: String,
	
	#[serde(rename = "@RunTime")]
	RunTime: String,

	ReflectionData: ReflectionData,
    ReflectionFile: ReflectionFile,
    ScaleModelFail: String,
    OnlyMerge: String,
    RotationalOverlap:RotationalOverlap,

    #[serde(rename="$value")]
    fields:Vec<Fields>,

    AnomalousStatus:String,
    Result:AimlessResult,
    OutputFiles:OutputFiles,

}

#[derive(Deserialize, Debug)]
enum Fields {
    OutlierControl(OutlierControl),
    BatchGroupMessage(String),
    BatchGroupWidth(f64),
    BatchGroupNumber(i32),
    Outliers(Outliers),
    CCP4Table,
}
#[derive(Deserialize, Debug)]
pub struct ReflectionFile {
    #[serde(rename="@stream")]
    stream:String,
    #[serde(rename="@name")]
    name:String,
    cell:Cell,
    SpacegroupName:String,
}
#[derive(Deserialize, Debug)]
pub struct Cell {
    a:f64,
    b:f64,
    c:f64,
    alpha:f64,
    beta:f64,
    gamma:f64
}

// #[derive(Deserialize, Debug)]
// pub struct ScaleModelFail {
//     #[serde(rename="@class")]
//     class:String,
//     #[serde(rename="$value")]
//     _value:Vec<String>,
// }
#[derive(Deserialize, Debug)]
pub struct RotationalOverlap {
    EnoughOverlap:bool,
    AverageOverlap:f64,
    Overlapthreshold:f64,
    AllowedGap:f64,
    MinimumOverlap:f64,
}
#[derive(Deserialize, Debug)]
pub enum CCP4Table {
    CCP4TableOne,
    CCP4TableTwo
}

#[derive(Deserialize, Debug)]
pub struct CCP4TableOne {
    #[serde(rename="@groupID")]
    groupID:String,
    #[serde(rename="@id")]
    id:String,
    #[serde(rename="@title")]
    title:String,
    
    headers: Option<Headers>,
    #[serde(rename="$value")]
    data: Vec<String>,
    #[serde(rename="$value")]
    plot: Option<Vec<Plot>>,
}

#[derive(Deserialize, Debug)]
pub struct CCP4TableTwo {
    #[serde(rename="@title")]
    title:String,
    
    headers: Option<Headers>,
    #[serde(rename="$value")]
    data: Vec<String>,
    #[serde(rename="$value")]
    plot: Option<Vec<Plot>>,
}

#[derive(Deserialize, Debug)]
pub struct Headers {
    #[serde(rename="@separator")]
    separator:Option<String>,
    #[serde(rename="$value")]
    _value:String,
}
#[derive(Deserialize, Debug)]
pub struct Data{
    #[serde(rename="@id")]
    id:Option<String>,
    #[serde(rename="$value")]
    _value:String,
}
#[derive(Deserialize, Debug)]
pub struct Plot {
    title:Option<String>,
    xrange:Option<Xrange>,
    yrange:Option<Yrange>,
    fixaspectratio:Option<bool>,
    xlabel:Option<String>,
    ylabel:Option<String>,
    showlegend:Option<bool>,
    legendposition:Option<LegendPosition>,
    #[serde(rename="$value")]
    plotline:Option<Vec<Plotline>>,
    #[serde(rename="$value")]
    line: Option<Vec<Line>>,
}

#[derive(Deserialize, Debug)]
pub struct Xrange{
    #[serde(rename="@min")]
    min:String,
    #[serde(rename="@max")]
    max:String,
}
#[derive(Deserialize, Debug)]
pub struct Yrange {
    #[serde(rename="@min")]
    min:String,
    #[serde(rename="@max")]
    max:String,
}
#[derive(Deserialize, Debug)]
pub struct LegendPosition {
    #[serde(rename="@x")]
    x:String,
    #[serde(rename="@y")]
    y:String,
}
#[derive(Deserialize, Debug)]
pub struct Plotline {
    #[serde(rename="@dataid")]
    dataid:Option<String>,
    #[serde(rename="@xcol")]
    xcol:Option<String>,
    #[serde(rename="@ycol")]
    ycol:Option<String>,
    label:Option<String>,
    symbolsize:Option<String>,
    colour:Option<String>,
    markeredgewidth:Option<String>,
    linestyle:Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Line {
    #[serde(rename="@x1")]
    x1:String,
    #[serde(rename="@y1")]
    y1:String,
    #[serde(rename="@x2")]
    x2:String,
    #[serde(rename="@y2")]
    y2:String,
    #[serde(rename="@linestyle")]
    linestyle:String,
    #[serde(rename="@linecolour")]
    linecolour:Option<String>,
}


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
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Dataset {
    Wavelength : f64,
    Run: Run,
	#[serde(rename = "@name")]
    name: String
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Run {
    number: i32,
    BatchOffset: i32,
	// #[serde(deserialize_with="to_tuple")]
    BatchRange:String,
    FileStream: String,
}
#[derive(Deserialize, Debug)]
pub struct Outliers {
    RejectNumberUnique: i32,
    RejectNumberFriedel: i32,
    RejectNumberEmax:i32,
}
#[derive(Deserialize, Debug)]
pub struct OutlierControl {
    OutlierWeightType: String,
    Main: Outlier_Main,
    Anom: Outlier_Anom,
    EmaxTest: Outlier_Emaxtest,
}

#[derive(Deserialize, Debug)]
struct Outlier_Main {
    SDrej: f64,
    SDrej2: f64,
    Reject2policy:String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Outlier_Anom {
    #[serde(rename="@dataset")]
    dataset: String,
    SDrej: f64,
    SDrej2: f64,
    Reject2policy:String,
}
#[derive(Deserialize, Debug)]
struct Outlier_Emaxtest {
    EmaxAcentric: f64,
    EmaxCentric: f64,
}

#[derive(Deserialize, Debug)]
pub struct AimlessResult {
    Dataset:DatasetResult,
}

#[derive(Deserialize, Debug)]
pub struct DatasetResult {
    #[serde(rename="@name")]
    name:String,
    #[serde(rename = "$value")]
    field: Vec<ResultEnum>,
}
#[derive(Deserialize, Debug)]
enum ResultEnum {
    ResolutionLow(shellResults),
    ResolutionHigh(shellResults),
    Rmerge(shellResults),
    RmergeOverall(shellResults),
    Rmeas(shellResults),
    RmeasOverall(shellResults),
    Rpim(shellResults),
    RpimOverall(shellResults),
    RmergeTopI(String),
    NumberObservations(shellResults),
    NumberReflections(shellResults),
    MeanIoverSD(shellResults),
    CChalf(shellResults),
    Completeness(shellResults),
    Multiplicity(shellResults),
    MeanChiSq(shellResults),
    AnomalousCompleteness(shellResults),
    AnomalousMultiplicity(shellResults),
    AnomalousCChalf(shellResults),
    AnomalousNPslope(f64),
    AnomalousLimitEstimate(AnomalousLimitEstimate),
    ResolutionLimitEstimate(ResolutionLimitEstimate),
    cell(Cell),
    SpacegroupName(String),
    Mosaicity(f64),
    MinimumSDcorrectionFulls(f64),
    MaximumSDcorrectionFulls(f64),
    MinimumSDcorrectionPartials(f64),
    MaximumSDcorrectionPartials(f64),

}
#[derive(Deserialize, Debug)]
pub struct AnomalousLimitEstimate {
    #[serde(rename="@type")]
    esttype:String,
    Direction:String,
    Threshold:f64,
    MaximumResolution:f64,
    Message:String,
}

#[derive(Deserialize, Debug)]
pub struct ResolutionLimitEstimate {
    #[serde(rename="@type")]
    esttype:String,
    Direction:String,
    Threshold:f64,
    MaximumResolution:f64,
    Message:Option<String>,
}

#[derive(Deserialize, Debug)]
struct shellResults {
    Overall:f64,
    Inner:f64,
    Outer:f64,
}
#[derive(Deserialize, Debug)]
pub struct OutputFiles {
    OutputType:String,
    SCAOutputType:String,
    MTZmergedfilename:String,
    MTZunmergedfilename:String,
    SCAmergedfilename:String,
    SCAunmergedfilename:String,
    SplitMerged:bool,
    SplitUnmerged:bool,
    OriginalHKL:bool,
}
fn to_tuple<'de, D>(deserializer: D) -> Result<(i32,i32), Box<dyn std::error::Error>>
where
    D: Deserializer<'de>,
{
	let f = match String::deserialize(deserializer) {
		Ok(x) => x,
		Err(e) => panic!("Error: {:?}",e)
	};
    
	let w = f.split_whitespace().collect::<Vec<&str>>();
    let k: Vec<i32> = w.into_iter()
        .map(|x| x.parse::<i32>())
        .try_collect()?;
    Ok((k[1],k[2]))
}
