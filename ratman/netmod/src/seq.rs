//! Sequence handling module

use crate::{Frame, Recipient};
use identity::Identity;
use {
    std::hash::{BuildHasher, Hasher},
    twox_hash::RandomXxHashBuilder64 as RXHash64,
};

/// An XxHash signature and initialisation seed
#[derive(Debug, Clone, Copy)]
pub struct XxSignature {
    sig: u64,
    seed: u64,
}

/// Encoded signature information related to a data sequence
///
/// When a large chunk of data is split across a `Frame` set,
/// signature hashes are used to verify data integrity, as well as
/// sequence ordering. The "Sequence ID" itself can be used to re-order
/// frames received out of order, as well as verifying that a `Frame`
/// was transmitted without error.
///
/// Check the `netmod` crate documentation for more details.
#[derive(Clone, Debug)]
pub struct SeqId {
    /// A hash signature of the payload
    pub sig: XxSignature,
    /// Global frame sequence ID
    pub seqid: [u8; 16],
    /// Next sequenced Frame SIG
    pub next: Option<u64>,
}

/// Utility wrapping around `Vec<Frame>` with `SeqId` initialisation.
///
/// This type implements a builder, which is initialised
pub struct Sequence {
    sender: Identity,
    recp: Recipient,
    data: Vec<Vec<u8>>,
}

impl Sequence {
    pub fn new(sender: Identity, recp: Recipient) -> Self {
        Self {
            sender,
            recp,
            data: vec![],
        }
    }

    pub fn add(mut self, data: Vec<u8>) -> Self {
        self.data.push(data);
        self
    }

    pub fn build(self) -> Vec<Frame> {
        let seqid = [0; 16];
        let sender = self.sender;
        let recipient = self.recp;
        let signed = self
            .data
            .into_iter()
            .map(|d| (hash_new(&d), d))
            .collect::<Vec<_>>();

        (0..signed.len())
            .map(|i| match (signed.get(i), signed.get(i + 1)) {
                (
                    Some((ref sig, data)),
                    Some((
                        XxSignature {
                            sig: ref next,
                            seed: _,
                        },
                        _,
                    )),
                ) => (
                    SeqId {
                        seqid,
                        sig: *sig,
                        next: Some(*next),
                    },
                    data,
                ),
                (Some((ref sig, data)), None) => (
                    SeqId {
                        seqid,
                        sig: *sig,
                        next: None,
                    },
                    data,
                ),
                _ => unreachable!(),
            })
            .map(|(seqid, data)| Frame {
                sender,
                recipient,
                seqid,
                payload: data.to_vec(),
            })
            .collect()
    }
}

fn hash_new(data: &Vec<u8>) -> XxSignature {
    let mut hasher = RXHash64::default().build_hasher();
    hasher.write(data);
    XxSignature {
        sig: hasher.finish(),
        seed: hasher.seed(),
    }
}

#[test]
fn foo() {
    let sender = Identity::with_digest(&vec![1]);
    let recp = Identity::with_digest(&vec![2]);
    let seq = Sequence::new(sender, Recipient::User(recp))
        .add(vec![42])
        .add(vec![13, 12])
        .add(vec![13, 37])
        .build();

    assert!(seq.len() == 3);
    assert!(seq.get(0).unwrap().seqid.next == Some(seq.get(1).unwrap().seqid.sig.sig));
}