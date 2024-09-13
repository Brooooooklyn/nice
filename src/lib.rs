#![deny(clippy::all)]

use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi]
pub enum WindowsThreadPriority {
  ThreadModeBackgroundBegin = 0x00010000,
  ThreadModeBackgroundEnd = 0x00020000,
  ThreadPriorityAboveNormal = 1,
  ThreadPriorityBelowNormal = -1,
  ThreadPriorityHighest = 2,
  ThreadPriorityIdle = -15,
  ThreadPriorityLowest = -2,
  ThreadPriorityNormal = 0,
  ThreadPriorityTimeCritical = 15,
}

impl TryFrom<i32> for WindowsThreadPriority {
  type Error = Error;

  fn try_from(value: i32) -> Result<Self> {
    match value {
      0x00010000 => Ok(Self::ThreadModeBackgroundBegin),
      0x00020000 => Ok(Self::ThreadModeBackgroundEnd),
      1 => Ok(Self::ThreadPriorityAboveNormal),
      -1 => Ok(Self::ThreadPriorityBelowNormal),
      2 => Ok(Self::ThreadPriorityHighest),
      -15 => Ok(Self::ThreadPriorityIdle),
      -2 => Ok(Self::ThreadPriorityLowest),
      0 => Ok(Self::ThreadPriorityNormal),
      15 => Ok(Self::ThreadPriorityTimeCritical),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("{value} is not a valid priority on Windows"),
      )),
    }
  }
}

#[napi]
/// This function set the priority of the current process.
/// On Unix, it uses the [`nice`](https://linux.die.net/man/2/nice) function.
///
/// On Windows, it uses the [`SetThreadPriority`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) function.
pub fn nice(incr: i32) -> Result<i32> {
  #[cfg(unix)]
  unsafe {
    let ret = libc::nice(incr);
    if ret == -1 {
      return Err(std::io::Error::last_os_error().into());
    }
    Ok(ret)
  }
  #[cfg(windows)]
  {
    use windows::Win32::System::Threading::{GetCurrentThread, SetThreadPriority, THREAD_PRIORITY};

    impl From<WindowsThreadPriority> for THREAD_PRIORITY {
      fn from(priority: WindowsThreadPriority) -> Self {
        match priority {
          WindowsThreadPriority::ThreadModeBackgroundBegin => {
            windows::Win32::System::Threading::THREAD_MODE_BACKGROUND_BEGIN
          }
          WindowsThreadPriority::ThreadModeBackgroundEnd => {
            windows::Win32::System::Threading::THREAD_MODE_BACKGROUND_END
          }
          WindowsThreadPriority::ThreadPriorityAboveNormal => {
            windows::Win32::System::Threading::THREAD_PRIORITY_ABOVE_NORMAL
          }
          WindowsThreadPriority::ThreadPriorityBelowNormal => {
            windows::Win32::System::Threading::THREAD_PRIORITY_BELOW_NORMAL
          }
          WindowsThreadPriority::ThreadPriorityHighest => {
            windows::Win32::System::Threading::THREAD_PRIORITY_HIGHEST
          }
          WindowsThreadPriority::ThreadPriorityIdle => {
            windows::Win32::System::Threading::THREAD_PRIORITY_IDLE
          }
          WindowsThreadPriority::ThreadPriorityLowest => {
            windows::Win32::System::Threading::THREAD_PRIORITY_LOWEST
          }
          WindowsThreadPriority::ThreadPriorityNormal => {
            windows::Win32::System::Threading::THREAD_PRIORITY_NORMAL
          }
          WindowsThreadPriority::ThreadPriorityTimeCritical => {
            windows::Win32::System::Threading::THREAD_PRIORITY_TIME_CRITICAL
          }
        }
      }
    }

    let current_thread = unsafe { GetCurrentThread() };
    let priority: WindowsThreadPriority = incr.try_into()?;
    unsafe { SetThreadPriority(current_thread, priority.into()) }
      .map_err(|e| Error::new(Status::GenericFailure, e.message().to_string()))?;
    Ok(priority as i32)
  }
}

#[napi]
/// This function get the priority of the current process.
/// On Unix, it uses the [`getpriority(2)`](https://linux.die.net/man/2/getpriority).
///
/// On Windows, it uses the [`GetThreadPriority`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority) function.
///
/// | Priority Constant                  | Value     | Description                                                                                                                                                                                                                       |
/// |------------------------------------|-----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | THREAD_MODE_BACKGROUND_BEGIN       | 0x00010000| Begin background processing mode. The system lowers the resource scheduling priorities of the thread so that it can perform background work without significantly affecting activity in the foreground.                              |
/// |                                    |           | This value can be specified only if hThread is a handle to the current thread. The function fails if the thread is already in background processing mode.                                                                           |
/// |                                    |           | Windows Server 2003: This value is not supported.                                                                                                                                                                                  |
/// | THREAD_MODE_BACKGROUND_END         | 0x00020000| End background processing mode. The system restores the resource scheduling priorities of the thread as they were before the thread entered background processing mode.                                                            |
/// |                                    |           | This value can be specified only if hThread is a handle to the current thread. The function fails if the thread is not in background processing mode.                                                                               |
/// |                                    |           | Windows Server 2003: This value is not supported.                                                                                                                                                                                  |
/// | THREAD_PRIORITY_ABOVE_NORMAL       | 1         | Priority 1 point above the priority class.                                                                                                                                                                                         |
/// | THREAD_PRIORITY_BELOW_NORMAL       | -1        | Priority 1 point below the priority class.                                                                                                                                                                                         |
/// | THREAD_PRIORITY_HIGHEST            | 2         | Priority 2 points above the priority class.                                                                                                                                                                                        |
/// | THREAD_PRIORITY_IDLE               | -15       | Base priority of 1 for IDLE_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, ABOVE_NORMAL_PRIORITY_CLASS, or HIGH_PRIORITY_CLASS processes, and a base priority of 16 for REALTIME_PRIORITY_CLASS processes.      |
/// | THREAD_PRIORITY_LOWEST             | -2        | Priority 2 points below the priority class.                                                                                                                                                                                        |
/// | THREAD_PRIORITY_NORMAL             | 0         | Normal priority for the priority class.                                                                                                                                                                                            |
/// | THREAD_PRIORITY_TIME_CRITICAL      | 15        | Base priority of 15 for IDLE_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, ABOVE_NORMAL_PRIORITY_CLASS, or HIGH_PRIORITY_CLASS processes, and a base priority of 31 for REALTIME_PRIORITY_CLASS processes.     |
pub fn get_current_process_priority() -> Result<i32> {
  #[cfg(unix)]
  unsafe {
    let ret = libc::getpriority(libc::PRIO_PROCESS, 0);
    let os_error = std::io::Error::last_os_error();
    if let Some(err) = os_error.raw_os_error() {
      if err != 0 {
        return Err(os_error.into());
      }
    };
    Ok(ret)
  }
  #[cfg(windows)]
  {
    use windows::Win32::System::Threading::{GetCurrentThread, GetThreadPriority};
    use windows::Win32::System::WindowsProgramming::THREAD_PRIORITY_ERROR_RETURN;

    let ret = unsafe { GetThreadPriority(GetCurrentThread()) };

    if ret == THREAD_PRIORITY_ERROR_RETURN as i32 {
      return Err(std::io::Error::last_os_error().into());
    }

    Ok(ret)
  }
}
