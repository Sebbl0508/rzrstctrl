use neli::{
    consts::genl::{self, NlAttrType},
    neli_enum,
};

#[neli_enum(serialized_type = "u8")]
pub enum RzrstCommand {
    SetVolumeState = 0,
}

#[neli_enum(serialized_type = "u16")]
pub enum RzrstAttr {
    Unspec = 0,
    State = 1,
    Volume = 2,
}

impl genl::Cmd for RzrstCommand {}
impl NlAttrType for RzrstAttr {}
