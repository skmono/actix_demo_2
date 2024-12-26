use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DataFormat {
    id : u32,
    name : String,
    data : String,
}

impl Responder for DataFormat {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .body(res_body)
    }
}
