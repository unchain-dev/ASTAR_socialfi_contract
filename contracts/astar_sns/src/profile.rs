use crate::metadata::*;
use ink_env::AccountId;
// use ink_lang as ink;
use ink_prelude::vec::Vec;

use crate::astar_sns::AstarSns;

impl AstarSns {
    // This function runs once when new user join this dapp
    pub fn create_profile(&mut self, account_id: AccountId) {
        let is_already_connected = self.profile_map.contains(&account_id);
        if !is_already_connected {
            self.profile_map.insert(
                &(account_id),
                &Profile {
                    following_list: Vec::new(),
                    follower_list: Vec::new(),
                    user_id: account_id,
                    name: None,
                    img_url: None,
                    message_list_id_list: Vec::new(),
                    post_id_list: Vec::new(),
                },
            );
        }
    }

    // set profile info(name, img_url)
    pub fn set_profile_info(&mut self, account_id: AccountId, name: String, img_url: String) {
        let mut profile: Profile = self.profile_map.get(&account_id).unwrap();
        profile.name = Some(name);
        profile.img_url = Some(img_url);
        self.profile_map.insert(&account_id, &profile);
    }
}
