mod attribute;
mod gtf;
mod reader;
mod record;

pub use attribute::{Attribute, AttributeRef, Field};
pub use gtf::Gtf;
pub use reader::GtfReader;
pub use record::{GtfRecord, GtfRecordRef};
