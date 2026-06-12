use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // TODO: Construct a CompactSize from a u64 value
        CompactSize { value }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Encode according to Bitcoin's CompactSize format:
        // [0x00–0xFC] => 1 byte
        // [0xFDxxxx] => 0xFD + u16 (2 bytes)
        // [0xFExxxxxxxx] => 0xFE + u32 (4 bytes)
        // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)
        if self.value <= 252 {
            return vec![self.value as u8];
        }

        if self.value <= 65535 {
            let mut result = Vec::new();
            result.push(0xFD);
            let le_bytes = (self.value as u16).to_le_bytes();
            result.push(le_bytes[0]);
            result.push(le_bytes[1]);
            return result;
        }

        if self.value <= 4294967295 {
            let mut result = Vec::new();
            result.push(0xFE);
            let le_bytes = (self.value as u32).to_le_bytes();
            result.push(le_bytes[0]);
            result.push(le_bytes[1]);
            result.push(le_bytes[2]);
            result.push(le_bytes[3]);
            return result;
        }

        let mut result = Vec::new();
        result.push(0xFF);
        let le_bytes = self.value.to_le_bytes();
        result.push(le_bytes[0]);
        result.push(le_bytes[1]);
        result.push(le_bytes[2]);
        result.push(le_bytes[3]);
        result.push(le_bytes[4]);
        result.push(le_bytes[5]);
        result.push(le_bytes[6]);
        result.push(le_bytes[7]);
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        // Check that enough bytes are available based on prefix.
        if bytes.is_empty() {
            return Err(BitcoinError::InsufficientBytes);
        }

        let first_byte = bytes[0];

        if first_byte <= 0xFC {
            let value = first_byte as u64;
            return Ok((CompactSize { value }, 1));
        }

        if first_byte == 0xFD {
            if bytes.len() < 3 {
                return Err(BitcoinError::InsufficientBytes);
            }
            let value = u16::from_le_bytes([bytes[1], bytes[2]]) as u64;
            return Ok((CompactSize { value }, 3));
        }

        if first_byte == 0xFE {
            if bytes.len() < 5 {
                return Err(BitcoinError::InsufficientBytes);
            }
            let value = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as u64;
            return Ok((CompactSize { value }, 5));
        }

        if bytes.len() < 9 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let value = u64::from_le_bytes([
            bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
        ]);
        Ok((CompactSize { value }, 9))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Serialize as a hex-encoded string (32 bytes => 64 hex characters)
        let hex_string = hex::encode(self.0);
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Parse hex string into 32-byte array
        // Use `hex::decode`, validate length = 32
        let hex_string = String::deserialize(deserializer)?;

        let decoded_bytes = match hex::decode(&hex_string) {
            Ok(bytes) => bytes,
            Err(e) => return Err(serde::de::Error::custom(e)),
        };

        if decoded_bytes.len() != 32 {
            return Err(serde::de::Error::custom("txid must be exactly 32 bytes"));
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&decoded_bytes[..32]);

        Ok(Txid(arr))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        // TODO: Create an OutPoint from raw txid bytes and output index
        OutPoint {
            txid: Txid(txid),
            vout,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize as: txid (32 bytes) + vout (4 bytes, little-endian)
        let mut result = Vec::new();

        for byte in &self.txid.0 {
            result.push(*byte);
        }

        let vout_bytes = self.vout.to_le_bytes();
        result.push(vout_bytes[0]);
        result.push(vout_bytes[1]);
        result.push(vout_bytes[2]);
        result.push(vout_bytes[3]);

        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize 36 bytes: txid[0..32], vout[32..36]
        // Return error if insufficient bytes
        if bytes.len() < 36 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let mut txid_bytes = [0u8; 32];
        txid_bytes.copy_from_slice(&bytes[..32]);

        let vout = u32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]);

        let outpoint = OutPoint {
            txid: Txid(txid_bytes),
            vout,
        };

        Ok((outpoint, 36))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        // TODO: Simple constructor
        Script { bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Prefix with CompactSize (length), then raw bytes
        let mut result = Vec::new();

        let script_length = self.bytes.len() as u64;
        let length_prefix = CompactSize::new(script_length).to_bytes();
        for byte in length_prefix {
            result.push(byte);
        }

        for byte in &self.bytes {
            result.push(*byte);
        }

        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Parse CompactSize prefix, then read that many bytes
        // Return error if not enough bytes
        let (compact_size, header_size) = CompactSize::from_bytes(bytes)?;
        let script_length = compact_size.value as usize;

        if bytes.len() < header_size + script_length {
            return Err(BitcoinError::InsufficientBytes);
        }

        let mut script_bytes = Vec::new();
        for byte in bytes.iter().skip(header_size).take(script_length) {
            script_bytes.push(*byte);
        }

        let total_consumed = header_size + script_length;
        Ok((
            Script {
                bytes: script_bytes,
            },
            total_consumed,
        ))
    }
}

