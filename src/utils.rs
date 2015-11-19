use mesh::*;
use types::*;

#[link(name = "assimp")]
extern {
    pub fn FindAABBTransformed(mesh: *const AiMesh, min: *mut AiVector3D, max: *mut AiVector3D, transform: *const AiMatrix4x4);
}
