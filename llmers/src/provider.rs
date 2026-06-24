use crate::types::LlmProvider;

pub fn llm_provider_default_url(p: LlmProvider) -> Option<&'static str> {
    match p {
        LlmProvider::OpenAI => Some("https://api.openai.com/v1"),
        LlmProvider::Anthropic => Some("https://api.anthropic.com/v1"),
        LlmProvider::Groq => Some("https://api.groq.com/openai/v1"),
        LlmProvider::Ollama => Some("http://localhost:11434/v1"),
        LlmProvider::Together => Some("https://api.together.xyz/v1"),
        LlmProvider::Mistral => Some("https://api.mistral.ai/v1"),
        LlmProvider::Cohere => Some("https://api.cohere.ai/v1"),
        LlmProvider::Gemini => Some("https://generativelanguage.googleapis.com/v1beta/openai"),
        LlmProvider::DeepSeek => Some("https://api.deepseek.com/v1"),
        LlmProvider::OpenRouter => Some("https://openrouter.ai/api/v1"),
        LlmProvider::Perplexity => Some("https://api.perplexity.ai"),
        LlmProvider::Fireworks => Some("https://api.fireworks.ai/inference/v1"),
        LlmProvider::Vllm => Some("http://localhost:8000/v1"),
        LlmProvider::Custom => None,
    }
}

pub fn llm_provider_uses_openai_compat(p: LlmProvider) -> bool {
    !matches!(p, LlmProvider::Anthropic | LlmProvider::Cohere)
}

pub fn llm_provider_from_string(s: &str) -> LlmProvider {
    match s {
        "openai" => LlmProvider::OpenAI,
        "anthropic" => LlmProvider::Anthropic,
        "groq" => LlmProvider::Groq,
        "ollama" => LlmProvider::Ollama,
        "together" => LlmProvider::Together,
        "mistral" => LlmProvider::Mistral,
        "cohere" => LlmProvider::Cohere,
        "gemini" => LlmProvider::Gemini,
        "deepseek" => LlmProvider::DeepSeek,
        "openrouter" => LlmProvider::OpenRouter,
        "perplexity" => LlmProvider::Perplexity,
        "fireworks" => LlmProvider::Fireworks,
        "vllm" => LlmProvider::Vllm,
        "custom" => LlmProvider::Custom,
        _ => LlmProvider::Custom,
    }
}

pub fn llm_provider_name(p: LlmProvider) -> &'static str {
    match p {
        LlmProvider::OpenAI => "openai",
        LlmProvider::Anthropic => "anthropic",
        LlmProvider::Groq => "groq",
        LlmProvider::Ollama => "ollama",
        LlmProvider::Together => "together",
        LlmProvider::Mistral => "mistral",
        LlmProvider::Cohere => "cohere",
        LlmProvider::Gemini => "gemini",
        LlmProvider::DeepSeek => "deepseek",
        LlmProvider::OpenRouter => "openrouter",
        LlmProvider::Perplexity => "perplexity",
        LlmProvider::Fireworks => "fireworks",
        LlmProvider::Vllm => "vllm",
        LlmProvider::Custom => "custom",
    }
}
