use std::{future::Future, pin::Pin, thread::sleep, time::Duration};

use crate::temporal::api::enums::v1::{CommandType, EventType};
use crate::temporal::api::workflowservice::v1::workflow_service_client::WorkflowServiceClient;

type ServiceClient = WorkflowServiceClient<tonic::transport::Channel>;

struct Poller {
    client: WorkflowServiceClient<tonic::transport::Channel>,
}

type AsyncResult<'a> = Pin<Box<dyn Future<Output=Result<(), Box<dyn std::error::Error>>> + 'a>>;

impl Poller {
    pub async fn run(&mut self, poll_call: for<'a> fn(
        client: &'a mut ServiceClient,
    ) -> AsyncResult<'a>)
                     -> Result<(), Box<dyn std::error::Error>>
    {
        poll_call(&mut self.client).await
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time;

    use crate::temporal::api::workflowservice::v1::DescribeNamespaceRequest;
    use crate::temporal::api::workflowservice::v1::workflow_service_client::WorkflowServiceClient;

    use super::*;

    // #[tokio::test]
    // async fn describe_namespace_works() -> Result<(), Box<dyn std::error::Error>> {
    //     let mut client = WorkflowServiceClient::connect("http://[::1]:7233").await?;
    //     let request = tonic::Request::new(DescribeNamespaceRequest {
    //         name: "default".into(),
    //         id: "".into(),
    //     });
    //     let response = client.describe_namespace(request).await?;
    //     println!("RESPONSE={:?}", response);
    //     assert_eq!("default", response.get_ref().namespace_info.as_ref().unwrap().name);
    //     Ok(())
    // }

    async fn poll_task(client: &mut ServiceClient) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(DescribeNamespaceRequest {
            name: "default".into(),
            id: "".into(),
        });
        let response = client.describe_namespace(request).await?;
        println!("RESPONSE={:?}", response);
        assert_eq!("default", response.get_ref().namespace_info.as_ref().unwrap().name);
        // time::delay_for(Duration::from_secs(1)).await;
        Ok(())
    }

    fn poll_task_async<'a>(client: &'a mut ServiceClient) -> AsyncResult<'a> {
        Box::pin(poll_task(client))
    }

    #[tokio::test]
    async fn poller_works() -> Result<(), Box<dyn std::error::Error>> {
        let client = WorkflowServiceClient::connect("http://[::1]:7233").await?;
        let mut poller = Poller { client };
        poller.run(poll_task_async).await?;
        Ok(())
    }
}