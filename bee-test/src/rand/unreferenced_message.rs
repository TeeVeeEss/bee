// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_tangle::unreferenced_message::UnreferencedMessage;

use crate::rand::message::rand_message_id;

/// Generates a random unrefere,ced message.
pub fn rand_unreferenced_message() -> UnreferencedMessage {
    rand_message_id().into()
}
