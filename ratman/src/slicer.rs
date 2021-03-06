// SPDX-FileCopyrightText: 2019-2022 Katharina Fey <kookie@spacekookie.de>
//
// SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

//! Slices `Message` into a series of Frames

use crate::{Message, Payload};
use types::{Frame, SeqBuilder};

/// Slices messages into managable chunks
pub(crate) struct Slicer;

impl Slicer {
    /// Take a `Message` and split it into a list of `Frames`
    pub(crate) fn slice(max: usize, msg: Message) -> Vec<Frame> {
        let payload = bincode::serialize(&Payload {
            payload: msg.payload,
            timesig: msg.timesig,
            sign: msg.sign,
        })
        .unwrap();

        payload
            .as_slice()
            .chunks(max)
            .fold(
                SeqBuilder::new(msg.sender, msg.recipient, msg.id),
                |seq, chunk| seq.add(chunk.into_iter().cloned().collect()),
            )
            .build()
    }
}
