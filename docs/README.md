# Conctr documentation

Conctr is an enterprise integration platform under development. Its purpose is to
connect two or more systems whose APIs, protocols or message formats differ.
For example, one system might publish an order in JSON while its trading partner
expects an XML document. Receiving, translating and delivering that order are
three distinct integration concerns.

Gregor Hohpe and Bobby Woolf's *Enterprise Integration Patterns* is the primary
design reference. Use its vocabulary to describe concrete needs such as a
Message Translator or a Message Router. Pattern names describe design options.
See the [authors' pattern catalog](https://www.enterpriseintegrationpatterns.com/).

## Where to start

| I want to… | Read |
| --- | --- |
| Run the product | [Local development](development/setup.md), [first sign-in](../crates/conctr-api-dioxus/README.md) |
| Explore the component examples | [Design kit gallery](../apps/conctr-design-kit/README.md) |
| Understand what works and where code belongs | [Workspace and crate map](architecture/crate-map.md) |
| Change a screen or reusable control | [Presentation ownership](architecture/dioxus-presentation.md), [theming](development/theming.md) |
| Call the management API | [HTTP reference](architecture/api-and-events.md) |
| Apply project conventions or troubleshoot a build | [Engineering learnings](development/learnings.md) |
| Work with an AI coding assistant | [Repository instructions](../AGENTS.md) |

## Direction

Rust owns backend and business logic. Dioxus serves the web and desktop
frontends. Keep product rules and HTTP contracts usable from a different UI,
such as GPUI or egui. Mobile remains in the workspace pending planned removal.

The integration runtime, storage technology, connector interfaces and deployment
model are open. Document a decision when a feature gives it concrete requirements.
Earlier architecture proposals, wireframes and verification history are preserved
in the local pre-refresh archive under `backups/`.
