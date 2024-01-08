use anyhow::Result;
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use html_parser::Dom;

#[tokio::main]
async fn main() -> Result<()> {
    let ids = get_reviewed_card_ids().await?;
    get_reviewed_card_data(ids).await?;

    Ok(())
}

async fn get_reviewed_card_ids() -> Result<Vec<i64>> {
    let res: AnkiFindCardsRes = Client::new()
        .post("http://localhost:8765")
        .json(&serde_json::json!({
            "action": "findCards",
            "version": 6,
            "params": {
                "query": format!("deck:\"{}\" is:review", "Refold JP1K v3")
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res.result)
}

async fn get_reviewed_card_data(card_ids: Vec<i64>) -> Result<()> {
    let res: AnkiCardInfoRes = Client::new()
        .post("http://localhost:8765")
        .json(&serde_json::json!({
            "action": "cardsInfo",
            "version": 6,
            "params": {
                "cards": [1634251759782i64]
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    let first_div = res.result[0].question.find("<div").unwrap();
    let last_div = res.result[0].question.rfind("</div>").unwrap();

    println!("first: {}", first_div);
    println!("last: {}", last_div);

    let html = &res.result[0].question[first_div..last_div];
    let html = Dom::parse(html)?;
    let node = html.children.get(0).expect("Could not find first child");
    let el = node.element().expect("Could not find node element");

    let mut children = true;
    let mut children_idx = 0;

    while children {
        match el.children.get(children_idx) {
            Some(c) => {
                 
            },
            None => {
                children = false;
            }
        }

    }

    // println!("values: {:?}", &res.result[0].question[first_div..last_div]);
    // println!("values: {:?}", &res.result[0].question);

    Ok(())
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnkiFindCardsRes {
    pub result: Vec<i64>,
    pub error: Value,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnkiCardInfoRes {
    pub result: Vec<AnkiCardInfoResult>,
    pub error: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnkiCardInfoResult {
    pub answer: String,
    pub question: String,
    pub deck_name: String,
    pub model_name: String,
    pub field_order: i64,
    pub css: String,
    pub card_id: i64,
    pub interval: i64,
    pub note: i64,
    pub ord: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub queue: i64,
    pub due: i64,
    pub reps: i64,
    pub lapses: i64,
    pub left: i64,
    #[serde(rename = "mod")]
    pub mod_field: Option<i64>,
}
