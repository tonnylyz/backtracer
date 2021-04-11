#[derive(Debug, Clone)]
pub struct Frame {
    pub fp: u64,
    pub sp: u64,
    pub pc: u64,
    pub lr: u64,
}

impl Frame {
    pub fn new(fp: u64, sp: u64, pc: u64, lr: u64) -> Frame {
        Frame {
            fp,
            sp,
            pc,
            lr,
        }
    }

    pub fn ip(&self) -> *mut u8 {
        self.pc as *mut u8
    }
    pub fn symbol_address(&self) -> *mut u8 {
        0 as *mut u8
    }
}

#[inline(always)]
pub fn trace_from(mut curframe: Frame, cb: &mut dyn FnMut(&super::Frame) -> bool) {
    loop {
        let mut bomb = ::Bomb { enabled: true };
        let ctxt = super::Frame {
            inner: curframe.clone(),
        };

        let keep_going = cb(&ctxt);
        bomb.enabled = false;

        if keep_going {
            unsafe {
                #[cfg(target_arch = "aarch64")]
                {
                    curframe.pc = *((curframe.fp + 8) as *mut u64);
                    curframe.fp = *(curframe.fp as *mut u64);
                }
                #[cfg(target_arch = "riscv64")]
                {
                    curframe.pc = ((curframe.fp - 8) as *mut u64).read();
                    curframe.sp = curframe.fp;
                    curframe.fp = ((curframe.fp - 16) as *mut u64).read();
                }

                if curframe.pc == 0 || curframe.fp <= 0xfff {
                    break;
                }
            }
        } else {
            break;
        }
    }
}

#[inline(always)]
#[cfg(target_arch = "aarch64")]
pub fn trace(cb: &mut dyn FnMut(&super::Frame) -> bool) {
    use cortex_a::regs::*;

    let curframe = Frame::new(FP.get(), SP.get(), PC.get(), LR.get());
    trace_from(curframe.clone(), cb);
}

#[inline(always)]
#[cfg(target_arch = "riscv64")]
pub fn trace(cb: &mut dyn FnMut(&super::Frame) -> bool) {
    let curframe = Frame::new(0, 0, 0, 0);
    trace_from(curframe.clone(), cb);
}
