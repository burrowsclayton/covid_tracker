

// *** VERSION 3 ***

extern crate reqwest;
use std::string::String;

use rusqlite::{params, Connection, Result}

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;


#[tokio::main]
async fn req(path: &str) -> Result<String, Box<dyn std::error::Error>>{

    let url = format!("https://api.covidtracking.com/v1/{}", path);

    println!("{}", url);

    let content = reqwest::get(&url)
        .await?
        .text()
        .await?;

    Ok(content)
}

#[derive(Serialize, Deserialize, Debug)]
struct row{
    date: i32, // 20200924,
    states: Option<i32>, // 56,
    positive: Option<i32>, // 6941911,
    negative: Option<i32>, // 88239824,
    pending: Option<i32>, // 12008,
    hospitalizedCurrently: Option<i32>, // 30043,
    hospitalizedCumulative: Option<i32>, // 400840,
    inIcuCurrently: Option<i32>, // 6158,
    inIcuCumulative: Option<i32>, // 19555,
    onVentilatorCurrently: Option<i32>, // 1560,
    onVentilatorCumulative: Option<i32>, // 2177,
    recovered: Option<i32>, // 2710183,
    dateChecked: Option<String>, // "2020-09-24T00:00:00Z",
    death: Option<i32>, // 194852,
    hospitalized: Option<i32>, // 400840
    totalTestResults: Option<i32>, // 98481026
    lastModified: String, // "2020-09-24T00:00:00Z"
    total: Option<i32>, // 0
    posNeg: Option<i32>, // 0
    deathIncrease: Option<i32>, // 921
    hospitalizedIncrease: Option<i32>, // 1588
    negativeIncrease: Option<i32>, // 823449
    positiveIncrease: Option<i32>, // 43772
    totalTestResultsIncrease: Option<i32>, // 940415
    hash: String, // "4d38b04e3fb5e1a355ed84a12e5dccde2a6dce9a"},
}

#[derive(Serialize, Deserialize, Debug)]
struct row_state{
    date: Option<i32>, //20200924,
    state: Option<String>, //"CA",
    positive: Option<i32>, //790640,
    negative: Option<i32>, //13162217,
    pending: Option<i32>, //null,
    totalTestResults: Option<i32>, //13952857,
    hospitalizedCurrently: Option<i32>, //3484,
    hospitalizedCumulative: Option<i32>, //null,
    inIcuCurrently: Option<i32>, //914,
    inIcuCumulative: Option<i32>, //null,
    onVentilatorCurrently: Option<i32>, //null,
    onVentilatorCumulative: Option<i32>, //null,
    recovered: Option<i32>, //null,
    dataQualityGrade: Option<String>, //"B",
    lastUpdateEt: Option<String>, //"9/24/2020 02:59",
    dateModified: Option<String>, //"2020-09-24T02:59:00Z",
    checkTimeEt: Option<String>, //"09/23 22:59",
    death: Option<i32>, //15314,
    hospitalized: Option<i32>, //null,
    dateChecked: Option<String>, //"2020-09-24T02:59:00Z",
    totalTestsViral: Option<i32>, //13952857,
    positiveTestsViral: Option<i32>, //null,
    negativeTestsViral: Option<i32>, //null,
    positiveCasesViral: Option<i32>, //790640,
    deathConfirmed: Option<i32>, //null,
    deathProbable: Option<i32>, //null,
    totalTestEncountersViral: Option<i32>, //null,
    totalTestsPeopleViral: Option<i32>, //null,
    totalTestsAntibody: Option<i32>, //null,
    positiveTestsAntibody: Option<i32>, //null,
    negativeTestsAntibody: Option<i32>, //null,
    totalTestsPeopleAntibody: Option<i32>, //null,
    positiveTestsPeopleAntibody: Option<i32>, //null,
    negativeTestsPeopleAntibody: Option<i32>, //null,
    totalTestsPeopleAntigen: Option<i32>, //null,
    positiveTestsPeopleAntigen: Option<i32>, //null,
    totalTestsAntigen: Option<i32>, //null,
    positiveTestsAntigen: Option<i32>, //null,
    fips: Option<String>, //"06",
    positiveIncrease: Option<i32>, //3170,
    negativeIncrease: Option<i32>, //70264,
    total: Option<i32>, //13952857,
    totalTestResultsSource: Option<String>, //"posNeg",
    totalTestResultsIncrease: Option<i32>, //73434,
    posNeg: Option<i32>, //13952857,
    deathIncrease: Option<i32>, //110,
    hospitalizedIncrease: Option<i32>, //0,
    hash: Option<String>, //"87788642c4d9f1deb4ce4c35adb758e1c0e423f1",
    commercialScore: Option<i32>, //0,
    negativeRegularScore: Option<i32>, //0,
    negativeScore: Option<i32>, //0,
    positiveScore: Option<i32>, //0,
    score: Option<i32>, //0,
    grade: Option<String>, //""},
}

fn main() -> serde_json::Result<()> {
    let mut strng = "".to_string();

    // whole country
    let path = "us/daily.json";

    //match req(path){
    //    Ok(n) => strng.push_str(&n.to_string()),
    //    Err(e) => println!("Error: {}", e),
    //};

    //let rows: Vec<row> = serde_json::from_str(strng.as_str())?; 

    //for r in rows.iter() {
    //    //println!("{:?}", elem);
    //    println!("{:?},\t{:?}, \t{:?}, \t{:?}, \t{:?}", 
    //             r.date, 
    //             r.positive.unwrap_or(0),
    //             r.death.unwrap_or(0),
    //             r.totalTestResults.unwrap_or(0),
    //             r.totalTestResultsIncrease.unwrap_or(0)
    //             )
    //}


    /* state of <state> */

    let path = "states/ga/daily.json";
    let path = "states/daily.json";

    match req(path){
        Ok(n) => strng.push_str(&n.to_string()),
        Err(e) => println!("Error: {}", e),
    };

    let mut rows: Vec<row_state> = serde_json::from_str(strng.as_str())?; 

    rows.reverse();

    for r in rows.iter() {
        let mut line: Vec<String> = Vec::new();
        line.push(r.date.unwrap_or(0).to_string());
        line.push(r.state.as_ref().unwrap_or(&"".to_string()).clone());
        line.push(r.positive.unwrap_or(0).to_string());
        line.push(r.negative.unwrap_or(0).to_string());
        line.push(r.positiveIncrease.unwrap_or(0).to_string());
        line.push(r.negativeIncrease.unwrap_or(0).to_string());
        line.push(r.death.unwrap_or(0).to_string());
        line.push(r.deathProbable.unwrap_or(0).to_string());
        line.push(r.deathConfirmed.unwrap_or(0).to_string());
        let joined = line.join(",\t");
        println!("{}", joined);
    }

    Ok(())
}
