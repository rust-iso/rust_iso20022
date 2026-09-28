macro_rules! assert_type {
    ($ty:path) => {
        let _ = core::any::type_name::<$ty>();
    };
}

fn main() {
    #[cfg(feature = "model-acmt")]
    assert_type!(rust_iso20022::generated::acmt::acmt_001_001_08::Document);
    #[cfg(feature = "model-admi")]
    assert_type!(rust_iso20022::generated::admi::admi_002_001_01::Document);
    #[cfg(feature = "model-auth")]
    assert_type!(rust_iso20022::generated::auth::auth_001_001_01::Document);
    #[cfg(feature = "model-caaa")]
    assert_type!(rust_iso20022::generated::caaa::caaa_001_001_10::Document);
    #[cfg(feature = "model-caad")]
    assert_type!(rust_iso20022::generated::caad::caad_001_001_01::Document);
    #[cfg(feature = "model-caam")]
    assert_type!(rust_iso20022::generated::caam::caam_001_001_03::Document);
    #[cfg(feature = "model-cafc")]
    assert_type!(rust_iso20022::generated::cafc::cafc_001_001_01::Document);
    #[cfg(feature = "model-cafm")]
    assert_type!(rust_iso20022::generated::cafm::cafm_001_001_01::Document);
    #[cfg(feature = "model-cafr")]
    assert_type!(rust_iso20022::generated::cafr::cafr_001_001_01::Document);
    #[cfg(feature = "model-cain")]
    assert_type!(rust_iso20022::generated::cain::cain_001_001_02::Document);
    #[cfg(feature = "model-camt")]
    assert_type!(rust_iso20022::generated::camt::camt_003_001_07::Document);
    #[cfg(feature = "model-canm")]
    assert_type!(rust_iso20022::generated::canm::canm_001_001_02::Document);
    #[cfg(feature = "model-casp")]
    assert_type!(rust_iso20022::generated::casp::casp_001_001_03::Document);
    #[cfg(feature = "model-casr")]
    assert_type!(rust_iso20022::generated::casr::casr_001_001_01::Document);
    #[cfg(feature = "model-catm")]
    assert_type!(rust_iso20022::generated::catm::catm_001_001_03::Document);
    #[cfg(feature = "model-catp")]
    assert_type!(rust_iso20022::generated::catp::catp_001_001_02::Document);
    #[cfg(feature = "model-colr")]
    assert_type!(rust_iso20022::generated::colr::colr_001_001_01::Document);
    #[cfg(feature = "model-fxtr")]
    assert_type!(rust_iso20022::generated::fxtr::fxtr_008_001_06::Document);
    #[cfg(feature = "model-head")]
    assert_type!(rust_iso20022::generated::head::head_001_001_02::BusinessApplicationHeaderV02);
    #[cfg(feature = "model-pacs")]
    assert_type!(rust_iso20022::generated::pacs::pacs_002_001_10::Document);
    #[cfg(feature = "model-pain")]
    assert_type!(rust_iso20022::generated::pain::pain_001_001_09::Document);
    #[cfg(feature = "model-reda")]
    assert_type!(rust_iso20022::generated::reda::reda_001_001_04::Document);
    #[cfg(feature = "model-remt")]
    assert_type!(rust_iso20022::generated::remt::remt_001_001_05::Document);
    #[cfg(feature = "model-secl")]
    assert_type!(rust_iso20022::generated::secl::secl_001_001_03::Document);
    #[cfg(feature = "model-seev")]
    assert_type!(rust_iso20022::generated::seev::seev_001_001_08::Document);
    #[cfg(feature = "model-semt")]
    assert_type!(rust_iso20022::generated::semt::semt_001_001_03::Document);
    #[cfg(feature = "model-sese")]
    assert_type!(rust_iso20022::generated::sese::sese_001_001_10::Document);
    #[cfg(feature = "model-setr")]
    assert_type!(rust_iso20022::generated::setr::setr_001_001_05::Document);
    #[cfg(feature = "model-trck")]
    assert_type!(rust_iso20022::generated::trck::trck_001_001_05::Document);
    #[cfg(feature = "model-tsin")]
    assert_type!(rust_iso20022::generated::tsin::tsin_001_001_01::Document);
    #[cfg(feature = "model-tsmt")]
    assert_type!(rust_iso20022::generated::tsmt::tsmt_001_001_03::Document);
    #[cfg(feature = "model-tsrv")]
    assert_type!(rust_iso20022::generated::tsrv::tsrv_001_001_01::Document);
}
