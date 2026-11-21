//! Protocol, authentication and integration pattern artwork on Conctr's 64-unit grid.
//! Unqualified constants and groups use the monochrome edition.

macro_rules! icon_catalog {
    ($( $collection:ident { $( $family:literal => { $( $id:ident, $label:literal, $color:literal, $mono:literal; )+ } )+ } )+) => {
        /// Brand-color SVG assets grouped by family.
        pub mod color {
            use dioxus::prelude::*;
            $($($( #[doc = $label] pub const $id: Asset = asset!($color); )+)+)+
            $( /// Icon families and their labeled assets in display order.
            pub const $collection: &[(&str, &[(&str, Asset)])] = &[
                $( ($family, &[$(($label, $id)),+]), )+
            ]; )+
        }

        /// Single-color SVG assets grouped by family.
        pub mod mono {
            use dioxus::prelude::*;
            $($($( #[doc = $label] pub const $id: Asset = asset!($mono); )+)+)+
            $( /// Icon families and their labeled assets in display order.
            pub const $collection: &[(&str, &[(&str, Asset)])] = &[
                $( ($family, &[$(($label, $id)),+]), )+
            ]; )+
        }
    };
}

pub use mono::*;

icon_catalog! {
    BRAND_GROUPS {
        "Protocols" => {
            REST, "REST / HTTP",
                "/assets/icons/color/protocols/rest.svg",
                "/assets/icons/mono/protocols/rest.svg";
            SOAP, "SOAP / XML",
                "/assets/icons/color/protocols/soap.svg",
                "/assets/icons/mono/protocols/soap.svg";
            ODATA, "OData",
                "/assets/icons/color/protocols/odata.svg",
                "/assets/icons/mono/protocols/odata.svg";
            GRAPHQL, "GraphQL",
                "/assets/icons/color/protocols/graphql.svg",
                "/assets/icons/mono/protocols/graphql.svg";
            GRPC, "gRPC",
                "/assets/icons/color/protocols/grpc.svg",
                "/assets/icons/mono/protocols/grpc.svg";
            WEBHOOK, "Webhook / events",
                "/assets/icons/color/protocols/webhook.svg",
                "/assets/icons/mono/protocols/webhook.svg";
        }
        "Authentication" => {
            OAUTH2, "OAuth 2.0",
                "/assets/icons/color/authentication/oauth2.svg",
                "/assets/icons/mono/authentication/oauth2.svg";
            APIKEY, "API key",
                "/assets/icons/color/authentication/apikey.svg",
                "/assets/icons/mono/authentication/apikey.svg";
            BASIC, "Basic auth",
                "/assets/icons/color/authentication/basic.svg",
                "/assets/icons/mono/authentication/basic.svg";
            JWT, "JWT / bearer",
                "/assets/icons/color/authentication/jwt.svg",
                "/assets/icons/mono/authentication/jwt.svg";
            MTLS, "mTLS",
                "/assets/icons/color/authentication/mtls.svg",
                "/assets/icons/mono/authentication/mtls.svg";
            SAML, "SAML / SSO",
                "/assets/icons/color/authentication/saml.svg",
                "/assets/icons/mono/authentication/saml.svg";
        }
    }
    PATTERN_GROUPS {
        "Integration styles" => {
            FILE_TRANSFER, "File Transfer",
                "/assets/icons/color/patterns/integration-styles/file-transfer.svg",
                "/assets/icons/mono/patterns/integration-styles/file-transfer.svg";
            SHARED_DATABASE, "Shared Database",
                "/assets/icons/color/patterns/integration-styles/shared-database.svg",
                "/assets/icons/mono/patterns/integration-styles/shared-database.svg";
            REMOTE_PROCEDURE_INVOCATION, "Remote Procedure Invocation",
                "/assets/icons/color/patterns/integration-styles/remote-procedure-invocation.svg",
                "/assets/icons/mono/patterns/integration-styles/remote-procedure-invocation.svg";
            MESSAGING, "Messaging",
                "/assets/icons/color/patterns/integration-styles/messaging.svg",
                "/assets/icons/mono/patterns/integration-styles/messaging.svg";
        }
        "Messaging systems" => {
            MESSAGE_CHANNEL, "Message Channel",
                "/assets/icons/color/patterns/messaging-systems/message-channel.svg",
                "/assets/icons/mono/patterns/messaging-systems/message-channel.svg";
            MESSAGE, "Message",
                "/assets/icons/color/patterns/messaging-systems/message.svg",
                "/assets/icons/mono/patterns/messaging-systems/message.svg";
            PIPES_AND_FILTERS, "Pipes and Filters",
                "/assets/icons/color/patterns/messaging-systems/pipes-and-filters.svg",
                "/assets/icons/mono/patterns/messaging-systems/pipes-and-filters.svg";
            MESSAGE_ROUTER, "Message Router",
                "/assets/icons/color/patterns/messaging-systems/message-router.svg",
                "/assets/icons/mono/patterns/messaging-systems/message-router.svg";
            MESSAGE_TRANSLATOR, "Message Translator",
                "/assets/icons/color/patterns/messaging-systems/message-translator.svg",
                "/assets/icons/mono/patterns/messaging-systems/message-translator.svg";
            MESSAGE_ENDPOINT, "Message Endpoint",
                "/assets/icons/color/patterns/messaging-systems/message-endpoint.svg",
                "/assets/icons/mono/patterns/messaging-systems/message-endpoint.svg";
        }
        "Messaging channels" => {
            POINT_TO_POINT_CHANNEL, "Point-to-Point Channel",
                "/assets/icons/color/patterns/messaging-channels/point-to-point-channel.svg",
                "/assets/icons/mono/patterns/messaging-channels/point-to-point-channel.svg";
            PUBLISH_SUBSCRIBE_CHANNEL, "Publish-Subscribe Channel",
                "/assets/icons/color/patterns/messaging-channels/publish-subscribe-channel.svg",
                "/assets/icons/mono/patterns/messaging-channels/publish-subscribe-channel.svg";
            DATATYPE_CHANNEL, "Datatype Channel",
                "/assets/icons/color/patterns/messaging-channels/datatype-channel.svg",
                "/assets/icons/mono/patterns/messaging-channels/datatype-channel.svg";
            INVALID_MESSAGE_CHANNEL, "Invalid Message Channel",
                "/assets/icons/color/patterns/messaging-channels/invalid-message-channel.svg",
                "/assets/icons/mono/patterns/messaging-channels/invalid-message-channel.svg";
            DEAD_LETTER_CHANNEL, "Dead Letter Channel",
                "/assets/icons/color/patterns/messaging-channels/dead-letter-channel.svg",
                "/assets/icons/mono/patterns/messaging-channels/dead-letter-channel.svg";
            GUARANTEED_DELIVERY, "Guaranteed Delivery",
                "/assets/icons/color/patterns/messaging-channels/guaranteed-delivery.svg",
                "/assets/icons/mono/patterns/messaging-channels/guaranteed-delivery.svg";
            CHANNEL_ADAPTER, "Channel Adapter",
                "/assets/icons/color/patterns/messaging-channels/channel-adapter.svg",
                "/assets/icons/mono/patterns/messaging-channels/channel-adapter.svg";
            MESSAGING_BRIDGE, "Messaging Bridge",
                "/assets/icons/color/patterns/messaging-channels/messaging-bridge.svg",
                "/assets/icons/mono/patterns/messaging-channels/messaging-bridge.svg";
            MESSAGE_BUS, "Message Bus",
                "/assets/icons/color/patterns/messaging-channels/message-bus.svg",
                "/assets/icons/mono/patterns/messaging-channels/message-bus.svg";
        }
        "Message construction" => {
            COMMAND_MESSAGE, "Command Message",
                "/assets/icons/color/patterns/message-construction/command-message.svg",
                "/assets/icons/mono/patterns/message-construction/command-message.svg";
            DOCUMENT_MESSAGE, "Document Message",
                "/assets/icons/color/patterns/message-construction/document-message.svg",
                "/assets/icons/mono/patterns/message-construction/document-message.svg";
            EVENT_MESSAGE, "Event Message",
                "/assets/icons/color/patterns/message-construction/event-message.svg",
                "/assets/icons/mono/patterns/message-construction/event-message.svg";
            REQUEST_REPLY, "Request-Reply",
                "/assets/icons/color/patterns/message-construction/request-reply.svg",
                "/assets/icons/mono/patterns/message-construction/request-reply.svg";
            RETURN_ADDRESS, "Return Address",
                "/assets/icons/color/patterns/message-construction/return-address.svg",
                "/assets/icons/mono/patterns/message-construction/return-address.svg";
            CORRELATION_IDENTIFIER, "Correlation Identifier",
                "/assets/icons/color/patterns/message-construction/correlation-identifier.svg",
                "/assets/icons/mono/patterns/message-construction/correlation-identifier.svg";
            MESSAGE_SEQUENCE, "Message Sequence",
                "/assets/icons/color/patterns/message-construction/message-sequence.svg",
                "/assets/icons/mono/patterns/message-construction/message-sequence.svg";
            MESSAGE_EXPIRATION, "Message Expiration",
                "/assets/icons/color/patterns/message-construction/message-expiration.svg",
                "/assets/icons/mono/patterns/message-construction/message-expiration.svg";
            FORMAT_INDICATOR, "Format Indicator",
                "/assets/icons/color/patterns/message-construction/format-indicator.svg",
                "/assets/icons/mono/patterns/message-construction/format-indicator.svg";
        }
        "Message routing" => {
            CONTENT_BASED_ROUTER, "Content-Based Router",
                "/assets/icons/color/patterns/message-routing/content-based-router.svg",
                "/assets/icons/mono/patterns/message-routing/content-based-router.svg";
            MESSAGE_FILTER, "Message Filter",
                "/assets/icons/color/patterns/message-routing/message-filter.svg",
                "/assets/icons/mono/patterns/message-routing/message-filter.svg";
            DYNAMIC_ROUTER, "Dynamic Router",
                "/assets/icons/color/patterns/message-routing/dynamic-router.svg",
                "/assets/icons/mono/patterns/message-routing/dynamic-router.svg";
            RECIPIENT_LIST, "Recipient List",
                "/assets/icons/color/patterns/message-routing/recipient-list.svg",
                "/assets/icons/mono/patterns/message-routing/recipient-list.svg";
            SPLITTER, "Splitter",
                "/assets/icons/color/patterns/message-routing/splitter.svg",
                "/assets/icons/mono/patterns/message-routing/splitter.svg";
            AGGREGATOR, "Aggregator",
                "/assets/icons/color/patterns/message-routing/aggregator.svg",
                "/assets/icons/mono/patterns/message-routing/aggregator.svg";
            RESEQUENCER, "Resequencer",
                "/assets/icons/color/patterns/message-routing/resequencer.svg",
                "/assets/icons/mono/patterns/message-routing/resequencer.svg";
            COMPOSED_MESSAGE_PROCESSOR, "Composed Message Processor",
                "/assets/icons/color/patterns/message-routing/composed-message-processor.svg",
                "/assets/icons/mono/patterns/message-routing/composed-message-processor.svg";
            SCATTER_GATHER, "Scatter-Gather",
                "/assets/icons/color/patterns/message-routing/scatter-gather.svg",
                "/assets/icons/mono/patterns/message-routing/scatter-gather.svg";
            ROUTING_SLIP, "Routing Slip",
                "/assets/icons/color/patterns/message-routing/routing-slip.svg",
                "/assets/icons/mono/patterns/message-routing/routing-slip.svg";
            PROCESS_MANAGER, "Process Manager",
                "/assets/icons/color/patterns/message-routing/process-manager.svg",
                "/assets/icons/mono/patterns/message-routing/process-manager.svg";
            MESSAGE_BROKER, "Message Broker",
                "/assets/icons/color/patterns/message-routing/message-broker.svg",
                "/assets/icons/mono/patterns/message-routing/message-broker.svg";
        }
        "Message transformation" => {
            ENVELOPE_WRAPPER, "Envelope Wrapper",
                "/assets/icons/color/patterns/message-transformation/envelope-wrapper.svg",
                "/assets/icons/mono/patterns/message-transformation/envelope-wrapper.svg";
            CONTENT_ENRICHER, "Content Enricher",
                "/assets/icons/color/patterns/message-transformation/content-enricher.svg",
                "/assets/icons/mono/patterns/message-transformation/content-enricher.svg";
            CONTENT_FILTER, "Content Filter",
                "/assets/icons/color/patterns/message-transformation/content-filter.svg",
                "/assets/icons/mono/patterns/message-transformation/content-filter.svg";
            CLAIM_CHECK, "Claim Check",
                "/assets/icons/color/patterns/message-transformation/claim-check.svg",
                "/assets/icons/mono/patterns/message-transformation/claim-check.svg";
            NORMALIZER, "Normalizer",
                "/assets/icons/color/patterns/message-transformation/normalizer.svg",
                "/assets/icons/mono/patterns/message-transformation/normalizer.svg";
            CANONICAL_DATA_MODEL, "Canonical Data Model",
                "/assets/icons/color/patterns/message-transformation/canonical-data-model.svg",
                "/assets/icons/mono/patterns/message-transformation/canonical-data-model.svg";
        }
        "Messaging endpoints" => {
            MESSAGING_GATEWAY, "Messaging Gateway",
                "/assets/icons/color/patterns/messaging-endpoints/messaging-gateway.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/messaging-gateway.svg";
            MESSAGING_MAPPER, "Messaging Mapper",
                "/assets/icons/color/patterns/messaging-endpoints/messaging-mapper.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/messaging-mapper.svg";
            TRANSACTIONAL_CLIENT, "Transactional Client",
                "/assets/icons/color/patterns/messaging-endpoints/transactional-client.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/transactional-client.svg";
            POLLING_CONSUMER, "Polling Consumer",
                "/assets/icons/color/patterns/messaging-endpoints/polling-consumer.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/polling-consumer.svg";
            EVENT_DRIVEN_CONSUMER, "Event-Driven Consumer",
                "/assets/icons/color/patterns/messaging-endpoints/event-driven-consumer.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/event-driven-consumer.svg";
            COMPETING_CONSUMERS, "Competing Consumers",
                "/assets/icons/color/patterns/messaging-endpoints/competing-consumers.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/competing-consumers.svg";
            MESSAGE_DISPATCHER, "Message Dispatcher",
                "/assets/icons/color/patterns/messaging-endpoints/message-dispatcher.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/message-dispatcher.svg";
            SELECTIVE_CONSUMER, "Selective Consumer",
                "/assets/icons/color/patterns/messaging-endpoints/selective-consumer.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/selective-consumer.svg";
            DURABLE_SUBSCRIBER, "Durable Subscriber",
                "/assets/icons/color/patterns/messaging-endpoints/durable-subscriber.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/durable-subscriber.svg";
            IDEMPOTENT_RECEIVER, "Idempotent Receiver",
                "/assets/icons/color/patterns/messaging-endpoints/idempotent-receiver.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/idempotent-receiver.svg";
            SERVICE_ACTIVATOR, "Service Activator",
                "/assets/icons/color/patterns/messaging-endpoints/service-activator.svg",
                "/assets/icons/mono/patterns/messaging-endpoints/service-activator.svg";
        }
        "System management" => {
            CONTROL_BUS, "Control Bus",
                "/assets/icons/color/patterns/system-management/control-bus.svg",
                "/assets/icons/mono/patterns/system-management/control-bus.svg";
            DETOUR, "Detour",
                "/assets/icons/color/patterns/system-management/detour.svg",
                "/assets/icons/mono/patterns/system-management/detour.svg";
            WIRE_TAP, "Wire Tap",
                "/assets/icons/color/patterns/system-management/wire-tap.svg",
                "/assets/icons/mono/patterns/system-management/wire-tap.svg";
            MESSAGE_HISTORY, "Message History",
                "/assets/icons/color/patterns/system-management/message-history.svg",
                "/assets/icons/mono/patterns/system-management/message-history.svg";
            MESSAGE_STORE, "Message Store",
                "/assets/icons/color/patterns/system-management/message-store.svg",
                "/assets/icons/mono/patterns/system-management/message-store.svg";
            SMART_PROXY, "Smart Proxy",
                "/assets/icons/color/patterns/system-management/smart-proxy.svg",
                "/assets/icons/mono/patterns/system-management/smart-proxy.svg";
            TEST_MESSAGE, "Test Message",
                "/assets/icons/color/patterns/system-management/test-message.svg",
                "/assets/icons/mono/patterns/system-management/test-message.svg";
            CHANNEL_PURGER, "Channel Purger",
                "/assets/icons/color/patterns/system-management/channel-purger.svg",
                "/assets/icons/mono/patterns/system-management/channel-purger.svg";
        }
    }
}
