use azure_core::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Hash {
    MD5([u8; 16]),
    CRC64(u64),
}

impl AddAsHeader for Hash {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            Hash::MD5(md5) => builder.header(azure_core::headers::CONTENT_MD5, base64::encode(md5)),
            Hash::CRC64(crc64) => {
                builder.header(azure_core::headers::CONTENT_CRC64, &format!("{}", crc64))
            }
        }
    }
}

impl From<md5::Digest> for Hash {
    fn from(md5: md5::Digest) -> Self {
        Hash::MD5(md5.0)
    }
}
