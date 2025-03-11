// use std::{ slice, ffi::OsString, os::windows::ffi::OsStringExt };
use windows::{ core::{ w, PWSTR }, Win32::{ System::Com::*, UI::Shell::* } };
use windows::Win32::UI::WindowsAndMessaging::{ MessageBoxW, MB_OK };

// Convert a null-terminated UTF-16 string to a Rust String
fn convert_nt_utf16_string_to_string(nt_string: PWSTR) -> String {
    unsafe {
        // Calculate the length of the string
        let mut length = 0;
        while *nt_string.0.offset(length) != 0 {
            length += 1;
        }

        let slice = std::slice::from_raw_parts(nt_string.0, length as usize);
        let string = String::from_utf16_lossy(slice);

        // Free the memory allocated by CoTaskMemAlloc
        windows::Win32::System::Com::CoTaskMemFree(Some(nt_string.0 as *mut _));

        return string;
    }
}

pub fn open_file_dialog() {
    unsafe {
        println!("Opening file dialog");

        // Initialize the COM library
        // CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)
        //     .ok()
        //     .expect("Failed to initialize COM library");

        // Create the file open dialog
        let file_dialog: IFileOpenDialog = CoCreateInstance(
            &FileOpenDialog,
            None,
            CLSCTX_INPROC_SERVER
        ).expect("Failed to create FileOpenDialog");

        // Show the file open dialog
        if let Err(e) = file_dialog.Show(None) {
            println!("Failed to select file: {:?}", e);
            return;
        }

        // Get the selected file
        let selected_file = file_dialog.GetResult();
        if let Err(e) = selected_file {
            println!("Failed to get selected file: {:?}", e);
            return;
        }

        // Get file path
        let file_path = selected_file.unwrap().GetDisplayName(SIGDN_FILESYSPATH);
        if let Err(e) = file_path {
            println!("Failed to get file name: {:?}", e);
            return;
        }

        let file_path_ptr = file_path.unwrap();

        /*  The .clone() call is needed here to avoid moving file_path_ptr
            because convert_nt_utf16_string_to_string takes ownership of the pointer.
            Cloning preserves the original file_path_ptr for later use in MessageBoxW. */
        let file_path_string = convert_nt_utf16_string_to_string(file_path_ptr.clone());
        println!("Selected file: {}", file_path_string);

        // Show MessageBox
        // Notice that `file_path_str` isn't being used here
        // `w!` is a macro that converts a string literal to a wide string, making it more compatible with windows APIs
        MessageBoxW(None, file_path_ptr, w!("file"), MB_OK);

        CoUninitialize();
    }
}
