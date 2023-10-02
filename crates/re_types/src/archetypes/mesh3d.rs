// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/mesh3d.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A 3D triangle mesh as specified by its per-mesh and per-vertex properties.
///
/// ## Examples
///
/// ### Simple indexed 3D mesh
/// ```ignore
/// //! Log a simple colored triangle with indexed drawing.
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) =
///         rerun::RecordingStreamBuilder::new("rerun_example_mesh3d_indexed").memory()?;
///
///     rec.log(
///         "triangle",
///         &rerun::Mesh3D::new([[0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
///             .with_vertex_normals([[0.0, 0.0, 1.0]])
///             .with_vertex_colors([0x0000FFFF, 0x00FF00FF, 0xFF0000FF])
///             .with_mesh_properties(rerun::MeshProperties::from_triangle_indices([[2, 1, 0]]))
///             .with_mesh_material(rerun::Material::from_albedo_factor(0xCC00CCFF)),
///     )?;
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
///
/// ### 3D mesh with partial updates
/// ```ignore
/// //! Log a simple colored triangle, then update its vertices' positions each frame.
///
/// use rerun::external::glam;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) =
///         rerun::RecordingStreamBuilder::new("rerun_example_mesh3d_partial_updates").memory()?;
///
///     let vertex_positions = [[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
///
///     // Log the initial state of our triangle
///     rec.set_time_sequence("frame", 0);
///     rec.log(
///         "triangle",
///         &rerun::Mesh3D::new(vertex_positions)
///             .with_vertex_normals([[0.0, 0.0, 1.0]])
///             .with_vertex_colors([0xFF0000FF, 0x00FF00FF, 0x0000FFFF]),
///     )?;
///
///     // Only update its vertices' positions each frame
///     for i in 1..300 {
///         rec.set_time_sequence("frame", i);
///
///         let factor = (i as f32 * 0.04).sin().abs();
///         let vertex_positions: [rerun::Position3D; 3] = [
///             (glam::Vec3::from(vertex_positions[0]) * factor).into(),
///             (glam::Vec3::from(vertex_positions[1]) * factor).into(),
///             (glam::Vec3::from(vertex_positions[2]) * factor).into(),
///         ];
///         rec.log_component_batches("triangle", false, [&vertex_positions as _])?;
///     }
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh3D {
    /// The positions of each vertex.
    ///
    /// If no `indices` are specified, then each triplet of positions is interpreted as a triangle.
    pub vertex_positions: Vec<crate::components::Position3D>,

    /// Optional properties for the mesh as a whole (including indexed drawing).
    pub mesh_properties: Option<crate::components::MeshProperties>,

    /// An optional normal for each vertex.
    ///
    /// If specified, this must have as many elements as `vertex_positions`.
    pub vertex_normals: Option<Vec<crate::components::Vector3D>>,

    /// An optional color for each vertex.
    pub vertex_colors: Option<Vec<crate::components::Color>>,

    /// Optional material properties for the mesh as a whole.
    pub mesh_material: Option<crate::components::Material>,

    /// Optional class Ids for the vertices.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Unique identifiers for each individual vertex in the mesh.
    pub instance_keys: Option<Vec<crate::components::InstanceKey>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Position3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Mesh3DIndicator".into(),
            "rerun.components.MeshProperties".into(),
            "rerun.components.Vector3D".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.ClassId".into(),
            "rerun.components.Color".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.Material".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Position3D".into(),
            "rerun.components.Mesh3DIndicator".into(),
            "rerun.components.MeshProperties".into(),
            "rerun.components.Vector3D".into(),
            "rerun.components.ClassId".into(),
            "rerun.components.Color".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.Material".into(),
        ]
    });

impl Mesh3D {
    pub const NUM_COMPONENTS: usize = 8usize;
}

/// Indicator component for the [`Mesh3D`] [`crate::Archetype`]
pub type Mesh3DIndicator = crate::GenericIndicatorComponent<Mesh3D>;

