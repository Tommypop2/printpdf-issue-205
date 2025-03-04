use std::fs;

use printpdf::*;

fn main() {
    let mut doc = PdfDocument::new("My first PDF");

    let helvetica_bytes = include_bytes!("../fonts/Helvetica.ttf");
    let font = ParsedFont::from_bytes(helvetica_bytes, 33, &mut vec![]).unwrap();
    let font_id = doc.add_font(&font);

    let text_pos = Point {
        x: Mm(10.0).into(),
        y: Mm(277.0).into(),
    }; // from bottom left

    let page1_contents = vec![
        Op::SetLineHeight { lh: Pt(12.0) },
        Op::SetTextCursor { pos: text_pos },
        Op::SetFontSize {
            size: Pt(12.0),
            font: font_id.clone(),
        },
        // Red Text
        Op::SetFillColor {
            col: color::Color::Rgb(Rgb {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                icc_profile: None,
            }),
        },
        Op::SetOutlineColor {
            col: color::Color::Rgb(Rgb {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                icc_profile: None,
            }),
        },
        Op::SetTextRenderingMode {
            mode: TextRenderingMode::Fill,
        },
        Op::WriteText {
            items: vec![TextItem::Text(
                "Text rendered with rendering mode: fill".into(),
            )],
            font: font_id.clone(),
        },
        Op::AddLineBreak,
        Op::SetTextRenderingMode {
            mode: TextRenderingMode::Stroke,
        },
        Op::WriteText {
            items: vec![TextItem::Text(
                "Text rendered with rendering mode: stroke".into(),
            )],
            font: font_id.clone(),
        },
        // Blue Text
        Op::AddLineBreak,
        Op::SetFillColor {
            col: color::Color::Rgb(Rgb {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                icc_profile: None,
            }),
        },
        Op::SetOutlineColor {
            col: color::Color::Rgb(Rgb {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                icc_profile: None,
            }),
        },
        Op::SetTextRenderingMode {
            mode: TextRenderingMode::Fill,
        },
        Op::WriteText {
            items: vec![TextItem::Text(
                "Text rendered with rendering mode: fill".into(),
            )],
            font: font_id.clone(),
        },
        Op::AddLineBreak,
        Op::SetTextRenderingMode {
            mode: TextRenderingMode::Stroke,
        },
        Op::WriteText {
            items: vec![TextItem::Text(
                "Text rendered with rendering mode: stroke".into(),
            )],
            font: font_id.clone(),
        },
    ];

    let save_options = PdfSaveOptions {
        subset_fonts: true, // auto-subset fonts on save
        ..Default::default()
    };

    let page1 = PdfPage::new(Mm(210.0), Mm(297.0), page1_contents);
    let mut warnings = vec![];
    let pdf_bytes: Vec<u8> = doc
        .with_pages(vec![page1])
        .save(&save_options, &mut warnings);
    dbg!(warnings);
    fs::write("./repro.pdf", pdf_bytes).unwrap();
}
