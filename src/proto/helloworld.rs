/// The request message containing the user's name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
