
use lpc11uxx::{IOCON, USART, SYSCON};

pub struct Uart;

impl Uart {
    /// Initialize the uart.
    ///
    /// * Set pin functions (USART, pull up)
    /// * Set pin direction (output)
    pub fn init(syscon: &mut SYSCON, iocon: &mut IOCON, usart: &mut USART) -> Self {

        // enable usart peripheral
        syscon.sysahbclkctrl.modify(|_,w| w.usart().enabled());

        unsafe {
            usart.fcr.fcr.write(|w| w
                                .fifoen().enabled()
                                .txfifores().clear_bit()
                                .rxfifores().clear_bit()
                                .rxtl().bits(0)
                                );
            usart.dlm.ier.write(|w| w.bits(0));
        }


        unsafe {
            // set baudrate. TODO calculate it ;)
            syscon.uartclkdiv.write(|w| w.bits(0x1));

            // set LCR[DLAB] to enable writing to divider registers
            usart.lcr.modify(|r,w| w
                             .bits(r.bits())
                             .dlab().enable_access_to_div());
            
            usart.dlm.dlm.write(|w| w.bits(1));
            usart.dll.dll.write(|w| w.bits(1));

            usart.lcr.modify(|r,w| w
                             .bits(r.bits())
                             .dlab().disable_access_to_di());

            usart.lcr.write(|w| w
                            .wls()._8_bit_character_leng()
                            .sbs()._1_stop_bit()
                            .pe().disabled()
                            .bc().disable_break_transm());
        }

        // Set port functions
        (*iocon).pio1_13.write(|w| w
            .func().txd()
            .mode().pull_up());
        //(*iocon).pio1_14.write(|w| w
        //    .func().rxd()
        //    .mode().pull_up());

        (*iocon).pio0_19.write(|w| w
            .func().txd()
            .mode().pull_up());
        //(*iocon).pio0_18.write(|w| w
        //    .func().rxd()
        //    .mode().pull_up());

        //unsafe {
            // Set pin directions to output
            //(*gpio).dir[1].modify(|r, w| w.bits(r.bits() | (1<<13)));
        //}

        Uart { }
    }

    pub fn putc(&mut self, usart: &mut USART, value: u8) {
        unsafe {
            usart.dll.thr.write(|w| w.bits(value.into()));
        }
    }
}
