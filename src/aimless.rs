#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Aimless {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@RunTime")]
    RunTime: String,
    #[serde(rename = "$value")]
    pub fields: Vec<AimlessFields>,
}

#[derive(Deserialize, Debug)]
pub enum AimlessFields {
    ReflectionData(ReflectionData),
    ReflectionFile(ReflectionFile),
    ScaleModelFail(ScaleModelFail),
    OnlyMerge {},
    RotationalOverlap(RotationalOverlap),
    OutlierControl(OutlierControl),
    BatchGroupMessage(String),
    BatchGroupWidth(f64),
    BatchGroupNumber(i32),
    Outliers(Outliers),
    CCP4Table(CCP4Table),
    AnomalousStatus(String),
    Result(AimlessResult),
    OutputFiles(OutputFiles),
    #[serde(other)]
    etc,
}

#[derive(Deserialize, Debug)]
pub struct ReflectionFile {
    #[serde(rename = "@stream")]
    stream: String,
    #[serde(rename = "@name")]
    name: String,
    cell: Cell,
    SpacegroupName: String,
}
#[derive(Deserialize, Debug)]
pub struct Cell {
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

#[derive(Deserialize, Debug)]
pub struct ScaleModelFail {
    #[serde(rename = "@class")]
    class: String,
    #[serde(rename = "$value")]
    value: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct RotationalOverlap {
    EnoughOverlap: bool,
    AverageOverlap: f64,
    Overlapthreshold: f64,
    AllowedGap: f64,
    MinimumOverlap: f64,
}

#[derive(Deserialize, Debug)]
pub struct CCP4Table {
    #[serde(rename = "@title", default)]
    title: Option<String>,
    #[serde(rename = "@groupID", default)]
    groupID: Option<String>,
    #[serde(rename = "@id", default)]
    id: Option<String>,
    #[serde(rename = "$value")]
    fields: Vec<CCP4Choice>,
}
#[derive(Deserialize, Debug)]
pub enum CCP4Choice {
    plot(Plot),
    headers(Headers),
    #[serde(deserialize_with = "to_vecvecf64")]
    data(Vec<Vec<f64>>),
}

#[derive(Deserialize, Debug)]
pub struct Headers {
    #[serde(rename = "@separator")]
    separator: Option<String>,
    #[serde(rename = "$value")]
    _value: String,
}
#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "$value")]
    _value: String,
}
#[derive(Deserialize, Debug)]
pub struct Plot {
    #[serde(rename = "$value")]
    ploptions: Vec<Ploptions>,
}

#[derive(Deserialize, Debug)]
pub enum Ploptions {
    title(String),
    xrange {
        #[serde(rename = "@min")]
        min: String,
        #[serde(rename = "@max")]
        max: String,
    },
    yrange {
        #[serde(rename = "@min")]
        min: String,
        #[serde(rename = "@max")]
        max: String,
    },
    description(String),
    fixaspectratio(bool),
    xlabel(String),
    ylabel(String),
    showlegend(bool),
    legendposition(LegendPosition),
    plotline(Plotline),
    line(Line),
    circle(Circle),
    xintegral(bool),
    xscale(String),
}

#[derive(Deserialize, Debug)]
pub struct Xrange {
    #[serde(rename = "@min")]
    min: String,
    #[serde(rename = "@max")]
    max: String,
}
#[derive(Deserialize, Debug)]
pub struct Yrange {
    #[serde(rename = "@min")]
    min: String,
    #[serde(rename = "@max")]
    max: String,
}
#[derive(Deserialize, Debug)]
pub struct LegendPosition {
    #[serde(rename = "@x")]
    x: String,
    #[serde(rename = "@y")]
    y: String,
}
#[derive(Deserialize, Debug)]
pub struct Plotline {
    #[serde(rename = "@dataid")]
    dataid: Option<String>,
    #[serde(rename = "@xcol")]
    xcol: Option<String>,
    #[serde(rename = "@ycol")]
    ycol: Option<String>,
    label: Option<String>,
    symbolsize: Option<String>,
    colour: Option<String>,
    markeredgewidth: Option<String>,
    linestyle: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Line {
    #[serde(rename = "@x1", deserialize_with = "to_f64")]
    x1: f64,
    #[serde(rename = "@y1", deserialize_with = "to_f64")]
    y1: f64,
    #[serde(rename = "@x2", deserialize_with = "to_f64")]
    x2: f64,
    #[serde(rename = "@y2", deserialize_with = "to_f64")]
    y2: f64,
    #[serde(rename = "@linestyle")]
    linestyle: String,
    #[serde(rename = "@linecolour")]
    linecolour: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Circle {
    #[serde(rename = "@xpos", deserialize_with = "to_f64")]
    xpos: f64,
    #[serde(rename = "@ypos", deserialize_with = "to_f64")]
    ypos: f64,
    #[serde(rename = "@radius", deserialize_with = "to_f64")]
    radius: f64,
    #[serde(rename = "@linecolour")]
    linecolour: String,
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
    Wavelength: f64,
    Run: Run,
    #[serde(rename = "@name")]
    name: String,
}
#[derive(Deserialize, Debug)]
pub struct Run {
    number: i32,
    BatchOffset: i32,
    #[serde(deserialize_with = "to_tuple")]
    BatchRange: (i32, i32),
    FileStream: String,
}
#[derive(Deserialize, Debug)]
pub struct Outliers {
    RejectNumberUnique: i32,
    RejectNumberFriedel: i32,
    RejectNumberEmax: i32,
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
    Reject2policy: String,
}
#[derive(Deserialize, Debug)]
pub struct OutlierAnom {
    #[serde(rename = "@dataset")]
    dataset: String,
    SDrej: f64,
    SDrej2: f64,
    Reject2policy: String,
}
#[derive(Deserialize, Debug)]
pub struct OutlierEmaxtest {
    EmaxAcentric: f64,
    EmaxCentric: f64,
}

#[derive(Deserialize, Debug)]
pub struct AimlessResult {
    Dataset: DatasetResult,
}

#[derive(Deserialize, Debug)]
pub struct DatasetResult {
    #[serde(rename = "@name")]
    name: String,
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
    #[serde(rename = "@type")]
    esttype: String,
    Direction: String,
    Threshold: f64,
    MaximumResolution: f64,
    Message: String,
}

#[derive(Deserialize, Debug)]
pub struct ResolutionLimitEstimate {
    #[serde(rename = "@type")]
    esttype: String,
    Direction: String,
    Threshold: f64,
    MaximumResolution: f64,
    Message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ShellResults {
    Overall: f64,
    Inner: f64,
    Outer: f64,
}
#[derive(Deserialize, Debug)]
pub struct OutputFiles {
    OutputType: String,
    SCAOutputType: String,
    MTZmergedfilename: String,
    MTZunmergedfilename: String,
    SCAmergedfilename: String,
    SCAunmergedfilename: String,
    SplitMerged: bool,
    SplitUnmerged: bool,
    OriginalHKL: bool,
}
fn to_tuple<'de, D>(deserializer: D) -> Result<(i32, i32), D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let f = match String::deserialize(deserializer) {
        Ok(x) => x,
        Err(e) => panic!("Error: {:?}", e),
    };

    let w = f.split_whitespace().collect::<Vec<&str>>();
    let k: Vec<i32> = w
        .into_iter()
        .map(|x| x.parse::<i32>().map_err(D::Error::custom))
        .flatten()
        .collect::<Vec<i32>>();
    Ok((k[0], k[1]))
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let f = match String::deserialize(deserializer) {
        Ok(x) => x,
        Err(e) => panic!("Error: {:?}", e),
    };
    f.trim().parse::<f64>().map_err(Error::custom)
}

fn to_vecvecf64<'de, D>(deserializer: D) -> Result<Vec<Vec<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let datastr = String::deserialize(deserializer)?;
    let dataf64 = datastr
        .split('\n')
        .map(|f| {
            f.split_whitespace()
                .map(|g| fast_float::parse(g))
                .flatten()
                .collect()
        })
        .collect::<Vec<Vec<f64>>>();

    Ok(dataf64)
}
