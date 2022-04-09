// SPDX-FileCopyrightText: 2019-2022 Katharina Fey <kookie@spacekookie.de>
//
// SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

//! Ratman message abstraction

pub use crate::proto::message::Message;
use ratman_identity::Identity;

/// Create a new `Message`
pub fn new(
    sender: Identity,
    recipients: Vec<Identity>,
    payload: Vec<u8>,
    signature: Vec<u8>,
) -> Message {
    let mut inner = Message::new();
    inner.set_sender(sender.as_bytes().to_vec());
    inner.set_recipients(
        recipients
            .iter()
            .map(|r| r.as_bytes().to_vec())
            .collect::<Vec<_>>()
            .into(),
    );
    inner.set_payload(payload);
    inner.set_signature(signature);
    inner
}

pub fn received(
    id: Identity,
    sender: Identity,
    recipient: Option<Identity>,
    payload: Vec<u8>,
    timesig: String,
    sign: Vec<u8>,
) -> Message {
    let mut inner = Message::new();
    inner.set_id(id.as_bytes().to_vec());
    inner.set_sender(sender.as_bytes().to_vec());
    if let Some(r) = recipient {
        inner.set_recipients(vec![r.as_bytes().to_vec()].into());
    }
    inner.set_time(timesig);
    inner.set_payload(payload);
    inner.set_signature(sign);

    inner
}
