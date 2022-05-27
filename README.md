# Bounce

A multithreaded raytracer written in Rust, inspired by the [Ray Tracing in One Weekend](https://raytracing.github.io/) series.

<!-- ![](assets/example.png) -->

<img src="assets/example.png" alt="example" width="50%">

## Features

Currently, Bounce supports:

- Rendering spheres
- Camera with adjustable position, direction, depth of field, and field of view
- Diffuse (Lambertian), glass (Schlick), and metallic material
- Multithreaded rendering using [rayon](https://crates.io/crates/rayon)

## Planned features

In the future, I plan to add:

- Textures
- Additional object types beyond spheres
- Lights
- Rendering optimizations (such as bounding volume hierarchies)
