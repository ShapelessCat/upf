use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "UPF")]
pub struct UpfData {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "PP_HEADER")]
    pub header: PpHeader,
    #[serde(rename = "PP_MESH")]
    pub mesh: PpMesh,
    #[serde(rename = "PP_LOCAL")]
    pub local: PpLocal,
    #[serde(rename = "PP_NONLOCAL")]
    pub nonlocal: PpNonlocalStub,
    #[serde(rename = "PP_RHOATOM")]
    pub rhoatom: PpRhoAtom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpHeader {
    #[serde(rename = "@generated")]
    pub generated: String,
    #[serde(rename = "@author")]
    pub author: String,
    #[serde(rename = "@date")]
    pub date: String,
    #[serde(rename = "@comment")]
    pub comment: String,
    #[serde(rename = "@element")]
    pub element: String,
    #[serde(rename = "@pseudo_type")]
    pub pseudo_type: String,
    #[serde(rename = "@relativistic")]
    pub relativistic: String,
    #[serde(rename = "@is_ultrasoft")]
    pub is_ultrasoft: String,
    #[serde(rename = "@is_paw")]
    pub is_paw: String,
    #[serde(rename = "@is_coulomb")]
    pub is_coulomb: String,
    #[serde(rename = "@has_so")]
    pub has_so: String,
    #[serde(rename = "@has_wfc")]
    pub has_wfc: String,
    #[serde(rename = "@has_gipaw")]
    pub has_gipaw: String,
    #[serde(rename = "@core_correction")]
    pub core_correction: String,
    #[serde(rename = "@z_valence")]
    pub z_valence: f64,
    #[serde(rename = "@total_psenergy")]
    pub total_psenergy: f64,
    #[serde(rename = "@wfc_cutoff")]
    pub wfc_cutoff: f64,
    #[serde(rename = "@rho_cutoff")]
    pub rho_cutoff: f64,
    #[serde(rename = "@l_max")]
    pub l_max: usize,
    #[serde(rename = "@l_max_rho")]
    pub l_max_rho: usize,
    #[serde(rename = "@l_local")]
    pub l_local: isize,
    #[serde(rename = "@mesh_size")]
    pub mesh_size: usize,
    #[serde(rename = "@number_of_wfc")]
    pub number_of_wfc: usize,
    #[serde(rename = "@number_of_proj")]
    pub number_of_proj: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpMesh {
    #[serde(rename = "@dx")]
    pub dx: f64,
    #[serde(rename = "@mesh")]
    pub mesh: usize,
    #[serde(rename = "@xmin")]
    pub xmin: f64,
    #[serde(rename = "@rmax")]
    pub rmax: f64,
    #[serde(rename = "@zmesh")]
    pub zmesh: f64,
    #[serde(rename = "PP_R")]
    pub r: PpR,
    #[serde(rename = "PP_RAB")]
    pub rab: PpRab,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumericArray {
    #[serde(rename = "$text")]
    pub values: Vec<f64>,
}

pub type PpR = NumericArray;
pub type PpRab = NumericArray;
pub type PpLocal = NumericArray;
pub type PpRhoAtom = NumericArray;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PpNonlocalStub {}
