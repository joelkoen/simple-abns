use std::str::FromStr;

use anyhow::{bail, ensure, Context, Result};
use chrono::NaiveDate;
use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

use crate::model::{ABNRecord, EntityName, EntityType, Status};

pub fn parse_record(xml: &str) -> Result<ABNRecord> {
    let mut helper = Helper::default();
    for e in EventReader::from_str(xml) {
        helper.handle(e?)?;
    }

    let replaced = helper
        .replaced
        .and_then(|x| yn(&x))
        .context("invalid replaced")?;
    ensure!(!replaced, "replaced was ignored as it was never true");

    let abn = helper.abn.context("missing abn")?;
    let status = match &*helper.abn_status.context("missing abn status")? {
        "ACT" => Status::Active,
        "CAN" => Status::Cancelled,
        _ => bail!("unknown abn status"),
    };
    let status_since = NaiveDate::parse_from_str(
        &helper.abn_status_since.context("missing abn since")?,
        "%Y%m%d",
    )?;
    let last_updated = NaiveDate::parse_from_str(
        &helper.last_updated.context("missing last updated")?,
        "%Y%m%d",
    )?;

    // TODO: some have a postcode but no state
    let postcode = helper
        .postcode
        .and_then(|x| if x == "0000" { None } else { Some(x) });
    let state = helper.state;

    let asic_number = match helper.asic_number {
        Some(x) => {
            ensure!(
                helper.asic_number_type.is_some_and(|x| x == "undetermined"),
                "unexpected asic number type"
            );
            Some(x)
        }
        None => {
            ensure!(
                helper.asic_number_type.is_none(),
                "unexpected asic number with type"
            );
            None
        }
    };

    let gst_status = match helper.gst_status.as_deref() {
        Some("ACT") => Some(Status::Active),
        Some("CAN") => Some(Status::Cancelled),
        Some("NON") => None,
        _ => bail!("invalid gst status"),
    };
    let gst_status_since = match helper.gst_status_since.as_deref() {
        Some("19000101") | None => None,
        Some(x) => Some(NaiveDate::parse_from_str(x, "%Y%m%d")?),
    };
    ensure!(
        gst_status.is_some() == gst_status_since.is_some(),
        "invalid gst status combo"
    );

    let entity_type = EntityType::from_str(
        helper
            .entity_type_id
            .as_deref()
            .context("missing entity type id")?,
    )
    .with_context(|| {
        format!(
            "unknown entity type: {:?} - {:?}",
            helper.entity_type, helper.entity_type_id
        )
    })?;

    let entity_name = match helper.individual_name_type.as_deref() {
        Some("LGL") => EntityName::Individual {
            title: helper.individual_name_title,
            given: helper.individual_name_given_1,
            given_2: helper.individual_name_given_2,
            family: helper
                .individual_name_family
                .context("missing family name")?,
        },
        Some(x) => bail!("unexpected individual name type: {x}"),
        None => {
            ensure!(
                helper.non_individual_name_type.is_some_and(|x| x == "MN"),
                "unexpected individual name type"
            );
            EntityName::NonIndividual {
                name: helper.non_individual_name.context("missing name")?,
            }
        }
    };

    let mut business_names = Vec::new();
    let mut trade_names = Vec::new();
    for (name, t) in helper.other_names.into_iter().zip(helper.other_name_types) {
        match &*t {
            // OTN - old trade name? definitely linked to pre-2012 and shown under trade names on website so im putting them here for now
            "TRD" | "OTN" => trade_names.push(name),
            "BN" => business_names.push(name),
            _ => bail!("unknown name type {t}"),
        }
    }

    Ok(ABNRecord {
        abn,
        status,
        status_since,
        last_updated,
        entity_name,
        entity_type,
        business_names,
        trade_names,
        postcode,
        state,
        asic_number,
        gst_status,
        gst_status_since,
    })
}

#[derive(Debug, Default)]
struct Helper {
    path: Vec<String>,

    last_updated: Option<String>,
    replaced: Option<String>,

    abn: Option<String>,
    abn_status: Option<String>,
    abn_status_since: Option<String>,

    entity_type: Option<String>,
    entity_type_id: Option<String>,

    individual_name_title: Option<String>,
    individual_name_given_1: Option<String>,
    individual_name_given_2: Option<String>,
    individual_name_family: Option<String>,
    individual_name_type: Option<String>,
    non_individual_name: Option<String>,
    non_individual_name_type: Option<String>,
    other_names: Vec<String>,
    other_name_types: Vec<String>,

    dgr_dates: Vec<String>,
    dgr_names: Vec<String>,

    state: Option<String>,
    postcode: Option<String>,

    asic_number: Option<String>,
    asic_number_type: Option<String>,

    gst_status: Option<String>,
    gst_status_since: Option<String>,
}

