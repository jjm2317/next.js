use turbo_tasks::{RcStr, Vc};
use turbopack_core::module::Module;

use crate::app_segment_config::NextSegmentConfig;

/// The entry module asset for a Next.js app route or page.
#[turbo_tasks::value(shared, unresolved)]
pub struct AppEntry {
    /// The pathname of the route or page.
    pub pathname: RcStr,
    /// The original Next.js name of the route or page. This is used instead of
    /// the pathname to refer to this entry.
    pub original_name: RcStr,
    /// The RSC module asset for the route or page.
    pub rsc_entry: Vc<Box<dyn Module>>,
    /// The source code config for this entry.
    pub config: Vc<NextSegmentConfig>,
}
