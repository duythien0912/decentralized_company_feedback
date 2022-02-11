/*!
Private sale contract implementation with JSON serialization.
NOTES:
  -
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::panic;

near_sdk::setup_alloc!();

pub type FeedbackId = usize;
pub type UserId = usize;
pub type CompanyId = usize;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    /// contract owner
    pub owner_id: String,

    /// All feedbacks
    pub feedbacks: UnorderedMap<FeedbackId, Feedback>,

    /// All user
    pub users: UnorderedMap<UserId, User>,

    /// All companies
    pub companies: UnorderedMap<CompanyId, Company>,

    // Next id for feedback
    pub next_feedback_id: FeedbackId,

    // Next id for user
    pub next_user_id: UserId,

    // Next id for company
    pub next_company_id: CompanyId,
}

#[derive(Serialize, Deserialize, Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Feedback {
    pub id: FeedbackId,
    pub parent_id: FeedbackId,
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub content: String,
    pub reaction: usize,
    pub rating: usize,
    pub up_vote: usize,
    pub down_vote: usize,
    pub report_vote: usize,
    pub create_at: u64,
    pub update_at: u64,
    pub activate: bool,
}
impl Feedback {
    // reaction = 0 // like
    // reaction = 1 // dislike
    // reaction = 2 // ban

    // Return mock feedback
    pub fn mock() -> Feedback {
        Feedback {
            id: 0,
            parent_id: 0,
            user_id: 0,
            company_id: 0,
            content: "Mock feedback".to_string(),
            reaction: 0,
            rating: 0,
            up_vote: 0,
            down_vote: 0,
            report_vote: 0,
            create_at: 0,
            update_at: 0,
            activate: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct User {
    pub id: UserId,
    pub address: String,
    pub name: String,
    pub title: String,
    pub create_at: u64,
    pub update_at: u64,
    pub activate: bool,
}
impl User {
    // Return mock user
    pub fn mock() -> Self {
        Self {
            id: 0,
            address: env::predecessor_account_id(),
            name: "Mock User".to_string(),
            title: "Mock Title".to_string(),
            create_at: 0,
            update_at: 0,
            activate: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub rating: usize,
    pub rating_count: usize,
    pub company_type: String,
    pub size: String,
    pub address: String,
    pub create_at: u64,
    pub update_at: u64,
    pub activate: bool,
}
impl Company {
    // Return mock company
    pub fn mock() -> Self {
        Self {
            id: 0,
            name: "Mock Company".to_string(),
            rating: 0,
            rating_count: 0,
            company_type: "Mock Company Type".to_string(),
            size: "Mock Company Size".to_string(),
            address: "Mock Company Address".to_string(),
            create_at: 0,
            update_at: 0,
            activate: true,
        }
    }
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Feedbacks,
    Users,
    Companies,
}

/// Contract impl.
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Owner's account ID is invalid."
        );
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: owner_id,
            feedbacks: UnorderedMap::new(StorageKey::Feedbacks),
            users: UnorderedMap::new(StorageKey::Users),
            companies: UnorderedMap::new(StorageKey::Companies),
            next_feedback_id: 0,
            next_user_id: 0,
            next_company_id: 0,
        }
    }

    pub fn only_contract_owner(&mut self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only contract owner can call this method."
        );
    }

    /// Update activate feedback
    /// @param FeedbackId id
    /// @param bool activate
    /// @return feedback
    pub fn update_active_feedback(&mut self, id: FeedbackId, activate: bool) -> Feedback {
        self.only_contract_owner();
        match self.feedbacks.get(&id).as_mut() {
            Some(feedback) => {
                feedback.activate = activate;
                feedback.update_at = env::block_timestamp();
                self.feedbacks.insert(&id, feedback);
                feedback.clone()
            }
            None => panic!("Feedback does not exist"),
        }
    }

    /// Update activate user
    /// @param UserId id
    /// @param bool activate
    /// @return user
    pub fn update_active_user(&mut self, id: UserId, activate: bool) -> User {
        self.only_contract_owner();
        match self.users.get(&id).as_mut() {
            Some(user) => {
                user.activate = activate;
                user.update_at = env::block_timestamp();
                self.users.insert(&id, user);
                user.clone()
            }
            None => panic!("User does not exist"),
        }
    }

    /// Update activate company
    /// @param CompanyId id
    /// @param bool activate
    /// @return company
    pub fn update_active_company(&mut self, id: CompanyId, activate: bool) -> Company {
        self.only_contract_owner();
        match self.companies.get(&id).as_mut() {
            Some(company) => {
                company.activate = activate;
                company.update_at = env::block_timestamp();
                self.companies.insert(&id, company);
                company.clone()
            }
            None => panic!("Company does not exist"),
        }
    }

    /// Get feedbacks paging
    /// @param usize page
    /// @param usize size
    /// @return feedbacks
    pub fn get_feedbacks(&mut self, page: usize, size: usize) -> Vec<Feedback> {
        let mut feedbacks = vec![];
        for (_, feedback) in self.feedbacks.iter() {
            feedbacks.push(feedback);
        }
        feedbacks.sort_by(|a, b| b.create_at.cmp(&a.create_at));
        feedbacks.into_iter().skip(page * size).take(size).collect()
    }

    /// Get activate feedback by id
    /// @param FeedbackId id
    /// @return feedback
    pub fn get_feedback(&self, id: FeedbackId) -> Feedback {
        let feedback = self.feedbacks.get(&id).unwrap();
        assert!(feedback.id == id, "Feedback does not exist");

        feedback
    }

    /// Get activate feedbacks by user_id paging
    /// @param UserId user_id
    /// @param usize page
    /// @param usize size
    /// @return feedbacks
    pub fn get_feedbacks_by_user_id_paging(
        &self,
        user_id: UserId,
        page: usize,
        size: usize,
    ) -> Vec<Feedback> {
        let mut feedbacks = vec![];
        for (_, feedback) in self.feedbacks.iter() {
            if feedback.user_id == user_id && feedback.activate {
                feedbacks.push(feedback);
            }
        }
        feedbacks.sort_by(|a, b| b.create_at.cmp(&a.create_at));
        feedbacks.into_iter().skip(page * size).take(size).collect()
    }

    /// Get activate feedbacks by parent_id paging
    /// @param FeedbackId parent_id
    /// @param usize page
    /// @param usize size
    /// @return feedbacks
    pub fn get_feedbacks_by_parent_id_paging(
        &self,
        parent_id: FeedbackId,
        page: usize,
        size: usize,
    ) -> Vec<Feedback> {
        let mut feedbacks = vec![];
        for (_, feedback) in self.feedbacks.iter() {
            if feedback.parent_id == parent_id && feedback.activate {
                feedbacks.push(feedback);
            }
        }
        feedbacks.sort_by(|a, b| b.create_at.cmp(&a.create_at));
        feedbacks.into_iter().skip(page * size).take(size).collect()
    }

    /// Get activate feedbacks by company_id paging
    /// @param CompanyId company_id
    /// @param usize page
    /// @param usize size
    /// @return feedbacks
    pub fn get_feedbacks_by_company_id_paging(
        &self,
        company_id: CompanyId,
        page: usize,
        size: usize,
    ) -> Vec<Feedback> {
        let mut feedbacks = vec![];
        for (_, feedback) in self.feedbacks.iter() {
            if feedback.company_id == company_id && feedback.activate {
                feedbacks.push(feedback);
            }
        }
        feedbacks.sort_by(|a, b| b.create_at.cmp(&a.create_at));
        feedbacks.into_iter().skip(page * size).take(size).collect()
    }

    /// Get activate companies paging
    /// @return companies
    pub fn get_companies_paging(&self, page: usize, size: usize) -> Vec<Company> {
        let mut companies = vec![];
        for (_, company) in self.companies.iter() {
            if company.activate {
                companies.push(company);
            }
        }
        companies.sort_by(|a, b| b.update_at.cmp(&a.update_at));
        companies.into_iter().skip(page * size).take(size).collect()
    }

    /// Get activate company by id
    /// @param CompanyId id
    /// @return company
    pub fn get_company(&self, id: CompanyId) -> Company {
        let company = self.companies.get(&id).unwrap();
        assert!(company.id == id, "Company does not exist");

        company
    }

    /// Create new feedback
    /// @param Feedback feedback
    /// @return feedback
    pub fn create_feedback(&mut self, feedback: Feedback) -> Feedback {
        let id = self.next_feedback_id;

        let mut feedback = feedback;
        feedback.id = id;
        feedback.up_vote = 0;
        feedback.down_vote = 0;
        feedback.report_vote = 0;
        feedback.create_at = env::block_timestamp();
        feedback.update_at = env::block_timestamp();
        feedback.activate = false;

        // Create new feedback
        self.feedbacks.insert(&id, &feedback);
        self.next_feedback_id += 1;

        // Update parent feedback up_vote, down_vote, ban_vote by reaction
        match self.feedbacks.get(&feedback.parent_id).as_mut() {
            Some(parent_feedback) => {
                if feedback.reaction == 0 {
                    parent_feedback.up_vote += 1;
                }
                if feedback.reaction == 1 {
                    parent_feedback.down_vote += 1;
                }
                if feedback.reaction == 2 {
                    parent_feedback.report_vote += 1;
                }

                self.feedbacks.insert(&parent_feedback.id, &parent_feedback);
            }
            None => {
                env::log(format!("Didn't find feedback by parent_id.").as_bytes());
            }
        };

        // Update company rating
        match self.companies.get(&feedback.company_id).as_mut() {
            Some(company) => {
                company.rating = company.rating + feedback.rating;
                company.rating_count = company.rating_count + 1;
                self.companies.insert(&feedback.company_id, &company);
            }
            None => {
                env::log(format!("Didn't find company by id.").as_bytes());
            }
        };

        feedback
    }

    /// Update feedback by id
    /// @param FeedbackId id
    /// @param Feedback feedback
    /// @return feedback
    pub fn update_feedback(&mut self, id: FeedbackId, feedback: Feedback) -> Feedback {
        // Update feedback

        match self.feedbacks.get(&id).as_mut() {
            Some(feedback) => {
                feedback.update_at = env::block_timestamp();
                self.feedbacks.insert(&id, &feedback);
            }
            None => panic!("Feedback does not exist"),
        };

        // Update company rating
        match self.companies.get(&feedback.company_id).as_mut() {
            Some(company) => {
                let feedback_find_by_id = self.feedbacks.get(&id).unwrap();
                company.rating = company.rating - feedback_find_by_id.rating + feedback.rating;
                self.companies.insert(&company.id, &company);
            }
            None => {}
        };

        feedback
    }

    /// Create new user
    /// @param User user
    /// @return user
    pub fn create_user(&mut self, user: User) -> User {
        let id = self.next_user_id;

        let mut user = user;

        user.id = id;
        user.create_at = env::block_timestamp();
        user.update_at = env::block_timestamp();
        user.activate = false;

        self.users.insert(&id, &user);
        self.next_user_id += 1;

        user
    }

    /// Update user by id
    /// @param UserId id
    /// @param User user
    /// @return user
    pub fn update_user(&mut self, id: UserId, user: User) -> User {
        let user_find_by_id = self.users.get(&id).unwrap();
        assert!(user_find_by_id.id == id, "User does not exist");

        let mut user = user;

        user.update_at = env::block_timestamp();

        self.users.insert(&id, &user);

        user
    }

    /// Create new company
    /// @param Company company
    /// @return company
    pub fn create_company(&mut self, company: Company) -> Company {
        let id = self.next_company_id;
        let mut company = company;

        company.id = id;
        company.create_at = env::block_timestamp();
        company.update_at = env::block_timestamp();
        company.activate = false;

        self.companies.insert(&id, &company);
        self.next_company_id += 1;

        company
    }

    /// Update company by id
    /// @param CompanyId id
    /// @param Company company
    /// @return company
    pub fn update_company(&mut self, id: CompanyId, company: Company) -> Company {
        let company_find_by_id = self.companies.get(&id).unwrap();
        assert!(company_find_by_id.id == id, "Company does not exist");

        let mut company = company;

        company.update_at = env::block_timestamp();

        self.companies.insert(&id, &company);

        company
    }
}

/// Helper for init default test contract.
impl Default for Contract {
    fn default() -> Self {
        let owner_id = env::predecessor_account_id();
        Self {
            owner_id: owner_id,
            feedbacks: UnorderedMap::new(StorageKey::Feedbacks),
            users: UnorderedMap::new(StorageKey::Users),
            companies: UnorderedMap::new(StorageKey::Companies),
            next_feedback_id: 0,
            next_user_id: 0,
            next_company_id: 0,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    const CURRENT_ACCOUNT_ADDRESS: &str = "alice_near";
    const SIGNER_ACCOUNT_ADDRESS: &str = "bob_near";
    const PREDECESSOR_ACCOUNT_ADDRESS: &str = "carol_near";

    fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
        f: F,
    ) -> std::thread::Result<R> {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        result
    }

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: CURRENT_ACCOUNT_ADDRESS.to_string(),
            predecessor_account_id: PREDECESSOR_ACCOUNT_ADDRESS.to_string(),
            signer_account_id: SIGNER_ACCOUNT_ADDRESS.to_string(),
            signer_account_pk: vec![0, 1, 2],
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_owner_contract() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Contract::default();
        assert_eq!(contract.owner_id, PREDECESSOR_ACCOUNT_ADDRESS.to_string());
    }

    // Test fn create_feedback
    #[test]
    fn test_create_feedback() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
        let feedback = Feedback::mock();
        let new_feedback = contract.create_feedback(feedback.clone());

        assert_eq!(
            new_feedback.content, feedback.content,
            "Feedback is not equal"
        );
    }

    // Test fn update_feedback
    #[test]
    fn test_update_feedback() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_feedback = Feedback::mock();
        let first_feedback = contract.create_feedback(mock_feedback.clone());
        assert_eq!(
            first_feedback.content, mock_feedback.content,
            "first_feedback is not equal"
        );

        let mut mock_feedback_2 = Feedback::mock();
        mock_feedback_2.content = "feedback2".to_string();

        let first_feedback_updated =
            contract.update_feedback(first_feedback.id, mock_feedback_2.clone());

        assert_eq!(
            first_feedback_updated.content, mock_feedback_2.content,
            "first_feedback_updated is not equal"
        );
    }

    #[test]
    fn test_fail_update_feedback() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        catch_unwind_silent(move || {
            contract.update_feedback(99999, Feedback::mock().clone());
        })
        .unwrap_err();
    }

    // Test fn get_feedback
    #[test]
    fn test_get_feedback() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_feedback = Feedback::mock();
        let first_feedback = contract.create_feedback(mock_feedback.clone());
        assert_eq!(
            first_feedback.content, mock_feedback.content,
            "first_feedback is not equal"
        );

        let get_feedback = contract.get_feedback(first_feedback.id);
        assert_eq!(
            get_feedback.content, mock_feedback.content,
            "get_feedback not found"
        );
    }

    // Test fn get_feedbacks
    #[test]
    fn test_get_feedbacks() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_feedback = Feedback::mock();
        let first_feedback = contract.create_feedback(mock_feedback.clone());
        assert_eq!(
            first_feedback.content, mock_feedback.content,
            "first_feedback is not equal"
        );

        let mock_feedback_2 = Feedback::mock();
        let second_feedback = contract.create_feedback(mock_feedback_2.clone());
        assert_eq!(
            second_feedback.content, mock_feedback_2.content,
            "second_feedback is not equal"
        );

        let get_feedbacks = contract.get_feedbacks(0, 10);
        assert_eq!(get_feedbacks.len(), 2, "get_feedbacks not found");
    }

    // Test fn update_active_feedback
    #[test]
    fn test_update_active_feedback() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_feedback = Feedback::mock();
        let first_feedback = contract.create_feedback(mock_feedback.clone());
        assert_eq!(
            first_feedback.activate, false,
            "feedback should be deactivate by default"
        );

        let updated_feedback = contract.update_active_feedback(first_feedback.id, true);
        assert_eq!(
            updated_feedback.activate, true,
            "feedback should be activate after update"
        );
    }

    // Test fn get_feedbacks_by_user_id_paging
    #[test]
    fn test_get_feedbacks_by_user_id_paging() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mut mock_feedback = Feedback::mock();

        let mock_user = User::mock();
        let first_user = contract.create_user(mock_user.clone());

        mock_feedback.user_id = first_user.id;

        let first_feedback = contract.create_feedback(mock_feedback.clone());
        contract.update_active_feedback(first_feedback.id, true);

        assert_eq!(
            first_feedback.user_id, first_user.id,
            "first_feedback user_id is not correct"
        );

        let get_feedbacks = contract.get_feedbacks_by_user_id_paging(first_user.id, 0, 10);

        assert_eq!(get_feedbacks.len(), 1, "get_feedbacks not found");

        contract.create_feedback(mock_feedback.clone());

        assert_eq!(
            get_feedbacks.len(),
            1,
            "get_feedbacks length is not correct"
        );

        for feedback in get_feedbacks {
            assert_eq!(
                feedback.user_id, first_user.id,
                "feedback user_id is not correct"
            );
        }
    }

    // Test fn get_feedbacks_by_parent_id_paging
    #[test]
    fn test_get_feedbacks_by_parent_id_paging() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mut mock_feedback = Feedback::mock();
        mock_feedback.parent_id = 999;

        let first_feedback = contract.create_feedback(mock_feedback.clone());
        contract.update_active_feedback(first_feedback.id, true);

        mock_feedback.parent_id = first_feedback.id;
        let second_feedback = contract.create_feedback(mock_feedback.clone());
        contract.update_active_feedback(second_feedback.id, true);

        let feedbacks = contract.get_feedbacks_by_parent_id_paging(first_feedback.id, 0, 10);

        assert_eq!(feedbacks.len(), 1, "feedbacks not found");
        assert_eq!(
            feedbacks[0].parent_id, first_feedback.id,
            "feedback id is not correct"
        );

        let third_feedback = contract.create_feedback(mock_feedback.clone());
        contract.update_active_feedback(third_feedback.id, true);
        let feedbacks = contract.get_feedbacks_by_parent_id_paging(first_feedback.id, 0, 10);

        assert_eq!(feedbacks.len(), 2, "feedbacks not found");
        assert_eq!(
            feedbacks[1].parent_id, first_feedback.id,
            "third_feedback parent_id is not correct"
        );

        for feedback in feedbacks {
            assert_eq!(
                feedback.parent_id, first_feedback.id,
                "feedback parent_id is not correct"
            );
        }
    }

    // Test fn get_feedbacks_by_company_id_paging
    #[test]
    fn test_get_feedbacks_by_company_id_paging() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        // Create company
        let mock_company = Company::mock();
        let first_company = contract.create_company(mock_company.clone());

        // Create feedback by company
        let mut mock_feedback = Feedback::mock();
        mock_feedback.company_id = first_company.id;
        let first_feedback = contract.create_feedback(mock_feedback.clone());
        contract.update_active_feedback(first_feedback.id, true);

        // Verify feedback by company id
        let feedbacks = contract.get_feedbacks_by_company_id_paging(first_company.id, 0, 10);
        assert_eq!(feedbacks.len(), 1, "feedbacks not found");
        assert_eq!(
            feedbacks[0].company_id, first_company.id,
            "feedback company_id is not correct"
        );

        // Create feedback by company
        let second_feedback = contract.create_feedback(mock_feedback.clone());
        contract.update_active_feedback(second_feedback.id, true);

        // Verify feedback by company id
        let feedbacks = contract.get_feedbacks_by_company_id_paging(first_company.id, 0, 10);
        assert_eq!(feedbacks.len(), 2, "feedbacks not found");
        assert_eq!(
            feedbacks[1].company_id, first_company.id,
            "feedback company_id is not correct"
        );
    }

    // Test fn create_user
    #[test]
    fn test_create_user() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_user = User::mock();
        let first_user = contract.create_user(mock_user.clone());
        assert_eq!(first_user.name, mock_user.name, "first_user is not equal");
    }

    // Test fn update_user
    #[test]
    fn test_update_user() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_user = User::mock();
        let first_user = contract.create_user(mock_user.clone());
        assert_eq!(first_user.name, mock_user.name, "first_user is not equal");

        let mut mock_user_2 = User::mock();
        mock_user_2.name = "user2".to_string();

        let first_user_updated = contract.update_user(first_user.id, mock_user_2.clone());
        assert_eq!(
            first_user_updated.name, mock_user_2.name,
            "first_user_updated is not equal"
        );
    }

    // Test fn update_user fail
    #[test]
    fn test_fail_update_user() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        catch_unwind_silent(move || {
            contract.update_user(99999, User::mock().clone());
        })
        .unwrap_err();
    }

    // Test fn update_active_user
    #[test]
    fn test_update_active_user() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_user = User::mock();
        let first_user = contract.create_user(mock_user.clone());
        assert_eq!(
            first_user.activate, false,
            "user should be deactivate by default"
        );

        let updated_user = contract.update_active_user(first_user.id, true);
        assert_eq!(
            updated_user.activate, true,
            "user should be activate after update"
        );
    }

    // Test fn create_company
    #[test]
    fn test_create_company() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_company = Company::mock();
        let first_company = contract.create_company(mock_company.clone());
        assert_eq!(
            first_company.name, mock_company.name,
            "first_company is not equal"
        );
    }

    // Test fn update_company
    #[test]
    fn test_update_company() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_company = Company::mock();
        let first_company = contract.create_company(mock_company.clone());
        assert_eq!(
            first_company.name, mock_company.name,
            "first_company is not equal"
        );

        let mut mock_company_2 = Company::mock();
        mock_company_2.name = "company2".to_string();

        let first_company_updated =
            contract.update_company(first_company.id, mock_company_2.clone());
        assert_eq!(
            first_company_updated.name, mock_company_2.name,
            "first_company_updated is not equal"
        );
    }

    // Test fn update_company fail
    #[test]
    fn test_fail_update_company() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        catch_unwind_silent(move || {
            contract.update_company(99999, Company::mock().clone());
        })
        .unwrap_err();
    }

    // Test fn update_active company
    #[test]
    fn test_update_active_company() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();

        let mock_company = Company::mock();
        let first_company = contract.create_company(mock_company.clone());
        assert_eq!(
            first_company.activate, false,
            "company should be deactivate by default"
        );

        let updated_company = contract.update_active_company(first_company.id, true);
        assert_eq!(
            updated_company.activate, true,
            "company should be activate after update"
        );
    }
}
