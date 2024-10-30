// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/capsules3d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: 3D capsules; cylinders with hemispherical caps.
///
/// Capsules are defined by two endpoints (the centers of their end cap spheres), which are located
/// at (0, 0, 0) and (0, 0, length), that is, extending along the positive direction of the Z axis.
/// Capsules in other orientations may be produced by applying a rotation to the entity or
/// instances.
///
/// ## Example
///
/// ### Batch of capsules
/// ```ignore
/// use rerun::external::glam::vec3;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_capsule3d_batch").spawn()?;
///
///     rec.log(
///         "capsules",
///         &rerun::Capsules3D::from_lengths_and_radii([0., 2., 4., 6., 8.], [1., 0.5, 0.5, 0.5, 1.])
///             .with_colors([
///                 rerun::Color::from_rgb(255, 0, 0),
///                 rerun::Color::from_rgb(188, 188, 0),
///                 rerun::Color::from_rgb(0, 255, 0),
///                 rerun::Color::from_rgb(0, 188, 188),
///                 rerun::Color::from_rgb(0, 0, 255),
///             ])
///             .with_translations([
///                 vec3(0., 0., 0.),
///                 vec3(2., 0., 0.),
///                 vec3(4., 0., 0.),
///                 vec3(6., 0., 0.),
///                 vec3(8., 0., 0.),
///             ])
///             .with_rotation_axis_angles((0..5).map(|i| {
///                 rerun::RotationAxisAngle::new(
///                     [1.0, 0.0, 0.0],
///                     rerun::Angle::from_degrees(i as f32 * -22.5),
///                 )
///             })),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/capsule3d_batch/6e6a4acafcf528359372147d7247f85d84434101/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/capsule3d_batch/6e6a4acafcf528359372147d7247f85d84434101/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/capsule3d_batch/6e6a4acafcf528359372147d7247f85d84434101/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/capsule3d_batch/6e6a4acafcf528359372147d7247f85d84434101/1200w.png">
///   <img src="https://static.rerun.io/capsule3d_batch/6e6a4acafcf528359372147d7247f85d84434101/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Capsules3D {
    /// Lengths of the capsules, defined as the distance between the centers of the endcaps.
    pub lengths: Vec<crate::components::Length>,

    /// Radii of the capsules.
    pub radii: Vec<crate::components::Radius>,

    /// Optional translations of the capsules.
    ///
    /// If not specified, one end of each capsule will be at (0, 0, 0).
    /// Note that this uses a [`components::PoseTranslation3D`][crate::components::PoseTranslation3D] which is also used by [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
    pub translations: Option<Vec<crate::components::PoseTranslation3D>>,

    /// Rotations via axis + angle.
    ///
    /// If no rotation is specified, the capsules align with the +Z axis of the local coordinate system.
    /// Note that this uses a [`components::PoseRotationAxisAngle`][crate::components::PoseRotationAxisAngle] which is also used by [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
    pub rotation_axis_angles: Option<Vec<crate::components::PoseRotationAxisAngle>>,

    /// Rotations via quaternion.
    ///
    /// If no rotation is specified, the capsules align with the +Z axis of the local coordinate system.
    /// Note that this uses a [`components::PoseRotationQuat`][crate::components::PoseRotationQuat] which is also used by [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
    pub quaternions: Option<Vec<crate::components::PoseRotationQuat>>,

    /// Optional colors for the capsules.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the capsules, which will be located at their centers.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<crate::components::ShowLabels>,

    /// Optional class ID for the ellipsoids.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,
}

impl ::re_types_core::SizeBytes for Capsules3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.lengths.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.translations.heap_size_bytes()
            + self.rotation_axis_angles.heap_size_bytes()
            + self.quaternions.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::Length>>::is_pod()
            && <Vec<crate::components::Radius>>::is_pod()
            && <Option<Vec<crate::components::PoseTranslation3D>>>::is_pod()
            && <Option<Vec<crate::components::PoseRotationAxisAngle>>>::is_pod()
            && <Option<Vec<crate::components::PoseRotationQuat>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<crate::components::ShowLabels>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Length".into(),
            "rerun.components.Radius".into(),
        ]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.PoseTranslation3D".into(),
            "rerun.components.Color".into(),
            "rerun.components.Capsules3DIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.PoseRotationAxisAngle".into(),
            "rerun.components.PoseRotationQuat".into(),
            "rerun.components.Text".into(),
            "rerun.components.ShowLabels".into(),
            "rerun.components.ClassId".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 10usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Length".into(),
            "rerun.components.Radius".into(),
            "rerun.components.PoseTranslation3D".into(),
            "rerun.components.Color".into(),
            "rerun.components.Capsules3DIndicator".into(),
            "rerun.components.PoseRotationAxisAngle".into(),
            "rerun.components.PoseRotationQuat".into(),
            "rerun.components.Text".into(),
            "rerun.components.ShowLabels".into(),
            "rerun.components.ClassId".into(),
        ]
    });

