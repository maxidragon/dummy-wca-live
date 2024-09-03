use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnterAttemptOptions {
    pub competition_wca_id: String,
    pub event_id: String,
    pub round_number: i32,
    pub registrant_id: i32,
    pub attempt_number: i32,
    pub attempt_result: i32,
}
  
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnterResultsOptions {
    pub competition_wca_id: String,
    pub event_id: String,
    pub round_number: i32,
    pub results: Vec<Result>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub registrant_id: i32,
    pub attempts: Vec<Attempt>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
    pub result: i32,
}
