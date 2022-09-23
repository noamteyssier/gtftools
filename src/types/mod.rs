mod attribute;
mod record;
mod reader;

pub use attribute::{Attribute, AttributeRef, Field};
pub use record::{GtfRecord, GtfRecordRef};
pub use reader::GtfReader;