impl Deref for Script {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        // TODO: Allow &Script to be used as &[u8]
        &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        // TODO: Basic constructor
        TransactionInput {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize: OutPoint + Script (with CompactSize) + sequence (4 bytes LE)
        let mut result = Vec::new();

        let outpoint_bytes = self.previous_output.to_bytes();
        for byte in outpoint_bytes {
            result.push(byte);
        }

        let script_bytes = self.script_sig.to_bytes();
        for byte in script_bytes {
            result.push(byte);
        }

        let sequence_bytes = self.sequence.to_le_bytes();
        result.push(sequence_bytes[0]);
        result.push(sequence_bytes[1]);
        result.push(sequence_bytes[2]);
        result.push(sequence_bytes[3]);

        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize in order:
        // - OutPoint (36 bytes)
        // - Script (with CompactSize)
        // - Sequence (4 bytes)
        let (previous_output, outpoint_size) = OutPoint::from_bytes(bytes)?;

        let after_outpoint = &bytes[outpoint_size..];
        let (script_sig, script_size) = Script::from_bytes(after_outpoint)?;

        let sequence_start = outpoint_size + script_size;

        if bytes.len() < sequence_start + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let sequence = u32::from_le_bytes([
            bytes[sequence_start],
            bytes[sequence_start + 1],
            bytes[sequence_start + 2],
            bytes[sequence_start + 3],
        ]);

        let total_consumed = sequence_start + 4;

        let input = TransactionInput {
            previous_output,
            script_sig,
            sequence,
        };

        Ok((input, total_consumed))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        // TODO: Construct a transaction from parts
        BitcoinTransaction {
            version,
            inputs,
            lock_time,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Format:
        // - version (4 bytes LE)
        // - CompactSize (number of inputs)
        // - each input serialized
        // - lock_time (4 bytes LE)
        let mut result = Vec::new();

        let version_bytes = self.version.to_le_bytes();
        result.push(version_bytes[0]);
        result.push(version_bytes[1]);
        result.push(version_bytes[2]);
        result.push(version_bytes[3]);

        let input_count = self.inputs.len() as u64;
        let count_bytes = CompactSize::new(input_count).to_bytes();
        for byte in count_bytes {
            result.push(byte);
        }

        for input in &self.inputs {
            let input_bytes = input.to_bytes();
            for byte in input_bytes {
                result.push(byte);
            }
        }

        let lock_time_bytes = self.lock_time.to_le_bytes();
        result.push(lock_time_bytes[0]);
        result.push(lock_time_bytes[1]);
        result.push(lock_time_bytes[2]);
        result.push(lock_time_bytes[3]);

        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Read version, CompactSize for input count
        // Parse inputs one by one
        // Read final 4 bytes for lock_time
        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let mut offset = 4;

        let (input_count_cs, cs_size) = CompactSize::from_bytes(&bytes[offset..])?;
        let input_count = input_count_cs.value;
        offset += cs_size;

        let mut inputs = Vec::new();
        for _ in 0..input_count {
            let (input, input_size) = TransactionInput::from_bytes(&bytes[offset..])?;
            offset += input_size;
            inputs.push(input);
        }

        if bytes.len() < offset + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let lock_time = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let transaction = BitcoinTransaction {
            version,
            inputs,
            lock_time,
        };

        Ok((transaction, offset))
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format a user-friendly string showing version, inputs, lock_time
        // Display scriptSig length and bytes, and previous output info
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Inputs ({}):", self.inputs.len())?;

        for (index, input) in self.inputs.iter().enumerate() {
            writeln!(f, "  Input {}:", index)?;

            let txid_hex = hex::encode(input.previous_output.txid.0);
            writeln!(f, "    Previous Output Txid: {}", txid_hex)?;
            writeln!(
                f,
                "    Previous Output Vout: {}",
                input.previous_output.vout
            )?;

            let script_length = input.script_sig.bytes.len();
            writeln!(f, "    ScriptSig Length: {}", script_length)?;

            let script_hex = hex::encode(&input.script_sig.bytes);
            writeln!(f, "    ScriptSig Bytes: {}", script_hex)?;

            writeln!(f, "    Sequence: {}", input.sequence)?;
        }

        write!(f, "Lock Time: {}", self.lock_time)
    }
}
