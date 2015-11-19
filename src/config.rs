use std::os::raw::{c_float, c_int, c_uint};

macro_rules! define_properties {
    ($($i:ident),+) => (
        $(pub const $i: &'static str = stringify!($i);)+
    )
}

pub const AI_SBBC_DEFAULT_MAX_BONES: c_int = 60;
pub const AI_SLM_DEFAULT_MAX_TRIANGLES: c_int = 1000000;
pub const AI_SLM_DEFAULT_MAX_VERTICES: c_int = 1000000;
pub const AI_LMW_MAX_WEIGHTS: c_int = 0x4;
pub const AI_DEBONE_THRESHOLD: c_float = 1.0f32;
pub const PP_ICL_DEFAULT_PTCACHE_SIZE: c_int = 12;

define_properties! {
    // Library settings
    GLOB_MEASURE_TIME,
    IMPORT_NO_SKELETON_MESHES,
    GLOB_MULTITHREADING,

    // Post processing settings
    PP_SBBC_MAX_BONES,
    PP_CT_MAX_SMOOTHING_ANGLE,
    PP_CT_TEXTURE_CHANNEL_INDEX,
    PP_GSN_MAX_SMOOTHING_ANGLE,
    IMPORT_MDL_COLORMAP,
    PP_RRM_EXCLUDE_LIST,
    PP_PTV_KEEP_HIERARCHY,
    PP_PTV_NORMALIZE,
    PP_PTV_ADD_ROOT_TRANSFORMATION,
    PP_PTV_ROOT_TRANSFORMATION,
    PP_FD_REMOVE,
    PP_OG_EXCLUDE_LIST,
    PP_SLM_TRIANGLE_LIMIT,
    PP_SLM_VERTEX_LIMIT,
    PP_LBW_MAX_WEIGHTS,
    PP_DB_THRESHOLD,
    PP_DB_ALL_OR_NONE,
    PP_ICL_PTCACHE_SIZE,
    PP_RVC_FLAGS,
    PP_SBP_REMOVE,
    PP_FID_ANIM_ACCURACY,
    PP_TUV_EVALUATE,
    FAVOUR_SPEED,

    // Importer Settings
    IMPORT_FBX_READ_ALL_GEOMETRY_LAYERS,
    IMPORT_FBX_READ_ALL_MATERIALS,
    IMPORT_FBX_READ_MATERIALS,
    IMPORT_FBX_READ_CAMERAS,
    IMPORT_FBX_READ_LIGHTS,
    IMPORT_FBX_READ_ANIMATIONS,
    IMPORT_FBX_STRICT_MODE,
    IMPORT_FBX_PRESERVE_PIVOTS,
    IMPORT_FBX_OPTIMIZE_EMPTY_ANIMATION_CURVES,
    IMPORT_GLOBAL_KEYFRAME,
    IMPORT_MD3_KEYFRAME,
    IMPORT_MD2_KEYFRAME,
    IMPORT_MDL_KEYFRAME,
    IMPORT_MDC_KEYFRAME,
    IMPORT_SMD_KEYFRAME,
    IMPORT_UNREAL_KEYFRAME,
    IMPORT_AC_SEPARATE_BFCULL,
    IMPORT_AC_EVAL_SUBDIVISION,
    UNREAL_HANDLE_FLAGS,
    IMPORT_TER_MAKE_UVS,
    IMPORT_ASE_RECONSTRUCT_NORMALS,
    IMPORT_MD3_HANDLE_MULTIPART,
    IMPORT_MD3_SKIN_NAME,
    IMPORT_MD3_SHADER_SRC,
    IMPORT_LWO_ONE_LAYER_ONLY,
    IMPORT_MD5_NO_ANIM_AUTOLOAD,
    IMPORT_LWS_ANIM_START,
    IMPORT_LWS_ANIM_END,
    IMPORT_IRR_ANIM_FPS,
    IMPORT_OGRE_MATERIAL_FILE,
    IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME,
    IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS,
    IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS,
    IMPORT_IFC_CUSTOM_TRIANGULATION,
    IMPORT_COLLADA_IGNORE_UP_DIRECTION,
    IMPORT_COLLADA_INVERT_TRANSPARENCY,
    EXPORT_XFILE_64BIT
}

bitflags! {
    #[repr(C)]
    flags AiComponent: c_uint {
        const AICOMPONENT_NORMALS = 0x2,
        const AICOMPONENT_TANGENTS_AND_BITANGENTS = 0x4,
        const AICOMPONENT_COLORS = 0x8,
        const AICOMPONENT_TEXCOORDS = 0x10,
        const AICOMPONENT_BONE_WEIGHTS = 0x20,
        const AICOMPONENT_ANIMATIONS = 0x40,
        const AICOMPONENT_TEXTURES = 0x80,
        const AICOMPONENT_LIGHTS = 0x100,
        const AICOMPONENT_CAMERAS = 0x200,
        const AICOMPONENT_MESHES = 0x400,
        const AICOMPONENT_MATERIALS = 0x800
    }
}

bitflags! {
    #[repr(C)]
    flags AiUVTransformFlags : c_uint {
        const AI_UVTRAFO_SCALING = 0x1,
        const AI_UVTRAFO_ROTATION = 0x2,
        const AI_UVTRAFO_TRANSLATION = 0x4,
        const AI_UVTRAFO_ALL = AI_UVTRAFO_SCALING.bits
                             | AI_UVTRAFO_ROTATION.bits
                             | AI_UVTRAFO_TRANSLATION.bits
    }
}
