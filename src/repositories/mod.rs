pub mod catalogue;
pub mod event;
pub mod import;
pub mod source;

pub trait Repository {}

pub use catalogue::CatalogueRepository;
pub use event::EventRepository;
pub use import::ImportRepository;
