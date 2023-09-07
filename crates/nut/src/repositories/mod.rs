pub mod asset;
pub mod catalogue;
pub mod entry;
pub mod event;
pub mod import;
pub mod root;
pub mod source;
pub mod state;
pub mod stem;
pub mod preferences;

pub trait Repository {}

pub use asset::AssetRepository;
pub use catalogue::CatalogueRepository;
pub use entry::EntryRepository;
pub use event::EventRepository;
pub use import::ImportRepository;
pub use root::RootRepository;
pub use source::SourceRepository;
pub use state::StateRepository;
pub use stem::StemRepository;
pub use preferences::PreferencesRepository;
