extern crate atomic_float;
extern crate nih_plug;
extern crate nih_plug_vizia;

use atomic_float::AtomicF32;
use nih_plug::{
    prelude::*,
    wrapper::vst3::vst3_sys::vst::{Sample32, Sample64},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;
mod process;

pub struct SOC {
    params: Arc<SOCParams>,
}

#[derive(Params)]
struct SOCParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "MonoToggle"]
    pub monotoggle: BoolParam,
    #[id = "MonoMode"]
    pub monomode: EnumParam<MonoMode>
}

#[derive(Enum, Debug, PartialEq)]
pub enum MonoMode {
    #[id="L"]
    Left,
    #[id="LL"]
    LeftLeft,
    #[id="L+R"]
    LeftRightSum,
    #[id="L-R"]
    LeftRightDiff,
    #[id="RR"]
    RightRight,
    #[id="R"]
    Right
}

impl Default for SOC {
    fn default() -> Self {
        Self {
            params: Arc::new(SOCParams::default()),
        }
    }
}

impl Default for SOCParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            monotoggle: BoolParam::new("Mono", false),
            monomode: EnumParam::new("MonoMode", MonoMode::LeftRightSum)
        }
    }
}

impl Plugin for SOC {
    const NAME: &'static str = "Stereo Output Controller v.0";
    const VENDOR: &'static str = "Wirebender Audio";
    const URL: &'static str = "www.collardmusic.com";
    const EMAIL: &'static str = "collardmusic@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        match (
            self.params.monotoggle.value(),
            self.params.monomode.value(),
        ) {
            (false, _) => (),
            (true, MonoMode::Left) => process::left_only(buffer),
            (true, MonoMode::LeftLeft) => process::left_left(buffer),
            (true, MonoMode::LeftRightSum) => process::sum_mono(buffer),
            (true, MonoMode::LeftRightDiff) => process::diff_mono(buffer),
            (true, MonoMode::Right) => process::right_only(buffer),
            (true, MonoMode::RightRight) => process::right_right(buffer),
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for SOC {
    const CLAP_ID: &'static str =
        "wirebender audio stereo output controller v.0 discrete symbol continuous syntax";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Stereo to mono, headphone crossfeed");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for SOC {
    const VST3_CLASS_ID: [u8; 16] = *b"StereoOutput_Ctl";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(SOC);
nih_export_vst3!(SOC);
