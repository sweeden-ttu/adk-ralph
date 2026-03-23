# Legal Form Extraction and Automated Fill Pipeline

Build a document-processing subsystem for the Trustworthy AI project that performs end-to-end PDF form extraction, answer generation, overlay rendering, and final PDF assembly.

## Purpose

Create a pipeline that converts each page of a legal/government PDF form into machine-usable assets, fills extracted fields using an expert tax/accounting agent, and generates a high-resolution completed PDF with transparent overlays.

## Workflow

1. Upload a PDF form package.
2. Extract each PDF page as a high-quality image.
3. Detect all fillable fields per page with coordinates and bounding boxes.
4. Infer a descriptive label for each field and output structured JSON with an empty `answer` value.
5. Use a specialist form-filling agent (accountant and tax advisor persona) to populate answers in JSON.
6. Render transparent PNG overlays from answered JSON for each page.
7. Composite overlays onto original page images and rebuild a single high-resolution output PDF.

## Features

- **Page Extraction**
  - Convert input PDF into ordered page image set (loss-minimized, print-safe resolution)
  - Preserve page dimensions and coordinate consistency for later overlays

- **Field Detection and Labeling**
  - Detect candidate form fields and capture `x`, `y`, `width`, `height`, `page`
  - Classify field type (text, checkbox, signature, date, amount)
  - Infer human-readable labels from nearby text context
  - Emit per-page JSON and full-document JSON index

- **Answering Agent**
  - Specialized tax/accounting reasoning profile for legal/tax form completion
  - Fill `answer` values while preserving original extraction metadata
  - Flag uncertain fields with confidence and follow-up questions

- **Overlay and PDF Composer**
  - Generate transparent PNG overlays from answered fields for each page
  - Pixel-accurate placement using extracted bounding box coordinates
  - Merge base pages + overlays into a single high-resolution PDF
  - Produce downloadable artifacts for each stage

## Technical Requirements

- Deterministic coordinate system from extraction through overlay rendering
- JSON schema validation for extracted and answered field documents
- Support batch processing of multiple PDFs
- Audit metadata: model version, timestamp, operator, and provenance chain
- Clear separation of extractor, answerer, renderer, and composer modules

## Outputs

- `pages/` image assets for each source page
- `fields.extracted.json` with labels and empty answers
- `fields.answered.json` populated by the accounting/tax advisor agent
- `overlays/` transparent PNG files per page
- `final-output.pdf` high-resolution merged document