use std::{error::Error, fmt, str::FromStr};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ABNRecord {
    pub abn: String,
    pub status: Status,
    pub status_since: NaiveDate,
    pub last_updated: NaiveDate,

    pub entity_name: EntityName,
    pub entity_type: EntityType,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub business_names: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trade_names: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asic_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_status_since: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Active,
    Cancelled,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EntityName {
    Individual {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        given: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        given_2: Option<String>,
        family: String,
    },
    NonIndividual {
        name: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EntityType {
    IND, // Individual/Sole Trader
    PRV, // Australian Private Company
    FPT, // Family Partnership
    SMF, // ATO Regulated Self-Managed Superannuation Fund
    DIT, // Discretionary Investment Trust
    DTT, // Discretionary Trading Trust
    PTR, // Other Partnership
    FUT, // Fixed Unit Trust
    OIE, // Other Incorporated Entity
    TRT, // Other trust
    STR, // Strata-title
    UIE, // Other Unincorporated Entity
    DST, // Discretionary Services Management Trust
    PUB, // Australian Public Company
    DES, // Deceased Estate
    FXT, // Fixed Trust
    HYT, // Hybrid Trust
    SGE, // State Government Entity
    LPT, // Limited Partnership
    PQT, // Unlisted Public Unit Trust
    SAF, // Small APRA Fund
    CUT, // Corporate Unit Trust
    COP, // Co-operative
    NPF, // APRA Regulated Non-Public Offer Fund
    PTT, // Public Trading trust
    CMT, // Cash Management Trust
    NRF, // Non-Regulated Superannuation Fund
    LGE, // Local Government Entity
    CGE, // Commonwealth Government Entity
    PUT, // Listed Public Unit Trust
    SGA, // State Government Statutory Authority
    POF, // APRA Regulated Public Offer Fund
    TGE, // Territory Government Entity
    PST, // Pooled Superannuation Trust
    SCO, // State Government Other Incorporated Entity
    SCN, // State Government Other Unincorporated Entity
    SSS, // State Government Non-Regulated Super Fund
    CGA, // Commonwealth Government Statutory Authority
    ADF, // Approved Deposit Fund
    CSS, // Commonwealth Government Non-Regulated Super Fund
    LGA, // Local Government Statutory Authority
    STU, // State Government Fixed Unit Trust
    SCR, // State Government Private Company
    TGA, // Territory Government Statutory Authority
    LSS, // Local Government Non-Regulated Super Fund
    CCN, // Commonwealth Government Other Unincorporated Entity
    STI, // State Government Discretionary Investment Trust
    SUP, // Super Fund
    PDF, // Pooled Development Fund
    LCN, // Local Government Other Unincorporated Entity
    SCB, // State Government Public Company
    LCR, // Local Government Private Company
    SGP, // State Government Partnership
    TTF, // Territory Government Fixed Trust
    SGC, // State Government Company
    CCO, // Commonwealth Government Other Incorporated Entity
    LGC, // Local Government Company
    CCR, // Commonwealth Government Private Company
    CCB, // Commonwealth Government Public Company
    TSS, // Territory Government Non-Regulated Super Fund
    TCO, // Territory Government Other Incorporated Entity
    STF, // State Government Fixed Trust
    LCO, // Local Government Other Incorporated Entity
    TTI, // Territory Government Discretionary Investment Trust
    SSP, // State Government APRA Regulated Public Sector Scheme
    SGT, // State Government Trust
    SCC, // State Government Co-operative
    LTI, // Local Government Discretionary Investment Trust
    LSP, // Local Government APRA Regulated Public Sector Scheme
    CTI, // Commonwealth Government Discretionary Investment Trust
    CSF, // Corporate Collective Investment Vehicle (CCIV) Sub-Fund
    CSA, // Commonwealth Government APRA Regulated Public Sector Fund
    CGP, // Commonwealth Government Partnership
    TTU, // Territory Government Fixed Unit Trust
    TCN, // Territory Government Other Unincorporated Entity
    STD, // State Government Discretionary Services Management Trust
    LTT, // Local Government Discretionary Trading Trust
    LGP, // Local Government Partnership
    LCS, // Local Government Strata Title
    FHS, // First Home Saver Accounts Trust
    CTQ, // Commonwealth Government Unlisted Public Unit Trust
    CTF, // Commonwealth Government Fixed Trust
    CTD, // Commonwealth Government Discretionary Services Management Trust
    CSP, // Commonwealth Government APRA Regulated Public Sector Scheme
    CGC, // Commonwealth Government Company
}

#[derive(Debug)]
pub struct EntityTypeParseError;

impl fmt::Display for EntityTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown entity type")
    }
}

impl Error for EntityTypeParseError {}

impl FromStr for EntityType {
    type Err = EntityTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IND" => Ok(Self::IND),
            "PRV" => Ok(Self::PRV),
            "FPT" => Ok(Self::FPT),
            "SMF" => Ok(Self::SMF),
            "DIT" => Ok(Self::DIT),
            "DTT" => Ok(Self::DTT),
            "PTR" => Ok(Self::PTR),
            "FUT" => Ok(Self::FUT),
            "OIE" => Ok(Self::OIE),
            "TRT" => Ok(Self::TRT),
            "STR" => Ok(Self::STR),
            "UIE" => Ok(Self::UIE),
            "DST" => Ok(Self::DST),
            "PUB" => Ok(Self::PUB),
            "DES" => Ok(Self::DES),
            "FXT" => Ok(Self::FXT),
            "HYT" => Ok(Self::HYT),
            "SGE" => Ok(Self::SGE),
            "LPT" => Ok(Self::LPT),
            "PQT" => Ok(Self::PQT),
            "SAF" => Ok(Self::SAF),
            "CUT" => Ok(Self::CUT),
            "COP" => Ok(Self::COP),
            "NPF" => Ok(Self::NPF),
            "PTT" => Ok(Self::PTT),
            "CMT" => Ok(Self::CMT),
            "NRF" => Ok(Self::NRF),
            "LGE" => Ok(Self::LGE),
            "CGE" => Ok(Self::CGE),
            "PUT" => Ok(Self::PUT),
            "SGA" => Ok(Self::SGA),
            "POF" => Ok(Self::POF),
            "TGE" => Ok(Self::TGE),
            "PST" => Ok(Self::PST),
            "SCO" => Ok(Self::SCO),
            "SCN" => Ok(Self::SCN),
            "SSS" => Ok(Self::SSS),
            "CGA" => Ok(Self::CGA),
            "ADF" => Ok(Self::ADF),
            "CSS" => Ok(Self::CSS),
            "LGA" => Ok(Self::LGA),
            "STU" => Ok(Self::STU),
            "SCR" => Ok(Self::SCR),
            "TGA" => Ok(Self::TGA),
            "LSS" => Ok(Self::LSS),
            "CCN" => Ok(Self::CCN),
            "STI" => Ok(Self::STI),
            "SUP" => Ok(Self::SUP),
            "PDF" => Ok(Self::PDF),
            "LCN" => Ok(Self::LCN),
            "SCB" => Ok(Self::SCB),
            "LCR" => Ok(Self::LCR),
            "SGP" => Ok(Self::SGP),
            "TTF" => Ok(Self::TTF),
            "SGC" => Ok(Self::SGC),
            "CCO" => Ok(Self::CCO),
            "LGC" => Ok(Self::LGC),
            "CCR" => Ok(Self::CCR),
            "CCB" => Ok(Self::CCB),
            "TSS" => Ok(Self::TSS),
            "TCO" => Ok(Self::TCO),
            "STF" => Ok(Self::STF),
            "LCO" => Ok(Self::LCO),
            "TTI" => Ok(Self::TTI),
            "SSP" => Ok(Self::SSP),
            "SGT" => Ok(Self::SGT),
            "SCC" => Ok(Self::SCC),
            "LTI" => Ok(Self::LTI),
            "LSP" => Ok(Self::LSP),
            "CTI" => Ok(Self::CTI),
            "CSF" => Ok(Self::CSF),
            "CSA" => Ok(Self::CSA),
            "CGP" => Ok(Self::CGP),
            "TTU" => Ok(Self::TTU),
            "TCN" => Ok(Self::TCN),
            "STD" => Ok(Self::STD),
            "LTT" => Ok(Self::LTT),
            "LGP" => Ok(Self::LGP),
            "LCS" => Ok(Self::LCS),
            "FHS" => Ok(Self::FHS),
            "CTQ" => Ok(Self::CTQ),
            "CTF" => Ok(Self::CTF),
            "CTD" => Ok(Self::CTD),
            "CSP" => Ok(Self::CSP),
            "CGC" => Ok(Self::CGC),
            _ => Err(EntityTypeParseError),
        }
    }
}
