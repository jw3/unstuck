use hyper::Request;
use k8s_openapi::api::core::v1::Namespace;
use kube::{api::Api, Client};
use log::info;
use serde_json::json;

use crate::api;

pub async fn brute_force_fix(ns: &str) -> Result<(), api::Error> {
    let client = Client::try_default().await?;
    let namespace_api: Api<Namespace> = Api::all(client.clone());

    let mut the_ns = namespace_api.get(ns).await?;
    if let Some(ref spec) = the_ns.spec {
        let mut update = spec.clone();
        update.finalizers.clear();
        the_ns.spec.replace(update);
        let json: String = json!(the_ns).to_string();

        let req: Request<Vec<u8>> = Request::put(format!("/api/v1/namespaces/{}/finalize", ns))
            .body(json.into())
            .unwrap();
        match client.request::<Namespace>(req).await {
            Ok(_) => {
                info!("Removed finalizer for {}.  YOLO!", ns);
                Ok(())
            }
            Err(e) => Err(api::Error::from(e)),
        }
    } else {
        Err(api::Error::NamespaceNotFound)
    }
}
