use core::f32;

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

pub fn phono_mtx(buf: &mut Buffer, cf_l: f32, cf_d: f32) {

}

pub fn balance(buf: &mut Buffer, bal: f32) {
    let mut idx = 0;
    if bal = 0 {return;}
    else if bal < 0 {
        bal = bal * -1;
        idx = 1;
    }
    for mut chan_samps in buf.iter_samples() {
        scale(chan_samps.get_mut(idx).unwrap(), bal);
    }
}

fn scale(input: &f32, s: f32)  {
    //*input - *input * s
    *input * (1 - s)
}
