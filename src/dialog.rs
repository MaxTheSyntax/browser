use windows::Win32::{ System::Com::*, UI::Shell::* };

pub fn open_file_dialog() {
    unsafe {
        // Initialize the COM library
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        if hr.is_err() {
            println!("Failed to initialize COM library: 0x{:x}", hr.0);
            return;
        }

        // Create the file open dialog
        let hr: IFileOpenDialog = CoCreateInstance(
            &FileOpenDialog,
            None,
            CLSCTX_INPROC_SERVER
        ).expect("Failed to create FileOpenDialog");

        // Show the file open dialog
        let hr = hr.Show(None);
        if let Err(e) = hr {
            println!("Failed to select file: {:?}", e);
            return;
        }

        CoUninitialize();
    }
}
