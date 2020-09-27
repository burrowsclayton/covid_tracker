

// *** VERSION 3 ***
/*
   https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/arrays-vectors-and-slices.html
*/

extern crate reqwest;
use std::string::String;
//use csv::Error;
//use serde::Deserialize;

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


struct ReportRow {
    date: i32,
    state: String,
    positive: i32,
    positiveIncrease: i32,
    death: i32,
    deathIncrease: i32,
    testIncrease: i32,
    positiveRate: f32,
    positiveRate7Day: f32, // TODO: manually check that this is correct
    testRate7Day: f32,
    deathRate7Day: f32,
    positiveRollingRate7Day: f32,
    pom_positive_rate: f32,
    pom_test_rate: f32,
    pom_death_rate: f32,
}

struct Report{
    report_rows: Vec<ReportRow>,
    rolling_positive: Vec<i32>,
    rolling_total: Vec<i32>,
    rolling_death: Vec<i32>,
    rolling_pct_pos: Vec<f32>,
    lidx: usize,
    uidx: usize,
    total_positive: i32,
    max_positive_rate: f32, 
    max_test_rate: f32, 
    max_death_rate: f32, 
}

impl Default for Report{
    fn default() -> Report{
        Report{
            report_rows: Vec::new(),
            rolling_positive: vec![0, 0, 0, 0, 0, 0, 0],
            rolling_total: vec![0, 0, 0, 0, 0, 0, 0],
            rolling_death:  vec![0, 0, 0, 0, 0, 0, 0],
            rolling_pct_pos: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            lidx: 0,
            uidx: 6,
            total_positive: 0,
            max_positive_rate: 0.0, 
            max_test_rate: 0.0, 
            max_death_rate: 0.0, 
        }
    }
}

impl Report{
    fn compute_positiveRate7Day(&self) -> f32{
        let slice_positive: i32 = self.rolling_positive[self.lidx..self.uidx+1].iter().sum(); 
        let slice_total: i32 = self.rolling_total[self.lidx..self.uidx+1].iter().sum(); 
        let rate = slice_positive as f32 / slice_total as f32;

        //TODO: FIX THIS FUCKY PROCESS!!!
        if self.uidx as u32 >= 45 {
            if rate != 1.0 { return rate } else { return 0.0};
        } else {
            return 0.0
        }
    }

    fn compute_testRate7Day(&self) -> f32{
        let slice_total: i32 = self.rolling_total[self.lidx..self.uidx+1].iter().sum(); 
        slice_total as f32 / 7.0
    }

    fn compute_deathRate7Day(&self) -> f32{
        let slice_death: i32 = self.rolling_death[self.lidx..self.uidx+1].iter().sum(); 
        slice_death as f32 / 7.0
    }

    fn compute_positiveRollingRate7Day(&self) -> f32{
        let slice_pct_pos: f32 = self.rolling_pct_pos[self.lidx..self.uidx+1].iter().sum();
        slice_pct_pos / 7.0
    }

    fn check_max(cur_max: &f32, new_value: &f32) -> f32{
        if new_value > cur_max{
            let max = *new_value;
            return max
        } else {
            let max = *cur_max;
            return max
        }
    }

    fn compute_pom(max_val: &f32, cur_val: &f32) -> f32{
        cur_val / max_val
    }

    fn push(&mut self, r: &row_state){
        let positiveIncrease = r.positiveIncrease.unwrap_or(0);
        let negativeIncrease =  r.negativeIncrease.unwrap_or(0);

        let testIncrease = positiveIncrease + negativeIncrease;

        let positiveRate: f32 = positiveIncrease as f32 / ((testIncrease + 1) as f32);

        let deathIncrease = r.deathIncrease.unwrap_or(0);

        if testIncrease > 0{
            
            // 7-Day Average
            self.rolling_positive.push(positiveIncrease);
            self.rolling_total.push(testIncrease);
            self.rolling_pct_pos.push(positiveRate);
            self.rolling_death.push(deathIncrease);
            self.lidx += 1;
            self.uidx += 1;
            
            let positiveRate7Day = self.compute_positiveRate7Day();
            let testRate7Day = self.compute_testRate7Day();
            let deathRate7Day = self.compute_deathRate7Day();
            let positiveRollingRate7Day = self.compute_positiveRollingRate7Day();

            // Percent of Max Rates
            self.max_positive_rate = Report::check_max(
                &self.max_positive_rate, &positiveRate7Day);

            let pom_positive_rate= Report::compute_pom(
                &self.max_positive_rate, &positiveRate7Day);

            self.max_test_rate = Report::check_max(
                &self.max_test_rate, &testRate7Day);

            let pom_test_rate = Report::compute_pom(
                &self.max_test_rate, &testRate7Day);

            self.max_death_rate = Report::check_max(
                &self.max_death_rate, &deathRate7Day);

            let pom_death_rate = Report::compute_pom(
                &self.max_death_rate, &deathRate7Day);

            self.report_rows.push(ReportRow{
                date: r.date.unwrap_or(0),
                state: r.state.as_ref().unwrap_or(&"".to_string()).clone(),
                positive: r.positive.unwrap_or(0),
                positiveIncrease: r.positiveIncrease.unwrap_or(0),
                death: r.death.unwrap_or(0),
                deathIncrease: deathIncrease,
                testIncrease: testIncrease,
                positiveRate: positiveRate,
                positiveRate7Day: positiveRate7Day,
                testRate7Day: testRate7Day,
                deathRate7Day: deathRate7Day,
                positiveRollingRate7Day: positiveRollingRate7Day,
                pom_positive_rate: pom_positive_rate,
                pom_test_rate: pom_test_rate,
                pom_death_rate: pom_death_rate,
            });
        }

        self.total_positive += r.positive.unwrap_or(0);
    }

