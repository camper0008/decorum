use salvo::{
    oapi::{self, Components, Operation},
    prelude::{EndpointOutRegister, StatusCode, ToSchema},
    writing::Json,
};
use serde::Serialize;

pub type MessageResponseResult = Result<Response<Message>, Response<Message>>;

pub mod message_response {
    use super::{Message, Response};

    macro_rules! impl_message_response {
        ($name: ident, $code: expr, $ok: expr) => {
            pub fn $name<S: ToString>(message: S) -> Response<Message> {
                Response {
                    code: $code,
                    data: Message {
                        ok: $ok,
                        message: message.to_string(),
                    },
                }
            }
        };
    }

    impl_message_response!(ok, 200, true);
    impl_message_response!(created, 201, true);
    impl_message_response!(bad_request, 400, false);
    impl_message_response!(unauthorized, 403, false);
    impl_message_response!(internal_server_error, 500, false);
}

macro_rules! impl_response_with {
    ($name: ident, $code: expr) => {
        pub fn $name(data: T) -> Response<T> {
            Response { code: $code, data }
        }
    };
}

impl<T: ToSchema> EndpointOutRegister for Response<T> {
    #[inline]
    fn register(components: &mut Components, operation: &mut Operation) {
        let schema = T::to_schema(components);
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
pub struct Response<T: ToSchema + 'static> {
    data: T,
    code: u16,
}

impl<T: ToSchema> Response<T> {
    impl_response_with!(with_ok, 200);
}

#[derive(Serialize, oapi::ToSchema)]
pub struct Message {
    ok: bool,
    message: String,
}

#[salvo::async_trait]
impl<T: ToSchema + Send + Serialize> salvo::Writer for Response<T> {
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
