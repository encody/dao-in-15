mod contract;
pub use contract::*;

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId, VMContext};

    use crate::Contract;

    #[test]
    fn test() {
        let owner: AccountId = "owner_id".parse().unwrap();
        let mut dao = Contract::new(owner.clone());
        fn context(id: AccountId) -> VMContext {
            VMContextBuilder::new().predecessor_account_id(id).build()
        }
        testing_env!(context(owner.clone()));
        dao.add_member(owner.clone());

        let mut members = vec![];

        for i in 0..10 {
            let account: AccountId = format!("account_{i}").parse().unwrap();
            members.push(account);
        }

        for m in &members {
            dao.add_member(m.clone());
        }

        let proposal_id = dao.add_proposal(
            "My cool proposal".to_string(),
            "Let's do this thing!".to_string(),
        );

        for (i, m) in members.iter().enumerate() {
            testing_env!(context(m.clone()));
            dao.vote(proposal_id, i % 2 == 0);
        }

        assert_eq!(
            (5, 5),
            dao.tally(proposal_id),
            "Votes should be evenly split"
        );
    }
}
