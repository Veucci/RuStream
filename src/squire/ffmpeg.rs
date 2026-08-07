use std::path::Path;
use std::process::Command;

use crate::constant;

/// Reads the duration of a media file in seconds using `ffprobe`.
///
/// This is invoked on-demand per file while rendering the listing page and
/// the process is terminated right after the value is extracted.
///
/// # Arguments
///
/// * `filepath` - Path to the media file.
///
/// # Returns
///
/// Returns `Some(f64)` seconds if the probe succeeded, `None` otherwise.
pub fn get_duration(filepath: &Path) -> Option<f64> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(filepath)
        .output()
        .ok()?;
    if !output.status.success() {
        log::warn!("ffprobe failed for {:?}", filepath);
        return None;
    }
    let duration = String::from_utf8_lossy(&output.stdout).trim().to_string();
    match duration.parse::<f64>() {
        Ok(seconds) => Some(seconds),
        Err(_) => {
            log::warn!("ffprobe returned an invalid duration '{}' for {:?}", duration, filepath);
            None
        }
    }
}

/// Validates the target format requested for conversion.
///
/// # Arguments
///
/// * `current_format` - Extension of the file being converted.
/// * `target_format` - Extension requested by the user.
///
/// # Returns
///
/// * `Ok(String)` - Lowercased target format if it is valid.
/// * `Err(String)` - Reason for the rejection.
fn validate_format(current_format: &str, target_format: &str) -> Result<String, String> {
    let target = target_format.trim().to_lowercase();
    if !constant::VIDEO_FORMATS.contains(&target.as_str()) {
        return Err(format!("'{}' is not a supported video format", target_format));
    }
    if target == current_format {
        return Err(format!("'{}' is already in the requested format", current_format));
    }
    Ok(target)
}

/// Converts a media file to the requested format using `ffmpeg`.
///
/// ffmpeg is only spawned for this request and the process terminates as soon
/// as the conversion completes. The converted file is written next to the
/// original file and the original is NEVER removed.
///
/// # Arguments
///
/// * `filepath` - Path to the media file that has to be converted.
/// * `target_format` - Format the file should be converted into.
///
/// # Returns
///
/// * `Ok(String)` - Path of the converted file on success.
/// * `Err(String)` - Reason for the failure.
pub fn convert(filepath: &Path, target_format: &str) -> Result<String, String> {
    if !filepath.is_file() {
        return Err(format!("{:?} is not a valid file entry", filepath));
    }
    let current_format = match filepath.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_lowercase(),
        None => return Err(format!("{:?} has no file extension", filepath)),
    };
    if !constant::VIDEO_FORMATS.contains(&current_format.as_str()) {
        return Err(format!("'{}' is not a convertible video format", current_format));
    }
    let target = validate_format(&current_format, target_format)?;
    let output_path = filepath.with_extension(&target);
    if output_path.exists() {
        return Err(format!(
            "Target file already exists: '{}'. Delete it first, the original file is never overwritten",
            output_path.to_string_lossy()
        ));
    }

    log::info!("Converting {:?} to {}", filepath, target);
    let status = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-y", "-i"])
        .arg(filepath)
        .arg(&output_path)
        .status();
    match status {
        Ok(exit) if exit.success() => {
            log::info!("Converted {:?} to {:?}", filepath, output_path);
            Ok(output_path.to_string_lossy().to_string())
        }
        Ok(exit) => {
            let reason = format!("ffmpeg exited with status '{}' while converting to '{}'", exit, target);
            log::error!("{}", reason);
            Err(reason)
        }
        Err(error) => {
            let reason = format!("Unable to invoke ffmpeg: {}", error);
            log::error!("{}", reason);
            Err(reason)
        }
    }
}
