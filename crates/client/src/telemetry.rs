use clock::SystemClock;
use gpui::App;
use http_client::HttpClientWithUrl;
#[cfg(all(target_os = "macos", not(feature = "test-support")))]
use regex::Regex;
use release_channel::ReleaseChannel;
use std::sync::Arc;
#[cfg(all(target_os = "macos", not(feature = "test-support")))]
use std::sync::LazyLock;

// The shared client still asks for optional identity metadata. Keeping these
// empty hooks avoids changing its protocol while removing collection and upload.
pub struct Telemetry;

impl Telemetry {
    pub fn new(_: Arc<dyn SystemClock>, _: Arc<HttpClientWithUrl>, _: &mut App) -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn set_authenticated_user_info(&self, _: Option<String>, _: bool) {}

    pub fn system_id(&self) -> Option<Arc<str>> {
        None
    }
    pub fn metrics_id(&self) -> Option<Arc<str>> {
        None
    }
}

pub fn should_install_crash_handler(_: ReleaseChannel) -> bool {
    matches!(
        std::env::var("ZED_GENERATE_MINIDUMPS").as_deref(),
        Ok("true" | "1")
    )
}

pub fn os_name() -> String {
    #[cfg(target_os = "macos")]
    {
        "macOS".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        format!("Linux {}", gpui::guess_compositor())
    }
    #[cfg(target_os = "freebsd")]
    {
        format!("FreeBSD {}", gpui::guess_compositor())
    }

    #[cfg(target_os = "windows")]
    {
        "Windows".to_string()
    }
}

/// Note: This might do blocking IO! Only call from background threads
pub fn os_version() -> String {
    cfg_select! {
       feature = "test-support" => {
           // MacOS branch in particular is quite slow, hence we ought to "avoid" it in tests.
           "test binary".to_owned()
       }
       target_os = "macos" => {
           static MACOS_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
               Regex::new(r"(\s*\(Build [^)]*[0-9]\))").unwrap()
           });
           use objc2_foundation::NSProcessInfo;
           let process_info = NSProcessInfo::processInfo();
           let version_nsstring = process_info.operatingSystemVersionString();
           // "Version 15.6.1 (Build 24G90)" -> "15.6.1 (Build 24G90)"
           let version_string = version_nsstring.to_string().replace("Version ", "");
           // "15.6.1 (Build 24G90)" -> "15.6.1"
           // "26.0.0 (Build 25A5349a)" -> unchanged (Beta or Rapid Security Response; ends with letter)
           MACOS_VERSION_REGEX
               .replace_all(&version_string, "")
               .to_string()
       }
       any(target_os = "linux", target_os = "freebsd") => {
           use std::path::Path;

           let content = if let Ok(file) = std::fs::read_to_string(&Path::new("/etc/os-release")) {
               file
           } else if let Ok(file) = std::fs::read_to_string(&Path::new("/usr/lib/os-release")) {
               file
           } else if let Ok(file) = std::fs::read_to_string(&Path::new("/var/run/os-release")) {
               file
           } else {
               log::error!(
                   "Failed to load /etc/os-release, /usr/lib/os-release, or /var/run/os-release"
               );
               "".to_string()
           };
           util::parse_os_release(&content).unwrap_or_else(|| "unknown".to_string())
       }
       target_os = "windows" => {
           let mut info = unsafe { std::mem::zeroed() };
           let status = unsafe { windows::Wdk::System::SystemServices::RtlGetVersion(&mut info) };
           if status.is_ok() {
               semver::Version::new(
                   info.dwMajorVersion as _,
                   info.dwMinorVersion as _,
                   info.dwBuildNumber as _,
               )
               .to_string()
           } else {
               "unknown".to_string()
           }
       }
    }
}
