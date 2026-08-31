use std::io;
#[cfg(unix)]
use std::{mem, os::unix::process::CommandExt, process::Command, ptr};

#[cfg(unix)]
unsafe fn install_handler(
    signum: libc::c_int,
    handler: libc::sighandler_t,
) -> io::Result<libc::sigaction> {
    unsafe {
        let mut new: libc::sigaction = mem::zeroed();
        new.sa_sigaction = handler;
        new.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&mut new.sa_mask);

        let mut previous: libc::sigaction = mem::zeroed();
        if libc::sigaction(signum, &new, &mut previous) != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(previous)
    }
}

#[cfg(unix)]
unsafe fn restore_handler(signum: libc::c_int, previous: &libc::sigaction) -> io::Result<()> {
    unsafe {
        if libc::sigaction(signum, previous, ptr::null_mut()) != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(unix)]
unsafe fn reset_handler_default(signum: libc::c_int) -> io::Result<()> {
    unsafe {
        let mut action: libc::sigaction = mem::zeroed();
        action.sa_sigaction = libc::SIG_DFL;
        action.sa_flags = 0;
        libc::sigemptyset(&mut action.sa_mask);
        if libc::sigaction(signum, &action, ptr::null_mut()) != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(unix)]
pub(crate) struct SigintGuard(libc::sigaction);

#[cfg(unix)]
impl SigintGuard {
    pub(crate) fn ignore() -> io::Result<Self> {
        let previous = unsafe { install_handler(libc::SIGINT, libc::SIG_IGN)? };
        Ok(Self(previous))
    }
}

#[cfg(unix)]
impl Drop for SigintGuard {
    fn drop(&mut self) {
        let _ = unsafe { restore_handler(libc::SIGINT, &self.0) };
    }
}

#[cfg(not(unix))]
pub(crate) struct SigintGuard;

#[cfg(not(unix))]
impl SigintGuard {
    pub(crate) fn ignore() -> io::Result<Self> {
        Ok(Self)
    }
}

#[cfg(unix)]
pub(crate) fn prepare_foreground_command(command: &mut Command) {
    unsafe {
        command.pre_exec(|| reset_handler_default(libc::SIGINT));
    }
}

#[cfg(unix)]
pub(crate) fn prepare_background_command(command: &mut Command) {
    unsafe {
        command.pre_exec(|| {
            if libc::setpgid(0, 0) != 0 {
                return Err(io::Error::last_os_error());
            }
            reset_handler_default(libc::SIGINT)
        });
    }
}

#[cfg(not(unix))]
pub(crate) fn prepare_foreground_command(_: &mut std::process::Command) {}

#[cfg(not(unix))]
pub(crate) fn prepare_background_command(_: &mut std::process::Command) {}

#[cfg(unix)]
pub(crate) fn continue_background_job(pid: u32) -> io::Result<()> {
    let pgid: libc::pid_t = libc::pid_t::try_from(pid).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "pid does not fit in the platform pid_t",
        )
    })?;
    let result = unsafe { libc::kill(-pgid, libc::SIGCONT) };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(unix))]
pub(crate) fn continue_background_job(_: u32) -> io::Result<()> {
    Ok(())
}

/// True when a signal failed because the target is already gone.
///
/// A job can exit between the `try_wait` that refreshes its state and
/// the `SIGCONT` that resumes it, and its process group disappears
/// with it. That is not a failure to report — the job simply
/// finished.
#[cfg(unix)]
pub(crate) fn is_missing_process(error: &io::Error) -> bool {
    error.raw_os_error() == Some(libc::ESRCH)
}

#[cfg(not(unix))]
pub(crate) fn is_missing_process(_: &io::Error) -> bool {
    false
}

#[cfg(unix)]
pub(crate) fn send_process_signal(pid: i32, signal: i32) -> io::Result<()> {
    let result = unsafe { libc::kill(pid, signal) };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(unix))]
pub(crate) fn send_process_signal(_: i32, _: i32) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "signals are only supported on unix targets",
    ))
}

#[cfg(unix)]
pub(crate) fn exit_status(status: std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;

    status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or(1))
}

#[cfg(not(unix))]
pub(crate) fn exit_status(status: std::process::ExitStatus) -> i32 {
    status.code().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    #[test]
    fn converts_sigint_into_shell_style_exit_code() {
        let status = std::process::Command::new("/bin/sh")
            .arg("-c")
            .arg("kill -INT $$")
            .status()
            .expect("run shell");

        assert_eq!(super::exit_status(status), 130);
    }

    #[cfg(unix)]
    #[test]
    fn rejects_pid_that_does_not_fit_pid_t() {
        let err = super::continue_background_job(u32::MAX).expect_err("pid must be rejected");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[cfg(unix)]
    #[test]
    fn sigint_guard_can_be_installed_and_dropped_repeatedly() {
        for _ in 0..3 {
            let guard = super::SigintGuard::ignore().expect("install guard");
            drop(guard);
        }
    }
}
