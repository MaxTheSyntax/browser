use windows::{ core::{ w, PWSTR }, Win32::{ System::Com::*, UI::Shell::*, UI::Shell::Common::* } };
use windows::Win32::UI::WindowsAndMessaging::{ MessageBoxW, MB_OK };
use crate::win_types::safe_mem_release;

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

        return string;
    }
}

pub fn open_file_dialog() -> Result<(), String> {
    unsafe {
        println!("Opening file dialog");

        // INFO: Code is commented out cuz im pretty sure the windows crate automatically initializes COM
        // Initialize the COM library
        // CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)
        //     .ok()
        //     .expect("Failed to initialize COM library");

        let file_dialog: IFileOpenDialog = match
            CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)
        {
            Ok(dialog) => dialog,
            Err(e) => {
                return Err(format!("Failed to create FileOpenDialog: {:?}", e));
            }
        };

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
            return Err(format!("Failed to select file: {:?}", e));
        }

        // Get the selected file
        let selected_file = match file_dialog.GetResult() {
            Ok(file) => file,
            Err(e) => {
                return Err(format!("Failed to get selected file: {:?}", e));
            }
        };

        // Get file path
        let mut file_path = match selected_file.GetDisplayName(SIGDN_FILESYSPATH) {
            Ok(path) => path,
            Err(e) => {
                return Err(format!("Failed to get file path: {:?}", e));
            }
        };

        let file_path_string = convert_nt_utf16_string_to_string(file_path);
        println!("Selected file: {}", file_path_string);

        // Show MessageBox
        // Convert file_path_string to a wide string using encode_utf16 and pass it to MessageBoxW
        let file_path_wide: Vec<u16> = file_path_string
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        MessageBoxW(None, PWSTR(file_path_wide.as_ptr() as *mut _), w!("file"), MB_OK);
        // COM objects are automatically released when they go out of scope.
        // Instead of making file_path mutable, bind the inner pointer to a mutable variable and release it.
        safe_mem_release(&mut file_path.0);

        // Check the commented `CoInitializeEx` for info on why this is commented out
        // CoUninitialize();

        return Ok(());
    }
}
