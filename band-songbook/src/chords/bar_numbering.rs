//! Bar numbering utilities for chord charts.

use std::collections::HashMap;

use super::parse::parse;
use crate::model::{SectionItem, StructureItem};

/// Computes bar numbers for sections.
///
/// Returns a HashMap where:
/// - Key: section id
/// - Value: (Vec of bar numbers for each row, total bar count for the section)
pub fn barcount_map_of_structure(structure: &[StructureItem]) -> HashMap<String, (Vec<i32>, i32)> {
    let mut result = HashMap::new();
    let mut current_bar_count = 1;

    // Build a map of section id -> rows for Chords sections
    let chords_rows: HashMap<String, &Vec<String>> = structure
        .iter()
        .filter_map(|item| match &item.item {
            SectionItem::Chords(chords) => Some((item.id.clone(), &chords.rows)),
            _ => None,
        })
        .collect();

    for item in structure {
        match &item.item {
            SectionItem::Chords(chords) => {
                let mut row_numbers = Vec::new();
                for row in &chords.rows {
                    row_numbers.push(current_bar_count);
                    if let Ok(parsed) = parse(row) {
                        current_bar_count += parsed.bars.len() as i32 * parsed.repeat.n as i32;
                    }
                }
                result.insert(item.id.clone(), (row_numbers, current_bar_count));
            }
            SectionItem::Ref(ref_section) => {
                let mut row_numbers = Vec::new();
                if let Some(rows) = chords_rows.get(&ref_section.link) {
                    for row in *rows {
                        row_numbers.push(current_bar_count);
                        if let Ok(parsed) = parse(row) {
                            current_bar_count += parsed.bars.len() as i32 * parsed.repeat.n as i32;
                        }
                    }
                }
                result.insert(item.id.clone(), (row_numbers, current_bar_count));
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChordsSection, RefSection};

    #[test]
    fn test_barcount_map_of_structure() {
        let structure = vec![
            StructureItem {
                id: "intro".to_string(),
                item: SectionItem::Chords(ChordsSection {
                    title: "Intro".to_string(),
                    section_type: "intro".to_string(),
                    section_body: None,
                    color: None,
                    rows: vec!["Am|G".to_string(), "F|E".to_string()], // 2 bars each row
                }),
            },
            StructureItem {
                id: "couplet1".to_string(),
                item: SectionItem::Chords(ChordsSection {
                    title: "Couplet 1".to_string(),
                    section_type: "couplet".to_string(),
                    section_body: None,
                    color: None,
                    rows: vec!["C|G|Am|F".to_string(), "C|G|x2".to_string()], // 4 bars, then 2 bars x2
                }),
            },
            StructureItem {
                id: "refrain1".to_string(),
                item: SectionItem::Ref(RefSection {
                    title: "Refrain 1".to_string(),
                    section_type: None,
                    section_body: None,
                    color: None,
                    link: "intro".to_string(), // links to intro (2 bars + 2 bars)
                }),
            },
        ];

        let result = barcount_map_of_structure(&structure);

        // intro: row 0 starts at 1, row 1 starts at 3 (1 + 2 bars)
        // total after intro: 1 + 2 + 2 = 5
        let intro = result.get("intro").unwrap();
        assert_eq!(intro.0, vec![1, 3]);
        assert_eq!(intro.1, 5);

        // couplet1: row 0 starts at 5, row 1 starts at 9 (5 + 4 bars)
        // total after couplet1: 5 + 4 + (2 * 2) = 13
        let couplet1 = result.get("couplet1").unwrap();
        assert_eq!(couplet1.0, vec![5, 9]);
        assert_eq!(couplet1.1, 13);

        // refrain1 (ref to intro): row 0 starts at 13, row 1 starts at 15
        // total after refrain1: 13 + 2 + 2 = 17
        let refrain1 = result.get("refrain1").unwrap();
        assert_eq!(refrain1.0, vec![13, 15]);
        assert_eq!(refrain1.1, 17);
    }
}
