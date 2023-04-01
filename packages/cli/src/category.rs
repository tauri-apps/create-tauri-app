use std::fmt::Display;

use crate::package_manager::PackageManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Category {
    Rust,
    #[default]
    JsTs,
}

impl<'a> Category {
    pub const ALL: &'a [Self] = &[Category::JsTs, Category::Rust];

    pub const fn package_managers(&self) -> &[PackageManager] {
        match self {
            Category::Rust => &[PackageManager::Cargo],
            Category::JsTs => &[
                PackageManager::Pnpm,
                PackageManager::Yarn,
                PackageManager::Npm,
            ],
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let managers = self
            .package_managers()
            .to_vec()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        match self {
            Category::Rust => write!(f, "Rust - ({managers})"),
            Category::JsTs => write!(f, "TypeScript / JavaScript - ({managers})"),
        }
    }
}
