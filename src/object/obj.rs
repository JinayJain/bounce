use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    iter::Peekable,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};

use crate::{
    geometry::{Point, Ray},
    material::Material,
    object::Tri,
};

use super::{bvh::Primitive, Visible, VisibleHit, VisibleList};

pub struct Object {
    triangles: Vec<Tri>,
}

impl Object {
    pub fn new(path: impl Into<PathBuf>, material: Arc<dyn Material>) -> Result<Self> {
        let path = path.into();

        let file = File::open(path)?;
        let file = BufReader::new(file);

        let tokens = tokenize(file);
        let (vertices, faces) = parse_obj(tokens);

        let triangles = triangulate(vertices, faces, material);

        Ok(Self { triangles })
    }

    pub fn to_primitives(self) -> Vec<Arc<dyn Primitive>> {
        self.triangles
            .into_iter()
            .map(|tri| {
                let tri: Arc<dyn Primitive> = Arc::new(tri);
                tri
            })
            .collect()
    }
}

fn triangulate(
    vertices: Vec<Point<f64>>,
    faces: Vec<Vec<usize>>,
    material: Arc<dyn Material>,
) -> Vec<Tri> {
    assert!(
        faces.iter().all(|x| x.len() == 3),
        "TODO: Handle non-triangle object faces"
    );

    let mut triangles = Vec::new();

    for face in faces {
        let face: Vec<_> = face.iter().map(|x| x - 1).collect();

        let a = vertices[face[0]];
        let b = vertices[face[1]];
        let c = vertices[face[2]];

        let tri = Tri::new(a, b, c, Arc::clone(&material));
        triangles.push(tri);
    }

    triangles
}

fn tokenize(file: BufReader<File>) -> impl Iterator<Item = String> {
    file.lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| String::from(x))
                .collect::<Vec<_>>()
        })
        .flatten()
        .skip_while(|x| is_skip_token(x))
}

fn is_skip_token(token: &String) -> bool {
    token == "\\" || token.trim().is_empty()
}

fn parse_obj(tokens: impl Iterator<Item = String>) -> (Vec<Point<f64>>, Vec<Vec<usize>>) {
    let mut tokens = tokens.peekable();

    let mut points = Vec::new();
    let mut faces = Vec::new();

    while let Some(token) = tokens.peek() {
        let token = token.as_str();

        match token {
            "v" => points.push(vertex(&mut tokens)),
            "f" => faces.push(face(&mut tokens)),
            _ => panic!("Unexpected token {}", token),
        }
    }

    (points, faces)
}

fn vertex(tokens: &mut Peekable<impl Iterator<Item = String>>) -> Point<f64> {
    let _vertex = tokens.next().expect("Missing 'v' for vertex");

    let x: f64 = tokens
        .next()
        .expect("Expected x coordinate for vertex")
        .parse()
        .expect("Unable to parse float");
    let y: f64 = tokens
        .next()
        .expect("Expected y coordinate for vertex")
        .parse()
        .expect("Unable to parse float");
    let z: f64 = tokens
        .next()
        .expect("Expected z coordinate for vertex")
        .parse()
        .expect("Unable to parse float");

    Point::new(x, y, z)
}

fn face(tokens: &mut Peekable<impl Iterator<Item = String>>) -> Vec<usize> {
    let mut vertices = Vec::new();

    let _face = tokens.next().expect("Missing 'f' for face");

    while let Some(token) = tokens.peek() {
        match token.parse::<usize>() {
            Ok(idx) => {
                tokens.next();
                vertices.push(idx);
            }
            _ => break,
        }
    }

    vertices
}
