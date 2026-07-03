//! Base set of common UI strings translated to all 8 supported languages.
//!
//! Apps can use these via `common_strings::lookup(CommonString::Cancel, language)`
//! instead of defining their own. Apps with custom wording keep their own
//! i18n modules.

use shared_core::i18n::Language;

/// Base set of common UI strings, used as keys for translation lookup.
///
/// Add a variant here, add a translation row in [`lookup`], and apps can
/// immediately use the new key. Keep this enum small — it is the shared
/// base every app can rely on, not an app-specific vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonString {
    Cancel,
    Save,
    Saved,
    Delete,
    Confirm,
    Loading,
    Error,
    Failed,
    Success,
    Close,
    Yes,
    No,
    Back,
    Settings,
    Logout,
    Print,
    Theme,
    Language,
}

/// Look up a common UI string in the given language.
///
/// All 8 languages have entries for every variant, so no fallback path is
/// needed; the match is exhaustive and the function never panics.
#[must_use]
pub fn lookup(key: CommonString, lang: Language) -> &'static str {
    match key {
        CommonString::Cancel => match lang {
            Language::English => "Cancel",
            Language::Chinese => "取消",
            Language::Spanish => "Cancelar",
            Language::German => "Abbrechen",
            Language::Japanese => "キャンセル",
            Language::French => "Annuler",
            Language::Portuguese => "Cancelar",
            Language::Russian => "Отмена",
        },
        CommonString::Save => match lang {
            Language::English => "Save",
            Language::Chinese => "保存",
            Language::Spanish => "Guardar",
            Language::German => "Speichern",
            Language::Japanese => "保存",
            Language::French => "Enregistrer",
            Language::Portuguese => "Salvar",
            Language::Russian => "Сохранить",
        },
        CommonString::Saved => match lang {
            Language::English => "Saved",
            Language::Chinese => "已保存",
            Language::Spanish => "Guardado",
            Language::German => "Gespeichert",
            Language::Japanese => "保存しました",
            Language::French => "Enregistré",
            Language::Portuguese => "Salvo",
            Language::Russian => "Сохранено",
        },
        CommonString::Delete => match lang {
            Language::English => "Delete",
            Language::Chinese => "删除",
            Language::Spanish => "Eliminar",
            Language::German => "Löschen",
            Language::Japanese => "削除",
            Language::French => "Supprimer",
            Language::Portuguese => "Excluir",
            Language::Russian => "Удалить",
        },
        CommonString::Confirm => match lang {
            Language::English => "Confirm",
            Language::Chinese => "确认",
            Language::Spanish => "Confirmar",
            Language::German => "Bestätigen",
            Language::Japanese => "確認",
            Language::French => "Confirmer",
            Language::Portuguese => "Confirmar",
            Language::Russian => "Подтвердить",
        },
        CommonString::Loading => match lang {
            Language::English => "Loading…",
            Language::Chinese => "加载中…",
            Language::Spanish => "Cargando…",
            Language::German => "Laden…",
            Language::Japanese => "読み込み中…",
            Language::French => "Chargement…",
            Language::Portuguese => "Carregando…",
            Language::Russian => "Загрузка…",
        },
        CommonString::Error => match lang {
            Language::English => "Error",
            Language::Chinese => "错误",
            Language::Spanish => "Error",
            Language::German => "Fehler",
            Language::Japanese => "エラー",
            Language::French => "Erreur",
            Language::Portuguese => "Erro",
            Language::Russian => "Ошибка",
        },
        CommonString::Failed => match lang {
            Language::English => "Failed",
            Language::Chinese => "失败",
            Language::Spanish => "Falló",
            Language::German => "Fehlgeschlagen",
            Language::Japanese => "失敗",
            Language::French => "Échec",
            Language::Portuguese => "Falhou",
            Language::Russian => "Не удалось",
        },
        CommonString::Success => match lang {
            Language::English => "Success",
            Language::Chinese => "成功",
            Language::Spanish => "Éxito",
            Language::German => "Erfolg",
            Language::Japanese => "成功",
            Language::French => "Succès",
            Language::Portuguese => "Sucesso",
            Language::Russian => "Успешно",
        },
        CommonString::Close => match lang {
            Language::English => "Close",
            Language::Chinese => "关闭",
            Language::Spanish => "Cerrar",
            Language::German => "Schließen",
            Language::Japanese => "閉じる",
            Language::French => "Fermer",
            Language::Portuguese => "Fechar",
            Language::Russian => "Закрыть",
        },
        CommonString::Yes => match lang {
            Language::English => "Yes",
            Language::Chinese => "是",
            Language::Spanish => "Sí",
            Language::German => "Ja",
            Language::Japanese => "はい",
            Language::French => "Oui",
            Language::Portuguese => "Sim",
            Language::Russian => "Да",
        },
        CommonString::No => match lang {
            Language::English => "No",
            Language::Chinese => "否",
            Language::Spanish => "No",
            Language::German => "Nein",
            Language::Japanese => "いいえ",
            Language::French => "Non",
            Language::Portuguese => "Não",
            Language::Russian => "Нет",
        },
        CommonString::Back => match lang {
            Language::English => "Back",
            Language::Chinese => "返回",
            Language::Spanish => "Atrás",
            Language::German => "Zurück",
            Language::Japanese => "戻る",
            Language::French => "Retour",
            Language::Portuguese => "Voltar",
            Language::Russian => "Назад",
        },
        CommonString::Settings => match lang {
            Language::English => "Settings",
            Language::Chinese => "设置",
            Language::Spanish => "Ajustes",
            Language::German => "Einstellungen",
            Language::Japanese => "設定",
            Language::French => "Paramètres",
            Language::Portuguese => "Configurações",
            Language::Russian => "Настройки",
        },
        CommonString::Logout => match lang {
            Language::English => "Log out",
            Language::Chinese => "退出",
            Language::Spanish => "Cerrar sesión",
            Language::German => "Abmelden",
            Language::Japanese => "ログアウト",
            Language::French => "Déconnexion",
            Language::Portuguese => "Sair",
            Language::Russian => "Выйти",
        },
        CommonString::Print => match lang {
            Language::English => "Print",
            Language::Chinese => "打印",
            Language::Spanish => "Imprimir",
            Language::German => "Drucken",
            Language::Japanese => "印刷",
            Language::French => "Imprimer",
            Language::Portuguese => "Imprimir",
            Language::Russian => "Печать",
        },
        CommonString::Theme => match lang {
            Language::English => "Theme",
            Language::Chinese => "主题",
            Language::Spanish => "Tema",
            Language::German => "Design",
            Language::Japanese => "テーマ",
            Language::French => "Thème",
            Language::Portuguese => "Tema",
            Language::Russian => "Тема",
        },
        CommonString::Language => match lang {
            Language::English => "Language",
            Language::Chinese => "语言",
            Language::Spanish => "Idioma",
            Language::German => "Sprache",
            Language::Japanese => "言語",
            Language::French => "Langue",
            Language::Portuguese => "Idioma",
            Language::Russian => "Язык",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KEYS: &[CommonString] = &[
        CommonString::Cancel,
        CommonString::Save,
        CommonString::Saved,
        CommonString::Delete,
        CommonString::Confirm,
        CommonString::Loading,
        CommonString::Error,
        CommonString::Failed,
        CommonString::Success,
        CommonString::Close,
        CommonString::Yes,
        CommonString::No,
        CommonString::Back,
        CommonString::Settings,
        CommonString::Logout,
        CommonString::Print,
        CommonString::Theme,
        CommonString::Language,
    ];

    #[test]
    fn translations_are_complete() {
        for lang in Language::all() {
            for key in ALL_KEYS {
                let s = lookup(*key, *lang);
                assert!(!s.is_empty(), "{key:?} missing translation for {lang:?}");
            }
        }
    }

    #[test]
    fn translations_match_table() {
        assert_eq!(lookup(CommonString::Cancel, Language::English), "Cancel");
        assert_eq!(lookup(CommonString::Cancel, Language::Chinese), "取消");
        assert_eq!(
            lookup(CommonString::Cancel, Language::Japanese),
            "キャンセル"
        );
        assert_eq!(lookup(CommonString::Save, Language::English), "Save");
        assert_eq!(lookup(CommonString::Save, Language::German), "Speichern");
        assert_eq!(lookup(CommonString::Loading, Language::English), "Loading…");
        assert_eq!(
            lookup(CommonString::Loading, Language::French),
            "Chargement…"
        );
        assert_eq!(lookup(CommonString::Error, Language::English), "Error");
        assert_eq!(lookup(CommonString::Error, Language::Russian), "Ошибка");
        assert_eq!(lookup(CommonString::Yes, Language::German), "Ja");
        assert_eq!(lookup(CommonString::No, Language::German), "Nein");
        assert_eq!(lookup(CommonString::Back, Language::French), "Retour");
        assert_eq!(
            lookup(CommonString::Settings, Language::English),
            "Settings"
        );
        assert_eq!(lookup(CommonString::Logout, Language::English), "Log out");
        assert_eq!(
            lookup(CommonString::Logout, Language::Spanish),
            "Cerrar sesión"
        );
        assert_eq!(lookup(CommonString::Print, Language::English), "Print");
        assert_eq!(lookup(CommonString::Print, Language::Japanese), "印刷");
        assert_eq!(lookup(CommonString::Theme, Language::English), "Theme");
        assert_eq!(
            lookup(CommonString::Language, Language::English),
            "Language"
        );
        assert_eq!(lookup(CommonString::Language, Language::Chinese), "语言");
    }
}
