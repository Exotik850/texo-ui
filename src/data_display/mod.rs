mod markdown;
pub use markdown::{Markdown, MarkdownType};

#[cfg(feature = "qrcode")]
mod qrcode;
#[cfg(feature = "qrcode")]
pub use qrcode::QRCode;

mod table;
pub use table::{Table, TableBody, TableBodyCell, TableHeadCell, TableBodyRow};