#[cfg(test)]
mod tests {
    use temporal::api::workflowservice::v1::DescribeNamespaceRequest;
    use temporal::api::workflowservice::v1::workflow_service_client::WorkflowServiceClient;

    pub mod temporal {
        pub mod api {
            pub mod command {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.command.v1");
                }
            }

            pub mod common {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.common.v1");
                }
            }

            pub mod enums {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.enums.v1");
                }
            }

            pub mod failure {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.failure.v1");
                }
            }

            pub mod filter {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.filter.v1");
                }
            }

            pub mod history {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.history.v1");
                }
            }

            pub mod namespace {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.namespace.v1");
                }
            }

            pub mod query {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.query.v1");
                }
            }

            pub mod replication {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.replication.v1");
                }
            }

            pub mod taskqueue {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.taskqueue.v1");
                }
            }

            pub mod version {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.version.v1");
                }
            }

            pub mod workflow {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.workflow.v1");
                }
            }

            pub mod workflowservice {
                pub mod v1 {
                    tonic::include_proto!("temporal.api.workflowservice.v1");
                }
            }
        }
    }

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
}
