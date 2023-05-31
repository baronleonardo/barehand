mod html;
pub use html::Html;

mod element;
pub use element::Element;

mod attribute;
pub use attribute::Attributes;

mod singlton_tag;
use singlton_tag::is_singleton_tag;

mod utils;
use utils::Utils;