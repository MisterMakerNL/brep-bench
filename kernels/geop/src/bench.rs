//! A part's program, written step by step ([`Bench`]), and what the cases ask of it as they write
//! it: an edge or a face picked by a point on it, as a designer picks one by clicking it and the
//! program then names it.
use geop_core_math::{geop_error::GeopError, vector::Vector3, scalars::Scalar};
use geop_core_program::{operation::StepArgs, part::Part, program::Program, runner::Runner};
use geop_core_topology::{
    EdgeId, FaceId,
    distance::{Feature, PlacedFace, closest_points},
};
use geop_ops_model::{NamedModel, read_model};

/// Why a case was not built: geop's error, in its own words, or a case that refuses in its own
/// because geop has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<GeopError> for Refusal {
    fn from(e: GeopError) -> Self {
        Refusal(said(&e))
    }
}

impl From<&str> for Refusal {
    fn from(why: &str) -> Self {
        Refusal(why.to_owned())
    }
}

impl From<String> for Refusal {
    fn from(why: String) -> Self {
        Refusal(why)
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// What a geop error says, as a user is shown it: what went wrong at its root, and where it was
/// reported from — the step that failed, which is the context it was given last.
fn said(e: &GeopError) -> String {
    let root = e.root_message().trim();
    match e {
        GeopError::Context { message, .. } => format!("{root} (in {})", message.trim()),
        _ => root.to_owned(),
    }
}

/// How near a point must be to an edge or a face to pick it.
pub const PICK_TOLERANCE: f64 = 1e-6;

/// A part's program as it is written, and the part it has built so far.
pub struct Bench {
    program: Program,
    part: Part,
    runner: Runner,
}

impl Bench {
    /// An empty program, run by every operation geop's editor offers.
    pub fn new() -> Self {
        Bench { program: Program::new(), part: Part::default(), runner: geop_cad_base::runner() }
    }

    /// Adds the step `id`, `args` for the operation they are for, and runs the program as far
    /// as it: the steps before it are kept as they were built. A step that fails is the kernel
    /// declining, in its own words.
    pub fn step<A: StepArgs>(&mut self, id: &str, args: A) -> Result<(), Refusal> {
        self.program.push(id, args);
        self.program.run(&self.runner, &mut self.part)?;
        Ok(())
    }

    /// The program written so far.
    pub fn program(&self) -> &Program {
        &self.program
    }

    /// The part's model as the steps so far have built it: its topology and every entity's name.
    pub fn model(&self) -> Result<NamedModel, Refusal> {
        Ok(read_model(&self.part.current())?.into_owned())
    }

    /// The names of the edges `point` lies on.
    fn edges_through(&self, point: [f64; 3]) -> Result<Vec<String>, Refusal> {
        let part = self.model()?;
        let model = part.topology();
        let at = Feature::Point(Vector3::from_array(point.map(Scalar::from_f64)));
        let mut found = Vec::new();
        for (&id, edge) in model.edges.iter() {
            let d = closest_points(&at, &Feature::Curve(edge.curve.clone()))?;
            if d.distance.lower().to_f64() < PICK_TOLERANCE {
                found.push(name(&part, id)?);
            }
        }
        Ok(found)
    }

    /// The name of the one edge of the part through `point`, as a designer picks it.
    pub fn edge(&self, point: [f64; 3]) -> Result<String, Refusal> {
        match &self.edges_through(point)?[..] {
            [e] => Ok(e.clone()),
            found => Err(format!("{} edges through {point:?}: {found:?}", found.len()).into()),
        }
    }

    /// The names of the edges through each of `points`, one each.
    pub fn edges(&self, points: &[[f64; 3]]) -> Result<Vec<String>, Refusal> {
        points.iter().map(|&p| self.edge(p)).collect()
    }

    /// The name of the one face of the part through `point`, as a designer picks it.
    pub fn face(&self, point: [f64; 3]) -> Result<String, Refusal> {
        let part = self.model()?;
        let model = part.topology();
        let at = Feature::Point(Vector3::from_array(point.map(Scalar::from_f64)));
        let mut found = Vec::new();
        for &id in model.faces.keys() {
            let face = Feature::Face(PlacedFace::new(model, id, None)?);
            let d = closest_points(&at, &face)?;
            if d.distance.lower().to_f64() < PICK_TOLERANCE {
                found.push(face_name(&part, id)?);
            }
        }
        match &found[..] {
            [f] => Ok(f.clone()),
            found => Err(format!("{} faces through {point:?}: {found:?}", found.len()).into()),
        }
    }

    /// The names of the faces through each of `points`, one each.
    pub fn faces(&self, points: &[[f64; 3]]) -> Result<Vec<String>, Refusal> {
        points.iter().map(|&p| self.face(p)).collect()
    }

    /// The name of the part's one solid.
    pub fn solid(&self) -> Result<String, Refusal> {
        let part = self.model()?;
        let names: Vec<String> = (part.topology().solids.keys())
            .map(|&s| part.name(s).map(str::to_owned))
            .collect::<Result<_, _>>()?;
        match &names[..] {
            [s] => Ok(s.clone()),
            found => Err(format!("{} solids: {found:?}", found.len()).into()),
        }
    }
}

fn name(part: &NamedModel, edge: EdgeId) -> Result<String, Refusal> {
    Ok(part.name(edge)?.to_owned())
}

fn face_name(part: &NamedModel, face: FaceId) -> Result<String, Refusal> {
    Ok(part.name(face)?.to_owned())
}
