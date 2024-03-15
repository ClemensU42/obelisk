use windows::Win32::Storage::FileSystem::GetLogicalDriveStringsW;

#[derive(serde::Serialize)]
pub struct ListDrivesResponse{
    drives: Vec<String>
}

#[tauri::command]
pub fn list_drives() -> ListDrivesResponse{
    let buf_len : u32 = unsafe {GetLogicalDriveStringsW(None)};

    // TODO: add proper error handling
    if(buf_len == 0){return ListDrivesResponse{ drives: vec![] };}

    let buffer: Vec<u16> = Vec::with_capacity((buf_len + 1) as usize);

    ListDrivesResponse{
        drives: vec!["A".parse().unwrap(), "B".parse().unwrap()]
    }
}