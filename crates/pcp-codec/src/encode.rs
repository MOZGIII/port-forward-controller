mod needs_options;
mod needs_r;
mod needs_request_opcode;
mod needs_response_opcode;

pub struct State<Packet, Step> {
    step: Step,
    packet: Packet,
}

pub mod steps {
    #[derive(Debug)]
    pub struct NeedsR;

    #[derive(Debug)]
    pub struct NeedsRequestOpcode;

    #[derive(Debug)]
    pub struct NeedsResponseOpcode;

    #[derive(Debug)]
    pub struct NeedsOptions<const NEXT_OPTION_OFFSET: usize>;
}

impl<Packet, Step> core::fmt::Debug for State<Packet, Step>
where
    Step: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("State")
            .field("step", &self.step)
            .field("packet", &"...")
            .finish()
    }
}
