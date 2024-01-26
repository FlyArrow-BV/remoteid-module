use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // The imxrt1020EVK has 16 MiB of flash.
    RuntimeBuilder::from_flexspi(Family::Imxrt1020, 16 * 1024 * 1024)
        .build()
        .unwrap();
}
