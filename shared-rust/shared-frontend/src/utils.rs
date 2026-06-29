/// Auto-copies highlighted text from the current selection.
/// Returns Some(copied_text) if text was successfully copied, otherwise None.
pub fn copy_selection_to_clipboard() -> Option<String> {
    let window = web_sys::window()?;
    let selection = window.get_selection().ok()??;
    let selected_text = String::from(selection.to_string());
    
    if selected_text.trim().is_empty() {
        return None;
    }
    
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    
    // Write text to clipboard
    let _ = clipboard.write_text(&selected_text);
    
    Some(selected_text)
}
