# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/clear_settings.fbs".


from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from ._overrides import clear_settings__native_to_pa_array_override  # noqa: F401

__all__ = ["ClearSettings", "ClearSettingsArray", "ClearSettingsArrayLike", "ClearSettingsLike", "ClearSettingsType"]


@define
class ClearSettings:
    """Configures how a clear operation should behave."""

    # You can define your own __init__ function by defining a function called "clear_settings__init_override"

    recursive: bool = field(converter=bool)
    """
    If true, also clears all recursive children entities.
    """


if TYPE_CHECKING:
    ClearSettingsLike = Union[ClearSettings, bool]
else:
    ClearSettingsLike = Any

ClearSettingsArrayLike = Union[ClearSettings, Sequence[ClearSettingsLike], bool, npt.NDArray[np.bool_]]


# --- Arrow support ---


class ClearSettingsType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.bool_(), "rerun.components.ClearSettings")


class ClearSettingsArray(BaseExtensionArray[ClearSettingsArrayLike]):
    _EXTENSION_NAME = "rerun.components.ClearSettings"
    _EXTENSION_TYPE = ClearSettingsType

    @staticmethod
    def _native_to_pa_array(data: ClearSettingsArrayLike, data_type: pa.DataType) -> pa.Array:
        return clear_settings__native_to_pa_array_override(data, data_type)


ClearSettingsType._ARRAY_TYPE = ClearSettingsArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(ClearSettingsType())