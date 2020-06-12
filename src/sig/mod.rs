mod utility;
mod lin_exp;
mod detune;
mod sinewave;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sine {
    id: String,
    mute: bool,
    level: u8,           // 0 - 100
    ratio: u8,           // 1 - 9
    coarse: i8,          // -12 - 12
    fine: i8,            // -100 - 100
    frequencyOffset: u8, // 0 - 20
}

impl Sine {
    pub fn new (id: &String, mute: bool, level: u8, ratio: u8, coarse: i8, fine: i8, frequency_offset: u8) -> Self {
        Sine {
            id: id.to_owned(),
            mute: mute,
            level: level,
            ratio: ratio,
            coarse: coarse,
            fine: fine,
            frequencyOffset: frequency_offset,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Adsr {
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,
}

impl Adsr {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> Self {
        Adsr {
            attack: attack,
            decay: decay,
            sustain: sustain,
            release: release,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wave {
    sampleRate: u32,
    sampleLength: f64, // in seconds
    frequency: f64,
    sineWaves: Vec<Sine>,
    adsr: Adsr,
}

impl Wave {
    pub fn new(sample_rate: u32, sample_length: f64, frequency: f64, sine_waves: &Vec<Sine>, adsr: &Adsr) -> Self {
        Wave {
            sampleRate: sample_rate,
            sampleLength: sample_length,
            frequency: frequency,
            sineWaves: sine_waves.to_vec(),
            adsr: adsr.clone(),
        }
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sampleRate
    }

    pub fn get_sample_length(&self) -> f64 {
        self.sampleLength
    }
}

pub fn parse (wave: &Wave) -> Vec<f64> {
    let mut sine_waves: Vec<sinewave::SineWave> = vec![];

    detune::setup_detune_table();

    for sine in &wave.sineWaves {
        let detune = detune::get_detune_value((sine.coarse as isize) * 100 + (sine.fine as isize));
        sine_waves.push(
            sinewave::SineWave::new(
                wave.sampleRate,
                wave.frequency * (sine.ratio as f64) * detune + (sine.frequencyOffset as f64)
            )
        );
    }

    let mut samples: Vec<f64> = vec![];
    let sample_length = (wave.sampleRate as f64) * wave.sampleLength;

    let attack_length = wave.adsr.attack * (wave.sampleRate as f64);
    let decay_length = wave.adsr.decay * (wave.sampleRate as f64);
    let release_length = wave.adsr.release * (wave.sampleRate as f64);

    let mut sustain_length = sample_length - (attack_length + decay_length + release_length);
    if sustain_length < 0.0 {
        sustain_length = 0.0;
    }

    for sample_count in 0..(sample_length as u64) {
        let mut value = 0.0;

        for (index, sine) in wave.sineWaves.iter().enumerate() {
            if sine.mute || sine.level <= 0 {
                continue;
            }
            let env = sine.level as f64 * 0.01;
            value += env * sine_waves[index].tick();
        }

        let mut current_count = sample_count as f64;

        if current_count < attack_length {
            value *= lin_exp::lin_exp_value(0.0, 1.0, current_count, attack_length);
        } else if current_count < attack_length + decay_length {
            current_count -= attack_length;
            value *= lin_exp::lin_exp_value(1.0, wave.adsr.sustain, current_count, decay_length);
        } else if current_count < attack_length + decay_length + sustain_length {
            value *= wave.adsr.sustain;
        } else {
            current_count -= attack_length + decay_length + sustain_length;
            value *= lin_exp::lin_exp_value(wave.adsr.sustain, 0.0, current_count, release_length);
        }

        samples.push(value);
    }

    samples
}
