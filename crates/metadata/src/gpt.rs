use async_openai::types::{
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestSystemMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use anyhow::Result;
use common::BookMetadata;

pub async fn infer_metadata(text: &str) -> Result<BookMetadata> {
    let system_prompt = "You are a PDF metadata extraction agent. Extract the following metadata fields:
- Title
- Authors (as a list)
- ISBN (if available)
Return your result in strict JSON format.";

    let user_prompt = format!("PDF text:\n\"\"\"\n{}\n\"\"\"", text);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_prompt)
                .build()?
                .into(),
        ])
        .max_tokens(400u32)
        .temperature(0.3)
        .build()?;

    let client = Client::new();
    let response = client.chat().create(request).await?;

    let raw = response.choices[0]
        .message
        .content
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No response from GPT"))?;

    let metadata: BookMetadata = serde_json::from_str(raw)?;
    Ok(metadata)
}
