use kube::{
    core::{DynamicObject, GroupVersionKind},
    discovery, Api, Client,
};

pub async fn client(namespace: Option<&str>, resource: &str) -> anyhow::Result<Api<DynamicObject>> {
    let client = Client::try_default().await?;
    let gvk = GroupVersionKind::gvk("tekton.dev", "v1beta1", resource);
    let (ar, _) = discovery::pinned_kind(&client, &gvk).await?;
    // 4. create an Api based on parsed parameters
    let api: Api<DynamicObject> = if let Some(ns) = namespace {
        Api::namespaced_with(client.clone(), ns, &ar)
    } else {
        Api::default_namespaced_with(client.clone(), &ar)
    };
    Ok(api)
}
