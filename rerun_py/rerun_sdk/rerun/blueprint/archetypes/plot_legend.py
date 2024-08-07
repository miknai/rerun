# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/plot_legend.fbs".

# You can extend this class by creating a "PlotLegendExt" class in "plot_legend_ext.py".

from __future__ import annotations

from attrs import define, field

from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from .plot_legend_ext import PlotLegendExt

__all__ = ["PlotLegend"]


@define(str=False, repr=False, init=False)
class PlotLegend(PlotLegendExt, Archetype):
    """**Archetype**: Configuration for the legend of a plot."""

    # __init__ can be found in plot_legend_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            corner=None,  # type: ignore[arg-type]
            visible=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> PlotLegend:
        """Produce an empty PlotLegend, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    corner: blueprint_components.Corner2DBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.Corner2DBatch._optional,  # type: ignore[misc]
    )
    # To what corner the legend is aligned.
    #
    # Defaults to the right bottom corner.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    visible: blueprint_components.VisibleBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.VisibleBatch._optional,  # type: ignore[misc]
    )
    # Whether the legend is shown at all.
    #
    # True by default.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
