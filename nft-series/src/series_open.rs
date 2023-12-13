use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// Define the token structure
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Token {
    pub id: u64,
    pub series_id: u64,
    pub owner: String,
    pub image_url: String,
    pub reference: String,
    pub title: String,
    pub description: String,
}

// Define the series structure
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Series {
    pub id: u64,
    pub name: String,
}

// Define the contract
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct OpenCollection {
    pub tokens: Vec<Token>,
    pub series: Vec<Series>,
}

#[near_bindgen]
impl OpenCollection {
    // Mint a new token with specified details
    pub fn mint_token(
        &mut self,
        id: u64,
        series_id: u64,
        owner: String,
        image_url: String,
        reference: String,
        title: String,
        description: String,
    ) {
        let new_token = Token {
            id,
            series_id,
            owner,
            image_url,
            reference,
            title,
            description,
        };
        self.tokens.push(new_token);
    }

    // Create a new series
    pub fn create_series(&mut self, id: u64, name: String) {
        let new_series = Series { id, name };
        self.series.push(new_series);
    }

    // Get the token details by ID
    pub fn get_token(&self, id: u64) -> Option<Token> {
        self.tokens.iter().find(|token| token.id == id).cloned()
    }

    // Get the series details by ID
    pub fn get_series(&self, id: u64) -> Option<Series> {
        self.series.iter().find(|series| series.id == id).cloned()
    }

    // Update the series name by ID
    pub fn update_series_name(&mut self, id: u64, name: String) {
        if let Some(series) = self.series.iter_mut().find(|series| series.id == id) {
            series.name = name;
        }
    }

    // Update the token details by ID
    pub fn update_token_details(
        &mut self,
        id: u64,
        owner: String,
        image_url: String,
        reference: String,
        title: String,
        description: String,
    ) {
        if let Some(token) = self.tokens.iter_mut().find(|token| token.id == id) {
            token.owner = owner;
            token.image_url = image_url;
            token.reference = reference;
            token.title = title;
            token.description = description;
        }
    }
}
