use std::error::Error;
use std::fs;

enum Parts {
    Source,
    Void,
    Splitter,
    Beam,
    Other(char),
}

struct Manifold {
    segments: Vec<Vec<(usize, Parts)>>,
    beam_positions: Vec<usize>,
    split_count: usize,
}

impl Manifold {
    fn new(segments_raw: Vec<&str>) -> Self {
        let segments = segments_raw
            .iter()
            .map(|segment| {
                segment
                    .chars()
                    .enumerate()
                    .map(|(position, part)| match part {
                        '.' => (position, Parts::Void),
                        '^' => (position, Parts::Splitter),
                        'S' => (position, Parts::Source),
                        _ => (position, Parts::Other(part)),
                    })
                    .collect::<Vec<(usize, Parts)>>()
            })
            .collect::<Vec<Vec<(usize, Parts)>>>();
        Self {
            segments,
            beam_positions: Vec::new(),
            split_count: 0,
        }
    }

    fn propagate_in_segment(&self) {}

    fn popagate(&self) {
        self.segments.iter().for_each(self.propagate_in_segment);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "test";
    let file = fs::read_to_string(path)?;
    let segments = file.lines().collect();
    let manifold = Manifold::new(segments);

    Ok(())
    // let beam = Beam::new(origin);
}
