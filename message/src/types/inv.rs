use std::io;
use ser::{Stream, Reader};
use common::InventoryVector;
use {Payload, MessageResult};

#[derive(Debug, PartialEq)]
pub struct Inv {
	pub inventory: Vec<InventoryVector>,
}

impl Payload for Inv {
	fn version() -> u32 {
		0
	}

	fn command() -> &'static str {
		"inv"
	}

	fn deserialize_payload<T>(reader: &mut Reader<T>, _version: u32) -> MessageResult<Self> where T: io::Read {
		let inv = Inv {
			inventory: try!(reader.read_list_max(50_000)),
		};

		Ok(inv)
	}

	fn serialize_payload(&self, stream: &mut Stream, _version: u32) -> MessageResult<()> {
		stream.append_list(&self.inventory);
		Ok(())
	}
}
