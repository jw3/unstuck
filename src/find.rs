use crate::api;
use k8s_openapi::api::core::v1::Namespace;
use kube::api::DynamicObject;
use kube::discovery::verbs;
use kube::{Api, Client, Discovery, ResourceExt};

pub async fn whats_stuck(ns: &str) -> Result<(), api::Error> {
    let client = Client::try_default().await?;
    let namespace_api: Api<Namespace> = Api::all(client.clone());

    let mut the_ns = namespace_api.get(ns).await?;
    if let Some(ref spec) = the_ns.spec {
        let mut update = spec.clone();
        update.finalizers.clear();
        the_ns.spec.replace(update);

        let d = Discovery::new(client.clone()).run().await?;
        for group in d.groups() {
            for (ar, caps) in group.recommended_resources() {
                if caps.supports_operation(verbs::LIST) {
                    let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), ns, &ar);
                    if let Some(dyno) = match api.list(&Default::default()).await {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    } {
                        for obj in dyno {
                            println!("{} {}: {}", ar.api_version, ar.kind, obj.name());
                            for f in obj.metadata.finalizers {
                                println!("  - {}", f);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
