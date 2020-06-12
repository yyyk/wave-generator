pub fn lin_exp(from: f64, to: f64, at: f64, len: f64) -> f64 {
    if (to - from).abs() <= 0.00001 {
        return from
    }
    if at <= 0.0 {
        return from
    }
    if at >= len {
        return to
    }
    if from > to {
        let mut x = at * (to - from) / len + from;
        x = ((x - to) / (from - to)).powi(3);
        return x * (from - to) + to
    } else {
        let mut x = at * (to - from) / len + from;
        x = ((x - from) / (to - from)).powf(1.0 / 3.0);
        return x * (to - from) + from
    }
}

// 0 -> 65535
const LIN_EXP_TABLE_SIZE: usize = 65536; // 2 ** 16
static mut LIN_EXP_UP_TABLE: [f64; LIN_EXP_TABLE_SIZE as usize] = [0.0; LIN_EXP_TABLE_SIZE];
static mut LIN_EXP_DOWN_TABLE: [f64; LIN_EXP_TABLE_SIZE as usize] = [0.0; LIN_EXP_TABLE_SIZE];
static mut IS_LIN_EXP_TABLE_READY: bool = false;

pub fn setup_lin_exp_table() {
    unsafe {
        if !IS_LIN_EXP_TABLE_READY {
            for i in 0..LIN_EXP_TABLE_SIZE {
                LIN_EXP_UP_TABLE[i] = lin_exp(0.0, 1.0, i as f64, (LIN_EXP_TABLE_SIZE - 1) as f64);
                LIN_EXP_DOWN_TABLE[i] = lin_exp(1.0, 0.0, i as f64, (LIN_EXP_TABLE_SIZE - 1) as f64);
            }
            IS_LIN_EXP_TABLE_READY = true;
        }
    }
}

pub fn lin_exp_value(from: f64, to: f64, at: f64, len: f64) -> f64 {
    unsafe {
        if !IS_LIN_EXP_TABLE_READY {
            setup_lin_exp_table();
        }
    }
    if (to - from).abs() <= 0.00001 {
        return from
    }
    if at <= 0.0 {
        return from
    }
    if at >= len {
        return to
    }
    let mut result: f64 = 0.0;
    unsafe {
        let x = (LIN_EXP_TABLE_SIZE - 1) as f64 * at / len;
        let base_index = x as usize;
        let fraction = x - base_index as f64;

        if to > from {
            result = LIN_EXP_UP_TABLE[base_index] * (to - from) + from;
            if base_index >= LIN_EXP_TABLE_SIZE - 1 {
                result += fraction * (to - result);
            } else {
                result += fraction * (LIN_EXP_UP_TABLE[base_index + 1] * (to - from) + from - result);
            }
        } else {
            result = (1.0 - LIN_EXP_DOWN_TABLE[base_index]) * (to - from) + from;
            if base_index >= LIN_EXP_TABLE_SIZE - 1 {
                result += fraction * (to - result);
            } else {
                result += fraction * ((1.0 - LIN_EXP_DOWN_TABLE[base_index + 1]) * (to - from) + from - result);
            }
        }
    }
    result
}
