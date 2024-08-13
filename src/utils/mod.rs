pub mod colors;
pub mod dialoguer_theme;
pub mod lte;

pub fn is_valid_pkg_name(project_name: &str) -> bool {
    let mut chars = project_name.chars().peekable();
    !project_name.is_empty()
        && !chars.peek().map(|c| c.is_ascii_digit()).unwrap_or_default()
        && !chars.any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_') || ch.is_uppercase())
}

pub fn to_valid_pkg_name(project_name: &str) -> String {
    let ret = project_name
        .trim()
        .to_lowercase()
        .replace([':', ';', ' ', '~'], "-")
        .replace(['.', '\\', '/'], "");

    let ret = ret
        .chars()
        .skip_while(|ch| ch.is_ascii_digit() || *ch == '-')
        .collect::<String>();

    if ret.is_empty() {
        "tauri-app".to_string()
    } else {
        ret
    }
}

#[inline]
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for (s, c) in s.chars().enumerate() {
        if s == 0 {
            result.push(c.to_ascii_uppercase());
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else if ['_', '-'].contains(&c) {
            capitalize_next = true;
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valiadtes_pkg_name() {
        assert!(is_valid_pkg_name("tauri-app"));
        assert!(is_valid_pkg_name("tauri_app"));
        assert!(is_valid_pkg_name("t2auriapp"));
        assert!(!is_valid_pkg_name("1tauriapp"));
        assert!(!is_valid_pkg_name("tauri app"));
        assert!(!is_valid_pkg_name("tauri:app"));
        assert!(!is_valid_pkg_name("tauri.app"));
        assert!(!is_valid_pkg_name("tauri/app"));
        assert!(!is_valid_pkg_name("tauri\\app"));
        assert!(!is_valid_pkg_name("tauri~app"));
        assert!(!is_valid_pkg_name("Tauriapp"));
    }

    #[test]
    fn converts_to_valid_pkg_name() {
        assert_eq!(to_valid_pkg_name("tauri-app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri_app"), "tauri_app");
        assert_eq!(to_valid_pkg_name("t2auriapp"), "t2auriapp");
        assert_eq!(to_valid_pkg_name("1tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("123tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("123-tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri:app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri;app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri/app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri\\app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri~app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("-tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("-123tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("-2-123tau2ri-app-2"), "tau2ri-app-2");
        assert_eq!(to_valid_pkg_name("1-2-3tau2ri-app2-"), "tau2ri-app2-");
        assert_eq!(to_valid_pkg_name("Tauriapp"), "tauriapp");
    }

    #[test]
    fn converts_to_pascal_case() {
        assert_eq!(to_pascal_case("tauri-app"), "TauriApp");
        assert_eq!(to_pascal_case("tau2ri-app"), "Tau2riApp");
    }
}