impl Capsules3D {
    /// The total number of components in the archetype: 2 required, 3 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 10usize;
}

/// Indicator component for the [`Capsules3D`] [`::re_types_core::Archetype`]
pub type Capsules3DIndicator = ::re_types_core::GenericIndicatorComponent<Capsules3D>;

impl ::re_types_core::Archetype for Capsules3D {
    type Indicator = Capsules3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Capsules3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Capsules 3D"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: Capsules3DIndicator = Capsules3DIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let lengths = {
            let array = arrays_by_name
                .get("rerun.components.Length")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Capsules3D#lengths")?;
            <crate::components::Length>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Capsules3D#lengths")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Capsules3D#lengths")?
        };
        let radii = {
            let array = arrays_by_name
                .get("rerun.components.Radius")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Capsules3D#radii")?;
            <crate::components::Radius>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Capsules3D#radii")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Capsules3D#radii")?
        };
        let translations =
            if let Some(array) = arrays_by_name.get("rerun.components.PoseTranslation3D") {
                Some({
                    <crate::components::PoseTranslation3D>::from_arrow_opt(&**array)
                        .with_context("rerun.archetypes.Capsules3D#translations")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.archetypes.Capsules3D#translations")?
                })
            } else {
                None
            };
        let rotation_axis_angles =
            if let Some(array) = arrays_by_name.get("rerun.components.PoseRotationAxisAngle") {
                Some({
                    <crate::components::PoseRotationAxisAngle>::from_arrow_opt(&**array)
                        .with_context("rerun.archetypes.Capsules3D#rotation_axis_angles")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.archetypes.Capsules3D#rotation_axis_angles")?
                })
            } else {
                None
            };
        let quaternions =
            if let Some(array) = arrays_by_name.get("rerun.components.PoseRotationQuat") {
                Some({
                    <crate::components::PoseRotationQuat>::from_arrow_opt(&**array)
                        .with_context("rerun.archetypes.Capsules3D#quaternions")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.archetypes.Capsules3D#quaternions")?
                })
            } else {
                None
            };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Capsules3D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Capsules3D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Capsules3D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Capsules3D#labels")?
            })
        } else {
            None
        };
        let show_labels = if let Some(array) = arrays_by_name.get("rerun.components.ShowLabels") {
            <crate::components::ShowLabels>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Capsules3D#show_labels")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Capsules3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Capsules3D#class_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            lengths,
            radii,
            translations,
            rotation_axis_angles,
            quaternions,
            colors,
            labels,
            show_labels,
            class_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Capsules3D {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.lengths as &dyn ComponentBatch).into()),
            Some((&self.radii as &dyn ComponentBatch).into()),
            self.translations
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.rotation_axis_angles
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.quaternions
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.show_labels
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Capsules3D {}

impl Capsules3D {
    /// Create a new `Capsules3D`.
    #[inline]
    pub(crate) fn new(
        lengths: impl IntoIterator<Item = impl Into<crate::components::Length>>,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        Self {
            lengths: lengths.into_iter().map(Into::into).collect(),
            radii: radii.into_iter().map(Into::into).collect(),
            translations: None,
            rotation_axis_angles: None,
            quaternions: None,
            colors: None,
            labels: None,
            show_labels: None,
            class_ids: None,
        }
    }

    /// Optional translations of the capsules.
    ///
    /// If not specified, one end of each capsule will be at (0, 0, 0).
    /// Note that this uses a [`components::PoseTranslation3D`][crate::components::PoseTranslation3D] which is also used by [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
    #[inline]
    pub fn with_translations(
        mut self,
        translations: impl IntoIterator<Item = impl Into<crate::components::PoseTranslation3D>>,
    ) -> Self {
        self.translations = Some(translations.into_iter().map(Into::into).collect());
        self
    }

    /// Rotations via axis + angle.
    ///
    /// If no rotation is specified, the capsules align with the +Z axis of the local coordinate system.
    /// Note that this uses a [`components::PoseRotationAxisAngle`][crate::components::PoseRotationAxisAngle] which is also used by [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
    #[inline]
    pub fn with_rotation_axis_angles(
        mut self,
        rotation_axis_angles: impl IntoIterator<
            Item = impl Into<crate::components::PoseRotationAxisAngle>,
        >,
    ) -> Self {
        self.rotation_axis_angles =
            Some(rotation_axis_angles.into_iter().map(Into::into).collect());
        self
    }

    /// Rotations via quaternion.
    ///
    /// If no rotation is specified, the capsules align with the +Z axis of the local coordinate system.
    /// Note that this uses a [`components::PoseRotationQuat`][crate::components::PoseRotationQuat] which is also used by [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D].
    #[inline]
    pub fn with_quaternions(
        mut self,
        quaternions: impl IntoIterator<Item = impl Into<crate::components::PoseRotationQuat>>,
    ) -> Self {
        self.quaternions = Some(quaternions.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the capsules.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional text labels for the capsules, which will be located at their centers.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = Some(show_labels.into());
        self
    }

    /// Optional class ID for the ellipsoids.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }
}