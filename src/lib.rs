// Using algorithm from: https://www.quinapalus.com/efunc.html

const Q: u32 = 16;

fn fp_to_float(fp: u64) -> f64 {
    (fp as f64) / 2f64.powf(Q as f64)
}

fn sh_fp(value: u64, shift: i64) -> u64 {
    if shift > 0 {
        value << shift
    } else {
        (1 << Q) + ((1 << Q) >> shift)
    }
}

fn k_shift_to_ln_k(k_shift: i64) -> u64 {
    let k = sh(1, k_shift).ln() * 2f64.powf(Q as f64);
    k as u64
}

fn exp_no_mul_div_fp(x: u64) -> u64 {
    let mut y = 1 << Q;
    let mut cur_x = x;
    if x == 0 {
        return 1 << Q;
    }
    for _ in 0..64 {
        //println!("x: {}\ty: {}\texp({}) = {}\ty * exp(x) = {}", fp_to_float(cur_x), fp_to_float(y), fp_to_float(x), fp_to_float(x).exp(), fp_to_float(y) * fp_to_float(cur_x).exp());
        let k_shift = generate_k(fp_to_float(cur_x));
        if k_shift > 0 {
            y = y << k_shift;
        } else {
            y = y + (y >> -k_shift);
        }
        let k = k_shift_to_ln_k(k_shift);
        cur_x = cur_x - k;
    }
    y
}

fn sh(value: i64, shift: i64) -> f64 {
    if shift > 0 {
        (value << shift) as f64
    } else {
        1f64 + 2.0f64.powf(shift as f64)
    }
}

fn generate_k(target: f64) -> i64 {
    let mut shift = -63;
    let mut k = (sh(1, shift) as f64).ln();
    while k < target {
        shift += 1;
        k = (sh(1, shift) as f64).ln();
        //println!("Sh: {}, exp(k): {}, k: {}", shift, sh(1, shift), k);
    }
    shift - 1
}

fn exp_no_mul_div(x: i64) -> f64 {
    let mut y = 1f64;
    let mut cur_x = x as f64;
    for _ in 0..64 {
        //println!("x: {}\ty: {}\texp({}) = {}\ty * exp(x) = {}", cur_x, y, x, (x as f64).exp(), y * cur_x.exp());
        let k_shift = generate_k(cur_x);
        if k_shift > 0 {
            y = y * sh(1, k_shift);
        } else {
            y = y * sh(1, k_shift);
        }
        let k = sh(1, k_shift).ln();
        cur_x = cur_x - k;
    }
    y
}


#[cfg(test)]
mod tests {
    use super::*;
    use speculoos::prelude::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn generate_k_simple() {
        assert_eq!(generate_k(6.0), 8);
    }

    #[test]
    fn generate_k_negative() {
        assert_eq!(generate_k(0.07), -4);
    }

    #[test]
    fn small_x() {
        let xs = [
            20f64,
            10f64,
            5f64,
            0.0002f64,
            0.2f64,
            0.00000000022314243834656339,
        ];
        for x in xs {
            let k = sh(1, generate_k(x)).ln();
            println!("x: {}\nk: {}", x, k);
            assert!(x > k);
        }
    }

    #[test]
    fn exp_no_mul_div_test() {
        for i in 0..100 {
            if (i as f64).exp() > (u64::MAX >> Q) as f64 {
                println!("Reached max representation for 64 bit with current Q value");
                break
            }
            println!("======================== Real Exp");
            let expected_result = (i as f64).exp();
            println!("\t{:e}", expected_result);
            println!("======================== Floating Point");
            let float_result = exp_no_mul_div(i);
            println!("\t{:e}", float_result);
            println!("======================== Fixed Point");
            let q_result = fp_to_float(exp_no_mul_div_fp((i as u64) << Q));
            println!("\t{:e}", q_result);
            assert_that(&q_result).is_close_to(expected_result, (expected_result / 1e3) + 1e-10);
            assert_that(&float_result).is_close_to(expected_result, (expected_result / 1e3) + 1e-10);
        }
    }
}
