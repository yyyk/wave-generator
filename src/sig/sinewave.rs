use super::utility::TWO_PI;

const TABLE_SIZE: usize = 4096;
static mut SINE_TABLE: [f64; TABLE_SIZE] = [0.0; TABLE_SIZE];
static mut IS_TABLE_READY: bool = false;

#[derive(Debug, Clone, PartialEq)]
pub struct SineWave {
    sample_rate: u32,
    increment: f64,
    current_phase: f64,
    phase_offset: f64,
    base_index: usize,
    fraction: f64,
    last_value: f64,
}

impl SineWave {
    pub fn new(sample_rate: u32, frequency: f64) -> Self {
        unsafe {
            if !IS_TABLE_READY {
                for i in 0..TABLE_SIZE {
                  SINE_TABLE[i] = ((i as f64) * TWO_PI / (TABLE_SIZE as f64)).sin();
                }
                IS_TABLE_READY = true;
            }
        }
        let mut sine_wave = SineWave {
            sample_rate: sample_rate,
            increment: 1.0,
            current_phase: 0.0,
            phase_offset: 0.0,
            base_index: 0,
            fraction: 0.0,
            last_value: 0.0
        };
        sine_wave.set_frequency(frequency);
        sine_wave
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.increment = frequency * (TABLE_SIZE as f64) / (self.sample_rate as f64);
    }

    pub fn reset(&mut self) {
        self.current_phase = 0.0;
        self.last_value = 0.0;
    }

    pub fn add_phase(&mut self, phase: f64) {
        self.current_phase += phase * TABLE_SIZE as f64;
    }

    pub fn add_phase_offset(&mut self, phase_offset: f64) {
        self.current_phase += (phase_offset - self.phase_offset) * TABLE_SIZE as f64;
        self.phase_offset = phase_offset;
    }

    pub fn tick(&mut self) -> f64 {
        while self.current_phase < 0.0 {
            self.current_phase += TABLE_SIZE as f64
        }

        while self.current_phase >= (TABLE_SIZE as f64) {
            self.current_phase -= TABLE_SIZE as f64
        }

        self.base_index = self.current_phase as usize;
        self.fraction = self.current_phase - self.base_index as f64;

        let mut tmp: f64 = 0.0;

        unsafe {
            tmp = SINE_TABLE[self.base_index];
            tmp += self.fraction * (SINE_TABLE[(self.base_index + 1) % TABLE_SIZE] - tmp);
        }

        self.current_phase += self.increment;
        self.last_value = tmp;

        self.last_value
    }
}
