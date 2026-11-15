use sigmaos_craft_pdf::{PdfDocument, Page, PageSize};

#[test]
fn pdf_document_integration_aaa() {
    let mut d = PdfDocument::new("My Doc").unwrap();
    d.add_page(Page { id: "p1".into(), page_size: PageSize::A4, rotation: 0 }).unwrap();
    assert_eq!(d.page_count(), 1);
}
