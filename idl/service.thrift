namespace rs dummys


struct DummyRequest {
    1: string id
}
struct DummyResponse {
    2: string id
}
exception DummyException {
    1: string msg
}

service DummyService {

    DummyResponse dummyMethod(1: DummyRequest request) throws (1: DummyException e)

}