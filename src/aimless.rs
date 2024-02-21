#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Aimless {
	#[serde(rename = "@version")]
	version: String,
	
	#[serde(rename = "@RunTime")]
	RunTime: String,

	ReflectionData: ReflectionData,
    ReflectionFile: ReflectionFile,
    ScaleModelFail: ScaleModelFail,
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

#[derive(Deserialize, Debug)]
pub struct ScaleModelFail {
    #[serde(rename="@class")]
    class:String,
    #[serde(rename="$value")]
    _value:Vec<String>,
}
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


#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct Dataset {
    Wavelength : f64,
    Run: Run,
	#[serde(rename = "@name")]
    name: String
}
#[derive(Deserialize, Debug)]
pub struct Run {
    number: i32,
    BatchOffset: i32,
	#[serde(deserialize_with="to_tuple")]
    BatchRange:(i32,i32),
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
    Main: OutlierMain,
    Anom: OutlierAnom,
    EmaxTest: OutlierEmaxtest,
}

#[derive(Deserialize, Debug)]
pub struct OutlierMain {
    SDrej: f64,
    SDrej2: f64,
    Reject2policy:String,
}
#[derive(Deserialize, Debug)]
pub struct OutlierAnom {
    #[serde(rename="@dataset")]
    dataset: String,
    SDrej: f64,
    SDrej2: f64,
    Reject2policy:String,
}
#[derive(Deserialize, Debug)]
pub struct OutlierEmaxtest {
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
    ResolutionLow(ShellResults),
    ResolutionHigh(ShellResults),
    Rmerge(ShellResults),
    RmergeOverall(ShellResults),
    Rmeas(ShellResults),
    RmeasOverall(ShellResults),
    Rpim(ShellResults),
    RpimOverall(ShellResults),
    RmergeTopI(String),
    NumberObservations(ShellResults),
    NumberReflections(ShellResults),
    MeanIoverSD(ShellResults),
    CChalf(ShellResults),
    Completeness(ShellResults),
    Multiplicity(ShellResults),
    MeanChiSq(ShellResults),
    AnomalousCompleteness(ShellResults),
    AnomalousMultiplicity(ShellResults),
    AnomalousCChalf(ShellResults),
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
struct ShellResults {
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
fn to_tuple<'de, D>(deserializer: D) -> Result<(i32,i32), D::Error>
where
    D: Deserializer<'de>,
{
	let f = match String::deserialize(deserializer) {
		Ok(x) => x,
		Err(e) => panic!("Error: {:?}",e)
	};
    
	let w = f.split_whitespace().collect::<Vec<&str>>();
    let k: Vec<i32> = w.into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    Ok((k[0],k[1]))
}