impl Helper {
    pub fn handle(&mut self, e: XmlEvent) -> Result<()> {
        self.handle_(e)
            .with_context(|| format!("in {:?}", self.path))
    }

    fn handle_(&mut self, e: XmlEvent) -> Result<()> {
        match e {
            XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            } => {
                self.path.push(name.local_name);

                self.handle_attrs(attributes)?;
            }
            XmlEvent::EndElement { name: _ } => {
                self.path.pop();
            }
            XmlEvent::Characters(x) => {
                let path = self.path();
                match path[..] {
                    ["ABR", "ABN"] => set(&mut self.abn, x)?,

                    ["ABR", "EntityType", "EntityTypeInd"] => set(&mut self.entity_type_id, x)?,
                    ["ABR", "EntityType", "EntityTypeText"] => set(&mut self.entity_type, x)?,

                    ["ABR", "LegalEntity", "IndividualName", "NameTitle"] => {
                        set(&mut self.individual_name_title, x)?
                    }
                    ["ABR", "LegalEntity", "IndividualName", "GivenName"] => {
                        if self.individual_name_given_1.is_none() {
                            set(&mut self.individual_name_given_1, x)?
                        } else {
                            set(&mut self.individual_name_given_2, x)?
                        }
                    }
                    ["ABR", "LegalEntity", "IndividualName", "FamilyName"] => {
                        set(&mut self.individual_name_family, x)?
                    }

                    ["ABR", "MainEntity", "BusinessAddress", "AddressDetails", "State"]
                    | ["ABR", "LegalEntity", "BusinessAddress", "AddressDetails", "State"] => {
                        set(&mut self.state, x)?
                    }
                    ["ABR", "MainEntity", "BusinessAddress", "AddressDetails", "Postcode"]
                    | ["ABR", "LegalEntity", "BusinessAddress", "AddressDetails", "Postcode"] => {
                        set(&mut self.postcode, x)?
                    }

                    ["ABR", "MainEntity", "NonIndividualName", "NonIndividualNameText"] => {
                        set(&mut self.non_individual_name, x)?
                    }
                    ["ABR", "OtherEntity", "NonIndividualName", "NonIndividualNameText"] => {
                        self.other_names.push(x)
                    }
                    ["ABR", "DGR", "NonIndividualName", "NonIndividualNameText"] => {
                        self.dgr_names.push(x)
                    }

                    ["ABR", "ASICNumber"] => set(&mut self.asic_number, x)?,

                    _ => eprintln!("unhandled text: {path:?}: {x}"),
                }
            }

            _ => (),
        }

        Ok(())
    }

    fn path(&self) -> Vec<&str> {
        self.path.iter().map(|s| s.as_str()).collect()
    }

    fn handle_attrs(&mut self, attrs: Vec<OwnedAttribute>) -> Result<()> {
        for (k, v) in attrs.into_iter().map(|x| (x.name.local_name, x.value)) {
            match (&self.path()[..], &*k) {
                (["ABR"], "recordLastUpdatedDate") => set(&mut self.last_updated, v)?,
                (["ABR"], "replaced") => set(&mut self.replaced, v)?,
                (["ABR", "ABN"], "status") => set(&mut self.abn_status, v)?,
                (["ABR", "ABN"], "ABNStatusFromDate") => set(&mut self.abn_status_since, v)?,
                (["ABR", "LegalEntity", "IndividualName"], "type") => {
                    set(&mut self.individual_name_type, v)?
                }
                (["ABR", "MainEntity", "NonIndividualName"], "type") => {
                    set(&mut self.non_individual_name_type, v)?
                }
                (["ABR", "OtherEntity", "NonIndividualName"], "type") => {
                    self.other_name_types.push(v)
                }
                (["ABR", "GST"], "status") => set(&mut self.gst_status, v)?,
                (["ABR", "GST"], "GSTStatusFromDate") => set(&mut self.gst_status_since, v)?,
                (["ABR", "ASICNumber"], "ASICNumberType") => set(&mut self.asic_number_type, v)?,
                (["ABR", "DGR"], "DGRStatusFromDate") => self.dgr_dates.push(v),
                (["ABR", "DGR"], "status") if v == "ACT" => (),
                (["ABR", "DGR", "NonIndividualName"], "type") => {
                    ensure!(v == "DGR", "dgr name with unexpected type: {v}");
                }
                x => eprintln!("unhandled attr: {x:?}: {v}"),
            }
        }

        Ok(())
    }
}

fn set<T>(o: &mut Option<T>, x: T) -> Result<()> {
    if o.is_some() {
        bail!("already set")
    }

    *o = Some(x);

    Ok(())
}

fn yn(x: &str) -> Option<bool> {
    match x {
        "Y" => Some(true),
        "N" => Some(false),
        _ => None,
    }
}
