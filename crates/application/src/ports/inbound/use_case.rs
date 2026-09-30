use std::future::Future;

/// Inbound contract every application service exposes to driving adapters.
///
/// Implementers must accept a request carrying only primitive data, perform the
/// whole operation asynchronously, and answer either a primitive response or a
/// typed error. An implementer must depend on outbound ports only, never on
/// another inbound port.
pub trait UseCase {
    type Request;
    type Response;
    type Error;

    /// Runs the whole operation described by `request`.
    fn execute(
        &self,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}
