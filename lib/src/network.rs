use std::io::{Error as IoError, Read, Write};

use serde::{Deserialize, Serialize};

use crate::{
    crypto::PublicKey,
    types::{Block, Transaction, TransactionOutput},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    /// Fetch  all pubkey UTXOs
    FetchUTXO(PublicKey),
    /// pubkey UTXOs with marker
    UTXOs(Vec<(TransactionOutput, bool)>),
    /// Send a tx to network
    SubmitTransaction(Transaction),
    /// Broadcast a new tx to other nodes
    NewTransaction(Transaction),
    /// Ask the node to prepare the optimal block template with coinbase tx paying the specified pub key
    FetchTemplate(PublicKey),
    /// The Template
    Template(Block),
    /// Ask the node to validate block template
    ValidateTemplate(Block),
    /// If template is valid
    TemplateValidity(bool),
    /// Submit a mined block to a node
    SubmitTemplate(Block),
    /// Ask a node to report all the other nodes it knows about
    DiscoverNodes,
    /// Response of DiscoverNodes
    NodeList(Vec<String>),
    /// Ask a node whats the highest block it knows about in comparison to the local blockchain
    AskDifference(u32),
    /// Response of AskDifference
    Difference(i32),
    /// Ask a node to send block with the specified height
    FetchBlock(usize),
    /// Broadcast a new block to the other nodes
    NewBlock(Block),
}

impl Message {
    pub fn encode(&self) -> Result<Vec<u8>, ciborium::ser::Error<IoError>> {
        let mut bytes = Vec::new();
        ciborium::into_writer(self, &mut bytes)?;
        Ok(bytes)
    }

    pub fn decode(data: &[u8]) -> Result<Self, ciborium::de::Error<IoError>> {
        ciborium::from_reader(data)
    }

    pub fn send(&self, stream: &mut impl Write) -> Result<(), ciborium::ser::Error<IoError>> {
        let bytes = self.encode()?;
        let len = bytes.len() as u64;
        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(&bytes)?;
        Ok(())
    }

    pub fn receive(stream: &mut impl Read) -> Result<Self, ciborium::de::Error<IoError>> {
        let mut len_bytes = [0u8; 8];
        stream.read_exact(&mut len_bytes)?;
        let len = u64::from_be_bytes(len_bytes) as usize;
        let mut data = vec![0u8; len];
        stream.read_exact(&mut data)?;
        Self::decode(&data)
    }
}
