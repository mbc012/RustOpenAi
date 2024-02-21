use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Moderation {
    id: String,
    model: String,
    results: Vec<ModerationRecord>,
}

impl Moderation {
    pub fn is_flagged(&self, idx: usize) -> bool {
        self.results[idx].is_flagged()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModerationRecord {
    flagged: bool,
    categories: ModerationCategories,
    category_scores: ModerationScores,
}

impl ModerationRecord {
    pub fn is_flagged(&self) -> bool {
        return self.flagged;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModerationCategories {
    sexual: bool,
    hate: bool,
    harassment: bool,
    self_harm: bool,
    #[serde(rename = "sexual/minors")]
    sexual_minors: bool,
    #[serde(rename = "hate/threatening")]
    hate_threatening: bool,
    #[serde(rename = "violence/graphic")]
    violence_graphic: bool,
    #[serde(rename = "self-harm/intent")]
    self_harm_intent: bool,
    #[serde(rename = "self-harm/instructions")]
    self_harm_instructions: bool,
    #[serde(rename = "harassment/threatening")]
    harassment_threatening: bool,
    violence: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModerationScores {
    sexual: f64,
    hate: f64,
    harassment: f64,
    self_harm: f64,
    #[serde(rename = "sexual/minors")]
    sexual_minors: f64,
    #[serde(rename = "hate/threatening")]
    hate_threatening: f64,
    #[serde(rename = "violence/graphic")]
    violence_graphic: f64,
    #[serde(rename = "self-harm/intent")]
    self_harm_intent: f64,
    #[serde(rename = "self-harm/instructions")]
    self_harm_instructions: f64,
    #[serde(rename = "harassment/threatening")]
    harassment_threatening: f64,
    violence: f64,
}
