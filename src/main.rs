#![feature(test)]
extern crate test;

fn main() {
    eprintln!("DO NOT RUN ME, I'M JUST A BENCHMARK!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use eui48::MacAddress;
    use std::str::FromStr;

    #[bench]
    fn bench_serialize(b: &mut Bencher) {
        let mac = MacAddress::from_str("00:01:02:03:04:05").unwrap();
        let mut buffer = Vec::with_capacity(4096);

        b.iter(|| {
            bincode::serialize_into(&mut buffer, &mac).unwrap()
        })
    }

    #[bench]
    fn bench_deserialize(b: &mut Bencher) {
        let mac = MacAddress::from_str("00:01:02:03:04:05").unwrap();
        let mut buffer = Vec::with_capacity(4096);

        bincode::serialize_into(&mut buffer, &mac).unwrap();

        b.iter(|| {
            let mac2: MacAddress = bincode::deserialize_from(&*buffer).unwrap();
            mac2
        })
    }
}