use std::{char, f32};

use nih_plug::prelude::*;

pub fn sum_mono(buf: &mut Buffer) {
    for mut channel_samples in buf.iter_samples() {
        let mut new_samp: f32 = 0.0;
        for samp in channel_samples.iter_mut() {
            new_samp += *samp;
        }
        new_samp = new_samp / 2.0;
        for samp in channel_samples {
            *samp = new_samp;
        }
    }
}

pub fn diff_mono(buf: &mut Buffer) {
    for mut chan_samps in buf.iter_samples() {
        let mut new_samp: f32 = 0.0;
        let mut chans = chan_samps.iter_mut();
        new_samp += *chans.next().unwrap();
        new_samp -= *chans.next().unwrap();
        for samp in chan_samps {
            *samp = new_samp;
        }
    }
}

pub fn left_only(buf: &mut Buffer) {
    for mut chan_samps in buf.iter_samples() {
        *chan_samps.get_mut(1).unwrap() = 0.0;
    }
}

pub fn left_left(buf: &mut Buffer) {
    for mut chan_samps in buf.iter_samples() {
        *chan_samps.get_mut(1).unwrap() = *chan_samps.get_mut(0).unwrap();
    }
}

pub fn right_only(buf: &mut Buffer) {
    for mut chan_samps in buf.iter_samples() {
        *chan_samps.get_mut(0).unwrap() = 0.0;
    }
}

pub fn right_right(buf: &mut Buffer) {
    for mut chan_samps in buf.iter_samples() {
        *chan_samps.get_mut(0).unwrap() = *chan_samps.get_mut(1).unwrap();
    }
}

// this function will mix small amounts of each audio channel into the other according to cf_l
// todo implement cf_delay
pub fn crossfeed(buf: &mut Buffer, cf_l: f32, cf_d: f32, del: DelayBuffer) {
    // let del = del.
    // let _ = buf.iter_samples().zip(del.iter_samples(cf_d))
    //     .map(|mut chans, delchan| {
    //         *chans.get_mut(0).unwrap() += *delchan.get_mut(1).unwrap() * cf_l;
    //         *chans.get_mut(1).unwrap() += *delchan.get_mut(0).unwrap() * cf_l;
    //     });
    // });
}

pub fn balance(buf: &mut Buffer, bal: f32) {
    let mut bal = bal;
    let mut idx = 0;
    if bal == 0.0 {
        return;
    } else if bal < 0.0 {
        bal = bal * -1.0;
        idx = 1;
    }
    let _ = buf.iter_samples().map(|mut chan_samps| {
        scale(chan_samps.get_mut(idx).unwrap(), bal);
    });
}

fn scale(inp: &mut f32, s: f32) {
    *inp = *inp * (1.0 - s)
}
