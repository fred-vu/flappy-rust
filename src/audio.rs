use macroquad::audio::{
    load_sound_from_bytes, play_sound, set_sound_volume, stop_sound, PlaySoundParams, Sound,
};
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44_100;
const BGM_VOLUME: f32 = 0.20;
const BGM_REPEATS: usize = 4;

pub struct AudioBank {
    flap: Option<Sound>,
    score: Option<Sound>,
    game_over: Option<Sound>,
    bgm: Option<Sound>,
}

impl AudioBank {
    pub async fn load() -> Self {
        let (flap, flap_failed) = load_sound_bytes(make_flap_bytes()).await;
        let (score, score_failed) = load_sound_bytes(make_score_bytes()).await;
        let (game_over, game_over_failed) = load_sound_bytes(make_game_over_bytes()).await;
        let (bgm, bgm_failed) = load_sound_bytes(make_bgm_bytes()).await;
        let sfx_failed = flap_failed || score_failed || game_over_failed;
        if sfx_failed {
            eprintln!("warn: sound effects unavailable, continuing without them");
        }
        if bgm_failed {
            eprintln!("warn: background music unavailable, continuing without it");
        }

        let bank = Self {
            flap,
            score,
            game_over,
            bgm,
        };

        bank.play_bgm_loop();
        bank
    }

    pub fn play_flap(&self) {
        self.play(&self.flap, 0.28);
    }

    pub fn play_score(&self) {
        self.play(&self.score, 0.32);
    }

    pub fn play_game_over(&self) {
        self.play(&self.game_over, 0.38);
    }

    pub fn set_muted(&self, muted: bool) {
        if muted {
            if let Some(sound) = &self.flap {
                stop_sound(sound);
            }
            if let Some(sound) = &self.score {
                stop_sound(sound);
            }
            if let Some(sound) = &self.game_over {
                stop_sound(sound);
            }
        }
        if let Some(bgm) = &self.bgm {
            let volume = if muted { 0.0 } else { BGM_VOLUME };
            set_sound_volume(bgm, volume);
        }
    }

    fn play(&self, sound: &Option<Sound>, volume: f32) {
        if let Some(sound) = sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume,
                },
            );
        }
    }

    fn play_bgm_loop(&self) {
        if let Some(bgm) = &self.bgm {
            // Stop first to avoid stacking loops when toggling mute/unmute.
            stop_sound(bgm);
            play_sound(
                bgm,
                PlaySoundParams {
                    looped: true,
                    volume: BGM_VOLUME,
                },
            );
        }
    }
}

async fn load_sound_bytes(bytes: Vec<u8>) -> (Option<Sound>, bool) {
    match load_sound_from_bytes(&bytes).await {
        Ok(sound) => (Some(sound), false),
        Err(_) => (None, true),
    }
}

fn make_flap_bytes() -> Vec<u8> {
    let samples = tone(860.0, 0.08, 0.42);
    build_wav(&samples, SAMPLE_RATE)
}

fn make_score_bytes() -> Vec<u8> {
    let mut samples = tone(1180.0, 0.06, 0.4);
    append_silence(&mut samples, 0.01);
    samples.extend(tone(1520.0, 0.08, 0.38));
    build_wav(&samples, SAMPLE_RATE)
}

fn make_game_over_bytes() -> Vec<u8> {
    let mut samples = tone(230.0, 0.12, 0.45);
    append_silence(&mut samples, 0.02);
    samples.extend(tone(150.0, 0.14, 0.45));
    build_wav(&samples, SAMPLE_RATE)
}

fn make_bgm_bytes() -> Vec<u8> {
    // Semitone offsets from A4 for a cheerful chiptune-style melody.
    const BAR: &[(i32, f32)] = &[
        (3, 0.18),    // C5
        (7, 0.18),    // E5
        (10, 0.18),   // G5
        (14, 0.18),   // B5
        (12, 0.20),   // A5
        (10, 0.18),   // G5
        (7, 0.18),    // E5
        (3, 0.22),    // C5
        (5, 0.18),    // D5
        (8, 0.18),    // F5
        (12, 0.18),   // A5
        (8, 0.18),    // F5
        (10, 0.20),   // G5
        (7, 0.20),    // E5
        (3, 0.18),    // C5
        (-999, 0.12), // rest
    ];

    let mut samples = Vec::new();
    for _ in 0..BGM_REPEATS {
        for &(semi, duration) in BAR {
            if semi == -999 {
                append_silence(&mut samples, duration);
                continue;
            }
            samples.extend(chiptune_note(semi, duration, 0.23));
        }
    }

    build_wav(&samples, SAMPLE_RATE)
}

fn tone(frequency: f32, duration: f32, volume: f32) -> Vec<i16> {
    let sample_count = (duration * SAMPLE_RATE as f32).ceil() as usize;
    let attack_samples = ((0.008 * SAMPLE_RATE as f32).ceil() as usize).max(1);
    let release_samples = ((0.02 * SAMPLE_RATE as f32).ceil() as usize).max(1);
    let mut samples = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut env = 1.0;

        if i < attack_samples {
            env = i as f32 / attack_samples as f32;
        } else if i > sample_count.saturating_sub(release_samples) {
            let remaining = sample_count.saturating_sub(i);
            env = remaining as f32 / release_samples as f32;
        }

        let amp = (2.0 * PI * frequency * t).sin() * volume * env;
        let scaled = (amp * i16::MAX as f32) as i16;
        samples.push(scaled);
    }

    samples
}

fn chiptune_note(semitone_from_a4: i32, duration: f32, volume: f32) -> Vec<i16> {
    let frequency = 440.0 * 2f32.powf(semitone_from_a4 as f32 / 12.0);
    let sample_count = (duration * SAMPLE_RATE as f32).ceil() as usize;
    let attack_samples = ((0.004 * SAMPLE_RATE as f32).ceil() as usize).max(1);
    let release_samples = ((0.03 * SAMPLE_RATE as f32).ceil() as usize).max(1);
    let mut samples = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let t = i as f32 / SAMPLE_RATE as f32;
        let lead = square_wave(frequency, t, 0.125) * 0.62;
        let harmony = square_wave(frequency * 0.5, t, 0.5) * 0.30;
        let accent = square_wave(frequency * 2.0, t, 0.25) * 0.08;

        let mut env = 1.0;
        if i < attack_samples {
            env = i as f32 / attack_samples as f32;
        } else if i > sample_count.saturating_sub(release_samples) {
            let remain = sample_count.saturating_sub(i);
            env = remain as f32 / release_samples as f32;
        }

        let amp = (lead + harmony + accent) * volume * env;
        let sample = (amp.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        samples.push(sample);
    }

    samples
}

fn square_wave(freq: f32, t: f32, duty: f32) -> f32 {
    let phase = (t * freq).fract();
    if phase < duty { 1.0 } else { -1.0 }
}

fn append_silence(samples: &mut Vec<i16>, duration: f32) {
    let silence_samples = (duration * SAMPLE_RATE as f32).ceil() as usize;
    samples.extend(std::iter::repeat(0).take(silence_samples));
}

fn build_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut bytes = Vec::with_capacity(44 + data_len as usize);

    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");

    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());

    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());

    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }

    bytes
}
