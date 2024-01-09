use anyhow::{Result, anyhow, bail};

use async_openai::{
    types::{
        CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    },
    Client,
};


pub async fn chat_raw(mut messages: Vec<ChatCompletionRequestMessage>, model: &str) -> Result<String> {
    let mut retry_count = 0;
    let max_retries = 5;

    while retry_count < max_retries {
        match inner_chat_raw(&messages, model).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                eprintln!("Raw Chat failure | error: {}. Retrying... ({}/{})", e, retry_count + 1, max_retries);

                // only want to system modify on first failure and not always
                if retry_count == 0 {
                    let system_prompt = messages.first()
                        .ok_or_else(|| anyhow!("Failed to find system prompt when generating chat completion in error retry"))?
                        .clone();

                    let new_system_prompt = format!("
                        You are being invoked as a result of a previous inference failure. Please review the system prompt carefully and response accurately.
                        
                        {:?}
                    ", system_prompt);

                    let new_system_prompt = ChatCompletionRequestSystemMessage {
                        content: Some(new_system_prompt),
                        role: async_openai::types::Role::System,
                        name: None
                    };

                    let message = ChatCompletionRequestMessage::System(new_system_prompt);

                    messages[0] = message;
                }
                retry_count += 1;
            }
        }
    }

    bail!("Failed to generate chat completion, error. Retried: {} times, giving up", retry_count);
}

async fn inner_chat_raw(messages: &Vec<ChatCompletionRequestMessage>, model: &str) -> Result<String> {
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .stream(false)
        .model(model)
        .temperature(0.2)
        .messages(messages.clone())
        .build()?;

    Ok(client.chat().create(request).await?
        .choices
        .first()
        .ok_or_else(|| anyhow!("First option missing from OAI prompt return"))?
        .message
        .content
        .clone()
        .ok_or_else(|| anyhow!("Content missing from OAI prompt message"))?)
}
