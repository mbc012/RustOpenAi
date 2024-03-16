use crate::common::Identifiable;
use crate::networking::Networking;
use serde::Serialize;

pub struct FineTuningJob {
    id: String,
    created_at: i64,
    error: Option<FineTuningError>,
    fine_tuned_model: Option<String>,
    finished_at: Option<i64>,
    hyperparameters: HyperParams,
    model: String,
    object: String,
    organization_id: String,
    result_files: Vec<String>,
    status: String,
    trained_tokens: Option<u32>,
    training_file: String,
    validation_file: Option<String>,
}

#[derive(Default, Serialize)]
pub struct FineTuningJobBuilder {
    model: String,
    training_file: String,
    hyperparameters: HyperParams,
    suffix: Option<String>,
    validation_file: Option<String>,
}

impl FineTuningJobBuilder {
    pub fn new<M: Identifiable, TF: Identifiable>(model: M, training_file_id: TF) -> Self {
        Self {
            model,
            training_file: training_file_id,
            ..Self::default()
        }
    }

    pub fn with_hyperparams(mut self, hyper_params: HyperParams) -> Self {
        self.hyperparameters = hyper_params;
        self
    }

    pub fn with_suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn with_validation_file(mut self, validation_file: String) -> Self {
        self.validation_file = Some(validation_file);
        self
    }

    pub fn build(&self, networking: &Networking) -> FineTuningJob {
        todo!()
    }
}

pub struct FineTuningJobEvent {
    id: String,
    created_at: i64,
    level: String,
    message: String,
    object: String,
}

pub struct FineTuningError {
    code: String,
    message: String,
    param: Option<String>,
}

pub struct HyperParams {
    n_epochs: Option<String>,
    batch_size: Option<String>,
    learning_rate_multiplier: Option<String>,
}
