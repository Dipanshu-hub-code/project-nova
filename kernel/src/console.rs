use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_COLOR: u8 = 0x0f;
const SERIAL_PORT: u16 = 0x3f8;
const SERIAL_LINE_STATUS: u16 = SERIAL_PORT + 5;
const SERIAL_TRANSMIT_EMPTY: u8 = 0x20;
const SERIAL_WAIT_LIMIT: usize = 100_000;

static CONSOLE: Mutex<Console> = Mutex::new(Console::new());
static SERIAL_AVAILABLE: AtomicBool = AtomicBool::new(false);

struct Console {
    column: usize,
    row: usize,
}

impl Console {
    const fn new() -> Self {
        Self { column: 0, row: 0 }
    }

    fn clear(&mut self) {
        for index in 0..VGA_WIDTH * VGA_HEIGHT {
            self.write_cell(index, b' ');
        }
        self.column = 0;
        self.row = 0;
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.column = 0;
                self.row += 1;
                self.scroll_if_needed();
            }
            b'\r' => self.column = 0,
            b'\t' => {
                let spaces = 4 - self.column % 4;
                for _ in 0..spaces {
                    self.write_printable(b' ');
                }
            }
            0x20..=0x7e => self.write_printable(byte),
            _ => self.write_printable(b'?'),
        }
    }

    fn write_printable(&mut self, byte: u8) {
        self.write_cell(self.row * VGA_WIDTH + self.column, byte);
        self.column += 1;
        if self.column == VGA_WIDTH {
            self.column = 0;
            self.row += 1;
            self.scroll_if_needed();
        }
    }

    fn scroll_if_needed(&mut self) {
        if self.row < VGA_HEIGHT {
            return;
        }

        for index in 0..(VGA_HEIGHT - 1) * VGA_WIDTH {
            let cell = unsafe { read_volatile(VGA_BUFFER.add(index + VGA_WIDTH)) };
            unsafe {
                write_volatile(VGA_BUFFER.add(index), cell);
            }
        }
        for column in 0..VGA_WIDTH {
            self.write_cell((VGA_HEIGHT - 1) * VGA_WIDTH + column, b' ');
        }
        self.row = VGA_HEIGHT - 1;
    }

    fn write_cell(&self, index: usize, byte: u8) {
        let cell = (u16::from(VGA_COLOR) << 8) | u16::from(byte);
        unsafe {
            write_volatile(VGA_BUFFER.add(index), cell);
        }
    }
}

impl Write for Console {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            match byte {
                b'\n' => {
                    write_serial_byte(b'\r');
                    self.write_byte(byte);
                    write_serial_byte(byte);
                }
                b'\t' => {
                    let spaces = 4 - self.column % 4;
                    for _ in 0..spaces {
                        self.write_byte(b' ');
                        write_serial_byte(b' ');
                    }
                }
                _ => {
                    self.write_byte(byte);
                    write_serial_byte(byte);
                }
            }
        }
        Ok(())
    }
}

struct PanicWriter {
    column: usize,
}

impl Write for PanicWriter {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            write_serial_byte(byte);
            match byte {
                b'\n' => self.column = 0,
                b'\r' => self.column = 0,
                0x20..=0x7e => self.write_panic_cell(byte),
                _ => self.write_panic_cell(b'?'),
            }
        }
        Ok(())
    }
}

impl PanicWriter {
    fn write_panic_cell(&mut self, byte: u8) {
        if self.column < VGA_WIDTH {
            let index = (VGA_HEIGHT - 1) * VGA_WIDTH + self.column;
            let cell = (u16::from(VGA_COLOR) << 8) | u16::from(byte);
            unsafe {
                write_volatile(VGA_BUFFER.add(index), cell);
            }
            self.column += 1;
        }
    }
}

pub fn init() {
    init_serial();
    CONSOLE.lock().clear();
}

pub fn write_fmt(args: fmt::Arguments<'_>) -> fmt::Result {
    CONSOLE.lock().write_fmt(args)
}

pub fn show_software_center() {
    let mut console = CONSOLE.lock();
    console.clear();
    for (index, line) in nova_kernel::software_center::text_screen()
        .iter()
        .enumerate()
    {
        if index + 1 == nova_kernel::software_center::text_screen().len() {
            let _ = console.write_str(line);
        } else {
            let _ = writeln!(console, "{line}");
        }
    }
}

pub fn write_panic(info: &core::panic::PanicInfo<'_>) {
    if let Some(mut console) = CONSOLE.try_lock() {
        let _ = writeln!(console, "\nKERNEL PANIC: {info}");
    } else {
        let mut writer = PanicWriter { column: 0 };
        let _ = writeln!(writer, "KERNEL PANIC: {info}");
    }
}

fn init_serial() {
    unsafe {
        write_port(SERIAL_PORT + 1, 0x00);
        write_port(SERIAL_PORT + 3, 0x80);
        write_port(SERIAL_PORT, 0x03);
        write_port(SERIAL_PORT + 1, 0x00);
        write_port(SERIAL_PORT + 3, 0x03);
        write_port(SERIAL_PORT + 2, 0xc7);
        write_port(SERIAL_PORT + 4, 0x0b);
    }
    let available = unsafe { read_port(SERIAL_LINE_STATUS) != 0xff };
    SERIAL_AVAILABLE.store(available, Ordering::Relaxed);
}

fn write_serial_byte(byte: u8) {
    if !SERIAL_AVAILABLE.load(Ordering::Relaxed) {
        return;
    }

    for _ in 0..SERIAL_WAIT_LIMIT {
        let status = unsafe { read_port(SERIAL_LINE_STATUS) };
        if status & SERIAL_TRANSMIT_EMPTY != 0 {
            unsafe {
                write_port(SERIAL_PORT, byte);
            }
            return;
        }
        core::hint::spin_loop();
    }
}

unsafe fn write_port(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

unsafe fn read_port(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nomem, nostack, preserves_flags)
    );
    value
}
