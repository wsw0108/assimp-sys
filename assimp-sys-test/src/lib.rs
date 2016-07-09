#![cfg_attr(not(test), allow(unused))]

extern crate assimp_sys;
extern crate libc;

use assimp_sys::*;
use std::mem::size_of;

use libc::{c_int, size_t};

#[link(name = "tests")]
extern {
    fn get_struct_sizes(sizes: *mut size_t, len: c_int) -> bool;
}

#[test]
fn check_struct_sizes() {
    let mut sizes: [size_t; 40] = [0; 40];
    let result = unsafe { get_struct_sizes(sizes.as_mut_ptr(), 40) };
    assert!(result, "Array size mismatch in get_struct_sizes.");

    assert_eq!(sizes[0], size_of::<AiAnimation>());
    assert_eq!(sizes[1], size_of::<AiAnimMesh>());
    assert_eq!(sizes[2], size_of::<AiBone>());
    assert_eq!(sizes[3], size_of::<AiCamera>());
    assert_eq!(sizes[4], size_of::<AiColor3D>());
    assert_eq!(sizes[5], size_of::<AiColor4D>());
    assert_eq!(sizes[6], size_of::<AiExportDataBlob>());
    assert_eq!(sizes[7], size_of::<AiExportFormatDesc>());
    assert_eq!(sizes[8], size_of::<AiFace>());
    assert_eq!(sizes[9], size_of::<AiFile>());
    assert_eq!(sizes[10], size_of::<AiFileIO>());
    assert_eq!(sizes[11], size_of::<AiImporterDesc>());
    assert_eq!(sizes[12], size_of::<AiLight>());
    assert_eq!(sizes[13], size_of::<AiLogStream>());
    assert_eq!(sizes[14], size_of::<AiMaterial>());
    assert_eq!(sizes[15], size_of::<AiMaterialProperty>());
    assert_eq!(sizes[16], size_of::<AiMatrix3x3>());
    assert_eq!(sizes[17], size_of::<AiMatrix4x4>());
    assert_eq!(sizes[18], size_of::<AiMemoryInfo>());
    assert_eq!(sizes[19], size_of::<AiMesh>());
    assert_eq!(sizes[20], size_of::<AiMeshAnim>());
    assert_eq!(sizes[21], size_of::<AiMeshKey>());
    assert_eq!(sizes[22], size_of::<AiMetadata>());
    assert_eq!(sizes[23], size_of::<AiMetadataEntry>());
    assert_eq!(sizes[24], size_of::<AiNode>());
    assert_eq!(sizes[25], size_of::<AiNodeAnim>());
    assert_eq!(sizes[26], size_of::<AiPlane>());
    assert_eq!(sizes[27], size_of::<AiPropertyStore>());
    assert_eq!(sizes[28], size_of::<AiQuaternion>());
    assert_eq!(sizes[29], size_of::<AiQuatKey>());
    assert_eq!(sizes[30], size_of::<AiRay>());
    assert_eq!(sizes[31], size_of::<AiScene>());
    assert_eq!(sizes[32], size_of::<AiString>());
    assert_eq!(sizes[33], size_of::<AiTexel>());
    assert_eq!(sizes[34], size_of::<AiTexture>());
    assert_eq!(sizes[35], size_of::<AiUVTransform>());
    assert_eq!(sizes[36], size_of::<AiVector2D>());
    assert_eq!(sizes[37], size_of::<AiVector3D>());
    assert_eq!(sizes[38], size_of::<AiVectorKey>());
    assert_eq!(sizes[39], size_of::<AiVertexWeight>());
}
