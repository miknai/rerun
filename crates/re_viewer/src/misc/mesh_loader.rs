use re_log_types::{EncodedMesh3D, Mesh3D, MeshFormat, RawMesh3D};
use re_renderer::{resource_managers::ResourceLifeTime, RenderContext};

pub struct CpuMesh {
    name: String,

    // TODO(andreas): We should only have MeshHandles here (which are generated by the MeshManager!)
    // Can't do that right now because it's too hard to pass the render context through.
    pub mesh_instances: Vec<re_renderer::renderer::MeshInstance>,

    bbox: macaw::BoundingBox,
}

impl CpuMesh {
    pub fn load(
        name: String,
        mesh: &Mesh3D,
        render_ctx: &mut RenderContext,
    ) -> anyhow::Result<Self> {
        // TODO(emilk): load CpuMesh in background thread.
        match mesh {
            // Mesh from user logging some triangles.
            Mesh3D::Encoded(encoded_mesh) => {
                Self::load_encoded_mesh(name, encoded_mesh, render_ctx)
            }
            // Mesh from some file format. File passed in bytes.
            Mesh3D::Raw(raw_mesh) => Ok(Self::load_raw_mesh(name, raw_mesh, render_ctx)?),
        }
    }

    pub fn load_raw(
        name: String,
        format: MeshFormat,
        bytes: &[u8],
        render_ctx: &mut RenderContext,
    ) -> anyhow::Result<Self> {
        crate::profile_function!();

        let mesh_instances = match format {
            MeshFormat::Glb | MeshFormat::Gltf | MeshFormat::Obj => {
                re_renderer::importer::gltf::load_gltf_from_buffer(
                    &name,
                    bytes,
                    ResourceLifeTime::LongLived,
                    render_ctx,
                )
            }
        }?;
        let bbox = re_renderer::importer::calculate_bounding_box(&mesh_instances);

        Ok(Self {
            name,
            bbox,
            mesh_instances,
        })
    }

    fn load_encoded_mesh(
        name: String,
        encoded_mesh: &EncodedMesh3D,
        render_ctx: &mut RenderContext,
    ) -> anyhow::Result<Self> {
        crate::profile_function!();
        let EncodedMesh3D {
            mesh_id: _,
            format,
            bytes,
            transform,
        } = encoded_mesh;

        let mut slf = Self::load_raw(name, *format, bytes, render_ctx)?;

        let (scale, rotation, translation) =
            glam::Affine3A::from_cols_array_2d(transform).to_scale_rotation_translation();
        let transform = macaw::Conformal3::from_scale_rotation_translation(
            re_renderer::importer::to_uniform_scale(scale),
            rotation,
            translation,
        );
        for instance in &mut slf.mesh_instances {
            instance.world_from_mesh = transform * instance.world_from_mesh;
        }
        slf.bbox = re_renderer::importer::calculate_bounding_box(&slf.mesh_instances);

        Ok(slf)
    }

    fn load_raw_mesh(
        name: String,
        raw_mesh: &RawMesh3D,
        render_ctx: &mut RenderContext,
    ) -> anyhow::Result<Self> {
        crate::profile_function!();

        let bbox = macaw::BoundingBox::from_points(
            raw_mesh.positions.iter().map(|p| glam::Vec3::from(*p)),
        );

        let mesh_instances = vec![re_renderer::renderer::MeshInstance {
            gpu_mesh: render_ctx.mesh_manager.create(
                &mut render_ctx.gpu_resources,
                &render_ctx.texture_manager_2d,
                &re_renderer::mesh::Mesh {
                    label: name.clone().into(),
                    indices: raw_mesh.indices.iter().flatten().cloned().collect(),
                    vertex_positions: raw_mesh
                        .positions
                        .iter()
                        .map(|p| glam::Vec3::from(*p))
                        .collect(),
                    // TODO(andreas): Calculate normals
                    vertex_data: std::iter::repeat(
                        re_renderer::mesh::mesh_vertices::MeshVertexData {
                            normal: glam::Vec3::ZERO,
                            texcoord: glam::Vec2::ZERO,
                        },
                    )
                    .take(raw_mesh.positions.len())
                    .collect(),
                    materials: smallvec::smallvec![re_renderer::mesh::Material {
                        label: name.clone().into(),
                        index_range: 0..raw_mesh.indices.len() as _,
                        albedo: render_ctx.texture_manager_2d.white_texture_handle().clone(),
                    }],
                },
                ResourceLifeTime::LongLived,
            )?,
            mesh: None, // Don't need to keep cpu-mesh data around, we already have everything we wanted from it (the bounding box)
            world_from_mesh: Default::default(),
            additive_tint: egui::Color32::TRANSPARENT,
        }];

        Ok(Self {
            name,
            bbox,
            mesh_instances,
        })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bbox(&self) -> &macaw::BoundingBox {
        &self.bbox
    }
}
