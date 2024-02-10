use std::io::BufRead;
use std::io::Lines;
use std::str::FromStr;
use std::{clone, convert::From};

// https://osu.ppy.sh/wiki/Osu_(file_format)

#[derive(Debug)]
pub enum BeatmapMode {
    Standard = 0,
    Taiko = 1,
    CatchTheBeat = 2,
    Mania = 3,
}

impl Default for BeatmapMode {
    fn default() -> BeatmapMode {
        BeatmapMode::Standard
    }
}

#[derive(Debug)]
pub enum BeatmapModeError {
    Parse(std::num::ParseIntError),
    Unknown,
}

impl From<std::num::ParseIntError> for BeatmapModeError {
    fn from(err: std::num::ParseIntError) -> BeatmapModeError {
        BeatmapModeError::Parse(err)
    }
}

impl FromStr for BeatmapMode {
    type Err = BeatmapModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i32::from_str(s)? {
            0 => Ok(BeatmapMode::Standard),
            1 => Ok(BeatmapMode::Taiko),
            2 => Ok(BeatmapMode::CatchTheBeat),
            3 => Ok(BeatmapMode::Mania),
            _ => Err(BeatmapModeError::Unknown),
        }
    }
}

#[derive(Debug, Default)]
pub struct BeatmapGeneral {
    pub audio_filename: String,
    pub audio_lead_in: i32,
    pub preview_time: i32,
    pub countdown: bool,
    pub sample_set: String,
    pub stack_leniency: f32,
    pub mode: BeatmapMode,
    pub letterbox_in_breaks: bool,
    pub widescreen_storyboard: bool,
}

#[derive(Debug, Default)]
pub struct BeatmapMetadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: u64,
    pub beatmap_set_id: u64,
}

#[derive(Debug, Default)]
pub struct BeatmapDifficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,
}

#[derive(Debug, Default, Clone)]
pub struct TimingPoint {
    pub offset: i32,
    pub milliseconds_per_beat: f32,
    pub meter: i32, // Number of beats in a measure
    pub sample_type: i32,
    pub sample_set: i32,
    pub volume: i32, // Hit sound volume, 0 to 100
    pub kiai_mode: bool,
    pub inherited: bool,
}

impl TimingPoint {
    pub fn inherit(&self, prev: &TimingPoint) -> TimingPoint {
        let mut point = self.clone();
        if !self.inherited {
            return point;
        }

        point.milliseconds_per_beat = prev.milliseconds_per_beat + self.milliseconds_per_beat;
        point.inherited = prev.inherited;
        return point;
    }
}

#[derive(Debug, Default, Clone)]
pub struct HitObjectBase {
    pub x: i32,           // 0 to 512
    pub y: i32,           // 0 to 384
    pub time: i32,        // In ms
    pub object_type: i32, // Bitmap
    pub hit_sound: i32,
}

#[derive(Debug, Clone)]
pub enum HitObject {
    // Standard
    Circle {
        base: HitObjectBase,
    },
    Slider {
        base: HitObjectBase,
        curve_points: Vec<(i32, i32)>,
        slider_type: i32,
        //curve
        repeat: i32,
        //pixel_length
        edge_hitsound: i32,
        edge_addition: i32,
    },
    Spinner {
        base: HitObjectBase,
        end_time: i32,
    },

    // Mania
    LongNote {
        base: HitObjectBase,
        end_time: i32,
    },

    Other(HitObjectBase),
}

impl HitObject {
    pub fn base(&self) -> &HitObjectBase {
        match self {
            &HitObject::Circle { ref base, .. }
            | &HitObject::Slider { ref base, .. }
            | &HitObject::Spinner { ref base, .. }
            | &HitObject::LongNote { ref base, .. }
            | &HitObject::Other(ref base) => base,
        }
    }
}

#[derive(Debug, Default)]
pub struct Beatmap {
    pub general: BeatmapGeneral,
    pub metadata: BeatmapMetadata,
    pub difficulty: BeatmapDifficulty,
    pub timing_points: Vec<TimingPoint>,
    pub hit_objects: Vec<HitObject>,
}

fn parse_bool(s: &str) -> Result<bool, &'static str> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err("malformed bool"),
    }
}

pub struct Parser<U> {
    lines: Lines<U>,
    section: Option<String>,
    done: bool,
}

