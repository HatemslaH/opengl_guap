use crate::graphics::{Mesh, MeshTopology};

/// Reference to the mesh uploaded to the GPU and the type of primitive.
pub struct RenderMesh {
    pub mesh: Mesh,
    pub topology: MeshTopology,
}
