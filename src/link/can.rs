// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{
        CanBitTiming, CanCtrlMode, CanCtrlModeFlags, InfoCan, InfoData,
        InfoKind,
    },
    LinkMessageBuilder,
};

/// Represent CAN (Controller Area Network) interface.
///
/// Example code on creating a CAN interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkCan};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkCan::new("can0")
///                 .bitrate(500000)
///                 .build()
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkCan> for more detail.
#[derive(Default, Debug)]
pub struct LinkCan;

impl LinkCan {
    /// Equal to `LinkMessageBuilder::<LinkCan>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkCan>::new(name)
    }
}

impl LinkMessageBuilder<LinkCan> {
    /// Create [LinkMessageBuilder] for CAN interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkCan>::new_with_info_kind(InfoKind::Can)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoCan) -> Self {
        let mut ret = self;
        if let InfoData::Can(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Can(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// Set bit timing parameters.
    ///
    /// This is equivalent to setting bitrate, sample_point, tq, prop_seg,
    /// phase_seg1, phase_seg2, sjw, brp.
    pub fn bit_timing(self, timing: CanBitTiming) -> Self {
        self.append_info_data(InfoCan::BitTiming(timing))
    }

    /// Set the CAN bitrate (a shorthand for setting just the bitrate in
    /// [bit_timing]).
    ///
    /// This is equivalent to `ip link add ... type can bitrate BITRATE`.
    pub fn bitrate(self, bitrate: u32) -> Self {
        self.append_info_data(InfoCan::BitTiming(CanBitTiming::new(bitrate)))
    }

    /// Set the CAN FD data phase bit timing.
    pub fn data_bit_timing(self, timing: CanBitTiming) -> Self {
        self.append_info_data(InfoCan::DataBitTiming(timing))
    }

    /// Set CAN controller mode.
    ///
    /// `mask` specifies which flags should be changed, `flags` specifies the
    /// new values of those flags.
    pub fn ctrl_mode(
        self,
        mask: CanCtrlModeFlags,
        flags: CanCtrlModeFlags,
    ) -> Self {
        self.append_info_data(InfoCan::CtrlMode(CanCtrlMode { mask, flags }))
    }

    /// Set extended CAN controller mode.
    pub fn ctrl_mode_ext(
        self,
        mask: CanCtrlModeFlags,
        flags: CanCtrlModeFlags,
    ) -> Self {
        self.append_info_data(InfoCan::CtrlModeExt(CanCtrlMode { mask, flags }))
    }

    /// Set restart ms.
    pub fn restart_ms(self, restart_ms: u32) -> Self {
        self.append_info_data(InfoCan::RestartMs(restart_ms))
    }

    /// Set bus termination value (in Ohms).
    pub fn termination(self, termination: u16) -> Self {
        self.append_info_data(InfoCan::Termination(termination))
    }

    /// Set CAN FD Transmitter Delay Compensation parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn tdc(
        self,
        tdcv_min: u32,
        tdcv_max: u32,
        tdcv: u32,
        tdco_min: u32,
        tdco_max: u32,
        tdco: u32,
        tdcf: u32,
    ) -> Self {
        use crate::packet_route::link::CanTdc;
        self.append_info_data(InfoCan::Tdc(CanTdc {
            tdcv_min,
            tdcv_max,
            tdcv,
            tdco_min,
            tdco_max,
            tdco,
            tdcf,
        }))
    }
}
