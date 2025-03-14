use windows::{ core::{ w, PWSTR }, Win32::{ System::Com::*, UI::Shell::*, UI::Shell::Common::* } };
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

        // WARNING: Don't uncomment this line, it will cause a double free error (It's freed later on)
        // Free the memory allocated by CoTaskMemAlloc
        // CoTaskMemFree(Some(nt_string.0 as *mut _));

        return string;
    }
}

pub fn open_file_dialog() {
    unsafe {
        println!("Opening file dialog");

        // INFO: Code is commented out cuz im pretty sure the windows crate automatically initializes COM
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

        // Set the dialog options
        let filters = [
            COMDLG_FILTERSPEC {
                pszName: w!("Text Files"),
                pszSpec: w!("*.txt"),
            },
            COMDLG_FILTERSPEC {
                pszName: w!("All Files"),
                pszSpec: w!("*.*"),
            },
        ];
        file_dialog.SetFileTypes(&filters).expect("Failed to set file types");
        file_dialog.SetFileTypeIndex(1).expect("Failed to set file type index"); // Select default filter, in this case "Text Files"

        // Show the file open dialog
        if let Err(e) = file_dialog.Show(None) {
            println!("Failed to select file: {:?}", e);
            return;
        }

        // Get the selected file
        let selected_file = match file_dialog.GetResult() {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to get selected file: {:?}", e);
                return;
            }
        };

        // Get file path
        let file_path = match selected_file.GetDisplayName(SIGDN_FILESYSPATH) {
            Ok(path) => path,
            Err(e) => {
                println!("Failed to get file path: {:?}", e);
                return;
            }
        };

        /*  The `.clone()` call is needed here to avoid moving file_path
            because convert_nt_utf16_string_to_string takes ownership of the pointer.
            Cloning preserves the original 'file_path' pointer for later use in MessageBoxW. */
        let file_path_string = convert_nt_utf16_string_to_string(file_path.clone());
        println!("Selected file: {}", file_path_string);

        // Show MessageBox
        // Notice that `file_path_str` isn't being used here, as it's a Rust String and MessageBoxW expects a wide string
        // `w!` is a macro that converts a string literal to a wide string, making it compatible with Windows's API
        MessageBoxW(None, file_path, w!("file"), MB_OK);

        CoTaskMemFree(Some(file_path.0 as *mut _));
        // COM objects are automatically released when they go out of scope, so in this case selected_file, file_dialog and other variables are released automatically.
        // The Rust implementation of the Windows API takes care of memory managment (I think) | OLD COMMENT: // `file_path` is of type `PWSTR` and is not a COM object, so it needs to be freed manually.

        // Check the commented `CoInitializeEx` for info on why this is commented out
        // CoUninitialize();
    }
}