    fn print(self){
        for row in self.report_rows{
            let mut line: Vec<String> = Vec::new();

            line.push(row.date.to_string());
            line.push(row.state);
            line.push(row.positive.to_string());
            line.push(row.positiveIncrease.to_string());
            line.push(row.death.to_string());
            line.push(row.deathIncrease.to_string());
            line.push(row.testIncrease.to_string());
            line.push(row.positiveRate.to_string());
            line.push(row.positiveRate7Day.to_string());
            line.push(row.positiveRollingRate7Day.to_string());
            line.push(row.pom_positive_rate.to_string());
            line.push(row.pom_test_rate.to_string());
            line.push(row.pom_death_rate.to_string());
            let joined = line.join(",\t");
            println!("{}", joined);
        }
    }

    fn printf(self){
        let mut cnt = -1;
        for row in self.report_rows{
            cnt += 1;
            if cnt % 20 == 0{
                println!("\n{0: ^10} | {1: ^10} | {2: ^8} | {3: ^8} | {4: ^8} | {5: ^8} | {6: ^10} | {7: ^10} | {8: ^10} | {9: ^10} | {10: ^10} | {11: ^10} | {12: ^10} | {13: ^10} | {14: ^10}",
                "DATE",
                "STATE",
                "POS",
                "+POS",
                "DEATH",
                "+DEATH",
                "+TESTING",
                "POSRT",
                "POSRT7",
                "POSROLL7",
                "TESTRT7",
                "DEATHRT7",
                "POM_POSRT7",
                "POM_TESTRT7",
                "POM_DEATHRT7",
                );
            }

            println!("{0: ^10} | {1: ^10} | {2: >8} | {3: >8} | {4: >8} | {5: >8} | {6: >10} | {7: >10.5} | {8: >10.5} | {9: >10.5} | {10: >10.0} | {11: >10.0} | {12: >10.5} | {13: >10.5} | {14: >10.5}",
                row.date.to_string(),
                row.state,
                row.positive.to_string(),
                row.positiveIncrease.to_string(),
                row.death.to_string(),
                row.deathIncrease.to_string(),
                row.testIncrease.to_string(),
                row.positiveRate,
                row.positiveRate7Day,
                row.positiveRollingRate7Day,
                row.testRate7Day,
                row.deathRate7Day,
                row.pom_positive_rate,
                row.pom_test_rate,
                row.pom_death_rate,
                );
        }
    }
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

    let path = "states/az/daily.json";
    //let path = "states/daily.json";

    match req(path){
        Ok(n) => strng.push_str(&n.to_string()),
        Err(e) => println!("Error: {}", e),
    };

    let mut rows: Vec<row_state> = serde_json::from_str(strng.as_str())?; 

    rows.reverse();

    let mut report = Report{..Default::default()};

    for r in rows.iter() {
        report.push(r);


    //    let mut line: Vec<String> = Vec::new();

    //    if r.positiveIncrease.unwrap_or(0) > 8000{
    //        line.push("*".to_string());
    //    } else {
    //        line.push(" ".to_string());
    //    };
 
    //    line.push(r.date.unwrap_or(0).to_string());
    //    line.push(r.state.as_ref().unwrap_or(&"".to_string()).clone());
    //    line.push(r.positive.unwrap_or(0).to_string());
    //    //line.push(r.negative.unwrap_or(0).to_string());

    //    let testIncrease = r.positiveIncrease.unwrap_or(0) + r.negativeIncrease.unwrap_or(0);
    //    let positiveRate: f32 = r.positiveIncrease.unwrap_or(0) as f32 / (testIncrease + 1) as f32;
    //    line.push(r.positiveIncrease.unwrap_or(0).to_string());
    //    line.push(testIncrease.to_string());
    //    line.push(positiveRate.to_string());
    //    //line.push(r.negativeIncrease.unwrap_or(0).to_string());
    //    line.push(r.death.unwrap_or(0).to_string());
    //    //line.push(r.deathProbable.unwrap_or(0).to_string());
    //    //line.push(r.deathConfirmed.unwrap_or(0).to_string());
    //    line.push(r.deathIncrease.unwrap_or(0).to_string());
    //    let joined = line.join(",\t");
    //    println!("{}", joined);
    }

    report.printf();

    Ok(())
}
