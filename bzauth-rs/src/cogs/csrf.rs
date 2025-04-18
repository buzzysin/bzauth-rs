use super::{CoreError, request::CoreRequest, response::CoreResponse};

pub async fn csrf<Payload>(_request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    todo!()
}
