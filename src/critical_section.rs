use core::mem::MaybeUninit;

use spin::{Mutex, MutexGuard};

struct SpinLockCriticalSection {}

critical_section::set_impl!(SpinLockCriticalSection);

static CRITICAL_SECTION_MUTEX: Mutex<()> = Mutex::new(());
static mut CRITICAL_SECTION_MUTEX_GUARD: MaybeUninit<MutexGuard<'static, ()>> =
    MaybeUninit::uninit();

unsafe impl critical_section::Impl for SpinLockCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let guard = CRITICAL_SECTION_MUTEX.lock();
        CRITICAL_SECTION_MUTEX_GUARD.write(guard);
        false
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        if !restore_state {
            CRITICAL_SECTION_MUTEX_GUARD.assume_init_drop();
        }
    }
}
