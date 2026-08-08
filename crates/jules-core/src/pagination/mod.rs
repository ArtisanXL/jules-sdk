//! Pagination module.

/// A single page of results from a paginated list endpoint, along with an optional token to
/// fetch the next page.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Page<T> {
    items: Vec<T>,
    next_page_token: Option<String>,
}

impl<T> Page<T> {
    /// Creates a new `Page` from its items and an optional continuation token.
    #[must_use]
    pub fn new(items: Vec<T>, next_page_token: Option<String>) -> Self {
        Self {
            items,
            next_page_token,
        }
    }

    /// Returns the items in this page.
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Consumes the page, returning its items.
    #[must_use]
    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    /// Returns the token to fetch the next page, if any further pages remain.
    #[must_use]
    pub fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_with_next_token() {
        let page = Page::new(vec![1, 2, 3], Some("token-1".to_string()));
        assert_eq!(page.items(), &[1, 2, 3]);
        assert_eq!(page.next_page_token(), Some("token-1"));
    }

    #[test]
    fn test_page_last_page() {
        let page = Page::new(vec!["a".to_string()], None);
        assert_eq!(page.items(), &["a".to_string()]);
        assert_eq!(page.next_page_token(), None);
        assert_eq!(page.into_items(), vec!["a".to_string()]);
    }
}
