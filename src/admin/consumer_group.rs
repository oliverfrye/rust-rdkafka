//! Data structures and associated methods for consumer groups in the Admin API.

use crate::admin::Broker;
use crate::error::RDKafkaError;
use crate::util::cstr_to_owned;
use crate::TopicPartitionList;
use rdkafka_sys as rdsys;
use rdsys::{
    RDKafkaConsumerGroupDescription, RDKafkaConsumerGroupListing, RDKafkaConsumerGroupState,
    RDKafkaMemberAssignment, RDKafkaMemberDescription,
};

/// A consumer group description.
#[derive(Debug)]
pub struct ConsumerGroupDescription {
    /// The group ID.
    pub group_id: String,

    /// Is the group simple?
    pub is_simple_consumer_group: bool,

    /// The members of the consumer group.
    pub members: Vec<MemberDescription>,

    /// The partition assignor identifier of the consumer group.
    pub partition_assignor: String,

    /// The state of the consumer group.
    pub state: RDKafkaConsumerGroupState,

    /// The coordinator of the consumer group.
    pub coordinator: Broker,

    /// A group-specific error encountered by librdkafka.
    pub error: RDKafkaError,
}

impl ConsumerGroupDescription {
    /// Creates a new `ConsumerGroupDescription` by copying from the provided
    /// native `ptr`.
    pub(crate) fn from_borrowed_ptr(ptr: *const RDKafkaConsumerGroupDescription) -> Self {
        assert!(!ptr.is_null());

        let group_id =
            unsafe { cstr_to_owned(rdsys::rd_kafka_ConsumerGroupDescription_group_id(ptr)) };
        let is_simple_consumer_group =
            unsafe { rdsys::rd_kafka_ConsumerGroupDescription_is_simple_consumer_group(ptr) } != 0;
        let members = {
            let n = unsafe { rdsys::rd_kafka_ConsumerGroupDescription_member_count(ptr) };
            let mut out = Vec::with_capacity(n);
            for i in 0..n {
                out.push(MemberDescription::from_borrowed_ptr(unsafe {
                    rdsys::rd_kafka_ConsumerGroupDescription_member(ptr, i)
                }));
            }
            out
        };
        let partition_assignor = unsafe {
            cstr_to_owned(rdsys::rd_kafka_ConsumerGroupDescription_partition_assignor(
                ptr,
            ))
        };
        let state = unsafe { rdsys::rd_kafka_ConsumerGroupDescription_state(ptr) };
        let coordinator = Broker::from_borrowed_ptr(unsafe {
            rdsys::rd_kafka_ConsumerGroupDescription_coordinator(ptr)
        });
        let error = unsafe {
            RDKafkaError::from_borrowed_ptr(rdsys::rd_kafka_ConsumerGroupDescription_error(ptr))
        };

        Self {
            group_id,
            is_simple_consumer_group,
            members,
            partition_assignor,
            state,
            coordinator,
            error,
        }
    }
}

/// A member of a consumer group.
#[derive(Debug)]
pub struct MemberDescription {
    /// The client ID of the member.
    pub client_id: String,

    /// The consumer ID of the member.
    pub consumer_id: String,

    /// The group instance ID of the member.
    pub group_instance_id: Option<String>,

    /// The hostname of the member.
    pub host: String,

    /// The assignments of the member.
    pub assignment: MemberAssignment,
}

impl MemberDescription {
    /// Creates a new `MemberDescription` by copying from the provided native
    /// `ptr`.
    pub(crate) fn from_borrowed_ptr(ptr: *const RDKafkaMemberDescription) -> Self {
        assert!(!ptr.is_null());

        let client_id = unsafe { cstr_to_owned(rdsys::rd_kafka_MemberDescription_client_id(ptr)) };
        let group_instance_id = unsafe {
            let id_ptr = rdsys::rd_kafka_MemberDescription_group_instance_id(ptr);
            if id_ptr.is_null() {
                None
            } else {
                Some(cstr_to_owned(id_ptr))
            }
        };
        let consumer_id =
            unsafe { cstr_to_owned(rdsys::rd_kafka_MemberDescription_consumer_id(ptr)) };
        let host = unsafe { cstr_to_owned(rdsys::rd_kafka_MemberDescription_host(ptr)) };
        let assignment = unsafe {
            MemberAssignment::from_borrowed_ptr(rdsys::rd_kafka_MemberDescription_assignment(ptr))
        };

        Self {
            client_id,
            consumer_id,
            group_instance_id,
            host,
            assignment,
        }
    }
}

/// The assignments of a member of a consumer group.
#[derive(Debug)]
pub struct MemberAssignment {
    /// The partitions assigned to the member.
    pub partitions: TopicPartitionList,
}

impl MemberAssignment {
    /// Creates a new `MemberAssignment` by copying from the provided native
    /// `ptr`.
    pub(crate) fn from_borrowed_ptr(ptr: *const RDKafkaMemberAssignment) -> Self {
        assert!(!ptr.is_null());

        let partitions = TopicPartitionList::from_borrowed_ptr(unsafe {
            rdsys::rd_kafka_MemberAssignment_partitions(ptr)
        });

        Self { partitions }
    }
}

/// A consumer group listing.
pub struct ConsumerGroupListing {
    /// The group ID.
    pub group_id: String,

    /// Is the group simple?
    pub is_simple_consumer_group: bool,

    /// The state of the consumer group.
    pub state: RDKafkaConsumerGroupState,
}

impl ConsumerGroupListing {
    /// Creates a new `ConsumerGroupListing` by copying from the provided native
    /// `ptr`.
    pub(crate) fn from_borrowed_ptr(ptr: *const RDKafkaConsumerGroupListing) -> Self {
        assert!(!ptr.is_null());

        let group_id = unsafe { cstr_to_owned(rdsys::rd_kafka_ConsumerGroupListing_group_id(ptr)) };
        let is_simple_consumer_group =
            unsafe { rdsys::rd_kafka_ConsumerGroupListing_is_simple_consumer_group(ptr) } != 0;
        let state = unsafe { rdsys::rd_kafka_ConsumerGroupListing_state(ptr) };

        Self {
            group_id,
            is_simple_consumer_group,
            state,
        }
    }
}
