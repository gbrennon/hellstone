use std::future::Future;

/// Inbound contract every application service exposes to driving adapters.
///
/// Implementers must accept a request carrying only primitive data, perform the
/// whole operation asynchronously, and answer either a primitive response or a
/// typed error. Callers must not depend on any other entry point of the
/// application layer.
pub trait UseCase {
    type Request;
    type Response;
    type Error;

    fn execute(
        &self,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}
