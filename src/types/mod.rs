mod attribute;
mod reader;
mod record;

pub use attribute::{Attribute, AttributeRef, Field};
pub use reader::GtfReader;
pub use record::{GtfRecord, GtfRecordRef};
