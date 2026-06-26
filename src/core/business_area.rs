//! ISO 20022 business areas (the 4-letter prefix of every message identifier).
//!
//! Mirrors prowide's `MxBusinessProcess`. The set is the full ISO 20022
//! catalogue, not just the families currently generated in this crate.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A business area / business process, e.g. `pacs` ("Payments Clearing and
/// Settlement"). This is the first component of an [`crate::MxId`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum BusinessArea {
    acmt,
    admi,
    auth,
    caaa,
    caad,
    caam,
    cafc,
    cafm,
    cafr,
    cain,
    camt,
    canm,
    casp,
    casr,
    catm,
    catp,
    cbrf,
    colr,
    fxtr,
    head,
    pacs,
    pain,
    reda,
    remt,
    secl,
    seev,
    semt,
    sese,
    seti,
    setr,
    supl,
    trck,
    trea,
    tsin,
    tsmt,
    tsrv,
    xsys,
}

impl BusinessArea {
    /// Every business area, in catalogue order.
    pub const ALL: [BusinessArea; 37] = {
        use BusinessArea::*;
        [
            acmt, admi, auth, caaa, caad, caam, cafc, cafm, cafr, cain, camt, canm, casp, casr,
            catm, catp, cbrf, colr, fxtr, head, pacs, pain, reda, remt, secl, seev, semt, sese,
            seti, setr, supl, trck, trea, tsin, tsmt, tsrv, xsys,
        ]
    };

    /// The 4-letter code, e.g. `"pacs"`.
    pub const fn code(self) -> &'static str {
        use BusinessArea::*;
        match self {
            acmt => "acmt",
            admi => "admi",
            auth => "auth",
            caaa => "caaa",
            caad => "caad",
            caam => "caam",
            cafc => "cafc",
            cafm => "cafm",
            cafr => "cafr",
            cain => "cain",
            camt => "camt",
            canm => "canm",
            casp => "casp",
            casr => "casr",
            catm => "catm",
            catp => "catp",
            cbrf => "cbrf",
            colr => "colr",
            fxtr => "fxtr",
            head => "head",
            pacs => "pacs",
            pain => "pain",
            reda => "reda",
            remt => "remt",
            secl => "secl",
            seev => "seev",
            semt => "semt",
            sese => "sese",
            seti => "seti",
            setr => "setr",
            supl => "supl",
            trck => "trck",
            trea => "trea",
            tsin => "tsin",
            tsmt => "tsmt",
            tsrv => "tsrv",
            xsys => "xsys",
        }
    }

    /// Human-readable name of the business area.
    pub const fn description(self) -> &'static str {
        use BusinessArea::*;
        match self {
            acmt => "Account Management",
            admi => "Administration",
            auth => "Authorities",
            caaa => "Acceptor to Acquirer Card Transactions",
            caad => "Card Administration",
            caam => "ATM Management",
            cafc => "Fee Collection",
            cafm => "File Management",
            cafr => "Fraud Reporting and Disposition",
            cain => "Acquirer to Issuer Card Transactions",
            camt => "Cash Management",
            canm => "Network Management",
            casp => "Sale to POI Card Transactions",
            casr => "Settlement Reporting",
            catm => "Terminal Management",
            catp => "ATM Card Transactions",
            cbrf => "Clearing Operations and Reporting",
            colr => "Collateral Management",
            fxtr => "Foreign Exchange Trade",
            head => "Business Application Header",
            pacs => "Payments Clearing and Settlement",
            pain => "Payments Initiation",
            reda => "Reference Data",
            remt => "Payments Remittance Advice",
            secl => "Securities Clearing",
            seev => "Securities Events",
            semt => "Securities Management",
            sese => "Securities Settlement",
            seti => "Securities Trade Initiation",
            setr => "Securities Trade",
            supl => "Supplementary Data",
            trck => "Payments Tracker",
            trea => "Treasury",
            tsin => "Trade Services Initiation",
            tsmt => "Trade Services Management",
            tsrv => "Trade Services",
            xsys => "System Message",
        }
    }

    /// Parse a 4-letter business-area code, e.g. `"pacs"`.
    pub fn from_code(code: &str) -> Option<BusinessArea> {
        BusinessArea::ALL.into_iter().find(|a| a.code() == code)
    }
}

impl core::fmt::Display for BusinessArea {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.code())
    }
}

impl core::str::FromStr for BusinessArea {
    type Err = crate::core::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BusinessArea::from_code(s).ok_or_else(|| crate::core::Error::UnknownBusinessArea(s.to_string()))
    }
}
