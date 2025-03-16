use darling::FromMeta;

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct ServiceOptions {}

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct PeerOptions {
    pub request: Option<String>,
    pub response: Option<String>,
    pub error: Option<String>,

    pub service: Option<String>,
}
