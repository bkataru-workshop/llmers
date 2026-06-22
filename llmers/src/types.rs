use std::fmt;

use crate::LlmError;

// -- Provider
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    Groq,
    Ollama,
    Together,
    Mistral,
    Cohere,
    Gemini,
    DeepSeek,
    OpenRouter,
    Perplexity,
    Fireworks,
    Vllm,
    Custom,
}

// -- Role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmRole {
    System,
    User,
    Assistant,
    Tool,
}

// -- Finish reason
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmFinishReason {
    Stop,
    Length,
    ToolCall,
    ContentFilter,
    Error,
    Unknown,
}

// -- Tool types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmToolType {
    Function,
}

#[derive(Debug, Clone)]
pub struct LlmHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct LlmFunctionDef {
    pub name: String,
    pub description: String,
    pub parameters_json: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LlmToolDef {
    pub tool_type: LlmToolType, // `type` is a keyword in Rust
    pub function: LlmFunctionDef,
}

#[derive(Debug, Clone)]
pub struct LlmToolCall {
    pub id: Option<String>,
    pub name: Option<String>,
    pub arguments_json: Option<String>,
}

// -- Message
#[derive(Debug, Clone)]
pub struct LlmMessage {
    pub role: LlmRole,
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Vec<LlmToolCall>,
}

// -- Usage / Stats
#[derive(Debug, Clone, Default)]
pub struct LlmUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub provided: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LlmStats {
    pub latency_ms: f64,
    pub time_to_first_token_ms: f64,
    pub retries: i32,
    pub stream_chunks: i32,
    pub has_time_to_first_token: bool,
}

// -- Response
#[derive(Debug)]
pub struct LlmResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub content: Option<String>,
    pub tool_calls: Vec<LlmToolCall>,
    pub finish_reason: LlmFinishReason,
    pub usage: LlmUsage,
    pub stats: LlmStats,
    pub error: LlmError,
    pub error_message: Option<String>,
    pub http_status: i32,
}

// -- Callbacks
pub type LlmStreamCallback = Box<dyn FnMut(&str, bool) + Send>;
pub type LlmAsyncCallback = Box<dyn FnOnce(Box<LlmResponse>) + Send>;

// -- Client
#[derive(Debug, Clone)]
pub struct LlmClient {
    pub provider: LlmProvider,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub timeout_seconds: u64,
    pub max_retries: i32,
    pub retry_delay_ms: i32,
    pub retry_delay_max_ms: i32,
    pub retry_backoff_multiplier: f64,
    pub retry_on_rate_limit: bool,
    pub extra_headers: Vec<LlmHeader>,
    pub verbosity: i32,
    pub verify_ssl: bool,
    pub proxy: Option<String>,
    pub org_id: Option<String>,
    pub project_id: Option<String>,
}

// -- Request
pub struct LlmRequest {
    pub messages: Vec<LlmMessage>,
    pub model: Option<String>,
    pub max_tokens: i32,
    pub temperature: f64,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub top_k: i32,
    pub stop: Vec<String>,
    pub stream: bool,
    pub stream_callback: Option<LlmStreamCallback>,
    pub tools: Vec<LlmToolDef>,
    pub tool_choice: Option<String>,
    pub system: Option<String>,
    pub min_p: f64,
    pub json_mode: bool,
    pub json_schema: Option<String>,
    pub seed: Option<u64>,
}

impl std::fmt::Debug for LlmRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlmRequest")
            .field("messages", &self.messages)
            .field("model", &self.model)
            .field("max_tokens", &self.max_tokens)
            .field("temperature", &self.temperature)
            .field("top_p", &self.top_p)
            .field("frequency_penalty", &self.frequency_penalty)
            .field("presence_penalty", &self.presence_penalty)
            .field("top_k", &self.top_k)
            .field("stop", &self.stop)
            .field("stream", &self.stream)
            .field("stream_callback", &"<callback>") // placeholder for the dyn fn pointer
            .field("tools", &self.tools)
            .field("tool_choice", &self.tool_choice)
            .field("system", &self.system)
            .field("min_p", &self.min_p)
            .field("json_mode", &self.json_mode)
            .field("json_schema", &self.json_schema)
            .field("seed", &self.seed)
            .finish()
    }
}

// -- Constants
pub const LLM_DEFAULT_TIMEOUT: u64 = 30;
pub const LLM_DEFAULT_MAX_RETRIES: i32 = 3;
pub const LLM_DEFAULT_RETRY_DELAY_MS: i32 = 1000;
pub const LLM_DEFAULT_MAX_TOKENS: i32 = 4096;
pub const LLM_DEFAULT_TEMPERATURE: f64 = 1.0;

impl Default for LlmClient {
    fn default() -> Self {
        Self {
            provider: LlmProvider::OpenAI,
            api_key: None,
            base_url: None,
            model: None,
            timeout_seconds: LLM_DEFAULT_TIMEOUT,
            max_retries: LLM_DEFAULT_MAX_RETRIES,
            retry_delay_ms: LLM_DEFAULT_RETRY_DELAY_MS,
            retry_delay_max_ms: 30000,
            retry_backoff_multiplier: 2.0,
            retry_on_rate_limit: true,
            extra_headers: Vec::new(),
            verbosity: 0,
            verify_ssl: true,
            proxy: None,
            org_id: None,
            project_id: None,
        }
    }
}
