//! Engine adapters selected after PoC bake-off (pdf_oxide).

pub mod extract;
pub mod pdf_oxide_stub;
pub mod redact;
pub mod sanitize;

pub use extract::PdfOxideExtract;
pub use redact::PdfOxideRedact;
pub use sanitize::PdfOxideSanitize;
