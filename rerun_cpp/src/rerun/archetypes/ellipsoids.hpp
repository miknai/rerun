// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/ellipsoids.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/half_size3d.hpp"
#include "../components/position3d.hpp"
#include "../components/radius.hpp"
#include "../components/rotation3d.hpp"
#include "../components/text.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 3D ellipsoids or spheres.
    ///
    /// This archetype is for ellipsoids or spheres whose size is a key part of the data
    /// (e.g. a bounding sphere).
    /// For points whose radii are for the sake of visualization, use `Points3D` instead.
    ///
    /// Currently, ellipsoids are always rendered as wireframes.
    /// Opaque and transparent rendering will be supported later.
    struct Ellipsoids {
        /// For each ellipsoid, half of its size on its three axes.
        ///
        /// If all components are equal, then it is a sphere with that radius.
        Collection<rerun::components::HalfSize3D> half_sizes;

        /// Optional center positions of the ellipsoids.
        ///
        /// If not specified, the centers will be at (0, 0, 0).
        std::optional<Collection<rerun::components::Position3D>> centers;

        /// Optional rotations of the ellipsoids.
        ///
        /// If not specified, the axes of the ellipsoid align with the axes of the coordinate system.
        std::optional<Collection<rerun::components::Rotation3D>> rotations;

        /// Optional colors for the ellipsoids.
        std::optional<Collection<rerun::components::Color>> colors;

        /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
        std::optional<Collection<rerun::components::Radius>> line_radii;

        /// Optional text labels for the ellipsoids.
        std::optional<Collection<rerun::components::Text>> labels;

        /// Optional `ClassId`s for the ellipsoids.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        std::optional<Collection<rerun::components::ClassId>> class_ids;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.EllipsoidsIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        // Extensions to generated type defined in 'ellipsoids_ext.cpp'

        /// Creates new `Ellipsoids` that are spheres, with `half_sizes` created from radii.
        //
        // TODO(andreas): This should not take an std::vector.
        static Ellipsoids from_radii(const std::vector<float>& sizes);

        /// Creates new `Ellipsoids` that are spheres, with `half_sizes` and `centers` created
        /// from centers and radii.
        //
        // TODO(andreas): This should not take an std::vector.
        static Ellipsoids from_centers_and_radii(
            const std::vector<datatypes::Vec3D>& centers, const std::vector<float>& radii
        );

        /// Creates new `Ellipsoids` with `half_sizes` centered around the local origin.
        static Ellipsoids from_half_sizes(Collection<components::HalfSize3D> half_sizes) {
            Ellipsoids ellipsoids;
            ellipsoids.half_sizes = std::move(half_sizes);
            return ellipsoids;
        }

        /// Creates new `Ellipsoids` with `centers` and `half_sizes`.
        static Ellipsoids from_centers_and_half_sizes(
            Collection<components::Position3D> centers,
            Collection<components::HalfSize3D> half_sizes
        ) {
            Ellipsoids ellipsoids;
            ellipsoids.half_sizes = std::move(half_sizes);
            ellipsoids.centers = std::move(centers);
            return ellipsoids;
        }

      public:
        Ellipsoids() = default;
        Ellipsoids(Ellipsoids&& other) = default;

        /// Optional center positions of the ellipsoids.
        ///
        /// If not specified, the centers will be at (0, 0, 0).
        Ellipsoids with_centers(Collection<rerun::components::Position3D> _centers) && {
            centers = std::move(_centers);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional rotations of the ellipsoids.
        ///
        /// If not specified, the axes of the ellipsoid align with the axes of the coordinate system.
        Ellipsoids with_rotations(Collection<rerun::components::Rotation3D> _rotations) && {
            rotations = std::move(_rotations);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the ellipsoids.
        Ellipsoids with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
        Ellipsoids with_line_radii(Collection<rerun::components::Radius> _line_radii) && {
            line_radii = std::move(_line_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional text labels for the ellipsoids.
        Ellipsoids with_labels(Collection<rerun::components::Text> _labels) && {
            labels = std::move(_labels);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional `ClassId`s for the ellipsoids.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        Ellipsoids with_class_ids(Collection<rerun::components::ClassId> _class_ids) && {
            class_ids = std::move(_class_ids);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Ellipsoids> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::Ellipsoids& archetype);
    };
} // namespace rerun
