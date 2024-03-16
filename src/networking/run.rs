use crate::common::ApiList;
use crate::error::OpenApiError;
use crate::networking::Networking;
use crate::run::{Run, RunStep};
use reqwest::Method;
use serde::Serialize;

pub trait RunActions {
    fn create_run<S: Serialize>(
        &self,
        payload: &S,
        thread_id: &Option<String>,
    ) -> Result<Run, OpenApiError>;
    fn retrieve_run(&self, thread_id: String, run_id: String) -> Result<Run, OpenApiError>;
    fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStep, OpenApiError>;
    fn list_runs(&self, thread_id: String) -> Result<ApiList<Run>, OpenApiError>;
    fn list_run_steps(
        &self,
        thread_id: String,
        run_id: String,
    ) -> Result<ApiList<RunStep>, OpenApiError>;
    fn cancel_run(&self, thread_id: String, run_id: String) -> Result<Run, OpenApiError>;
}

impl RunActions for Networking {
    fn create_run<S: Serialize>(
        &self,
        payload: &S,
        thread_id: &Option<String>,
    ) -> Result<Run, OpenApiError> {
        let endpoint: String = match thread_id {
            Some(tid) => format!("threads/{}/runs", tid),
            None => String::from("threads/runs"),
        };
        self.send_and_convert(
            Method::POST,
            endpoint,
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    fn retrieve_run(&self, thread_id: String, run_id: String) -> Result<Run, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs/{1}", thread_id, run_id),
            None,
            None,
        )
    }

    fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStep, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs/{1}/steps/{2}", thread_id, run_id, step_id),
            None,
            None,
        )
    }

    fn list_runs(&self, thread_id: String) -> Result<ApiList<Run>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs", thread_id),
            None,
            None,
        )
    }

    fn list_run_steps(
        &self,
        thread_id: String,
        run_id: String,
    ) -> Result<ApiList<RunStep>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs/{1}/steps", thread_id, run_id),
            None,
            None,
        )
    }

    fn cancel_run(&self, thread_id: String, run_id: String) -> Result<Run, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            format!("threads/{0}/runs/{1}/cancel", thread_id, run_id),
            None,
            None,
        )
    }
}
