---
title: "Transform3D"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

A transform between two 3D spaces, i.e. a pose.

All components are applied in the inverse order they are listed here.
E.g. if both a 4x4 matrix with a translation and a translation vector are present,
the translation is applied first, followed by the matrix.

Each transform component can be listed multiple times, but transform tree propagation is only possible
if there's only one instance for each transform component.
TODO(#6831): write more about the exact interaction with the to be written `OutOfTreeTransform` component.

## Components

**Optional**: [`Transform3D`](../components/transform3d.md), [`Translation3D`](../components/translation3d.md), [`Scale3D`](../components/scale3d.md), [`TransformMat3x3`](../components/transform_mat3x3.md), [`AxisLength`](../components/axis_length.md)

## Shown in
* [Spatial3DView](../views/spatial3d_view.md)
* [Spatial2DView](../views/spatial2d_view.md) (if logged above active projection)

## API reference links
 * 🌊 [C++ API docs for `Transform3D`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1archetypes_1_1Transform3D.html)
 * 🐍 [Python API docs for `Transform3D`](https://ref.rerun.io/docs/python/stable/common/archetypes#rerun.archetypes.Transform3D)
 * 🦀 [Rust API docs for `Transform3D`](https://docs.rs/rerun/latest/rerun/archetypes/struct.Transform3D.html)

## Examples

### Variety of 3D transforms

snippet: archetypes/transform3d_simple

<picture data-inline-viewer="snippets/transform3d_simple">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/1200w.png">
  <img src="https://static.rerun.io/transform3d_simple/141368b07360ce3fcb1553079258ae3f42bdb9ac/full.png">
</picture>

### Transform hierarchy

snippet: archetypes/transform3d_hierarchy

<picture data-inline-viewer="snippets/transform3d_hierarchy">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/1200w.png">
  <img src="https://static.rerun.io/transform_hierarchy/cb7be7a5a31fcb2efc02ba38e434849248f87554/full.png">
</picture>

