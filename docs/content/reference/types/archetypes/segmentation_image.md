---
title: "SegmentationImage"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

An image made up of integer [`components.ClassId`](https://rerun.io/docs/reference/types/components/class_id)s.

The shape of the [`components.TensorData`](https://rerun.io/docs/reference/types/components/tensor_data) must be mappable to an `HxW` tensor.
Each pixel corresponds to a [`components.ClassId`](https://rerun.io/docs/reference/types/components/class_id) that will be mapped to a color based on annotation context.

In the case of floating point images, the label will be looked up based on rounding to the nearest
integer value.

Leading and trailing unit-dimensions are ignored, so that
`1x640x480x1` is treated as a `640x480` image.

See also [`archetypes.AnnotationContext`](https://rerun.io/docs/reference/types/archetypes/annotation_context) to associate each class with a color and a label.

## Components

**Required**: [`TensorData`](../components/tensor_data.md)

**Optional**: [`Opacity`](../components/opacity.md), [`DrawOrder`](../components/draw_order.md)

## Shown in
* [Spatial2DView](../views/spatial2d_view.md)
* [Spatial3DView](../views/spatial3d_view.md) (if logged under a projection)

## API reference links
 * 🌊 [C++ API docs for `SegmentationImage`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1archetypes_1_1SegmentationImage.html)
 * 🐍 [Python API docs for `SegmentationImage`](https://ref.rerun.io/docs/python/stable/common/archetypes#rerun.archetypes.SegmentationImage)
 * 🦀 [Rust API docs for `SegmentationImage`](https://docs.rs/rerun/latest/rerun/archetypes/struct.SegmentationImage.html)

## Example

### Simple segmentation image

snippet: archetypes/segmentation_image_simple

<picture data-inline-viewer="snippets/segmentation_image_simple">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/1200w.png">
  <img src="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/full.png">
</picture>

