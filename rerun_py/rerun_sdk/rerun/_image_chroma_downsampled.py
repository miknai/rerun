from __future__ import annotations

import io
import pathlib
from typing import IO, Iterable

import numpy as np

from ._log import AsComponents, ComponentBatchLike
from .archetypes import Image
from .components import TensorData
from .datatypes import Float32Like, TensorBuffer, TensorDimension


class ImageFormat:
    """Image file format."""

    name: str

    NV12: type[NV12]
    """
    Raw NV12 encoded image.

    The type comes with a `size_hint` attribute, a tuple of (height, width)
    which has to be specified specifying in order to set the RGB size of the image.
    """

    YUY2: type[YUY2]
    """
    Raw YUY2 encoded image.

    YUY2 is a YUV422 encoding with bytes ordered as `yuyv`.

    The type comes with a `size_hint` attribute, a tuple of (height, width)
    which has to be specified specifying in order to set the RGB size of the image.
    """

    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name


class NV12(ImageFormat):
    """NV12 format."""

    name = "NV12"
    size_hint: tuple[int, int]

    def __init__(self, size_hint: tuple[int, int]) -> None:
        """
        An NV12 encoded image.

        Parameters
        ----------
        size_hint:
            A tuple of (height, width), specifying the RGB size of the image

        """
        self.size_hint = size_hint


class YUY2(ImageFormat):
    """YUY2 format."""

    name = "YUY2"
    size_hint: tuple[int, int]

    def __init__(self, size_hint: tuple[int, int]) -> None:
        """
        An YUY2 encoded image.

        YUY2 is a YUV422 encoding with bytes ordered as `yuyv`.

        Parameters
        ----------
        size_hint:
            A tuple of (height, width), specifying the RGB size of the image

        """
        self.size_hint = size_hint


# Assign the variants
# This allows for rust like enums, for example:
# ImageFormat.NV12(width=1920, height=1080)
# isinstance(ImageFormat.NV12, ImageFormat) == True and isinstance(ImageFormat.NV12, NV12) == True
ImageFormat.NV12 = NV12
ImageFormat.YUY2 = YUY2


class ImageChromaDownsampled(AsComponents):
    """
    A chroma downsampled image.

    The encoded image can be loaded from either a file using its `path` or
    provided directly via `contents`.
    """

    def __init__(
        self,
        format: ImageFormat,
        *,
        path: str | pathlib.Path | None = None,
        contents: bytes | IO[bytes] | None = None,
        draw_order: Float32Like | None = None,
    ) -> None:
        """
        Create a new image with a given format.

        Parameters
        ----------
        path:
            A path to a file stored on the local filesystem. Mutually
            exclusive with `contents`.
        contents:
            The contents of the file. Can be a BufferedReader, BytesIO, or
            bytes. Mutually exclusive with `path`.
        format:
            The format of the image file or image encoding.
        draw_order:
            An optional floating point value that specifies the 2D drawing
            order. Objects with higher values are drawn on top of those with
            lower values.

        """
        if (path is None) == (contents is None):
            raise ValueError("Must provide exactly one of 'path' or 'contents'")

        buffer: IO[bytes] | None
        if path is not None:
            buffer = io.BytesIO(pathlib.Path(path).read_bytes())
        elif isinstance(contents, bytes):
            buffer = io.BytesIO(contents)
        else:
            buffer = contents

        if buffer is None:
            raise ValueError("Input data could not be coerced to IO[bytes]")

        np_buf = np.frombuffer(buffer.read(), dtype=np.uint8)

        if isinstance(format, NV12):
            np_buf = np_buf.reshape(int(format.size_hint[0] * 1.5), format.size_hint[1])
            kind = "nv12"
        elif isinstance(format, YUY2):
            np_buf = np_buf.reshape(format.size_hint[0], int(format.size_hint[1] * 2))
            kind = "yuy2"
        else:
            raise ValueError(f"Unknown format: {format}")

        tensor_buffer = TensorBuffer(np_buf)
        tensor_buffer.kind = kind  # type: ignore[assignment]

        self.data = TensorData(
            buffer=tensor_buffer,
            shape=[
                TensorDimension(np_buf.shape[0], "height"),
                TensorDimension(np_buf.shape[1], "width"),
                TensorDimension(1, "depth"),
            ],
        )
        self.draw_order = draw_order
        return

    def as_component_batches(self) -> Iterable[ComponentBatchLike]:
        return Image(self.data, draw_order=self.draw_order).as_component_batches()
