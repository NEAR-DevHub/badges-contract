use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, CryptoHash, PanicOnDefault,
    Promise, PromiseOrValue,
};
use std::collections::HashMap;
use near_sdk::serde_json::json;

pub use crate::approval::*;
pub use crate::events::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::nft_core::*;
pub use crate::owner::*;
pub use crate::royalty::*;
pub use crate::series::*;

mod approval;
mod enumeration;
mod events;
mod internal;
mod metadata;
mod nft_core;
mod owner;
mod royalty;
mod series;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";


// Represents the series type. All tokens will derive this data.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Series {
    // Metadata including title, num copies etc.. that all tokens will derive from
    metadata: TokenMetadata,
    // Royalty used for all tokens in the collection
    royalty: Option<HashMap<AccountId, u32>>,
    // Set of tokens in the collection
    tokens: UnorderedSet<TokenId>,
    // What is the price of each token in this series? If this is specified, when minting,
    // Users will need to attach enough $NEAR to cover the price.
    price: Option<Balance>,
    // Owner of the collection
    owner_id: AccountId,
}

impl Series {
    pub fn update_metadata(&mut self, metadata: TokenMetadata) {
        self.metadata = metadata;
    }

    pub fn update_royalty(&mut self, royalty: Option<HashMap<AccountId, u32>>) {
        self.royalty = royalty;
    }

    pub fn update_price(&mut self, price: Option<Balance>) {
        self.price = price;
    }

    pub fn update_owner_id(&mut self, owner_id: AccountId) {
        self.owner_id = owner_id;
    }
}

pub type SeriesId = u64;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //approved minters
    pub approved_minters: LookupSet<AccountId>,

    //approved users that can create series
    pub approved_creators: LookupSet<AccountId>,

    //Map the collection ID (stored in Token obj) to the collection data
    pub series_by_id: UnorderedMap<SeriesId, Series>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: UnorderedMap<TokenId, Token>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
     
    // Add a new field for the allowed addresses
    pub allowed_transfers: UnorderedSet<AccountId>,
    
}



/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    ApprovedMinters,
    ApprovedCreators,
    SeriesById,
    SeriesByIdInner { account_id_hash: CryptoHash },
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    NFTContractMetadata,
    AllowedTransfers,
}


#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]

    pub fn new_default_meta(owner_id: AccountId) -> Self {
         const UPDATED_ICON: &str = "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";
        //const UPDATED_ICON: &str = "https://bafybeiaxaw2hkasbwfm27k56h2ntr2vkh2jzvdztqor6kjqzw7goqze7rm.ipfs.nftstorage.link/meteor_icon.png";
    
        let metadata = NFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "DevHub Badges".to_string(),
            symbol: "DEVHUB".to_string(),
            icon: Some(UPDATED_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
    
        let event_data = json!({
            "standard": "nep171",
            "version": "1.1.0",
            "event": "contract_metadata_update",
            "data": metadata
        });
        env::log_str(&event_data.to_string());
    
        Self::new(owner_id, metadata)
    }
   
    pub fn update_metadata(&mut self, owner_id: AccountId, metadata: NFTContractMetadata) {
        require!(env::predecessor_account_id() == self.owner_id, "Only the contract owner can update the metadata");
    
        // Update the metadata
        self.metadata = LazyOption::new(
            StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
            Some(&metadata),
        );
    
        // Log the event
        let event_data = json!({
            "standard": "nep171",
            "version": "1.1.0",
            "event": "contract_metadata_update",
            "data": metadata
        });
        env::log_str(&event_data.to_string());
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        // Create the approved minters set and insert the owner
        let mut approved_minters =
            LookupSet::new(StorageKey::ApprovedMinters.try_to_vec().unwrap());
        approved_minters.insert(&owner_id);

        // Create the approved creators set and insert the owner
        let mut approved_creators =
            LookupSet::new(StorageKey::ApprovedCreators.try_to_vec().unwrap());
        approved_creators.insert(&owner_id);
        
        
        // Create a variable of type Self with all the fields initialized.
        let this = Self {
            approved_minters,
            approved_creators,
            series_by_id: UnorderedMap::new(StorageKey::SeriesById.try_to_vec().unwrap()),
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: UnorderedMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            //set the &owner_id field equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            allowed_transfers: UnorderedSet::new(StorageKey::AllowedTransfers.try_to_vec().unwrap()),
        };

        //return the Contract object
        this
    }

  
    //near call CONTRACT_ACCOUNT_ID new_default_meta '{"owner_id": "OWNER_ACCOUNT_ID"}' --accountId YOUR_ACCOUNT_ID

    pub fn update_series_metadata(&mut self, series_id: SeriesId, metadata: TokenMetadata) {
        let mut series = self.series_by_id.get(&series_id).expect("Series not found");
        series.update_metadata(metadata);
        self.series_by_id.insert(&series_id, &series);

        // Emit the event
        let event_data = json!({
            "standard": "nep171",
            "version": "1.1.0",
            "event": "nft_metadata_update",
            "data": []
        });
        env::log_str(&event_data.to_string());
    }

    pub fn update_series_royalty(
        &mut self,
        series_id: SeriesId,
        royalty: Option<HashMap<AccountId, u32>>,
    ) {
        let mut series = self.series_by_id.get(&series_id).expect("Series not found");
        series.update_royalty(royalty);
        self.series_by_id.insert(&series_id, &series);

        // Emit the event
        let event_data = json!({
            "standard": "nep171",
            "version": "1.1.0",
            "event": "contract_metadata_update",
            "data": []
        });
        env::log_str(&event_data.to_string());
    }

    pub fn update_series_price(&mut self, series_id: SeriesId, price: Option<Balance>) {
        let mut series = self.series_by_id.get(&series_id).expect("Series not found");
        series.update_price(price);
        self.series_by_id.insert(&series_id, &series);

        // Emit the event
        let event_data = json!({
            "standard": "nep171",
            "version": "1.1.0",
            "event": "contract_metadata_update",
            "data": []
        });
        env::log_str(&event_data.to_string());
    }

    pub fn update_series_owner_id(&mut self, series_id: SeriesId, owner_id: AccountId) {
        let mut series = self.series_by_id.get(&series_id).expect("Series not found");
        series.update_owner_id(owner_id);
        self.series_by_id.insert(&series_id, &series);

        // Emit the event
        let event_data = json!({
            "standard": "nep171",
            "version": "1.1.0",
            "event": "contract_metadata_update",
            "data": []
        });
        env::log_str(&event_data.to_string());
    }

    
    // Add a new function for setting allowed addresses
    pub fn set_allowed_addresses(&mut self, addresses: Vec<AccountId>) {
        assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can set allowed addresses");
        for address in addresses {
            self.allowed_transfers.insert(&address);
        }
    }

    // Add a new function for transferring non-transferable tokens
    pub fn transfer(&mut self, new_owner_id: AccountId, token_id: String) {
        assert!(self.allowed_transfers.contains(&new_owner_id), "Transfer not allowed to this address");
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");
        token.owner_id = new_owner_id.clone();
        self.tokens_by_id.insert(&token_id, &token);
    }

}