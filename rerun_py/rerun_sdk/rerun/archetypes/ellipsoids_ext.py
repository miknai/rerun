from __future__ import annotations

from typing import Any

import numpy as np

from .. import datatypes
from ..error_utils import _send_warning_or_raise, catch_and_log_exceptions


class EllipsoidsExt:
    """Extension for [Ellipsoids][rerun.archetypes.Ellipsoids]."""

    def __init__(
        self: Any,
        *,
        half_sizes: datatypes.Vec3DArrayLike | None = None,
        radii: datatypes.Float32ArrayLike | None = None,
        centers: datatypes.Vec3DArrayLike | None = None,
        rotations: datatypes.Rotation3DArrayLike | None = None,
        colors: datatypes.Rgba32ArrayLike | None = None,
        line_radii: datatypes.Float32ArrayLike | None = None,
        labels: datatypes.Utf8ArrayLike | None = None,
        class_ids: datatypes.ClassIdArrayLike | None = None,
    ) -> None:
        """
        Create a new instance of the Ellipsoids archetype.

        Parameters
        ----------
        half_sizes:
            All half-extents that make up the batch of ellipsoids.
            Specify this instead of `radii`
        radii:
            All radii that make up this batch of spheres.
            Specify this instead of `half_sizes`
        centers:
            Optional center positions of the ellipsoids.
        rotations:
            Optional rotations of the ellipsoids.
        colors:
            Optional colors for the ellipsoids.
        line_radii:
            Optional radii for the lines that make up the ellipsoids.
        labels:
            Optional text labels for the ellipsoids.
        class_ids:
            Optional `ClassId`s for the ellipsoids.

            The class ID provides colors and labels if not specified explicitly.

        """

        with catch_and_log_exceptions(context=self.__class__.__name__):
            if radii is not None:
                if half_sizes is not None:
                    _send_warning_or_raise("Cannot specify both `radii` and `half_sizes` at the same time.", 1)

                radii = np.asarray(radii, dtype=np.float32)
                # Duplicate [r1, r2, ...] to [[r1, r1, r1], [r2, r2, r2], ...]
                half_sizes = np.repeat(np.expand_dims(radii, axis=1), 3, axis=1)

            self.__attrs_init__(
                half_sizes=half_sizes,
                centers=centers,
                rotations=rotations,
                colors=colors,
                line_radii=line_radii,
                labels=labels,
                class_ids=class_ids,
            )
            return

        self.__attrs_clear__()
