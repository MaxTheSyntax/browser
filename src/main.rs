use windows::{
    core::PCWSTR,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

// Window procedure function to handle events
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    unsafe {
        match msg {
            // Handle window close event
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            // Handle window destroy event
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                fn LOWORD(l: u32) -> u16 {
                    l as u16
                }
                fn HIWORD(l: u32) -> u16 {
                    (l >> 16) as u16
                }

                let width = LOWORD(lparam.0 as u32);
                let height = HIWORD(lparam.0 as u32);
                println!("Window resized to {}x{}", width, height);
                LRESULT(0)
            }
            // Default event handling
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() {
    unsafe {
        // Get a handle to the current module instance
        let instance = GetModuleHandleW(None).unwrap();
        let class_name = "RustWindowClass\0".encode_utf16().collect::<Vec<u16>>();

        // Define the window class structure
        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(window_proc), // Set the window procedure callback
            hInstance: HINSTANCE(instance.0), // Convert HMODULE to HINSTANCE
            lpszClassName: PCWSTR::from_raw(class_name.as_ptr()), // Set class name
            ..Default::default()
        };

        // Register the window class
        RegisterClassW(&wnd_class);

        // Create the window
        let _hwnd = CreateWindowExW(
            Default::default(), // No extended styles
            PCWSTR::from_raw(class_name.as_ptr()), // Window class name
            PCWSTR::from_raw("Rust UI\0".encode_utf16().collect::<Vec<u16>>().as_ptr()), // Window title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, // Window style
            CW_USEDEFAULT, // Default X position
            CW_USEDEFAULT, // Default Y position
            500, // Window width
            300, // Window height
            None, // No parent window
            None, // No menu
            Some(HINSTANCE(instance.0)), // Instance handle
            None // No additional parameters
        );

        // Message loop to process events
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg); // Translate keyboard input
            DispatchMessageW(&msg); // Dispatch message to window procedure
        }
    }
}
