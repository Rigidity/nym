// Copyright 2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: GPL-3.0-only

use axum::{
    extract::{ConnectInfo, Request},
    http::{
        header::{HOST, USER_AGENT},
        HeaderValue,
    },
    middleware::Next,
    response::IntoResponse,
};
use colored::*;
use std::net::SocketAddr;
use tokio::time::Instant;
use tracing::info;

/// Simple logger for requests
pub async fn logger(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let method = req.method().to_string().green();
    let uri = req.uri().to_string().blue();
    let agent = header_map(
        req.headers().get(USER_AGENT),
        "Unknown User Agent".to_string(),
    );

    let host = header_map(req.headers().get(HOST), "Unknown Host".to_string());

    let start = Instant::now();
    let res = next.run(req).await;
    let time_taken = start.elapsed();
    let status = res.status();
    let print_status = if status.is_client_error() || status.is_server_error() {
        status.to_string().red()
    } else if status.is_success() {
        status.to_string().green()
    } else {
        status.to_string().yellow()
    };

    let taken = "time taken".bold();

    let time_taken = match time_taken.as_millis() {
        ms if ms > 500 => format!("{taken}: {}", format!("{ms}ms").red()),
        ms if ms > 200 => format!("{taken}: {}", format!("{ms}ms").yellow()),
        ms if ms > 50 => format!("{taken}: {}", format!("{ms}ms").bright_yellow()),
        ms => format!("{taken}: {ms}ms"),
    };

    let agent_str = "agent".bold();
    info!("[{addr} -> {host}] {method} '{uri}': {print_status} {time_taken} {agent_str}: {agent}");

    res
}

fn header_map(header: Option<&HeaderValue>, msg: String) -> String {
    header
        .map(|x| x.to_str().unwrap_or(&msg).to_string())
        .unwrap_or(msg)
}
