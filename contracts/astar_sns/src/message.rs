use crate::metadata::*;
use ink_env::AccountId;
// use ink_lang as ink;
use ink_prelude::vec::Vec;

use crate::astar_sns::AstarSns;

impl AstarSns {
    // send message
    pub fn send_message(
        &mut self,
        message: String,
        message_list_id: u128,
        sender_id: AccountId,
        created_time: String,
    ) {
        // get message list of specified account id
        let mut message_list: Vec<Message> = self
            .message_list_map
            .get(&message_list_id)
            .unwrap_or(Vec::default());

        // add message info to copied message list
        message_list.push(Message {
            message,
            sender_id,
            created_time,
        });

        // override message list by updated message list
        self.message_list_map
            .insert(&message_list_id, &message_list);
    }

    // get message list
    // variable `num `
    pub fn get_message_list(&self, message_list_id: u128, num: usize) -> Vec<Message> {
        // get message list that contract has
        let self_message_list: Vec<Message> = self.message_list_map.get(&message_list_id).unwrap();

        // create message list to return
        let mut message_list: Vec<Message> = Vec::new();

        // this is index to specify how many post to get
        // for frontend, make it flexible how many post you can query
        let amount_index: usize = 5;

        // get message list's length
        let list_length: usize = self_message_list.len();

        // judge number of posts is less than specified number or not
        // this is because how to get is different
        if list_length < amount_index + 1 {
            for m in 0..(list_length) {
                // add to returning list
                message_list.push(self_message_list[m].clone());
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                // add to returning list
                message_list.push(self_message_list[n].clone());
            }
        }
        message_list
    }

    // get last message of specified account id's message list
    pub fn get_last_message(&self, message_list_id: u128) -> Message {
        let last_message: Message = self
            .message_list_map
            .get(&message_list_id)
            .unwrap()
            .last()
            .unwrap()
            .clone();
        last_message
    }
}
