use serde::Serialize;
use tag::tag::Tag;

#[derive(Debug, Serialize)]
pub struct ListTagReply {
    pub data: Vec<Tag>,
}

#[derive(Debug, Serialize)]
pub struct CreateTagReply {}

#[derive(Debug, Serialize)]
pub struct ModifyTagReply {}

#[derive(Debug, Serialize)]
pub struct RemoveTagReply {}
