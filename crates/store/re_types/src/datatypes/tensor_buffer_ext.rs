use crate::tensor_data::TensorDataType;

use super::TensorBuffer;

impl TensorBuffer {
    /// The underlying data type of the buffer.
    #[allow(clippy::match_same_arms)]
    pub fn dtype(&self) -> TensorDataType {
        match self {
            Self::U8(_) => TensorDataType::U8,
            Self::U16(_) => TensorDataType::U16,
            Self::U32(_) => TensorDataType::U32,
            Self::U64(_) => TensorDataType::U64,
            Self::I8(_) => TensorDataType::I8,
            Self::I16(_) => TensorDataType::I16,
            Self::I32(_) => TensorDataType::I32,
            Self::I64(_) => TensorDataType::I64,
            Self::F16(_) => TensorDataType::F16,
            Self::F32(_) => TensorDataType::F32,
            Self::F64(_) => TensorDataType::F64,
            Self::Nv12(_) => TensorDataType::U8,
            Self::Yuy2(_) => TensorDataType::U8,
        }
    }

    /// The size of the buffer in bytes.
    #[allow(clippy::match_same_arms)]
    pub fn size_in_bytes(&self) -> usize {
        match self {
            Self::U8(buf) => buf.size_in_bytes(),
            Self::U16(buf) => buf.size_in_bytes(),
            Self::U32(buf) => buf.size_in_bytes(),
            Self::U64(buf) => buf.size_in_bytes(),
            Self::I8(buf) => buf.size_in_bytes(),
            Self::I16(buf) => buf.size_in_bytes(),
            Self::I32(buf) => buf.size_in_bytes(),
            Self::I64(buf) => buf.size_in_bytes(),
            Self::F16(buf) => buf.size_in_bytes(),
            Self::F32(buf) => buf.size_in_bytes(),
            Self::F64(buf) => buf.size_in_bytes(),
            Self::Nv12(buf) => buf.size_in_bytes(),
            Self::Yuy2(buf) => buf.size_in_bytes(),
        }
    }

    /// Is this buffer empty?
    pub fn is_empty(&self) -> bool {
        self.size_in_bytes() == 0
    }

    /// Is this tensor represented by a compressed image format
    /// (JPEG, NV12, YUY2)?
    pub fn is_compressed_image(&self) -> bool {
        match self {
            Self::U8(_)
            | Self::U16(_)
            | Self::U32(_)
            | Self::U64(_)
            | Self::I8(_)
            | Self::I16(_)
            | Self::I32(_)
            | Self::I64(_)
            | Self::F16(_)
            | Self::F32(_)
            | Self::F64(_) => false,

            Self::Nv12(_) | Self::Yuy2(_) => true,
        }
    }
}

impl std::fmt::Debug for TensorBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(_) => write!(f, "U8({} bytes)", self.size_in_bytes()),
            Self::U16(_) => write!(f, "U16({} bytes)", self.size_in_bytes()),
            Self::U32(_) => write!(f, "U32({} bytes)", self.size_in_bytes()),
            Self::U64(_) => write!(f, "U64({} bytes)", self.size_in_bytes()),
            Self::I8(_) => write!(f, "I8({} bytes)", self.size_in_bytes()),
            Self::I16(_) => write!(f, "I16({} bytes)", self.size_in_bytes()),
            Self::I32(_) => write!(f, "I32({} bytes)", self.size_in_bytes()),
            Self::I64(_) => write!(f, "I64({} bytes)", self.size_in_bytes()),
            Self::F16(_) => write!(f, "F16({} bytes)", self.size_in_bytes()),
            Self::F32(_) => write!(f, "F32({} bytes)", self.size_in_bytes()),
            Self::F64(_) => write!(f, "F64({} bytes)", self.size_in_bytes()),
            Self::Nv12(_) => write!(f, "NV12({} bytes)", self.size_in_bytes()),
            Self::Yuy2(_) => write!(f, "YUY2({} bytes)", self.size_in_bytes()),
        }
    }
}
