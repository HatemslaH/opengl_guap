use std::sync::Arc;

use crate::engine::graphics::{Mesh, MeshTopology};

/// Reference to the mesh uploaded to the GPU and the type of primitive.
#[derive(Clone)]
pub struct RenderMesh {
    pub mesh: Arc<Mesh>,
    pub topology: MeshTopology,
}
