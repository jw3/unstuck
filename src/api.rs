use crate::api::Error::KubeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Namespace not found")]
    NamespaceNotFound,
    #[error("ApiError: {0} ({0:?})")]
    KubeError(#[source] kube::Error),
}

impl From<kube::Error> for Error {
    fn from(error: kube::Error) -> Self {
        match error {
            e => KubeError(e),
        }
    }
}
