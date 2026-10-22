//! This example uses the RP Pico W board Wifi chip (cyw43).
//! Creates an Access point Wifi network and creates a TCP endpoint on port 1234.
//! And then on top of that is the YAWS HTTPS Pipeline running Rick-Roll App responding to HTTPS requests.

#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

mod ab_io;
mod bp_https;
pub(crate) mod bp_known;

use crate::ab_io::{AbIoLeft, AbIoRight};
use cyw43::aligned_bytes;
//use cyw43::{ApAuth, aligned_bytes};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::iface::dhcpv4_server::DhcpServerConfig;
use embassy_net::tcp::{TcpListener, TcpSocket};
use embassy_net::wire::{DhcpOption, IpCidr, Ipv4Addr};
use embassy_net::{StackStorage, StaticPool};
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use panic_probe as _;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>;
// Enable USB Logging
//    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

// Enable USB Logging
/*
use embassy_rp::{peripherals::USB, usb};

#[embassy_executor::task]
async fn logger_task(usb: embassy_rp::Peri<'static, embassy_rp::peripherals::USB>) {
    let driver = embassy_rp::usb::Driver::new(usb, Irqs);
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}
*/

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<
        'static,
        cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>,
        cyw43::Cyw43439,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static>) -> ! {
    runner.run().await
}

#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    // Enable USB Logging
    //spawner.spawn(unwrap!(logger_task(p.USB)));
    //log::info!("I'm here");

    let fw = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        dma::Channel::new(p.DMA_CH0, Irqs),
        dma::Channel::new(p.DMA_CH1, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;
    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Use a link-local address for communication without DHCP server
    // Generate random seed
    let seed = rng.next_u64();

    // Init network stack
    static STACK: StaticCell<StackStorage> = StaticCell::new();
    // cyw43 receives each frame with its bus headers straight into these buffers.
    static POOL: StaticPool<{ cyw43::RX_BUF_SIZE }, 16, { cyw43::RX_BUF_ALIGN }> =
        StaticPool::new();
    let (stack, runner) = embassy_net::Stack::new(STACK.init(StackStorage::new()), &POOL, seed);

    // Add the network interface to the stack.
    static DEVICE: StaticCell<cyw43::NetDriver<'static>> = StaticCell::new();
    let iface = unwrap!(stack.add_iface_borrowed(DEVICE.init(net_device)));
    // Static address, we're the access point.
    unwrap!(iface.add_ip_addr(IpCidr::new(Ipv4Addr::new(10, 0, 0, 1).into(), 24)));
    let mut dhcp_config =
        DhcpServerConfig::new(Ipv4Addr::new(10, 0, 0, 100), Ipv4Addr::new(10, 0, 0, 199));
    dhcp_config.outgoing_options = &[DhcpOption {
        kind: 114,
        data: &[
            16, 104, 116, 116, 112, 58, 47, 47, 49, 48, 46, 48, 46, 48, 46, 49, 47,
        ],
    }];
    unwrap!(iface.set_dhcpv4_server(Some(dhcp_config)));

    spawner.spawn(unwrap!(net_task(runner)));

    control.start_ap_open("Rick-Roll", 5).await;
    //control.start_ap("cyw43", "password", ApAuth::Wpa2, 5).await;
    // WPA3 requires compatible CYW43 firmware and client support.
    // control.start_ap("cyw43", "password", ApAuth::Wpa3, 5).await;
    // control.start_ap("cyw43", "password", ApAuth::Wpa2Wpa3, 5).await;

    let mut rx_buffer = [0; 8192];
    let mut tx_buffer = [0; 8192];

    let mut listener = unwrap!(TcpListener::new(stack));
    unwrap!(listener.listen(1234));

    loop {
        control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        let token = match listener.accept().await {
            Ok(token) => token,
            Err(e) => {
                warn!("accept error: {:?}", e);
                continue;
            }
        };
        //log::info!("Accepted");
        let mut socket = unwrap!(TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer));
        socket.set_timeout(Some(Duration::from_secs(10)));
        if let Err(e) = socket.accept(token).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_addr());
        control.gpio_set(0, true).await;

        let mut left = AbIoLeft::default();
        let mut right = AbIoRight::default();

        info!("Loading Blueprints for ytls+h11spec+App");
        let mut bp = bp_https::ytls_server_blueprints();
        info!("Blueprints loaded");

        let mut app_ready = false;

        loop {
            let n_try = match socket.try_read(&mut left.in_bytes[left.in_bytes_len..]) {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => Some(n),
                Err(embassy_net::TryError::WouldBlock) => {
                    Timer::after_millis(10).await;
                    None
                }
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };
            if let Some(n) = n_try {
                info!("Read {} bytes", n);
                left.in_bytes_len += n;
            }

            struct NothingBurger;
            use blueprint::{Left, NoRight, Orbit};

            let layers = bp.layers_as_mut();
            let _pos = layers[0].advance_with(&mut NothingBurger, &mut left, &mut right);

            if left.is_ready() || app_ready {
                if right.in_bytes_len > 0 {
                    info!(
                        "Request? [{}]",
                        unwrap!(core::str::from_utf8(&right.in_bytes[..right.in_bytes_len]))
                    );
                }
                app_ready = true;
                let mut app_left = right.impl_left();
                let app = bp.app_as_mut();

                let _pos = app.advance_with(&mut NothingBurger, &mut app_left, &mut NoRight);
            }

            if left.out_bytes_len > 0 {
                info!("Sending out {} bytes", left.out_bytes_len);
                match socket
                    .write_all(&left.out_bytes[..left.out_bytes_len])
                    .await
                {
                    Ok(()) => {}
                    Err(e) => {
                        warn!("write error: {:?}", e);
                        break;
                    }
                };

                left.out_bytes = [0; 8192];
                left.out_bytes_len = 0;
            }
        }
    }
}
