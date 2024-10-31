use serde::Serialize;
use tag::tag::Tag;

#[derive(Debug, Serialize)]
pub struct ListTagReply {
    pub data: Vec<Tag>,
}
