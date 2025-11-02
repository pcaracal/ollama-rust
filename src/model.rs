// https://ollama.readthedocs.io/en/modelfile/#valid-parameters-and-values

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ModelOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirostat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirostat_eta: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirostat_tau: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_last_n: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfs_z: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,
}

#[allow(clippy::doc_markdown)]
impl ModelOptions {
    /// Enable Mirostat sampling for controlling perplexity. (default: 0, 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0)
    #[must_use]
    pub fn mirostat(mut self, mirostat: i32) -> Self {
        self.mirostat = Some(mirostat);
        self
    }

    /// Influences how quickly the algorithm responds to feedback from the generated text. A lower learning rate will result in slower adjustments, while a higher learning rate will make the algorithm more responsive. (Default: 0.1)
    #[must_use]
    pub fn mirostat_eta(mut self, mirostat_eta: f32) -> Self {
        self.mirostat_eta = Some(mirostat_eta);
        self
    }

    /// Controls the balance between coherence and diversity of the output. A lower value will result in more focused and coherent text. (Default: 5.0)
    #[must_use]
    pub fn mirostat_tau(mut self, mirostat_tau: f32) -> Self {
        self.mirostat_tau = Some(mirostat_tau);
        self
    }

    /// Sets the size of the context window used to generate the next token. (Default: 2048)
    #[must_use]
    pub fn num_ctx(mut self, num_ctx: i32) -> Self {
        self.num_ctx = Some(num_ctx);
        self
    }

    /// Sets how far back for the model to look back to prevent repetition. (Default: 64, 0 = disabled, -1 = num_ctx)
    #[must_use]
    pub fn repeat_last_n(mut self, repeat_last_n: i32) -> Self {
        self.repeat_last_n = Some(repeat_last_n);
        self
    }

    /// Sets how strongly to penalize repetitions. A higher value (e.g., 1.5) will penalize repetitions more strongly, while a lower value (e.g., 0.9) will be more lenient. (Default: 1.1)
    #[must_use]
    pub fn repeat_penalty(mut self, repeat_penalty: f32) -> Self {
        self.repeat_penalty = Some(repeat_penalty);
        self
    }

    /// The temperature of the model. Increasing the temperature will make the model answer more creatively. (Default: 0.8)
    #[must_use]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the random number seed to use for generation. Setting this to a specific number will make the model generate the same text for the same prompt. (Default: 0)
    #[must_use]
    pub fn seed(mut self, seed: i32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the stop sequences to use. When this pattern is encountered the LLM will stop generating text and return. Multiple stop patterns may be set by specifying multiple separate stop parameters in a modelfile.
    #[must_use]
    pub fn stop(mut self, stops: Vec<String>) -> Self {
        self.stop = Some(stops);
        self
    }

    /// Tail free sampling is used to reduce the impact of less probable tokens from the output. A higher value (e.g., 2.0) will reduce the impact more, while a value of 1.0 disables this setting. (default: 1)
    #[must_use]
    pub fn tfs_z(mut self, tfs_z: f32) -> Self {
        self.tfs_z = Some(tfs_z);
        self
    }

    /// Maximum number of tokens to predict when generating text. (Default: 128, -1 = infinite generation, -2 = fill context)
    #[must_use]
    pub fn num_predict(mut self, num_predict: i32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    /// Reduces the probability of generating nonsense. A higher value (e.g. 100) will give more diverse answers, while a lower value (e.g. 10) will be more conservative. (Default: 40)
    #[must_use]
    pub fn top_k(mut self, top_k: i32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Works together with top-k. A higher value (e.g., 0.95) will lead to more diverse text, while a lower value (e.g., 0.5) will generate more focused and conservative text. (Default: 0.9)
    #[must_use]
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Alternative to the top_p, and aims to ensure a balance of quality and variety. The parameter p represents the minimum probability for a token to be considered, relative to the probability of the most likely token. For example, with p=0.05 and the most likely token having a probability of 0.9, logits with a value less than 0.045 are filtered out. (Default: 0.0)
    #[must_use]
    pub fn min_p(mut self, min_p: f32) -> Self {
        self.min_p = Some(min_p);
        self
    }
}
