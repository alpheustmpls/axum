use crate::{
    common::errors::service::ServiceError, constants::env::get_version,
    modules::info::schema::Info,
};

pub async fn service_info() -> Result<Info, ServiceError> {
    Ok(Info {
        // API version
        version: get_version(),
    })
}
