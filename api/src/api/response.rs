use salvo::{
    oapi::{self, Components, Operation},
    prelude::{EndpointOutRegister, StatusCode, ToSchema},
    writing::Json,
};
use serde::Serialize;

pub type ResponseResult = Result<MessageResponse, MessageResponse>;

macro_rules! impl_response_helpers {
    ($name: ident, $code: expr, $ok: expr) => {
        pub fn $name<S: ToString>(message: S) -> Self {
            Self {
                code: $code,
                data: Data {
                    ok: $ok,
                    message: message.to_string(),
                },
            }
        }
    };
}

impl MessageResponse {
    impl_response_helpers!(ok, 200, true);
    impl_response_helpers!(created, 201, true);
    impl_response_helpers!(bad_request, 400, false);
    impl_response_helpers!(unauthorized, 403, true);
    impl_response_helpers!(internal_server_error, 500, false);
}

impl EndpointOutRegister for MessageResponse {
    #[inline]
    fn register(components: &mut Components, operation: &mut Operation) {
        let schema = Data::to_schema(components);
        for code in [
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::INTERNAL_SERVER_ERROR,
        ] {
            operation.responses.insert(
                code.as_str(),
                salvo::oapi::Response::new(
                    code.canonical_reason()
                        .unwrap_or("No further explanation is available."),
                )
                .add_content("application/json", schema.clone()),
            )
        }
    }
}

#[derive(oapi::ToSchema)]
pub struct MessageResponse {
    data: Data,
    code: u16,
}

#[derive(Serialize, oapi::ToSchema)]
struct Data {
    ok: bool,
    message: String,
}

#[salvo::async_trait]
impl salvo::Writer for MessageResponse {
    async fn write(
        mut self,
        _req: &mut salvo::Request,
        _depot: &mut salvo::Depot,
        res: &mut salvo::Response,
    ) {
        res.status_code(
            StatusCode::from_u16(self.code)
                .expect("should not be able to construct response with invalid code"),
        )
        .render(Json(self.data));
    }
}
