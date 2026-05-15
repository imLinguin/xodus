use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "s:Envelope")]
pub struct Envelope {
    #[serde(rename = "@xmlns:s")]
    pub s: String,
    #[serde(rename = "@xmlns:ps")]
    pub ps: String,
    #[serde(rename = "@xmlns:wsse")]
    pub wsse: String,
    #[serde(rename = "@xmlns:saml")]
    pub saml: String,
    #[serde(rename = "@xmlns:wsp")]
    pub wsp: String,
    #[serde(rename = "@xmlns:wsu")]
    pub wsu: String,
    #[serde(rename = "@xmlns:wsa")]
    pub wsa: String,
    #[serde(rename = "@xmlns:wssc")]
    pub wssc: String,
    #[serde(rename = "@xmlns:wst")]
    pub wst: String,

    #[serde(rename = "s:Header")]
    pub header: Header,
    #[serde(rename = "s:Body")]
    pub body: Body,
}

impl Envelope {
    pub fn new(header: Header, body: Body) -> Self {
        Self {
            s: "http://www.w3.org/2003/05/soap-envelope".to_owned(),
            ps: "http://schemas.microsoft.com/Passport/SoapServices/PPCRL".to_owned(),
            wsse:
                "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd"
                    .to_owned(),
            saml: "urn:oasis:names:tc:SAML:1.0:assertion".to_owned(),
            wsp: "http://schemas.xmlsoap.org/ws/2004/09/policy".to_owned(),
            wsu:
                "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd"
                    .to_owned(),
            wsa: "http://www.w3.org/2005/08/addressing".to_owned(),
            wssc: "http://schemas.xmlsoap.org/ws/2005/02/sc".to_owned(),
            wst: "http://schemas.xmlsoap.org/ws/2005/02/trust".to_owned(),
            header: header,
            body: body,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "wsa:Action")]
    pub action: MustUnderstandValue,
    #[serde(rename = "wsa:To")]
    pub to: MustUnderstandValue,
    #[serde(rename = "wsa:MessageID")]
    pub message_id: String,
    #[serde(rename = "ps:AuthInfo")]
    pub auth_info: AuthInfo,
    #[serde(rename = "wsse:Security")]
    pub security: Security,
}

impl Header {
    pub fn new(username: String, password: String, message_id: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            action: MustUnderstandValue {
                must_understand: "1".to_owned(),
                value: "http://schemas.xmlsoap.org/ws/2005/02/trust/RST/Issue".to_owned(),
            },
            to: MustUnderstandValue {
                must_understand: "1".to_owned(),
                value: "https://login.live.com:443/RST2.srf".to_owned(),
            },
            message_id,
            auth_info: AuthInfo::default(),
            security: Security {
                username_token: UsernameToken::new(username, password),
                timestamp: Timestamp {
                    id: "Timestamp".to_owned(),
                    created: now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                    expires: (now + std::time::Duration::from_mins(5))
                        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                },
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "wst:RequestSecurityToken")]
    pub request_security_token: RequestSecurityToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MustUnderstandValue {
    #[serde(rename = "@s:mustUnderstand")]
    pub must_understand: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "@xmlns:ps")]
    pub ps: String,
    #[serde(rename = "@Id")]
    pub id: String,

    #[serde(rename = "ps:HostingApp")]
    pub hosting_app: String,
    #[serde(rename = "ps:BinaryVersion")]
    pub binary_version: String,
    #[serde(rename = "ps:UIVersion")]
    pub ui_version: String,
    #[serde(rename = "ps:InlineUX")]
    pub inline_ux: String,
    #[serde(rename = "ps:IsAdmin")]
    pub is_admin: String,
    #[serde(rename = "ps:Cookies")]
    pub cookies: Option<String>,
    #[serde(rename = "ps:RequestParams")]
    pub request_params: String,
    #[serde(rename = "ps:WindowsClientString")]
    pub windows_client_string: String,
    #[serde(rename = "ps:LicenseSignatureKeyVersion")]
    pub license_signature_key_version: String,
    #[serde(rename = "ps:ClientCapabilities")]
    pub client_capabilities: String,
}

impl Default for AuthInfo {
    fn default() -> Self {
        Self {
            ps: "http://schemas.microsoft.com/Passport/SoapServices/PPCRL".to_owned(),
            id: "PPAuthInfo".to_owned(),
            hosting_app: "{DF60E2DF-88AD-4526-AE21-83D130EF0F68}".to_owned(),
            binary_version: "55".to_owned(),
            ui_version: "1".to_owned(),
            inline_ux: "TokenBroker".to_owned(),
            is_admin: "1".to_owned(),
            cookies: None,
            request_params: "AQAAAAIAAABsYwQAAAAxMDMz".to_owned(),
            windows_client_string: "b4d/QB7Zy5pjUAY9ByQ1echTyTITx6ZCErOEztuIVtw=".to_owned(),
            license_signature_key_version: "2".to_owned(),
            client_capabilities: "1".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    #[serde(rename = "wsse:UsernameToken")]
    pub username_token: UsernameToken,
    #[serde(rename = "wsu:Timestamp")]
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameToken {
    #[serde(rename = "@wsu:Id")]
    pub id: String,
    #[serde(rename = "wsse:Username")]
    pub username: String,
    #[serde(rename = "wsse:Password")]
    pub password: String
}

impl UsernameToken {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: "devicesoftware".to_string(),
            username,
            password
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    #[serde(rename = "@wsu:Id")]
    pub id: String,
    #[serde(rename = "wsu:Created")]
    pub created: String,
    #[serde(rename = "wsu:Expires")]
    pub expires: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestSecurityToken {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "wst:RequestType")]
    pub request_type: String,
    #[serde(rename = "wsp:AppliesTo")]
    pub applies_to: AppliesTo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliesTo {
    #[serde(rename = "wsa:EndpointReference")]
    pub endpoint_reference: EndpointReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointReference {
    #[serde(rename = "wsa:Address")]
    pub address: String,
}
