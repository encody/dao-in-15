use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, UnorderedSet, Vector},
    env, near_bindgen, require,
    serde::Serialize,
    AccountId, BorshStorageKey, PanicOnDefault,
};

type ProposalId = u64;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Members,
    Proposals,
    Votes,
    VotesForProposal(ProposalId),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: ProposalId,
    pub title: String,
    pub description: String,
    pub author: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum Vote {
    Yea,
    Nay,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub members: UnorderedSet<AccountId>,
    pub proposals: Vector<Proposal>,
    pub votes: LookupMap<ProposalId, UnorderedMap<AccountId, Vote>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            members: UnorderedSet::new(StorageKey::Members),
            proposals: Vector::new(StorageKey::Proposals),
            votes: LookupMap::new(StorageKey::Votes),
        }
    }

    pub fn add_proposal(&mut self, title: String, description: String) -> ProposalId {
        let author = env::predecessor_account_id();

        require!(
            self.members.contains(&author),
            "Must be a member to submit a proposal"
        );

        let proposal = Proposal {
            id: self.proposals.len(),
            title,
            description,
            author,
        };

        self.proposals.push(&proposal);

        proposal.id
    }

    pub fn get_proposal(&self, proposal_id: ProposalId) -> Option<Proposal> {
        self.proposals.get(proposal_id)
    }

    pub fn get_members(&self) -> Vec<AccountId> {
        self.members.iter().collect()
    }

    pub fn add_member(&mut self, member_id: AccountId) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only the owner can add members"
        );

        self.members.insert(&member_id);
    }

    pub fn remove_member(&mut self, member_id: AccountId) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only the owner can remove members"
        );

        self.members.remove(&member_id);
    }

    pub fn vote(&mut self, proposal_id: ProposalId, is_yes: bool) {
        let predecessor = env::predecessor_account_id();
        require!(
            self.members.contains(&predecessor),
            "Must be a member to vote on a proposal"
        );

        let mut votes_for_proposal = self
            .votes
            .get(&proposal_id)
            .unwrap_or_else(|| UnorderedMap::new(StorageKey::VotesForProposal(proposal_id)));

        require!(
            votes_for_proposal.get(&predecessor).is_none(),
            "Already voted"
        );

        votes_for_proposal.insert(&predecessor, &if is_yes { Vote::Yea } else { Vote::Nay });

        self.votes.insert(&proposal_id, &votes_for_proposal);
    }

    pub fn tally(&self, proposal_id: ProposalId) -> (u32, u32) {
        self.votes
            .get(&proposal_id)
            .map(|votes_for_proposal| {
                votes_for_proposal
                    .values()
                    .fold((0, 0), |(yeas, nays), vote| match vote {
                        Vote::Yea => (yeas + 1, nays),
                        Vote::Nay => (yeas, nays + 1),
                    })
            })
            .unwrap_or_default()
    }
}
