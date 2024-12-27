use serde::{Deserialize, Serialize};
use statistic::statistic::{Statistic, StatisticCategory};

#[derive(Debug, Serialize)]
pub struct ListStatisticReply {
    pub data: Vec<Statistic>,
}

#[derive(Debug, Serialize)]
pub struct GetStatisticReply {
    pub data: Option<StatisticCategory>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStatisticRequest {
    pub name: String,
    pub categories: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateStatisticReply {}

#[derive(Debug, Deserialize)]
pub struct ModifyStatisticRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ModifyStatisticReply {}

#[derive(Debug, Serialize)]
pub struct RemoveStatisticReply {}
