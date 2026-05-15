pub struct Block<'a> {
    pub block_id: BlockId,
    pub size: u32,
    pub data: &'a [u8],
}

#[repr(u32)]
pub enum BlockId {
    EkbEscrowedKeyType = 1,  // length 0x2, value 0x5
    EkbEscrowedKeyId = 2,    // length 0x20
    EkbEscrowingKeyType = 3, // length 0x2, value 0x7
    EkbEscrowingKeyId = 4,   // length 0x1, value 0x31
    EkbEscrowMethod = 5,     // length 0x2, value 0x1
    EkbCustomData = 6,
    EkbEscrowBlob = 7, // length 0x180
    EkbSignature = 8,
    EkbSigningKey = 9,
    EkbSigningKeyId = 10,
    EkbSignatureValue = 11,

    // SPLicenseBlock block IDs follow
    SpLicenseBlock = 20, // LicenseSection
    SpLicenseBlockVersion = 21,
    CryptoUsage = 22,
    EncryptedLicenseBlockIV = 23,
    UplinkKeyId = 24,
    GenerationId = 25,
    KeyId = 26,
    EscrowedKey = 27, // EncryptedCik
    SpPolicies = 28,  // DiscIdSection
    SocId = 29,
    ConsoleType = 30,
    AbsoluteExpirationDate = 31,
    AbsoluteBeginDate = 32,
    EncryptedLicenseBlock = 33,
    CipherText = 34,
    Padding = 35,
    LicenseUsage = 36,
    DiscId = 37,
    _Unknown_38 = 38,
    _Unknown_39 = 39,
    SpSignature = 40, // SignatureSection
    DigestAlgorithm = 41,
    SigningKeyId = 42,
    Signature = 43,
    SignatureAlgorithm = 44,

    Fingerprint = 99,
    SPHeader = 100,

    // UWP SPLicenseBlock IDs
    Unknown201 = 201, // 10 bytes, contains a unix timestamp
    UWPKeyId = 202, // 76 bytes = [2 bytes keyID size] [2 bytes unkData size] [0x20 bytes keyID] [0x28 bytes unkData]
    Unknown203 = 203, // 16 bytes
    Unknown204 = 204, // 68 bytes
    Unknown205 = 205, // 34 bytes
    UWPPackageId = 206,
    Unknown210 = 210, // 8 bytes
}
