use ink_env::AccountId;
use ink_prelude::vec::Vec;
use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};

//// Test
#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct TalkTest {
    pub message_1: String,
    pub message_2: String,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct MessageRoomTest {
    pub sender_id: AccountId,
    pub receiver_id: AccountId,
    pub message: String,
    pub created_time: String,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ProfileTest {
    pub name: String,
    pub address: AccountId,
    pub img_url: String,
    pub following_address_list: Vec<AccountId>,
    pub follower_address_list: Vec<AccountId>,
}

#[derive(
    Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, StorageLayout, PartialEq,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PostTest {
    pub description: String,
    pub img_url: String,
    pub num_of_likes: u128,
    pub created_time: String,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct MessageTest {
    pub created_time: String,
    pub description: String,
    pub sender_id: AccountId,
}
//// Test

#[derive(
    Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq, PackedLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Post {
    pub name: String,
    pub user_id: AccountId,
    pub created_time: String,
    pub img_url: String,
    pub description: String,
    pub num_of_likes: u128,
}

#[derive(
    Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq, PackedLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Profile {
    pub following_list: Vec<AccountId>,
    pub follower_list: Vec<AccountId>,
    pub user_id: AccountId,
    pub name: Option<String>,
    pub img_url: Option<String>,
    pub message_list_id_list: Vec<u128>,
    pub post_id_list: Vec<u128>,
}

#[derive(
    Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, StorageLayout, PartialEq, PackedLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Message {
    pub message: String,
    pub sender_id: AccountId,
    pub created_time: String,
}
