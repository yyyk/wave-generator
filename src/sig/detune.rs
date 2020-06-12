const MAX_COARSE: u64 = 12;
const MAX_FINE: u64 = 100;

// -1300 - 1300 => 0 - 2600
const DETUNE_TABLE_SIZE: isize = 2 * (MAX_COARSE as isize * 100 + MAX_FINE as isize);
const DETUNE_TABLE_HALF_SIZE: isize = DETUNE_TABLE_SIZE / 2;

static mut DETUNE_TABLE: [f64; DETUNE_TABLE_SIZE as usize] = [0.0; DETUNE_TABLE_SIZE as usize];
static mut IS_DETUNE_TABLE_READY: bool = false;

pub fn setup_detune_table() {
    unsafe {
        if !IS_DETUNE_TABLE_READY {
            for i in 0..DETUNE_TABLE_SIZE as usize {
                DETUNE_TABLE[i] = (2.0 as f64).powf((i as f64 - 1300.0) / 1200.0);
            }
            IS_DETUNE_TABLE_READY = true;
        }
    }
}

pub fn get_detune_value(detune: isize) -> f64 {
    unsafe {
        if !IS_DETUNE_TABLE_READY {
            setup_detune_table();
        }
    }
    let mut detune_index = detune;
    if detune_index > DETUNE_TABLE_HALF_SIZE {
        detune_index = DETUNE_TABLE_HALF_SIZE;
    } else if detune_index < -DETUNE_TABLE_HALF_SIZE {
        detune_index = -DETUNE_TABLE_HALF_SIZE;
    }
    let mut result: f64 = 0.0;
    unsafe {
        result = DETUNE_TABLE[(detune_index + DETUNE_TABLE_HALF_SIZE) as usize];
    }
    result
}
