use anyhow::Result;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs};
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use html_parser::{Dom, Element};

mod services;

const DECK_NAME: &str = "Refold JP1K v3";
const TARGET_LANGUAGE: &str = "JAPANESE";
const GPT_MODEL: &str = "gpt-4-1106-preview";

#[tokio::main]
async fn main() -> Result<()> {
    let ids = get_reviewed_card_ids().await?;
    let deck_contents = get_reviewed_card_data(ids).await?;
    let words = parse_deck_words(deck_contents);

    println!("reviewed words: {:?}", words);
    println!("");

    let prompt = gen_prompt(words);
    let oai_res = services::oai_chat::chat_raw(prompt, GPT_MODEL).await?;

    println!("{}", oai_res);

    Ok(())
}

async fn get_reviewed_card_ids() -> Result<Vec<i64>> {
    let res: AnkiFindCardsRes = Client::new()
        .post("http://localhost:8765")
        .json(&serde_json::json!({
            "action": "findCards",
            "version": 6,
            "params": {
                "query": format!("deck:\"{}\" is:review", DECK_NAME)
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res.result)
}

async fn get_reviewed_card_data(card_ids: Vec<i64>) -> Result<Vec<AnkiCardInfoResult>> {
    let res: AnkiCardInfoRes = Client::new()
        .post("http://localhost:8765")
        .json(&serde_json::json!({
            "action": "cardsInfo",
            "version": 6,
            "params": {
                "cards": card_ids
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res.result)
}

/// This entire function is hardcoded garbage but it's fine because
/// each deck is different and thus manual parsing like this needs to be done
/// for every deck to extract the words in the target lang
fn parse_deck_words(deck_contents: Vec<AnkiCardInfoResult>) -> Vec<String> {
    deck_contents
        .iter()
        .map(|d| {
            let el = gen_html_element(&d.question).expect("Failed to generate html");
            let mut last_child = el.children.get(0);

            loop {
                match last_child {
                    Some(c) => {
                        let element = c.element().expect("Failed to find element");
                        let classes = element.classes.contains(&"targetWordFront".to_string());

                        if classes {
                            let text = element.children[0].text().expect("Did not find front text");
                            return text.to_string();
                        } else {
                            last_child = Some(&element.children[0]);
                        }
                    },
                    None => unreachable!("Unexpected scenario, should not have reached this")
                }
            }
        })
        .collect::<Vec<String>>()
}

fn gen_html_element(question: &String) -> Result<Element> {
    let first_div = question.find("<div").unwrap();
    let last_div = question.rfind("</div>").unwrap();

    let html = &question[first_div..last_div];
    let html = Dom::parse(html)?;
    let node = html.children.get(0).unwrap();
    let el = node.element().unwrap();

    Ok(el.clone())
}

fn gen_prompt(words: Vec<String>) -> Vec<ChatCompletionRequestMessage> {
    let system_prompt = format!("
        You are an expert in language learning. You are helping a student learn a new language through reading comprehension.
        
        The student is going to pass you a list of words they have learned, you are to use these words to create a short to medium length story.

        You are allowed to use other words when necessary, like particles and other connecting words or phrases. 

        The goal is for the text to largely consist of the given words in order to maximize comprehension.

        Please only respond with text in the given target language.
    ");

    let user_prompt = format!("
        Target Language: {}
        Words: {:?}
    ", TARGET_LANGUAGE, words);

    let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build().expect("Failed to build system request")
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_prompt)
            .build().expect("Failed to build user request")
            .into(),
    ]; 

    messages
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
