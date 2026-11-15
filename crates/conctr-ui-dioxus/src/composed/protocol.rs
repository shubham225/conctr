/// Protocol identity; its shape and color always retain the kit's meaning.
#[derive(Clone, Copy, PartialEq)]
pub enum Protocol {
    Rest,
    Soap,
    OData,
    GraphQl,
    Grpc,
    Webhook,
}
impl Protocol {
    pub fn label(self) -> &'static str {
        match self {
            Self::Rest => "REST",
            Self::Soap => "SOAP",
            Self::OData => "OData",
            Self::GraphQl => "GraphQL",
            Self::Grpc => "gRPC",
            Self::Webhook => "Webhook",
        }
    }
}
