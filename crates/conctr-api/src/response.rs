//! JSON contracts shared by HTTP clients and the management API.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(default, skip_serializing_if = "ResponseMeta::is_empty")]
    pub meta: ResponseMeta,
}
impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            meta: ResponseMeta::default(),
        }
    }
}
#[derive(Default, Deserialize, Serialize)]
pub struct ResponseMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageMeta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<ApiWarning>,
}
impl ResponseMeta {
    fn is_empty(&self) -> bool {
        self.page.is_none() && self.warnings.is_empty()
    }
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
#[derive(Deserialize, Serialize)]
pub struct ApiWarning {
    pub code: String,
    pub message: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub code: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FieldError>,
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FieldError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
    pub code: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};
    #[test]
    fn metadata_is_sparse_and_execution_state_stays_in_data() {
        for state in ["queued", "failed", "partial_success"] {
            let resource = json!({"status":state});
            assert_eq!(
                to_value(ApiResponse::new(&resource)).unwrap(),
                json!({"data":resource})
            );
        }
        let mut response = ApiResponse::new([1]);
        response.meta.page = Some(PageMeta {
            next_cursor: Some("cursor".into()),
            has_more: true,
        });
        response.meta.warnings.push(ApiWarning {
            code: "FIELD_SKIPPED".into(),
            message: "Unknown field.".into(),
        });
        assert_eq!(
            to_value(response).unwrap(),
            json!({"data":[1],"meta":{"page":{"nextCursor":"cursor","hasMore":true},"warnings":[{"code":"FIELD_SKIPPED","message":"Unknown field."}]}})
        );
    }
}
