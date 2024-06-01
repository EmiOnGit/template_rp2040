use rp_pico::pac::SYST;

pub struct Delay {
    syst: SYST,
    /// `ahb_frequency` is a frequency of the AHB bus in Hz.
    frequency: u32,
}
impl Delay {
    pub fn new(mut syst: SYST, frequency: u32) -> Delay {
        syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        Delay { syst, frequency }
    }
    /// Delay using the Cortex-M systick for a certain duration, in ms.
    #[inline]
    pub fn delay_ms(&mut self, mut ms: u32) {
        // 4294967 is the highest u32 value which you can multiply by 1000 without overflow
        while ms > 4294967 {
            self.delay_us(4294967000u32);
            ms -= 4294967;
        }
        self.delay_us(ms * 1_000);
    }
    /// Delay using the Cortex-M systick for a certain duration, in µs.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn delay_us(&mut self, us: u32) {
        let ticks = (u64::from(us)) * (u64::from(self.frequency)) / 1_000_000;

        let full_cycles = ticks >> 24;
        if full_cycles > 0 {
            self.syst.set_reload(0xffffff);
            self.syst.clear_current();
            self.syst.enable_counter();

            for _ in 0..full_cycles {
                while !self.syst.has_wrapped() {}
            }
        }

        let ticks = (ticks & 0xffffff) as u32;
        if ticks > 1 {
            self.syst.set_reload(ticks - 1);
            self.syst.clear_current();
            self.syst.enable_counter();

            while !self.syst.has_wrapped() {}
        }

        self.syst.disable_counter();
    }
}
// pub fn sleep()
