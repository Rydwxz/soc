extern crate atomic_float;
extern crate nih_plug;
extern crate nih_plug_vizia;

use atomic_float::AtomicF32;
use nih_plug::{
    context::process,
    prelude::*,
    wrapper::vst3::vst3_sys::vst::{Sample32, Sample64},
};
use nih_plug_vizia::{vizia::views::normalized_map::DecibelMap, ViziaState};
use std::sync::Arc;

mod buffer;
mod editor;
mod proc;

pub struct SOC {
    params: Arc<SOCParams>,
    cf: CrossFeed,
}

struct CrossFeed {
    level: f32,
    delay: f32,
    filter: usize,
    // del_buf: DelayBuffer,
}

#[derive(Params)]
struct SOCParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "MonoMode"]
    pub output_mode: EnumParam<OutputMode>,
    #[id = "CrossFeed Width"]
    pub cf_width: FloatParam,
    #[id = "CrossFeed Distance"]
    pub cf_distance: FloatParam,
    #[id = "Channel Balance"]
    pub balance: FloatParam,
}

#[derive(Enum, Debug, PartialEq)]
pub enum OutputMode {
    #[id = "LR"]
    LeftRight,
    #[id = "L"]
    Left,
    #[id = "LL"]
    LeftLeft,
    #[id = "L+R"]
    LeftRightSum,
    #[id = "L-R"]
    LeftRightDiff,
    #[id = "RR"]
    RightRight,
    #[id = "R"]
    Right,
    #[id = "CF"]
    Crossfeed,
    #[id = "BAL"]
    Balance,
}

impl Default for SOC {
    fn default() -> Self {
        Self {
            params: Arc::new(SOCParams::default()),
            cf: CrossFeed::default(),
            // del_buf: DelayBuffer::default(),
        }
    }
}

impl Default for CrossFeed {
    fn default() -> Self {
        Self {
            level: -12.0,
            delay: 6.0,
            filter: 0,
        }
    }
}

impl Default for SOCParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            output_mode: EnumParam::new("MonoMode", OutputMode::LeftRightSum),
            cf_width: FloatParam::new(
                "Crossfeed Width",
                1.0,
                FloatRange::Linear {
                    min: -0.25,
                    max: 0.25,
                },
            ), // todo find better defaults
            cf_distance: FloatParam::new(
                "Crossfeed Distance",
                1.0,
                FloatRange::Linear {
                    min: -0.25,
                    max: 0.25,
                },
            ), // for both of these
            balance: FloatParam::new(
                "Balance",
                0.0,
                FloatRange::Linear {
                    min: -0.25,
                    max: 0.25,
                },
            ),
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
        audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // todo init a buffer for cf_delay
        // self.del_buf.init();
        if audio_io_layout
            .main_input_channels
            .expect("no input channels")
            != NonZeroU32::new(2).unwrap()
        {
            false
        } else {
            true
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        use crate::proc::*;
        match self.params.output_mode.value() {
            OutputMode::LeftRight => (),
            OutputMode::Left => left_only(buffer),
            OutputMode::LeftLeft => left_left(buffer),
            OutputMode::LeftRightSum => sum_mono(buffer),
            OutputMode::LeftRightDiff => diff_mono(buffer),
            OutputMode::Right => right_only(buffer),
            OutputMode::RightRight => right_right(buffer),
            OutputMode::Crossfeed => crossfeed(
                buffer,
                self.cf.level,
                self.cf.delay,
                // self.del_buf,
            ),
            OutputMode::Balance => balance(buffer, self.params.balance.value()),
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
