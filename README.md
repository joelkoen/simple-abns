# simple-abns

simple-abns parses the [ABR's Australian Business Number dataset](https://data.gov.au/data/dataset/abn-bulk-extract) and converts it to a simpler JSON format.

You can download a copy of the [converted dataset](https://pub.joel.net.au/datasets/simple-abns/2024-11-27-simple-abns.jsonl.zst). Note that this is not updated automatically - please contact me or open an issue if a refresh is long overdue.

You can also find machine-readable names for the entity types the ABR uses in [./entity_types.json](./entity_types.json).

If you'd like to generate the dataset yourself, you'll need to download the raw XML data and place all 20 chunks in `./raw`. simple-abns will parse them and print each ABN record as a seperate line. You can see progress and compress the output using:

```sh
cargo run --release | pv -ls 18M | zstd -T0 -9 > simple-abns.jsonl.zst
```

## Example

**Input**:

```xml
<ABR recordLastUpdatedDate="20240412" replaced="N">
	<ABN status="ACT" ABNStatusFromDate="19991101">88712649015</ABN>
	<EntityType>
		<EntityTypeInd>SGE</EntityTypeInd>
		<EntityTypeText>State Government Entity</EntityTypeText>
	</EntityType>
	<MainEntity>
		<NonIndividualName type="MN">
			<NonIndividualNameText>STATE EMERGENCY SERVICE (NSW)</NonIndividualNameText>
		</NonIndividualName>
		<BusinessAddress>
			<AddressDetails>
				<State>NSW</State>
				<Postcode>2500</Postcode>
			</AddressDetails>
		</BusinessAddress>
	</MainEntity>
	<GST status="ACT" GSTStatusFromDate="20000701" />
	<OtherEntity>
		<NonIndividualName type="TRD">
			<NonIndividualNameText>NEW SOUTH WALES STATE EMERGENCY SERVICE</NonIndividualNameText>
		</NonIndividualName>
	</OtherEntity>
</ABR>
```

**Output:**

```json
{
  "abn": "88712649015",
  "status": "Active",
  "status_since": "1999-11-01",
  "last_updated": "2024-04-12",
  "entity_name": {
    "type": "NonIndividual",
    "name": "STATE EMERGENCY SERVICE (NSW)"
  },
  "entity_type": "SGE",
  "trade_names": [
    "NEW SOUTH WALES STATE EMERGENCY SERVICE"
  ],
  "postcode": "2500",
  "state": "NSW",
  "gst_status": "Active",
  "gst_status_since": "2000-07-01"
}
```
