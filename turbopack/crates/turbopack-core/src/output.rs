use anyhow::Result;
use indexmap::IndexSet;
use turbo_tasks::Vc;

use crate::{asset::Asset, ident::AssetIdent};

/// An asset that should be outputted, e. g. written to disk or served from a
/// server.
#[turbo_tasks::value_trait]
pub trait OutputAsset: Asset {
    // TODO change this to path() -> Vc<FileSystemPath>
    /// The identifier of the [OutputAsset]. It's expected to be unique and
    /// capture all properties of the [OutputAsset]. Only path must be used.
    fn ident(&self) -> Vc<AssetIdent>;

    /// Other references [OutputAsset]s from this [OutputAsset].
    fn references(self: Vc<Self>) -> Vc<OutputAssets> {
        OutputAssets::empty()
    }
}

#[turbo_tasks::value(transparent, unresolved)]
pub struct OutputAssets(Vec<Vc<Box<dyn OutputAsset>>>);

#[turbo_tasks::value_impl]
impl OutputAssets {
    #[turbo_tasks::function]
    pub fn new(assets: Vec<Vc<Box<dyn OutputAsset>>>) -> Vc<Self> {
        Vc::cell(assets)
    }

    #[turbo_tasks::function]
    pub async fn concatenate(self: Vc<Self>, other: Vc<Self>) -> Result<Vc<Self>> {
        let mut assets: IndexSet<_> = self.await?.iter().copied().collect();
        assets.extend(other.await?.iter().copied());
        Ok(Vc::cell(assets.into_iter().collect()))
    }
}

impl OutputAssets {
    pub fn empty() -> Vc<Self> {
        Self::new(vec![])
    }
}

/// A set of [OutputAsset]s
#[turbo_tasks::value(transparent, unresolved)]
pub struct OutputAssetsSet(IndexSet<Vc<Box<dyn OutputAsset>>>);

// TODO All Vc::try_resolve_downcast::<Box<dyn OutputAsset>> calls should be
// removed
