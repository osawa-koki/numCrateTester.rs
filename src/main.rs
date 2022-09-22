extern crate num;
use num::*;


fn main() {
    // 切り捨て・切り上げ
    {
        println!("integer::div_floor(7, 3) -> {}", integer::div_floor(7, 3));
        println!("integer::div_ceil(7, 3) -> {}", integer::div_ceil(7, 3));
    }

    // 平方根
    {
        println!("integer::sqrt(2) -> {}", integer::sqrt(2));
        println!("Float::sqrt(2.0) -> {}", Float::sqrt(2.0));

        println!("integer::sqrt(25) -> {}", integer::sqrt(25));
        println!("Float::sqrt(25.0) -> {}", Float::sqrt(25.0));
    }

    // 最大公約数・最小公倍数
    {
        println!("integer::gcd(12, 18) -> {}", integer::gcd(12, 18));
        println!("integer::lcm(12, 18) -> {}", integer::lcm(12, 18));
    }

    // 対数
    {
        println!("Float::ln(2.0) -> {}", Float::ln(2.0));
        println!("Float::log(2.0, 10.0) -> {}", Float::log(2.0, 10.0));
    }

    // べき乗
    {
        println!("3.pow(3) -> {}", 3.pow(3));
        println!("(3.14).powi(3) -> {}", (3.14).powi(3));
        println!("(3.14).powf(3.14) -> {}", (3.14).powf(3.14));
        println!("Float::exp(10.0) -> {}", Float::exp(10.0));
        println!("Float::exp2(10.0) -> {}", Float::exp2(10.0));
    }

    // 二項計算
    {
        println!("integer::binomial(5, 2) -> {}", integer::binomial(5, 2));
    }

    // 複素数
    {
        let cplx_1 = Complex{re: 3.0, im: -1.0};
        let cplx_2 = Complex{re: 2.5, im: -1.5};
        println!("cplx_1 -> {}", cplx_1);
        println!("cplx_2 -> {}", cplx_2);
        println!("cplx_1 + cplx_2 -> {}", cplx_1 + cplx_2);
        println!("cplx_1 - cplx_2 -> {}", cplx_1 - cplx_2);
        println!("cplx_1 * cplx_2 -> {}", cplx_1 * cplx_2);
        println!("cplx_1 / cplx_2 -> {}", cplx_1 / cplx_2);
        println!("cplx_1 % cplx_2 -> {}", cplx_1 % cplx_2);

        println!("cplx_1.norm() -> {}", cplx_1.norm());
        println!("cplx_1.norm_sqr() -> {}", cplx_1.norm_sqr());
        println!("cplx_1.norm().powi(2) -> {}", cplx_1.norm().powi(2));
        println!("cplx_1.exp() -> {}", cplx_1.exp());
        println!("cplx_1.ln() -> {}", cplx_1.ln());
        println!("cplx_1.sqrt() -> {}", cplx_1.sqrt());
        println!("cplx_1.cbrt() -> {}", cplx_1.cbrt());
        println!("cplx_1.log(10.0) -> {}", cplx_1.log(10.0));
        println!("cplx_1.powi(2) -> {}", cplx_1.powi(2));
        println!("cplx_1.powf(2.5) -> {}", cplx_1.powf(2.5));
        println!("cplx_1.powc(cplx_2) -> {}", cplx_1.powc(cplx_2));
    }
}
