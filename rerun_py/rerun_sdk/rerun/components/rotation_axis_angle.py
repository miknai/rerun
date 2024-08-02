# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/rotation_axis_angle.fbs".

# You can extend this class by creating a "RotationAxisAngleExt" class in "rotation_axis_angle_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["RotationAxisAngle", "RotationAxisAngleBatch", "RotationAxisAngleType"]


class RotationAxisAngle(datatypes.RotationAxisAngle, ComponentMixin):
    """**Component**: 3D rotation represented by a rotation around a given axis."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of RotationAxisAngleExt in rotation_axis_angle_ext.py

    # Note: there are no fields here because RotationAxisAngle delegates to datatypes.RotationAxisAngle
    pass


class RotationAxisAngleType(datatypes.RotationAxisAngleType):
    _TYPE_NAME: str = "rerun.components.RotationAxisAngle"


class RotationAxisAngleBatch(datatypes.RotationAxisAngleBatch, ComponentBatchMixin):
    _ARROW_TYPE = RotationAxisAngleType()


# This is patched in late to avoid circular dependencies.
RotationAxisAngle._BATCH_TYPE = RotationAxisAngleBatch  # type: ignore[assignment]