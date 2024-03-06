mod markdown;
pub use markdown::{Markdown, MarkdownType};

#[cfg(feature = "qrcode")]
mod qrcode;
#[cfg(feature = "qrcode")]
pub use qrcode::QRCode;

#[cfg(feature="typst")]
mod typst;
#[cfg(feature="typst")]
pub use typst::*;

// #[cfg(feature="latex")]
// mod latex;

mod table;
pub use table::{Table, TableBody, TableBodyCell, TableHeadCell, TableBodyRow};