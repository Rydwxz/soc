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
        chan_samps.get_mut(1).expect("Incorrect buffer access") = 0.0;
    }
}

pub fn left_left(buf: &mut Buffer) {

}

pub fn right_only(buf: &mut Buffer) {

}

pub fn right_right(buf: &mut Buffer) {

}
