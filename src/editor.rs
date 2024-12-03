use atomic_float::AtomicF32;
use nih_plug::params::EnumParam;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::borrow::{Borrow, BorrowMut};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::{OutputMode, SOCParams};

const ACTIVE_BGC: Color = Color::rgb(8u8, 255u8, 16u8);
const INACTIVE_BGC: Color = Color::rgb(64u8, 64u8, 64u8);

#[derive(Lens)]
struct Data {
    params: Arc<SOCParams>,
}

impl Model for Data {
    fn event(&mut self, ex: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| {
            match app_event {
                AppEvent::RBevent(outmode) => Data::params.map(|p| {
                    // let om = p.output_mode. ();
                    // *om = outmode;
                }),
            };
        })
    }
}

enum AppEvent {
    RBevent(crate::OutputMode),
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 150))
}

pub(crate) fn create(
    params: Arc<SOCParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load css");
        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Stereo Output Controller")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(26.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |ex| ex.emit(AppEvent::RBevent(OutputMode::Left)),
                    |cx| Label::new(cx, "L"),
                )
                .class("radio-button");
                VStack::new(cx, |cx| {
                    Button::new(cx, |_| {}, |cx| Label::new(cx, "")).class("radio-button");
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::LeftLeft)),
                        |cx| Label::new(cx, "LL"),
                    )
                    .class("radio-button");
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::Crossfeed)),
                        |cx| Label::new(cx, "CF"),
                    )
                    .class("radio-button");
                })
                .child_space(Stretch(1.))
                .child_top(Stretch(1.))
                .child_bottom(Stretch(1.))
                .child_left(Stretch(1.))
                .child_right(Stretch(1.));
                VStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::LeftRightSum)),
                        |cx| Label::new(cx, "L+R"),
                    )
                    .class("radio-button");
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::LeftRight)),
                        |cx| Label::new(cx, "LR"),
                    )
                    .class("radio-button");
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::LeftRightDiff)),
                        |cx| Label::new(cx, "L-R"),
                    )
                    .class("radio-button");
                })
                .child_space(Stretch(1.))
                .child_top(Stretch(1.))
                .child_bottom(Stretch(1.))
                .child_left(Stretch(1.))
                .child_right(Stretch(1.));
                VStack::new(cx, |cx| {
                    Button::new(cx, |_| {}, |cx| Label::new(cx, "")).class("radio-button");
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::RightRight)),
                        |cx| Label::new(cx, "RR"),
                    )
                    .class("radio-button");
                    Button::new(
                        cx,
                        |ex| ex.emit(AppEvent::RBevent(OutputMode::Balance)),
                        |cx| Label::new(cx, "Bal"),
                    )
                    .class("radio-button");
                })
                .child_space(Stretch(1.))
                .child_top(Stretch(1.))
                .child_bottom(Stretch(1.))
                .child_left(Stretch(1.))
                .child_right(Stretch(1.));
                Button::new(
                    cx,
                    |ex| ex.emit(AppEvent::RBevent(OutputMode::Right)),
                    |cx| Label::new(cx, "R"),
                )
                .class("radio-button");
            })
            .child_space(Stretch(1.0));
        });

        ResizeHandle::new(cx);
    })
}

// impl Data {
//     fn button_color(&self, mode: OutputMode) -> Color {
//         if mode == Data::params.map(|p| p.output_mode.value()) {
//             ACTIVE_BGC
//         } else {
//             INACTIVE_BGC
//         }
//     }
// }
