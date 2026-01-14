// SPDX-License-Identifier: MIT
// Copyright 2025. Triad National Security, LLC.

use clap::Args;

// use crate::commands::{AxumResponse, ManageBody, Cli, Handle, HandledResult};
use crate::commands::{AxumResponse, ManageBody, Cli};

// use crate::halo_capnp::halo_mgmt::{command_result, set_managed_results};

#[derive(Args, Debug, Clone)]
pub struct ManageArgs {
    /// Resource to manage
    resource_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct UnManageArgs {
    /// Resource to manage
    resource_id: String,
}

// pub async fn manage(cli: &Cli, args: &ManageArgs) -> HandledResult<()> {
pub async fn manage(cli: &Cli, args: &ManageArgs){
    send_command(cli, &args.resource_id, true).await
}

// pub async fn unmanage(cli: &Cli, args: &UnManageArgs) -> HandledResult<()> {
pub async fn unmanage(cli: &Cli, args: &UnManageArgs) {
    send_command(cli, &args.resource_id, false).await
}

// async fn send_command(cli: &Cli, resource: &str, manage: bool) -> HandledResult<()> {
async fn send_command(cli: &Cli, resource: &str, manage: bool){
    let socket: String = cli.socket.clone().unwrap_or_else(crate::default_socket);
    let reply = reqwest::Client::builder()
        .unix_socket(socket)
        .build().unwrap()
        .post("http://commands/manage")
        .json(&ManageBody{
            resource: resource.into(),
            manage,
        })
        .send()
        .await.unwrap();
    let body:AxumResponse = reply.json().await.expect("temp send");
    println!("error={}, text={}", body.error, body.text);


    // tokio::task::LocalSet::new()
    //     .run_until(async move {
    //         let reply = reqwest::Client::builder()
    //             .unix_socket(match cli.socket {
    //                 Some(s) => s,
    //                 None => crate::default_socket()
    //             })
    //             .build().unwrap()
    //             .post("http://commands/manage")
    //             .json(&ManageBody{
    //                 resource: resource.into(),
    //                 manage,
    //             })
    //             .send()
    //             .await?;
    //         let body:AxumResponse = reply.json().await.expect("temp send");
    //         println!("error={}, text={}", body.error, body.text);
    //     })
    //     .await
}

// fn decode_reply(
//     reply: &::capnp::capability::Response<set_managed_results::Owned>,
// ) -> Result<Option<&str>, capnp::Error> {
//     let reply = reply.get()?.get_res()?;

//     Ok(match reply.which()? {
//         command_result::Ok(()) => None,
//         command_result::Err(e) => Some(e?.to_str()?),
//     })
// }
