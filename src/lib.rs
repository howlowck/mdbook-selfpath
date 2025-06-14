use mdbook::BookItem;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
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
        // log to console src_dir_name
        eprintln!("Using source directory name: {:?}", src_dir_name.to_str());

        let book_title = ctx
            .config
            .book
            .title
            .clone()
            .unwrap_or_else(|| "Untitled Book".to_string());
        eprintln!("Processing book: {}", book_title);

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
                    // Perform the replacement in the chapter content
                    chapter.content = chapter.content.replace("{{ #selfpath }}", &rel_path_str);
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