impl<U> Parser<U>
where
    U: BufRead,
{
    pub fn new(lines: Lines<U>) -> Parser<U> {
        Parser {
            lines,
            section: None,
            done: false,
        }
    }

    fn read_header(&mut self) -> Result<String, &'static str> {
        match self.lines.by_ref().next() {
            Some(Ok(l)) => {
                if !l.starts_with("osu file format") {
                    Err("malformed header")
                } else {
                    Ok(l)
                }
            }
            Some(Err(_)) => Err("io error"),
            None => Err("empty file"),
        }
    }

    fn read_line(&mut self) -> Option<Result<String, &'static str>> {
        for line in self.lines.by_ref() {
            match line {
                Ok(l) => {
                    let s = l.trim();
                    if s.is_empty() || s.starts_with("//") {
                        continue;
                    }

                    if s.starts_with('[') {
                        let name = s.trim_matches(|c| c == '[' || c == ']').trim().to_string();
                        self.section = Some(name);
                        return None;
                    }

                    return Some(Ok(s.to_string()));
                }
                Err(_) => {
                    return Some(Err("io error"));
                }
            }
        }

        self.done = true;
        None
    }

    fn read_section(&mut self) -> Option<Result<String, &'static str>> {
        if self.done {
            return None;
        }

        if self.section.is_none() {
            match self.read_line() {
                None => (),
                Some(Ok(_)) => return Some(Err("expected a section, not a field")),
                Some(Err(err)) => return Some(Err(err)),
            }
        }

        match self.section.take() {
            None => Some(Err("expected a section")),
            Some(name) => Some(Ok(name)),
        }
    }

    fn read_key_value(&mut self) -> Option<Result<(String, String), &'static str>> {
        if let Some(line) = self.read_line() {
            match line {
                Ok(l) => {
                    let kv: Vec<&str> = l.splitn(2, ':').collect();
                    if kv.len() != 2 {
                        return Some(Err("malformed key-value field"));
                    }
                    Some(Ok((kv[0].trim().to_string(), kv[1].trim().to_string())))
                }
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }

    fn parse_section(&mut self, name: String, beatmap: &mut Beatmap) -> Result<(), &'static str> {
        println!("Parsing {name}");
        match name.as_ref() {
            "General" => self.parse_general(&mut beatmap.general),
            "Metadata" => self.parse_metadata(&mut beatmap.metadata),
            "Difficulty" => self.parse_difficulty(&mut beatmap.difficulty),
            "TimingPoints" => self.parse_timing_points(&mut beatmap.timing_points),
            "HitObjects" => self.parse_hit_objects(&mut beatmap.hit_objects),
            _ => {
                while self.read_line().is_some() {}
                Ok(())
            }
        }
    }

    fn parse_general(&mut self, section: &mut BeatmapGeneral) -> Result<(), &'static str> {
        while let Some(res) = self.read_key_value() {
            let (k, v) = res?;
            match k.as_ref() {
                "AudioFilename" => section.audio_filename = v,
                "AudioLeadIn" => section.audio_lead_in = i32::from_str(&v).unwrap(),
                "PreviewTime" => section.preview_time = i32::from_str(&v).unwrap(),
                "Countdown" => section.countdown = parse_bool(&v).unwrap(),
                "SampleSet" => section.sample_set = v,
                "StackLeniency" => section.stack_leniency = f32::from_str(&v).unwrap(),
                "Mode" => section.mode = BeatmapMode::from_str(&v).unwrap(),
                "LetterboxInBreaks" => section.letterbox_in_breaks = parse_bool(&v).unwrap(),
                "WidescreenStoryboard" => section.widescreen_storyboard = parse_bool(&v).unwrap(),
                _ => (),
            }
        }

        Ok(())
    }

    fn parse_metadata(&mut self, section: &mut BeatmapMetadata) -> Result<(), &'static str> {
        while let Some(res) = self.read_key_value() {
            let (k, v) = res?;
            match k.as_ref() {
                "Title" => section.title = v,
                "TitleUnicode" => section.title_unicode = v,
                "Artist" => section.artist = v,
                "ArtistUnicode" => section.artist_unicode = v,
                "Creator" => section.creator = v,
                "Version" => section.version = v,
                "Source" => section.source = v,
                "Tags" => section.tags = v.split(' ').map(|s| s.to_string()).collect(),
                "BeatmapID" => section.beatmap_id = u64::from_str(&v).unwrap(),
                "BeatmapSetID" => section.beatmap_set_id = u64::from_str(&v).unwrap(),
                _ => (),
            }
        }

        Ok(())
    }

    fn parse_difficulty(&mut self, section: &mut BeatmapDifficulty) -> Result<(), &'static str> {
        while let Some(res) = self.read_key_value() {
            let (k, v) = res?;
            match k.as_ref() {
                "HPDrainRate" => section.hp_drain_rate = f32::from_str(&v).unwrap(),
                "CircleSize" => section.circle_size = f32::from_str(&v).unwrap(),
                "OverallDifficulty" => section.overall_difficulty = f32::from_str(&v).unwrap(),
                "ApproachRate" => section.approach_rate = f32::from_str(&v).unwrap(),
                "SliderMultiplier" => section.slider_multiplier = f32::from_str(&v).unwrap(),
                "SliderTickRate" => section.slider_tick_rate = f32::from_str(&v).unwrap(),
                _ => (),
            }
        }

        Ok(())
    }

    fn parse_timing_points(&mut self, section: &mut Vec<TimingPoint>) -> Result<(), &'static str> {
        while let Some(res) = self.read_line() {
            let l = res?;
            let values: Vec<&str> = l.split(',').collect();
            if values.len() != 8 {
                return Err("malformed timing point");
            }

            section.push(TimingPoint {
                offset: i32::from_str(values[0]).unwrap(),
                milliseconds_per_beat: f32::from_str(values[1]).unwrap(),
                meter: i32::from_str(values[2]).unwrap(),
                sample_type: i32::from_str(values[3]).unwrap(),
                sample_set: i32::from_str(values[4]).unwrap(),
                volume: i32::from_str(values[5]).unwrap(),
                inherited: !parse_bool(values[6]).unwrap(),
                kiai_mode: parse_bool(values[7]).unwrap(),
            });
        }

        Ok(())
    }

    fn parse_hit_objects(&mut self, section: &mut Vec<HitObject>) -> Result<(), &'static str> {
        let mut i = 0;
        while let Some(res) = self.read_line() {
            let l = res?;
            let values: Vec<&str> = l.split(',').collect();

            println!("Object {i}|Values: {values:?}");
            i += 1;

            if values.len() < 6 {
                return Err("malformed hit object");
            }

            let base = HitObjectBase {
                x: i32::from_str(values[0]).unwrap(),
                y: i32::from_str(values[1]).unwrap(),
                time: i32::from_str(values[2]).unwrap(),
                object_type: i32::from_str(values[3]).unwrap(),
                hit_sound: i32::from_str(values[4]).unwrap(),
            };

            // TODO
            let object = if base.object_type & 0x01 != 0 {
                HitObject::Circle { base }
            } else if base.object_type & 0x02 != 0 {
                HitObject::Slider {
                    base,
                    curve_points: {
                        println!(
                            "parsing slider curve: {:?}",
                            values.get(5).unwrap().split('|').collect::<Vec<&str>>()
                        );
                        let mut out = Vec::new();
                        for pt in values
                            .get(5)
                            .unwrap()
                            .split('|')
                            .collect::<Vec<&str>>()
                            .iter()
                        {
                            let vals = pt.split(':').collect::<Vec<&str>>();
                            if vals.len() != 2 {
                                println!("Skipping {pt}");
                                continue;
                            }
                            println!("Vals: {vals:?}");
                            // let mut c = false;
                            // for v in &vals{
                            //     if v.contains('-'){
                            //         error!("Skipping negative");
                            //         c =true;
                            //         break;
                            //     }
                            // }

                            // if c{
                            //     continue
                            // }

                            out.push((
                                i32::from_str(vals.first().unwrap()).unwrap(),
                                i32::from_str(vals.get(1).unwrap()).unwrap(),
                            ));
                        }
                        out
                    },
                    slider_type: 0,
                    repeat: 0,
                    edge_hitsound: 0,
                    edge_addition: 0,
                }
            } else if base.object_type & 0x08 != 0 {
                HitObject::Spinner { base, end_time: 0 }
            } else if base.object_type & 0x80 != 0 {
                let additional: Vec<&str> = values[5].split(':').collect();
                HitObject::LongNote {
                    base,
                    end_time: i32::from_str(additional[0]).unwrap(),
                }
            } else {
                HitObject::Other(base)
            };

            section.push(object);
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<Beatmap, &'static str> {
        let mut beatmap = Beatmap::default();

        self.read_header()?;
        loop {
            match self.read_section() {
                None => break,
                Some(Err(err)) => return Err(err),
                Some(Ok(name)) => {
                    self.parse_section(name, &mut beatmap)?;
                }
            }
        }

        Ok(beatmap)
    }
}