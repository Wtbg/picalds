use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum ApiResponse<T> 
where T: Serialize
{
    Success(T),
    Error(ApiError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}
