use windows::Win32::{ System::Com::*, UI::Shell::* };

pub fn open_file_dialog() {
    unsafe {
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

        // let selected_file = file_dialog.GetResult();

        CoUninitialize();
    }
}
