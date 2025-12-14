use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum SplitterDirection {
    LEFT,
    RIGHT,
}

struct Source {
    position: usize,
}

#[derive(Clone)]
struct Splitter {
    position: usize,
    direction: SplitterDirection,
    flip_count: usize,
}

impl Splitter {
    fn new(position: usize) -> Self {
        Self {
            position,
            direction: SplitterDirection::LEFT,
            flip_count: 0,
        }
    }

    fn flip(&mut self) {
        self.direction = match self.direction {
            SplitterDirection::LEFT => SplitterDirection::RIGHT,
            SplitterDirection::RIGHT => SplitterDirection::LEFT,
        };
        self.flip_count += 1;
    }
}

enum Part {
    Source(Source),
    Void,
    Splitter(Splitter),
    Other(()),
}

struct Manifold {
    segments: Vec<Vec<Part>>,
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
                        '.' => Part::Void,
                        '^' => Part::Splitter(Splitter::new(position)),
                        'S' => Part::Source(Source { position }),
                        _ => Part::Other(()),
                    })
                    .collect::<Vec<Part>>()
            })
            .collect::<Vec<Vec<Part>>>();
        Self { segments }
    }

    fn any_bottom_splitter_reached_count(&self, count: usize) -> bool {
        let last_segment = self
            .segments
            .get(self.segments.len().saturating_sub(2))
            .expect("No vector to grab");
        last_segment
            .iter()
            .filter(|part| matches!(part, Part::Splitter(_)))
            .any(|part| match part {
                Part::Splitter(splitter) => {
                    let ret = splitter.flip_count >= count;
                    println!("{}{}", ret, splitter.flip_count);
                    ret
                }
                _ => false,
            })
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Ray {
    position: usize,
}

struct Beam {
    rays: Vec<Ray>,
    split_count: usize,
}

impl Beam {
    fn new() -> Self {
        Self {
            rays: Vec::new(),
            split_count: 0,
        }
    }

    fn spawn_beam(rays: &mut Vec<Ray>, position: usize) {
        let new_ray = Ray { position };
        if !rays.contains(&new_ray) {
            rays.push(new_ray);
        }
    }

    fn split_beam(rays: &mut Vec<Ray>, splitter: &Splitter) {
        let splitter_position = splitter.position;
        let split_left = splitter_position.saturating_sub(1);
        let split_right = splitter_position.saturating_add(1);
        let beam_position_idx = rays
            .iter()
            .position(|ray| splitter_position == ray.position)
            .expect("No index found");
        rays.remove(beam_position_idx);
        Self::spawn_beam(rays, split_left);
        Self::spawn_beam(rays, split_right);
    }

    fn simulate_beam_in_segment(&mut self, segment: &mut [Part]) {
        for part in segment.iter_mut() {
            match part {
                Part::Source(source) => Self::spawn_beam(&mut self.rays, source.position),
                Part::Void => (),
                Part::Splitter(splitter) => {
                    let is_beam_at_splitter = self.rays.contains(&Ray {
                        position: splitter.position,
                    });
                    if is_beam_at_splitter {
                        Self::split_beam(&mut self.rays, splitter);
                        self.split_count += 1
                    }
                }
                Part::Other(_) => (),
            }
        }
    }
}

// Quantum beam needs to be casted until every splitter in manifold has been flipped twice
#[derive(Clone)]
struct QuantumBeam {
    rays: Vec<Ray>,
}

impl QuantumBeam {
    fn new() -> Self {
        Self { rays: Vec::new() }
    }

    fn clear(&mut self) {
        self.rays = Vec::new();
    }

    fn spawn_quantum_beam(rays: &mut Vec<Ray>, position: usize) {
        let new_ray = Ray { position };
        if !rays.contains(&new_ray) {
            rays.push(new_ray);
        }
    }
    fn direct_quantum_beam(rays: &mut Vec<Ray>, splitter: &mut Splitter) {
        let beam_direction = match splitter.direction {
            SplitterDirection::LEFT => splitter.position.saturating_sub(1),
            SplitterDirection::RIGHT => splitter.position.saturating_add(1),
        };
        let beam_position_idx = rays
            .iter()
            .position(|ray| ray.position == splitter.position)
            .expect("No index found");
        rays.remove(beam_position_idx);
        Self::spawn_quantum_beam(rays, beam_direction);
    }

    fn simulate_quantum_beam_in_segment(&mut self, segment: &mut [Part]) {
        for part in segment.iter_mut() {
            match part {
                Part::Source(source) => Self::spawn_quantum_beam(&mut self.rays, source.position),
                Part::Void => (),
                Part::Splitter(splitter) => {
                    let is_beam_above = self.rays.contains(&Ray {
                        position: splitter.position,
                    });
                    if is_beam_above {
                        Self::direct_quantum_beam(&mut self.rays, splitter);
                        splitter.flip();
                    }
                }
                Part::Other(_) => (),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "test";
    let file = fs::read_to_string(path)?;
    let segments = file.lines().collect();
    let mut manifold = Manifold::new(segments);
    let mut beam = Beam::new();
    for segment in &mut manifold.segments {
        beam.simulate_beam_in_segment(segment);
    }

    println!("part 1: {}", beam.split_count);

    let mut quantum_beam = QuantumBeam::new();
    let mut timelines = 0;
    while !manifold.any_bottom_splitter_reached_count(2) {
        quantum_beam.clear();
        for segment in &mut manifold.segments {
            quantum_beam.simulate_quantum_beam_in_segment(segment);
        }
        timelines += 1;
    }

    println!("part 2: {}", timelines);

    Ok(())
}
