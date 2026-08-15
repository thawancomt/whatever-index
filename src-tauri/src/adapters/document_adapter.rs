use crate::{
    adapters::{textable_adapters::TextableAdapter, traits::Adapter},
    db::types::file::File,
    paths::get_resource_dir,
};
use quick_xml::{events::Event, Reader};
use std::{collections::HashMap, io::Read, process::Command};
use zip::ZipArchive;

pub struct DocumentAdapter;
pub struct PDFAdapter;
pub struct DOCXApter;

impl Adapter for DocumentAdapter {
    fn ingest(&self, paths: Vec<File>) -> HashMap<File, String> {
        let mut files_by_extension: HashMap<String, Vec<File>> = HashMap::new();

        for path in paths {
            files_by_extension
                .entry(path.extension.to_string())
                .or_insert_with(Vec::new)
                .push(path);
        }

        let mut content_by_file = HashMap::new();

        for (extension, paths) in files_by_extension {
            match extension.as_str() {
                "pdf" => {
                    content_by_file.extend(PDFAdapter.ingest(paths));
                }
                "docx" => {
                    content_by_file.extend(DOCXApter.ingest(paths));
                }
                _ => content_by_file.extend(TextableAdapter.ingest(paths)),
            }
        }

        content_by_file
    }
}

impl Adapter for PDFAdapter {
    fn ingest(&self, paths: Vec<File>) -> HashMap<File, String> {
        let python_pdf_extractor_dir = get_resource_dir().join("python-adapter");

        let mut cmd = Command::new("uv");

        cmd.current_dir(python_pdf_extractor_dir);
        cmd.args(["run", "pdf_extractor.py", "--spawn", "--files"]);

        cmd.args(&paths);

        let result = cmd.output();

        match result {
            Ok(output) if output.status.success() => {
                let stdout_str = String::from_utf8_lossy(&output.stdout);

                serde_json::from_str(&stdout_str).unwrap_or_else(|e| {
                    // Imprime o erro E o conteúdo bruto retornado no stdout
                    eprintln!("Erro ao deserializar: {:#?}", e);

                    HashMap::new()
                })
            }

            Ok(e) => {
                eprintln!(
                    "Fail while calling python pdf extractor: {}",
                    String::from_utf8_lossy(&e.stderr)
                );
                HashMap::new()
            }

            Err(_) => HashMap::new(),
        }
    }
}

impl Adapter for DOCXApter {
    fn ingest(&self, paths: Vec<File>) -> HashMap<File, String> {
        let mut content_by_file = HashMap::new();

        for path in paths {
            let Ok(file) = std::fs::File::open(&path.path) else {
                continue;
            };

            let Ok(mut archive) = ZipArchive::new(file) else {
                continue;
            };

            let Ok(mut doc_xml) = archive.by_name("word/document.xml") else {
                continue;
            };

            let mut xml = String::new();
            if doc_xml.read_to_string(&mut xml).is_err() {
                continue;
            }

            let mut reader = Reader::from_str(&xml);
            reader.config_mut().trim_text(true);

            let mut content = String::new();

            loop {
                match reader.read_event() {
                    Ok(Event::Text(text)) => {
                        let unescaped_text = text.xml_content().ok();
                        if let Some(value) = unescaped_text {
                            if !value.is_empty() {
                                if !content.is_empty() {
                                    content.push(' ');
                                }
                                content.push_str(&value);
                            }
                        }
                    }
                    Ok(Event::Eof) => break,
                    Ok(_) => {}
                    Err(_) => break,
                }
            }

            if !content.is_empty() {
                content_by_file.insert(path, content);
            }
        }

        content_by_file
    }
}
