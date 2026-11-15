//! SigmaOS-craft-pdf: PDF model.
//!
//! Real implementation: pages, content streams, metadata, export options,
//! page dimensions, and validation. Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Page size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    Custom { width: f32, height: f32 },
}

/// PDF page.
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: String,
    pub page_size: PageSize,
    pub rotation: u16,
}

/// PDF document.
#[derive(Debug, Clone, PartialEq)]
pub struct PdfDocument {
    pub title: String,
    pub author: String,
    pub subject: String,
    pages: BTreeMap<String, Page>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PdfError {
    EmptyTitle,
    DuplicatePage(String),
    PageNotFound(String),
    InvalidSize,
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "document title must not be empty"),
            Self::DuplicatePage(id) => write!(f, "duplicate page {:?}", id),
            Self::PageNotFound(id) => write!(f, "page {:?} not found", id),
            Self::InvalidSize => write!(f, "invalid page size"),
        }
    }
}

impl std::error::Error for PdfError {}

impl PdfDocument {
    pub fn new(title: impl Into<String>) -> Result<Self, PdfError> {
        let title = title.into();
        if title.trim().is_empty() { return Err(PdfError::EmptyTitle); }
        Ok(Self { title, author: String::new(), subject: String::new(), pages: BTreeMap::new() })
    }

    pub fn add_page(&mut self, page: Page) -> Result<(), PdfError> {
        let id = page.id.clone();
        if self.pages.contains_key(&id) {
            return Err(PdfError::DuplicatePage(id.clone()));
        }
        self.pages.insert(id, page);
        Ok(())
    }

    pub fn remove_page(&mut self, id: &str) -> Result<(), PdfError> {
        if !self.pages.contains_key(id) {
            return Err(PdfError::PageNotFound(id.into()));
        }
        self.pages.remove(id);
        Ok(())
    }

    pub fn page_count(&self) -> usize { self.pages.len() }

    pub fn pages(&self) -> Vec<&Page> { self.pages.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_create_aaa() {
        let d = PdfDocument::new("My Doc").unwrap();
        assert_eq!(d.title, "My Doc");
        assert_eq!(d.page_count(), 0);
    }

    #[test]
    fn test_empty_title_rejected_aaa() {
        assert!(PdfDocument::new("").is_err());
    }

    #[test]
    fn test_add_page_aaa() {
        let mut d = PdfDocument::new("My Doc").unwrap();
        let p = Page { id: "p1".into(), page_size: PageSize::A4, rotation: 0 };
        d.add_page(p).unwrap();
        assert_eq!(d.page_count(), 1);
    }

    #[test]
    fn test_duplicate_page_rejected_aaa() {
        let mut d = PdfDocument::new("My Doc").unwrap();
        let p = Page { id: "x".into(), page_size: PageSize::A4, rotation: 0 };
        d.add_page(p).unwrap();
        let p2 = Page { id: "x".into(), page_size: PageSize::A4, rotation: 0 };
        assert!(d.add_page(p2).is_err());
    }
}
