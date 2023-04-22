#[cfg(test)]
mod tests {
    use std::ops::Div;

    use super::*;
    use openssl::{bn, bn::BigNum, rsa, rsa::Rsa};

    const BITS: usize = 2048;
    const BYTES: usize = 256;

    trait BnResultTools {
        fn unwrap_vec(self) -> Vec<u8>;
    }

    impl<E: std::fmt::Debug> BnResultTools for Result<BigNum, E> {
        fn unwrap_vec(self) -> Vec<u8> {
            self.unwrap().to_vec_padded(BYTES as i32).unwrap()
        }
    }

    trait BnTools {
        fn to_vec(&self) -> Vec<u8>;
    }

    impl BnTools for BigNum {
        fn to_vec(&self) -> Vec<u8> {
            self.to_vec_padded(BYTES as i32).unwrap()
        }
    }

    #[test]
    fn rsa_mul_homomorphic() {
        let rsa = Rsa::generate(BITS as u32).unwrap();
        let e = rsa.e().to_vec();
        let n = rsa.n().to_vec();
        println!("e: {:?}{:?}", e.len(), e);
        println!("n: {:?}{:?}", n.len(), n);

        let num1 = 1288;
        let num2 = 3306;

        let num1_data = BigNum::from_u32(num1).unwrap_vec();
        let mut num1_buf = [0u8; BYTES];
        rsa.public_encrypt(&num1_data, &mut num1_buf, rsa::Padding::NONE)
            .unwrap();

        let num2_data = BigNum::from_u32(num2).unwrap_vec();
        let mut num2_buf = [0u8; BYTES];
        rsa.public_encrypt(&num2_data, &mut num2_buf, rsa::Padding::NONE)
            .unwrap();

        let mut bnctx = bn::BigNumContext::new().unwrap();
        let num1_enc = BigNum::from_slice(&num1_buf).unwrap();
        let num2_enc = BigNum::from_slice(&num2_buf).unwrap();
        let mut res_enc = BigNum::new().unwrap();
        res_enc
            .mod_mul(&num1_enc, &num2_enc, rsa.n(), &mut bnctx)
            .unwrap();
        let res_buf = res_enc.to_vec_padded(BYTES as i32).unwrap();

        let mut dec_buf = [0u8; BYTES];
        rsa.private_decrypt(&res_buf, &mut dec_buf, rsa::Padding::NONE)
            .unwrap();

        let expect = BigNum::from_u32(num1 * num2).unwrap_vec();
        assert_eq!(&dec_buf[..], &expect);
    }

    #[test]
    fn mod_inv() {
        let a = BigNum::from_u32(11).unwrap();
        let m = BigNum::from_u32(17).unwrap();
        let mut x = BigNum::new().unwrap();
        let mut bnctx = bn::BigNumContext::new().unwrap();
        x.mod_inverse(&a, &m, &mut bnctx).unwrap();

        assert_eq!(x, BigNum::from_u32(14).unwrap());
    }

    #[test]
    fn rsa_blind_signature() {
        let rsa = Rsa::generate(BITS as u32).unwrap();

        let msg_num = BigNum::from_u32(10).unwrap();
        let msg = msg_num.to_vec_padded(BYTES as i32).unwrap();
        println!("msg: {:?}", msg);

        let r = BigNum::from_u32(11).unwrap();
        let mut bnctx = bn::BigNumContext::new().unwrap();
        let mut re = BigNum::new().unwrap();
        re.mod_exp(&r, rsa.e(), rsa.n(), &mut bnctx).unwrap();
        let mut msg_blind_num = BigNum::new().unwrap();
        msg_blind_num
            .mod_mul(&re, &msg_num, rsa.n(), &mut bnctx)
            .unwrap();
        let mut msg_blind = msg_blind_num.to_vec();
        println!("msg_blind : {:?}", msg_blind);

        // additional msg to signature msg
        let add_msg = BigNum::from_u32(3).unwrap();
        let mut msg_blind_add = BigNum::new().unwrap();
        msg_blind_add
            .mod_mul(&add_msg, &msg_blind_num, rsa.n(), &mut bnctx)
            .unwrap();
        msg_blind = msg_blind_add.to_vec();

        let mut blind_signature = [0u8; BYTES];
        rsa.private_encrypt(&msg_blind, &mut blind_signature, rsa::Padding::NONE)
            .unwrap();
        println!("blind_signature: {:?}", blind_signature);

        let blind_signature_num = BigNum::from_slice(&blind_signature).unwrap();
        let mut r_inv = BigNum::new().unwrap();
        r_inv.mod_inverse(&r, rsa.n(), &mut bnctx).unwrap();
        let mut msg_signature_num = BigNum::new().unwrap();
        msg_signature_num
            .mod_mul(&blind_signature_num, &r_inv, rsa.n(), &mut bnctx)
            .unwrap();
        let msg_signature = msg_signature_num.to_vec();
        println!("msg_signature: {:?}", msg_signature);

        let mut msg_check = [0u8; BYTES];
        rsa.public_encrypt(&msg_signature, &mut msg_check, rsa::Padding::NONE)
            .unwrap();
        println!("msg_check: {:?}", msg_check);
    }
}
