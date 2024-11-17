use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::AsyncRunner, GenericImage, ImageExt};

use crate::prelude::*;

pub async fn setup_mysql_container(custom_port: u16, root_password: &str) -> Result<String> {
    let container = GenericImage::new("mysql", "latest")
        .with_exposed_port(custom_port.tcp())
        .with_wait_for(WaitFor::message_on_stdout("ready for connections"))
        .with_env_var("MYSQL_ROOT_PASSWORD", root_password)
        .start()
        .await?;
    let host = container.get_host().await?;
    let host_port = container.get_host_port_ipv4(custom_port).await?;
    let url = format!("mysql://root:{root_password}@{host}:{host_port}/processor_routing");

    Ok(url)
}