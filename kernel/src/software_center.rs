//! Static VGA-text-mode software-center screen; this is a catalog preview only.

pub const fn text_screen() -> &'static [&'static str; 25] {
    &[
        "==============================================================================",
        " NOVA SOFTWARE CENTER                                      PREVIEW / OFFLINE",
        " Discover useful apps for your Nova desktop",
        "------------------------------------------------------------------------------",
        "  SEARCH  [ Search apps and categories...                                  ]",
        "------------------------------------------------------------------------------",
        "  BROWSE             FEATURED",
        "  > Discover         +----------------------+  +----------------------+",
        "    Graphics         | PIXEL STUDIO          |  | ORBIT BROWSER        |",
        "    Utilities        | Graphics / Creativity |  | Internet / Web       |",
        "    Development      | Illustration toolkit  |  | Fast, private browsing|",
        "    Education        | [ PREVIEW ]           |  | [ PREVIEW ]           |",
        "                      +----------------------+  +----------------------+",
        "  INSTALLED          POPULAR",
        "  No installed apps  +----------------------+  +----------------------+",
        "                     | NOVA TERMINAL         |  | NOTEBOOK             |",
        "                     | Development           |  | Productivity         |",
        "                     | Command-line workspace|  | Notes and snippets   |",
        "                     | [ PREVIEW ]           |  | [ PREVIEW ]           |",
        "                     +----------------------+  +----------------------+",
        "------------------------------------------------------------------------------",
        " INSTALLATION NOT AVAILABLE: filesystem, signatures and app runtime missing.",
        " Catalog entries are demo content. No packages are downloaded or installed.",
        " Display: VGA text mode | Input: keyboard/mouse drivers unavailable",
        "==============================================================================",
    ]
}
