use spin::Mutex;

use cpuio::UnsafePort;
use cpuio::Port;

// Command sent to begin PIC initialization.
const CMD_INIT: u8 = 0x11;
// Command sent to acknowledge an interrupt
const CMD_END_OF_INTERRUPT: u8 = 0x20;
// The mode in which we want to run our PICs
const MODE_8086: u8 = 0x01;

// we have two 8259 PIC chips: A and B
// each has two 8 bit ports: one command port and one data port
// and each handles 8 interrupts
struct Pic {
    offset: u8,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>,
}


impl Pic {
    fn handles_interrupt(&self, interupt_id: u8) -> bool {
        // helper to check is this pic can processe this interrupt_id or not
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        // says to the PIC that we are done processing the interrupt
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}


pub struct Pic8259 {
    pics: [Pic; 2],
}

impl Pic8259 {
    pub unsafe fn new(offset1: u8, offset2: u8) -> Pic8259 {
        Pic8259 {
            pics: [
                Pic {
                    offset: offset1,
                    command: UnsafePort::new(0x20),
                    data: UnsafePort::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: UnsafePort::new(0xA0),
                    data: UnsafePort::new(0xA1),
                },
            ]
        }
    }

    pub unsafe fn initialize(&mut self) {
        // initialization: https://wiki.osdev.org/8259_PIC#Initialisation
        //
        // write some data to port 0x80 and this makes our computer to wait for a while
        // https://wiki.osdev.org/Inline_Assembly/Examples#IO_WAIT
        // https://github.com/mjg59/kexec-tools/blob/master/kexec_test/x86-setup-legacy-pic.S
        //
        //      outb	%al, $0x80	/* A short delay */
        //
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || { wait_port.write(0) };

        // save original pics data
        // why do I need to do this?  "save masks" osdev wiki
        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        self.pics[0].data.write(CMD_INIT);
        wait();
        self.pics[1].data.write(CMD_INIT);
        wait();

        // now we send three commands: offsets, chaining between pic1 and pic2 and our mode

        // setup offsets
        self.pics[0].data.write(self.pics[0].offset);
        wait();
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        // chaining
        self.pics[0].data.write(4);
        wait();
        self.pics[1].data.write(2);
        wait();

        // set mode
        self.pics[0].data.write(MODE_8086);
        wait();
        self.pics[1].data.write(MODE_8086);
        wait();

        // restore initial data
        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.pics[0].handles_interrupt(interrupt_id) {
            self.pics[0].end_of_interrupt();
        }
        if self.pics[1].handles_interrupt(interrupt_id) {
            self.pics[1].end_of_interrupt();
        }
    }
}


lazy_static! {
    pub static ref PICS: Mutex<Pic8259> = Mutex::new(unsafe{
        Pic8259::new(0x20, 0x28)
    });
}
