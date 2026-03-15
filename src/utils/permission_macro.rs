#[macro_export]
macro_rules! permission {
    ($name:ident, $($perm:expr),+) => {
        async fn $name(
            req: ServiceRequest,
            next: Next<impl MessageBody>,
        ) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
            require_permissions(req, next, &[$($perm),+]).await
        }
    };
}