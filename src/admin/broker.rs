//! Data structures and associated methods for brokers in the Admin API.

use crate::util::cstr_to_owned;
use rdkafka_sys as rdsys;
use rdsys::RDKafkaNode;

/// A representation of a Kafka Broker
#[derive(Debug)]
pub struct Broker {
    /// The ID of the broker.
    pub id: i32,

    /// The hostname of the broker.
    pub host: String,

    /// The ID of the broker.
    pub port: u16,
}

impl Broker {
    /// Creates a new `Broker` by copying from the provided native `ptr`.
    pub(crate) fn from_borrowed_ptr(ptr: *const RDKafkaNode) -> Self {
		assert!(!ptr.is_null());

        let id = unsafe { rdsys::rd_kafka_Node_id(ptr) };
        let host = unsafe { cstr_to_owned(rdsys::rd_kafka_Node_host(ptr)) };
        let port = unsafe { rdsys::rd_kafka_Node_port(ptr) };

        Self { id, host, port }
    }
}
