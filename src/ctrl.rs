use std::process;

use neli::{
    consts::{
        nl::{NlmF, NlmFFlags},
        socket::NlFamily,
    },
    genl::{Genlmsghdr, Nlattr},
    nl::{NlPayload, Nlmsghdr},
    socket::NlSocketHandle,
    types::{Buffer, GenlBuffer},
};

use crate::genl::{RzrstAttr, RzrstCommand};

/// State of the sidetone
#[derive(Debug, Clone, Copy)]
pub enum SidetoneState {
    On,
    Off,
}

/// Client for sending commands to the `rzrst` kernel module
pub struct NlClient {
    sock: NlSocketHandle,
    family_id: u16,
}

impl NlClient {
    pub const FAMILY_NAME: &'static str = "rzrst";

    pub fn new() -> color_eyre::Result<Self> {
        let mut sock = NlSocketHandle::connect(NlFamily::Generic, Some(0), &[])?;
        log::debug!("connected to generic netlink kernel socket");

        let family_id = sock.resolve_genl_family(Self::FAMILY_NAME)?;
        log::debug!("genl family id is {}", family_id);

        Ok(Self { sock, family_id })
    }

    pub fn send(&mut self, state: SidetoneState, vol: u8) -> color_eyre::Result<()> {
        if vol > 100 {
            log::error!("volume > 100 requested: {}", vol);
            panic!("volume can't be set higher than 100");
        }

        let state: u8 = state.into();

        // append attributes
        let mut attrs: GenlBuffer<RzrstAttr, Buffer> = GenlBuffer::new();
        attrs.push(Nlattr::new(false, false, RzrstAttr::State, state)?);

        attrs.push(Nlattr::new(false, false, RzrstAttr::Volume, vol)?);

        // specify generic netlink message header
        let gnmsghdr = Genlmsghdr::new(RzrstCommand::SetVolumeState, 1, attrs);

        // specify netlink message header
        let nlmsghdr = Nlmsghdr::new(
            None,
            self.family_id,
            NlmFFlags::new(&[NlmF::Request]),
            None,
            Some(process::id()),
            NlPayload::Payload(gnmsghdr),
        );

        self.sock.send(nlmsghdr)?;

        Ok(())
    }
}

impl From<bool> for SidetoneState {
    fn from(value: bool) -> Self {
        if value {
            SidetoneState::On
        } else {
            SidetoneState::Off
        }
    }
}

impl Into<u8> for SidetoneState {
    fn into(self) -> u8 {
        match self {
            SidetoneState::Off => 0x00,
            SidetoneState::On => 0x01,
        }
    }
}
