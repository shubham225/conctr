/// Protocol semantics from the brand shape language.
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
    pub fn color(self) -> &'static str {
        match self {
            Self::Rest | Self::GraphQl => "var(--cn-teal)",
            Self::Soap | Self::Webhook => "var(--cn-coral)",
            Self::OData => "var(--cn-amber)",
            Self::Grpc => "var(--cn-primary)",
        }
    }
    pub(crate) fn shape(self) -> &'static str {
        match self {
            Self::Rest => "circle",
            Self::Soap => "square",
            Self::OData => "triangle",
            Self::GraphQl => "diamond",
            Self::Grpc => "hex",
            Self::Webhook => "ring",
        }
    }
}
