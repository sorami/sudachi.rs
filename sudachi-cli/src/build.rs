/*
 *  Copyright (c) 2021 Works Applications Co., Ltd.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use memmap2::Mmap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use clap::{Args, Subcommand};

use sudachi::analysis::stateless_tokenizer::DictionaryAccess;
use sudachi::config::Config;
use sudachi::dic::build::report::DictPartReport;
use sudachi::dic::build::DictBuilder;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::grammar::Grammar;
use sudachi::dic::header::HeaderVersion;
use sudachi::dic::lexicon::word_infos::WordInfo;
use sudachi::dic::lexicon_set::LexiconSet;
use sudachi::dic::word_id::WordId;
use sudachi::dic::DictionaryLoader;
use sudachi::error::SudachiResult;

/// Check that the first argument is a subcommand and the file with the same name does
/// not exists.
/// If the file does exists, probably it's safer to use default Sudachi analysis mode.
pub fn is_build_mode(subcommand: &Option<BuildCli>) -> bool {
    match subcommand {
        Some(subcommand) => {
            let raw = match subcommand {
                BuildCli::System { .. } => "build",
                BuildCli::User { .. } => "ubuild",
                BuildCli::Dump { .. } => "dump",
            };

            !Path::new(&raw).exists()
        }
        None => false,
    }
}

#[derive(Subcommand)]
pub(crate) enum BuildCli {
    /// Builds system dictionary
    #[command(name = "build")]
    System {
        #[command(flatten)]
        common: BuildCmd,

        /// Path to matrix definition
        #[arg(short, long)]
        matrix: PathBuf,
    },

    /// Builds user dictionary
    #[command(name = "ubuild")]
    User {
        #[command(flatten)]
        common: BuildCmd,

        /// Path to system dictionary
        #[arg(short = 's', long = "system")]
        dictionary: PathBuf,
    },

    #[command(name = "dump")]
    Dump {
        /// target dictionary to dump
        dictionary: PathBuf,
        /// dump target (matrix, pos, winfo)
        part: String,
        /// output file
        output: PathBuf,

        /// reference system dictionary.
        /// required to dump winfo of an user dictionary
        #[arg(short = 's', long = "system")]
        system: Option<PathBuf>,
    },
}

#[derive(Args)]
pub(crate) struct BuildCmd {
    /// Input csv files
    inputs: Vec<PathBuf>,

    /// Where to place compiled dictionary.
    /// If there was an existing one it will be overwritten.
    #[arg(short = 'o', long = "output")]
    output_file: PathBuf,

    /// Description string to embed into dictionary
    #[arg(short, long, default_value = "")]
    description: String,
}

pub fn build_main(subcommand: BuildCli) {
    match subcommand {
        BuildCli::System { common, matrix } => build_system(common, matrix),
        BuildCli::User { common, dictionary } => build_user(common, dictionary),
        BuildCli::Dump {
            dictionary,
            part,
            output,
            system,
        } => dump_part(dictionary, system, part, output),
    }
}

fn build_system(mut cmd: BuildCmd, matrix: PathBuf) {
    let mut builder = DictBuilder::new_system();
    builder.set_description(std::mem::take(&mut cmd.description));
    builder
        .read_conn(matrix.as_path())
        .expect("failed to read matrix");
    for d in cmd.inputs.iter() {
        builder
            .read_lexicon(d.as_path())
            .unwrap_or_else(|e| panic!("failed to read {:?}\n{:?}", d, e));
    }
    builder.resolve().expect("failed to resolve references");
    let file = output_file(&cmd.output_file);
    let mut buf_writer = BufWriter::with_capacity(16 * 1024, file);
    builder
        .compile(&mut buf_writer)
        .expect("failed to compile dictionary");
    buf_writer.flush().expect("failed to flush");
    print_stats(builder.report());
}

fn build_user(mut cmd: BuildCmd, system: PathBuf) {
    let cfg =
        Config::new(None, None, Some(system)).expect("failed to create default configuration");
    let dict = JapaneseDictionary::from_cfg(&cfg).expect("failed to load system dictionary");

    let mut builder = DictBuilder::new_user(&dict);
    builder.set_description(std::mem::take(&mut cmd.description));
    for d in cmd.inputs.iter() {
        builder
            .read_lexicon(d.as_path())
            .unwrap_or_else(|e| panic!("failed to read {:?}\n{:?}", d, e));
    }
    builder.resolve().expect("failed to resolve references");
    let file = output_file(&cmd.output_file);
    let mut buf_writer = BufWriter::with_capacity(16 * 1024, file);
    builder
        .compile(&mut buf_writer)
        .expect("failed to compile dictionary");
    buf_writer.flush().expect("failed to flush");
    print_stats(builder.report());
}

fn print_stats(report: &[DictPartReport]) {
    let max_len = report.iter().map(|r| r.part().len()).max().unwrap_or(0);

    for part in report {
        let unit = if part.is_write() { "bytes" } else { "entries" };
        eprintln!(
            "{0:1$} {2} {3} in {4:.3} sec",
            part.part(),
            max_len,
            part.size(),
            unit,
            part.time().as_secs_f32()
        )
    }
}

fn output_file(p: &Path) -> File {
    if p.exists() {
        std::fs::remove_file(p).unwrap_or_else(|e| panic!("failed to delete {:?}\n{:?}", p, e));
    }

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .unwrap_or_else(|e| panic!("failed to open {:?} for writing:\n{:?}", p, e))
}

fn dump_part(dict: PathBuf, system: Option<PathBuf>, part: String, output: PathBuf) {
    let file = File::open(dict).expect("open dict failed");
    let data = unsafe { Mmap::map(&file) }.expect("mmap dict failed");
    let loader =
        unsafe { DictionaryLoader::read_any_dictionary(&data) }.expect("failed to load dictionary");

    let outf = output_file(&output);
    let mut writer = BufWriter::new(outf);

    match part.as_str() {
        "pos" => dump_pos(loader, &mut writer),
        "matrix" => dump_matrix(loader, &mut writer),
        "winfo" => dump_word_info(loader, system, &mut writer).unwrap(),
        _ => unimplemented!(),
    }
    writer.flush().unwrap();
}

fn dump_pos<W: Write>(dict: DictionaryLoader, w: &mut W) {
    let dict = dict
        .to_loaded()
        .expect("target dict should contain grammar");
    let grammar = dict.grammar();

    for (id, p) in grammar.pos_list.iter().enumerate() {
        write!(w, "{},", id).unwrap();
        for (i, e) in p.iter().enumerate() {
            w.write_all(e.as_bytes()).unwrap();
            if (i + 1) == p.len() {
                w.write_all(b"\n").unwrap();
            } else {
                w.write_all(b",").unwrap();
            }
        }
    }
}

fn dump_matrix<W: Write>(dict: DictionaryLoader, w: &mut W) {
    if let HeaderVersion::UserDict(_) = dict.header.version {
        panic!("user dictionary does not have connection matrix.")
    }

    let dict = dict
        .to_loaded()
        .expect("target dict should contain grammar");
    let grammar = dict.grammar();
    let conn = grammar.conn_matrix();

    writeln!(w, "{} {}", conn.num_left(), conn.num_right()).unwrap();
    for left in 0..conn.num_left() {
        for right in 0..conn.num_right() {
            let cost = conn.cost(left as _, right as _);
            writeln!(w, "{} {} {}", left, right, cost).unwrap();
        }
    }
}

fn dump_word_info<W: Write>(
    dict: DictionaryLoader,
    system: Option<PathBuf>,
    w: &mut W,
) -> SudachiResult<()> {
    let is_user = match dict.header.version {
        HeaderVersion::UserDict(_) => true,
        HeaderVersion::SystemDict(_) => false,
    };
    let did = if is_user { 1 } else { 0 };
    let size = dict.lexicon.size();

    let data = system.map(|system_path| {
        let file = File::open(system_path).expect("open system failed");
        unsafe { Mmap::map(&file) }.expect("mmap system failed")
    });
    let system = data.as_ref().map(|data| {
        let loader = DictionaryLoader::read_system_dictionary(data)
            .expect("failed to load system dictionary");
        loader
            .to_loaded()
            .expect("failed to load system dictionary")
    });

    let (base, user) = if is_user {
        (
            system.expect("system dictionary is required to dump user dictionary lexicon"),
            Some(dict),
        )
    } else {
        (dict.to_loaded().expect("failed to load dictionary"), None)
    };

    let mut lex = base.lexicon_set;
    let mut grammar = base.grammar;
    if let Some(udic) = user {
        lex.append(udic.lexicon, grammar.pos_list.len())?;
        if let Some(g) = udic.grammar {
            grammar.merge(g)
        }
    }

    for i in 0..size {
        let wid = WordId::checked(did, i)?;
        let (left, right, cost) = lex.get_word_param(wid);
        let winfo = lex.get_word_info(wid)?;
        write!(w, "{},", unicode_escape(winfo.surface()))?;
        write!(w, "{},{},{},", left, right, cost)?;
        write!(w, "{},", unicode_escape(winfo.surface()))?; // writing
        write!(w, "{},", pos_string(&grammar, winfo.pos_id()))?;
        write!(w, "{},", unicode_escape(winfo.reading_form()))?;
        write!(w, "{},", unicode_escape(winfo.normalized_form()))?;
        let dict_form = dictionary_form_string(&grammar, &lex, winfo.dictionary_form_word_id());
        write!(w, "{},", dict_form)?;
        write!(w, "{},", split_mode(&winfo))?;
        dump_wids(w, &grammar, &lex, winfo.a_unit_split())?;
        w.write_all(b",")?;
        dump_wids(w, &grammar, &lex, winfo.b_unit_split())?;
        w.write_all(b",")?;
        dump_wids(w, &grammar, &lex, winfo.word_structure())?;
        w.write_all(b",")?;
        dump_gids(w, winfo.synonym_group_ids())?;
        w.write_all(b"\n")?;
    }
    Ok(())
}

fn unicode_escape(raw: &str) -> String {
    // replace '"' and ','
    raw.to_string()
        .replace('"', "\\u0022")
        .replace(',', "\\u002c")
}

fn split_mode(winfo: &WordInfo) -> &str {
    let asplits = winfo.a_unit_split();
    if asplits.is_empty() {
        return "A";
    }
    let bsplits = winfo.b_unit_split();
    if bsplits.is_empty() {
        return "B";
    }
    "C"
}

fn pos_string(grammar: &Grammar, posid: u16) -> String {
    let pos_parts = grammar.pos_components(posid);
    pos_parts.join(",")
}

fn dictionary_form_string(grammar: &Grammar, lex: &LexiconSet, wid: i32) -> String {
    if wid < 0 {
        return "*".to_string();
    }
    let wid_with_dic = WordId::checked(0, wid as u32).expect("invalid wordid");
    format!("\"{}\"", wordref_string(grammar, lex, &wid_with_dic))
}

fn wordref_string(grammar: &Grammar, lex: &LexiconSet, wid: &WordId) -> String {
    let winfo = lex.get_word_info(*wid).expect("failed to get wordinfo");
    format!(
        "{},{},{}",
        unicode_escape(winfo.surface()),
        pos_string(grammar, winfo.pos_id()),
        unicode_escape(winfo.reading_form()),
    )
}

fn dump_wids<W: Write>(
    w: &mut W,
    grammar: &Grammar,
    lex: &LexiconSet,
    data: &[WordId],
) -> SudachiResult<()> {
    if data.is_empty() {
        write!(w, "*")?;
        return Ok(());
    }
    w.write_all(b"\"")?;
    for (i, e) in data.iter().enumerate() {
        write!(w, "{}", wordref_string(grammar, lex, e))?;
        if i + 1 != data.len() {
            w.write_all(b"/")?;
        }
    }
    w.write_all(b"\"")?;
    Ok(())
}

fn dump_gids<W: Write>(w: &mut W, data: &[u32]) -> SudachiResult<()> {
    if data.is_empty() {
        write!(w, "*")?;
        return Ok(());
    }
    for (i, e) in data.iter().enumerate() {
        write!(w, "{:06}", e)?;
        if i + 1 != data.len() {
            w.write_all(b"/")?;
        }
    }
    Ok(())
}
