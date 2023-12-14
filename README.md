## DevHub Badges -  NFT Series Contract

This project is a NEP-171 compliant Non-Fungible Token (NFT) contract that introduces the concept of a "series". A series is a collection of NFTs that share the same metadata and are owned by the same account. When a new NFT is minted, it is added to a specific series and inherits the metadata of that series.
Features

- Minting NFTs as part of a series
- Updating series metadata
- Setting allowed addresses for transfers
- Transferring non-transferable tokens

### Usage
#### Prerequisites

- Install and configure near-cli
#### Creating a Series

To create a series with all the correct metadata, use the create_series function:

```
near call YOUR_CONTRACT_ID create_series '{
  "id": 1,
  "metadata": {
    "title": "Series Title",
    "description": "Series Description",
    ...
  }
}' --accountId YOUR_ACCOUNT_ID --amount 1
```


#### Minting NFTs

To mint a new NFT that is part of a series, use the nft_mint function:
```near call YOUR_CONTRACT_ID nft_mint '{"id": 1, "receiver_id": "RECEIVER_ACCOUNT_ID"}' --accountId YOUR_ACCOUNT_ID --amount 1```


#### Setting Allowed Addresses

To add an address to set_allowed_addresses, use the set_allowed_addresses function:
```
near call YOUR_CONTRACT_ID set_allowed_addresses '{
  "addresses": ["address1", "address2", "address3"]
}' --accountId YOUR_ACCOUNT_ID
```

*Note: The set_allowed_addresses function will overwrite the existing list of allowed addresses each time it's called. It does not append to the existing list. If you want to add new addresses without removing the existing ones, you would need to include all addresses (both old and new) every time you call the function.
Contributing*

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

#### License

MIT
