#[cfg(not(target_arch = "wasm32"))]
mod native {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: tree_sitter::IncludedRangesError,
    }
    impl std::fmt::Display for IncludedRangesError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "{:?}", self.inner)
        }
    }
    impl std::error::Error for IncludedRangesError {
    }
    impl From<tree_sitter::IncludedRangesError> for IncludedRangesError {
        #[inline]
        fn from(inner: tree_sitter::IncludedRangesError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: tree_sitter::QueryError,
    }
    impl std::fmt::Display for QueryError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "{:?}", self.inner)
        }
    }
    impl std::error::Error for QueryError {
    }
    impl From<tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(inner: tree_sitter::QueryError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: tree_sitter::LanguageError,
    }
    impl std::fmt::Display for LanguageError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(fmt)
        }
    }
    impl std::error::Error for LanguageError {
    }
    impl From<tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(inner: tree_sitter::LanguageError) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: js_sys::Error,
    }
    impl std::fmt::Display for IncludedRangesError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for IncludedRangesError {
    }
    impl From<js_sys::Error> for IncludedRangesError {
        #[inline]
        fn from(inner: js_sys::Error) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: web_tree_sitter::LanguageError,
    }
    impl std::fmt::Display for LanguageError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for LanguageError {
    }
    impl From<web_tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(inner: web_tree_sitter::ParserError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: web_tree_sitter::QueryError,
    }
    impl std::fmt::Display for QueryError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(&<_ as Into<String>>::into(self.inner.message()))
        }
    }
    impl std::error::Error for QueryError {
    }
    impl From<web_tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(inner: web_tree_sitter::QueryError) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;