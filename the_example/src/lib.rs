#![feature(type_alias_impl_trait)]

use core::result::Result;

use pilota::FastStr;
use volo_gen::thrift_gen::dummys::{DummyRequest, DummyServiceDummyMethodException, DummyResponse};

pub struct DummyService;

#[volo::async_trait]
impl volo_gen::thrift_gen::dummys::DummyService for DummyService {
    
    async fn dummy_method(
        &self,
        request: DummyRequest,
    ) -> ::core::result::Result<
        DummyResponse,
        ::volo_thrift::error::UserError<DummyServiceDummyMethodException>,
    >
    {
        let id = request.id.and_then(|x| Some(FastStr::from_string(format!("resp-{}", x))));
        Ok(DummyResponse { id: id })
    }

}
