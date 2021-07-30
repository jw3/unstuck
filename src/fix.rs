use crate::api;
use crate::api::Error::Unimplemented;

pub async fn surgical_strike(_ns: &str) -> Result<(), api::Error> {
   Err(Unimplemented("there is no right way at this time, yolo".into()))
}
