use crate::temporal::api::workflowservice::v1::DescribeNamespaceRequest;
use crate::temporal::api::workflowservice::v1::workflow_service_client::WorkflowServiceClient;

#[tokio::test]
async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WorkflowServiceClient::connect("http://[::1]:7233").await?;
    let request = tonic::Request::new(DescribeNamespaceRequest {
        name: "default".into(),
        id: "".into(),
    });
    let response = client.describe_namespace(request).await?;
    println!("RESPONSE={:?}", response);
    assert_eq!("default", response.get_ref().namespace_info.as_ref().unwrap().name);
    Ok(())
}
