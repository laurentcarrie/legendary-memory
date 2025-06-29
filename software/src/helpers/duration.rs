use crate::model::model::{Song, StructureItemContent};
use chrono::TimeDelta;

pub fn duration_of_song(song: &Song) -> chrono::TimeDelta {
    let mut seconds: i64 = 0;
    for s in &song.structure {
        match &s.item {
            StructureItemContent::ItemChords(chords) => {
                //@ todo replace 4 with signature
                seconds += (chords.nb_bars as f64 * 4f64 / song.tempo as f64 * 60f64) as i64;
            }
            StructureItemContent::ItemRef(cref) => {
                seconds += (cref.nb_bars as f64 * 4f64 / song.tempo as f64 * 60f64) as i64;
            }
            StructureItemContent::ItemNewColumn => {}
            StructureItemContent::ItemHRule(_) => {}
        }
    }
    let d = chrono::Duration::new(seconds, 0).expect("huh, duration too big");
    d
}

mod tests {
    use crate::helpers::duration::duration_of_song;
    use crate::model::model::{
        Chords, Row, Song, StructureItem, StructureItemContent, TimeSignature,
    };
    #[test]
    fn test_duration() -> Result<(), Box<dyn std::error::Error>> {
        let song = Song {
            title: "".to_string(),
            author: "".to_string(),
            tempo: 60,
            time_signature: TimeSignature { top: 4, low: 4 },
            pdfname: "".to_string(),
            texfiles: vec![],
            builddir: Default::default(),
            lilypondfiles: vec![],
            wavfiles: vec![],
            date: "".to_string(),
            structure: vec![StructureItem {
                item: StructureItemContent::ItemChords(Chords {
                    section_title: "".to_string(),
                    section_id: "".to_string(),
                    section_type: "".to_string(),
                    section_body: "".to_string(),
                    row_start_bar_number: 0,
                    nb_bars: 16,
                    rows: vec![Row {
                        row_start_bar_number: 0,
                        // bars: vec![Bar{ chords: vec!["A".to_string()], time_signature: Some(TimeSignature{ top: 4, low: 4 }) }],
                        bars: vec![],
                        repeat: 1,
                    }],
                }),
            }],
            srcdir: "".to_string(),
        };
        let d = duration_of_song(&song);
        println!("{:?}", &d);
        // 15 bars, => 15*4 temps, tempo is 60, so 60 seconds
        assert!(d.num_seconds() == 64);
        assert!(d.num_minutes() == 1);

        let mut cumul = chrono::Duration::new(0, 0).unwrap();
        cumul += d;
        assert!(cumul.num_seconds() == 64);
        assert!(cumul.num_minutes() == 1);

        Ok(())
    }
}
