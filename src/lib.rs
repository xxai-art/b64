pub use base64::{engine::general_purpose::URL_SAFE_NO_PAD, DecodeError, Engine};

pub fn b64e(bin: impl AsRef<[u8]>) -> String {
  URL_SAFE_NO_PAD.encode(bin)
}

pub fn b64d(bin: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
  URL_SAFE_NO_PAD.decode(bin)
}

#[cfg(feature = "intbin")]
mod int;

#[cfg(feature = "intbin")]
pub use int::{b64_u64, u64_b64};
