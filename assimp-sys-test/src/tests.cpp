#include <assimp/cfileio.h>
#include <assimp/cimport.h>
#include <assimp/cexport.h>
#include <assimp/scene.h>

extern "C" {
    bool get_struct_sizes(size_t* sizes, int len) {
        if (len != 40) {
            return false;
        }

        sizes[0] = sizeof(aiAnimation);
        sizes[1] = sizeof(aiAnimMesh);
        sizes[2] = sizeof(aiBone);
        sizes[3] = sizeof(aiCamera);
        sizes[4] = sizeof(aiColor3D);
        sizes[5] = sizeof(aiColor4D);
        sizes[6] = sizeof(aiExportDataBlob);
        sizes[7] = sizeof(aiExportFormatDesc);
        sizes[8] = sizeof(aiFace);
        sizes[9] = sizeof(aiFile);
        sizes[10] = sizeof(aiFileIO);
        sizes[11] = sizeof(aiImporterDesc);
        sizes[12] = sizeof(aiLight);
        sizes[13] = sizeof(aiLogStream);
        sizes[14] = sizeof(aiMaterial);
        sizes[15] = sizeof(aiMaterialProperty);
        sizes[16] = sizeof(aiMatrix3x3);
        sizes[17] = sizeof(aiMatrix4x4);
        sizes[18] = sizeof(aiMemoryInfo);
        sizes[19] = sizeof(aiMesh);
        sizes[20] = sizeof(aiMeshAnim);
        sizes[21] = sizeof(aiMeshKey);
        sizes[22] = sizeof(aiMetadata);
        sizes[23] = sizeof(aiMetadataEntry);
        sizes[24] = sizeof(aiNode);
        sizes[25] = sizeof(aiNodeAnim);
        sizes[26] = sizeof(aiPlane);
        sizes[27] = sizeof(aiPropertyStore);
        sizes[28] = sizeof(aiQuaternion);
        sizes[29] = sizeof(aiQuatKey);
        sizes[30] = sizeof(aiRay);
        sizes[31] = sizeof(aiScene);
        sizes[32] = sizeof(aiString);
        sizes[33] = sizeof(aiTexel);
        sizes[34] = sizeof(aiTexture);
        sizes[35] = sizeof(aiUVTransform);
        sizes[36] = sizeof(aiVector2D);
        sizes[37] = sizeof(aiVector3D);
        sizes[38] = sizeof(aiVectorKey);
        sizes[39] = sizeof(aiVertexWeight);

        return true;
    }
}