impl crate::Archetype for Mesh3D {
    type Indicator = Mesh3DIndicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.archetypes.Mesh3D".into()
    }

    #[inline]
    fn indicator() -> crate::MaybeOwnedComponentBatch<'static> {
        static INDICATOR: Mesh3DIndicator = Mesh3DIndicator::DEFAULT;
        crate::MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow(
        arrow_data: impl IntoIterator<
            Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>),
        >,
    ) -> crate::DeserializationResult<Self> {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let vertex_positions = {
            let array = arrays_by_name
                .get("rerun.components.Position3D")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.Mesh3D#vertex_positions")?;
            <crate::components::Position3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Mesh3D#vertex_positions")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Mesh3D#vertex_positions")?
        };
        let mesh_properties =
            if let Some(array) = arrays_by_name.get("rerun.components.MeshProperties") {
                Some({
                    <crate::components::MeshProperties>::from_arrow_opt(&**array)
                        .with_context("rerun.archetypes.Mesh3D#mesh_properties")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.archetypes.Mesh3D#mesh_properties")?
                })
            } else {
                None
            };
        let vertex_normals = if let Some(array) = arrays_by_name.get("rerun.components.Vector3D") {
            Some({
                <crate::components::Vector3D>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#vertex_normals")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#vertex_normals")?
            })
        } else {
            None
        };
        let vertex_colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#vertex_colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#vertex_colors")?
            })
        } else {
            None
        };
        let mesh_material = if let Some(array) = arrays_by_name.get("rerun.components.Material") {
            Some({
                <crate::components::Material>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#mesh_material")?
                    .into_iter()
                    .next()
                    .flatten()
                    .ok_or_else(crate::DeserializationError::missing_data)
                    .with_context("rerun.archetypes.Mesh3D#mesh_material")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#class_ids")?
            })
        } else {
            None
        };
        let instance_keys = if let Some(array) = arrays_by_name.get("rerun.components.InstanceKey")
        {
            Some({
                <crate::components::InstanceKey>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#instance_keys")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#instance_keys")?
            })
        } else {
            None
        };
        Ok(Self {
            vertex_positions,
            mesh_properties,
            vertex_normals,
            vertex_colors,
            mesh_material,
            class_ids,
            instance_keys,
        })
    }
}

impl crate::AsComponents for Mesh3D {
    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use crate::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.vertex_positions as &dyn crate::ComponentBatch).into()),
            self.mesh_properties
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.vertex_normals
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.vertex_colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.mesh_material
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.instance_keys
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        self.vertex_positions.len()
    }
}

impl Mesh3D {
    pub fn new(
        vertex_positions: impl IntoIterator<Item = impl Into<crate::components::Position3D>>,
    ) -> Self {
        Self {
            vertex_positions: vertex_positions.into_iter().map(Into::into).collect(),
            mesh_properties: None,
            vertex_normals: None,
            vertex_colors: None,
            mesh_material: None,
            class_ids: None,
            instance_keys: None,
        }
    }

    pub fn with_mesh_properties(
        mut self,
        mesh_properties: impl Into<crate::components::MeshProperties>,
    ) -> Self {
        self.mesh_properties = Some(mesh_properties.into());
        self
    }

    pub fn with_vertex_normals(
        mut self,
        vertex_normals: impl IntoIterator<Item = impl Into<crate::components::Vector3D>>,
    ) -> Self {
        self.vertex_normals = Some(vertex_normals.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_vertex_colors(
        mut self,
        vertex_colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.vertex_colors = Some(vertex_colors.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_mesh_material(
        mut self,
        mesh_material: impl Into<crate::components::Material>,
    ) -> Self {
        self.mesh_material = Some(mesh_material.into());
        self
    }

    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_instance_keys(
        mut self,
        instance_keys: impl IntoIterator<Item = impl Into<crate::components::InstanceKey>>,
    ) -> Self {
        self.instance_keys = Some(instance_keys.into_iter().map(Into::into).collect());
        self
    }
}
