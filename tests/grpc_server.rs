use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::Request;
use trustlog_lib::opentelemetry::proto::{
    collector::logs::v1::{logs_service_client::LogsServiceClient, ExportLogsServiceRequest},
    common::v1::{any_value, AnyValue, KeyValue},
    logs::v1::{LogRecord, ResourceLogs, ScopeLogs},
    resource::v1::Resource as OtelResource,
};

#[tokio::test]
async fn receive_grpc_logs() {
    let grpc_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9300);

    let mut client = LogsServiceClient::connect(format!("http://{}", grpc_addr))
        .await
        .unwrap();

    for _ in 0..100 {
        let req = Request::new(ExportLogsServiceRequest {
            resource_logs: vec![ResourceLogs {
                resource: Some(OtelResource {
                    attributes: vec![KeyValue {
                        key: "res_key".into(),
                        value: Some(AnyValue {
                            value: Some(any_value::Value::StringValue("res_val".into())),
                        }),
                    }],
                    dropped_attributes_count: 0,
                }),
                scope_logs: vec![ScopeLogs {
                    scope: None,
                    log_records: vec![LogRecord {
                        time_unix_nano: 1,
                        observed_time_unix_nano: 2,
                        severity_number: 9,
                        severity_text: "info".into(),
                        body: Some(AnyValue {
                            value: Some(any_value::Value::StringValue("log body".into())),
                        }),
                        attributes: vec![KeyValue {
                            key: "attr_key".into(),
                            value: Some(AnyValue {
                                value: Some(any_value::Value::StringValue("attr_val".into())),
                            }),
                        }],
                        dropped_attributes_count: 3,
                        flags: 4,
                        trace_id: str_into_hex_bytes("4ac52aadf321c2e531db005df08792f5"),
                        span_id: str_into_hex_bytes("0b9e4bda2a55530d"),
                    }],
                    schema_url: "v1".into(),
                }],
                schema_url: "v1".into(),
            }],
        });
        _ = client.export(req).await;
    }
}

fn str_into_hex_bytes(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}
