fn split_parts(binary_input: u32) -> (u32, u32, u32) {
    // shift right by 31 bits, keeping the sign bit
    let sign = (binary_input >> 31) & 1;
    
    // the first operation shift right by 23 bits
    // then it remains the 9th first
    // then we use a mask of 8 bits to keep only the first 8 bits
    let exponent = (binary_input >> 23) & 0xFF;
    
    // mantissa is the first 23 bits
    // then we use a mask of 23 bits to keep only the first 23 bits
    let mantissa = binary_input & 0x7FFFFF;

    (sign, exponent, mantissa)
}

fn decode_mantissa(raw_mantissa: u32) -> f32 {
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let bit_i = raw_mantissa & mask;

        if bit_i != 0 {
            let i_f32 = i as f32;
            let weight = 2_f32.powf(i_f32 - 23.0);
            mantissa += weight;
        }
    }

    mantissa
}

pub fn decode (binary_input: u32) -> f32 {
    const BIAS: i32 = 127;
    const RADIX: f32 = std::f32::RADIX as f32;

    let (sign, exponent, raw_mantissa) = split_parts(binary_input);
    
    // convert sign bit to float (1.0 or -1.0)
    let signed_1 = (-1.0_f32).powi(sign as i32);

    // convert exponent sign in case substracting BIAS
    // results in a negative number
    let exponent = (exponent as i32) - BIAS;
    // it needs to be cast as f32 for exponentiation
    let exponent = RADIX.powf(exponent as f32);
    let mantissa = decode_mantissa(raw_mantissa);

    signed_1 * exponent * mantissa
}

