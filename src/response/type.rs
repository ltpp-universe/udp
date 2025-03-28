use super::error::Error;

pub type ResponseData = Vec<u8>;
pub type ResponseResult = Result<(), Error>;

#[derive(Clone, Debug)]
pub struct Response(pub(super) ResponseData);
