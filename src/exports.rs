#![allow(non_snake_case)]

#[unsafe(no_mangle)] //winmm.dll
fn DefDriverProc() {}
#[unsafe(no_mangle)] //winmm.dll
fn DriverCallback() {}
#[unsafe(no_mangle)] //winmm.dll
fn DrvGetModuleHandle() {}
#[unsafe(no_mangle)] //winmm.dll
fn GetDriverModuleHandle() {}
#[unsafe(no_mangle)] //winmm.dll
fn OpenDriver() {}
#[unsafe(no_mangle)] //winmm.dll
fn PlaySound() {}
#[unsafe(no_mangle)] //winmm.dll
fn PlaySoundA() {}
#[unsafe(no_mangle)] //winmm.dll
fn PlaySoundW() {}
#[unsafe(no_mangle)] //winmm.dll
fn SendDriverMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn WOWAppExit() {}
#[unsafe(no_mangle)] //winmm.dll
fn auxGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn auxGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn auxGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn auxGetVolume() {}
#[unsafe(no_mangle)] //winmm.dll
fn auxOutMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn auxSetVolume() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyConfigChanged() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyGetPos() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyGetPosEx() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyGetThreshold() {}
#[unsafe(no_mangle)] //winmm.dll
fn joyReleaseCapture() {}
#[unsafe(no_mangle)] //winmm.dll
fn joySetCapture() {}
#[unsafe(no_mangle)] //winmm.dll
fn joySetThreshold() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciDriverNotify() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciDriverYield() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciExecute() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciFreeCommandResource() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetCreatorTask() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetDeviceIDA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetDeviceIDFromElementIDA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetDeviceIDFromElementIDW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetDeviceIDW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetDriverData() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetErrorStringA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetErrorStringW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciGetYieldProc() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciLoadCommandResource() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciSendCommandA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciSendCommandW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciSendStringA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciSendStringW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciSetDriverData() {}
#[unsafe(no_mangle)] //winmm.dll
fn mciSetYieldProc() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiConnect() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiDisconnect() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInAddBuffer() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInGetErrorTextA() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInGetErrorTextW() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInGetID() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInOpen() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInPrepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInReset() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInStart() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInStop() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiInUnprepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutCacheDrumPatches() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutCachePatches() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetErrorTextA() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetErrorTextW() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetID() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutGetVolume() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutLongMsg() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutOpen() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutPrepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutReset() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutSetVolume() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutShortMsg() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiOutUnprepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamOpen() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamOut() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamPause() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamPosition() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamProperty() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamRestart() {}
#[unsafe(no_mangle)] //winmm.dll
fn midiStreamStop() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetControlDetailsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetControlDetailsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetID() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetLineControlsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetLineControlsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetLineInfoA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetLineInfoW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerOpen() {}
#[unsafe(no_mangle)] //winmm.dll
fn mixerSetControlDetails() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmDrvInstall() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmGetCurrentTask() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmTaskBlock() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmTaskCreate() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmTaskSignal() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmTaskYield() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioAdvance() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioAscend() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioCreateChunk() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioDescend() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioFlush() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioGetInfo() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioInstallIOProcA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioInstallIOProcW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioOpenA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioOpenW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioRead() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioRenameA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioRenameW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioSeek() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioSendMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioSetBuffer() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioSetInfo() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioStringToFOURCCA() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioStringToFOURCCW() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmioWrite() {}
#[unsafe(no_mangle)] //winmm.dll
fn mmsystemGetVersion() {}
#[unsafe(no_mangle)] //winmm.dll
fn sndPlaySoundA() {}
#[unsafe(no_mangle)] //winmm.dll
fn sndPlaySoundW() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeBeginPeriod() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeEndPeriod() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeGetDevCaps() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeGetSystemTime() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeGetTime() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeKillEvent() {}
#[unsafe(no_mangle)] //winmm.dll
fn timeSetEvent() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInAddBuffer() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetErrorTextA() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetErrorTextW() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetID() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInGetPosition() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInOpen() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInPrepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInReset() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInStart() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInStop() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveInUnprepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutBreakLoop() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutClose() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetDevCapsA() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetDevCapsW() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetErrorTextA() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetErrorTextW() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetID() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetNumDevs() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetPitch() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetPlaybackRate() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetPosition() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutGetVolume() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutMessage() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutOpen() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutPause() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutPrepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutReset() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutRestart() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutSetPitch() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutSetPlaybackRate() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutSetVolume() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutUnprepareHeader() {}
#[unsafe(no_mangle)] //winmm.dll
fn waveOutWrite() {}
use winapi::ctypes::c_void;
use winapi::shared::minwindef::UINT;
use winapi::shared::ntdef::LONG;
use winapi::um::libloaderapi::GetProcAddress;

#[unsafe(no_mangle)]
pub extern "system" fn CloseDriver(hDriver: *mut c_void, lParam1: LONG, lParam2: LONG) -> UINT {
    unsafe {
        let func = GetProcAddress(super::REAL_WINMM, b"CloseDriver\0".as_ptr() as *const i8);
        if !func.is_null() {
            let func: extern "system" fn(*mut c_void, LONG, LONG) -> UINT =
                std::mem::transmute(func);
            func(hDriver, lParam1, lParam2)
        } else {
            0
        }
    }
}
