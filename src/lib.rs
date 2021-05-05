#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub enum EffectSelector {
    Denoiser,
    Dereverb,
    DereverbDenoiser,
}

impl Into<NvAFX_EffectSelector> for EffectSelector {
    fn into(self) -> *const std::os::raw::c_char {
        let cstr = CString::new(match self {
            EffectSelector::Denoiser => {
                "denoiser"
            }
            EffectSelector::Dereverb => {
                "dereverb"
            }
            EffectSelector::DereverbDenoiser => {
                "dereverb_denoiser"
            }
        }).expect("Constant conversion failed.");

        unsafe { CStr::from_bytes_with_nul_unchecked(cstr.as_bytes_with_nul()).as_ptr() }
    }
}