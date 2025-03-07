// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_rest_api::types::responses::PeersResponse;

use crate::plugins::dashboard::websocket::{
    responses::{WsEvent, WsEventInner},
    topics::WsTopic,
};

impl From<PeersResponse> for WsEvent {
    fn from(val: PeersResponse) -> Self {
        Self::new(WsTopic::PeerMetrics, WsEventInner::PeerMetric(val))
    }
}
