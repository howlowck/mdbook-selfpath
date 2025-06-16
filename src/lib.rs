use mdbook::BookItem;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;

#[derive(Default)]
pub struct SelfPathPreprocessor;

impl SelfPathPreprocessor {
    pub fn new() -> SelfPathPreprocessor {
        SelfPathPreprocessor
    }
}

impl Preprocessor for SelfPathPreprocessor {
    fn name(&self) -> &str {
        "selfpath"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // Determine the include-file-ext setting from [preprocessor.selfpath] (default true)
        let include_ext = ctx
            .config
            .get_preprocessor(self.name())
            .and_then(|table| table.get("include-file-ext"))
            .and_then(|val| val.as_bool())
            .unwrap_or(true);

        // Determine the source directory name (relative to book root) from config or default "src"
        let src_dir_name = &ctx.config.book.src;

        // Regex to match {{#selfpath}}, {{ #selfpath }}, etc.
        let selfpath_re = Regex::new(r"\{\{\s*selfpath\s*\}\}").unwrap();
        let selftitle_re = Regex::new(r"\{\{\s*selftitle\s*\}\}").unwrap();

        // Iterate through all chapters in the book (including subchapters)
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(chapter) = item {
                // Only process real chapters that have a source path
                if let Some(ref source_path) = chapter.source_path {
                    let mut rel_path = PathBuf::new();
                    // push book src path only if it's not "."
                    if src_dir_name.as_path() != Path::new(".") {
                        rel_path.push(&src_dir_name)
                    }
                    rel_path.push(source_path);
                    if !include_ext {
                        // Remove the .md extension if present
                        rel_path.set_extension("");
                    }
                    // Convert path to a forward-slash string (for consistency across OS)
                    let rel_path_str = rel_path.to_string_lossy().replace('\\', "/");
                    // Perform the replacement in the chapter content using regex
                    chapter.content = selfpath_re.replace_all(&chapter.content, rel_path_str.as_str()).to_string();

                    let filename = source_path.file_name().and_then(|s| s.to_str()).unwrap_or("noname");
                    let without_ext = filename
                        .strip_suffix(".md")
                        .unwrap_or(filename);
                    chapter.content = selftitle_re.replace_all(&chapter.content, without_ext).to_string();
                }
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        // This is not strictly used in our setup, since main.rs handles "supports".
        // We claim support for all renderers.
        true
    }
}
