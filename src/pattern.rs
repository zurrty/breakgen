use apres::*;
use macroquad::prelude::*;
use quick_csv::Csv;

#[derive(Clone)]
pub struct Note {
    pub key: u8,
    pub len: usize,
    pub pos: usize,
}

#[derive(Clone)]
pub struct Pattern {
    pub notes: Vec<Note>,
    pub drums: u8,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            notes: vec![],
            drums: 3,
        }
    }
    pub fn from_csv(src: &str) -> Result<Self, String> {
        let mut this = Self::new();
        let csv = Csv::from_string(src);
        for line in csv {
            let row = match line {
                Ok(v) => v,
                Err(_) => continue,
            };
            let mut cols = match row.columns() {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            if cols.len() < 3 {
                continue;
            }
            let pos: usize = cols.next().unwrap().parse().unwrap();
            let len: usize = cols.next().unwrap().parse().unwrap();
            let key: u8 = cols.next().unwrap().parse().unwrap();
            this.notes.push(Note { pos, len, key })
        }
        Ok(this)
    }
    pub fn length(&self) -> usize {
        let mut size = 0;
        for n in self.notes.clone() {
            size += n.pos + n.len;
        }
        size
    }
    pub fn to_midi(&self) -> MIDI {
        let mut midi = MIDI::new();
        midi.push_event(0, 0, MIDIEvent::SetTempo(140));
        for note in self.notes.clone() {
            midi.push_event(0, note.pos, MIDIEvent::NoteOn(0, note.key, 100));
            midi.push_event(0, note.len, MIDIEvent::NoteOff(0, note.key, 100));
        }

        return midi;
    }
    pub fn draw(&self, r: Rect) {
        draw_rectangle(r.x, r.y, r.w, r.h, BLACK);
        let h = r.h / self.drums as f32;
        let base_w = r.w / self.length() as f32;
        let mut x0 = 0.0;
        for note in self.notes.clone() {
            let x1 = note.pos as f32;
            let y = r.y + (note.key as f32 * h);
            let w = note.len as f32;
            draw_rectangle(x1 * base_w + x0 + 1.0, y, w * base_w - 2.0, h, WHITE);
            x0 += x1 + w;
        }
    }
    pub fn generate(length: usize) -> Result<Self, String> {
        let length = length * 120;
        let mut this = Self::new();
        let mut patterns: Vec<Pattern> = match std::fs::read_dir("patterns") {
            Ok(v) => v
                .filter_map(|f| f.ok())
                .map(|f| std::fs::read_to_string(f.path()))
                .filter_map(|f| f.ok())
                .filter_map(|f| Pattern::from_csv(&f).ok())
                .collect(),
            Err(e) => return match e.kind() {
                std::io::ErrorKind::NotFound => Err(String::from("'patterns' directory not found!")),
                _ => Err(e.to_string())
            },
        };
        if patterns.len() < 1 {
            return Err(String::from("No patterns found."));
        }
        let mut i: usize = 0;
        while i < length {
            let p = rand::rand() as usize % patterns.len();
            let pat: Pattern = patterns[p].clone();
            if pat.length() < 1 {
                patterns.remove(p);
                if patterns.len() < 1 {
                    return Err(String::from("All patterns are empty!"))
                }
            }
            this.notes.append(&mut pat.notes.clone());
            i = i + pat.length();
        }
        Ok(this)
    }
}
