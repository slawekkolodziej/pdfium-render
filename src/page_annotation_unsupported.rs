//! Defines the [PdfPageUnsupportedAnnotation] struct, exposing functionality related to any
//! single annotation object of a type not supported by Pdfium.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation::PdfPageAnnotationType;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

/// A single `PdfPageAnnotation` of any annotation type not supported by Pdfium.
pub struct PdfPageUnsupportedAnnotation<'a> {
    annotation_type: PdfPageAnnotationType,
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageUnsupportedAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        annotation_type: PdfPageAnnotationType,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageUnsupportedAnnotation {
            annotation_type,
            handle: annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
                bindings,
            ),
            bindings,
        }
    }

    /// Returns the annotation type of this annotation recognized by Pdfium, but unsupported
    /// for creation, editing, or rendering.
    #[inline]
    pub fn get_type(&self) -> PdfPageAnnotationType {
        self.annotation_type
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageUnsupportedAnnotation<'a> {
    #[inline]
    fn handle(&self) -> FPDF_ANNOTATION {
        self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn objects_impl(&self) -> &PdfPageAnnotationObjects {
        &self.objects
    }

    #[inline]
    fn objects_mut_impl(&mut self) -> &mut PdfPageAnnotationObjects<'a> {
        &mut self.objects
    }
}
