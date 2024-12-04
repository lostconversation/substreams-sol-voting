// @generated
/// Main data message that holds lists of various program-related data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub add_candidate_list: ::prost::alloc::vec::Vec<AddCandidate>,
    #[prost(message, repeated, tag="2")]
    pub init_poll_list: ::prost::alloc::vec::Vec<InitPoll>,
    #[prost(message, repeated, tag="3")]
    pub vote_list: ::prost::alloc::vec::Vec<Vote>,
}
/// Message for initializing a poll
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPoll {
    #[prost(uint64, tag="1")]
    pub poll_id: u64,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub authority: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub account: ::prost::alloc::vec::Vec<u8>,
}
/// Message for adding a candidate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCandidate {
    #[prost(bytes="vec", tag="1")]
    pub poll_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub candidate_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub initial_votes: u64,
    #[prost(bytes="vec", tag="4")]
    pub candidate_address: ::prost::alloc::vec::Vec<u8>,
}
/// Message for voting
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(bytes="vec", tag="1")]
    pub candidate_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub voter: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub new_vote_count: u64,
}
// @@protoc_insertion_point(module)
