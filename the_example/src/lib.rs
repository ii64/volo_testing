#![feature(type_alias_impl_trait)]

use core::result::Result;

use volo_gen::thrift_gen::dummys::{DummyRequest, DummyServiceDummyMethodException, DummyResponse};

pub struct DummyService;

#[volo::async_trait]
impl volo_gen::thrift_gen::dummys::DummyService for DummyService {
    async fn dummy_method(
        &self, _req: DummyRequest
    ) -> Result<DummyResponse, volo_thrift::error::UserError<DummyServiceDummyMethodException>>{
        let id = _req.id.and_then(|x| Some(format!("resp-{}", x)));
        Ok(DummyResponse { id: id })
    }
}
