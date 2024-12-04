(function() {
    var implementors = Object.fromEntries([["sudachi",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/fancy-regex/0.13.0/fancy_regex/error/enum.Error.html\" title=\"enum fancy_regex::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"sudachi/config/enum.ConfigError.html\" title=\"enum sudachi::config::ConfigError\">ConfigError</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"sudachi/dic/character_category/enum.Error.html\" title=\"enum sudachi::dic::character_category::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"sudachi/dic/header/enum.HeaderError.html\" title=\"enum sudachi::dic::header::HeaderError\">HeaderError</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"sudachi/dic/lexicon_set/enum.LexiconSetError.html\" title=\"enum sudachi::dic::lexicon_set::LexiconSetError\">LexiconSetError</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"sudachi/plugin/enum.PluginError.html\" title=\"enum sudachi::plugin::PluginError\">PluginError</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/alloc/string/struct.FromUtf16Error.html\" title=\"struct alloc::string::FromUtf16Error\">FromUtf16Error</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/num/error/struct.ParseIntError.html\" title=\"struct core::num::error::ParseIntError\">ParseIntError</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/config/enum.ConfigError.html\" title=\"enum sudachi::config::ConfigError\">ConfigError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/dic/build/error/enum.BuildFailure.html\" title=\"enum sudachi::dic::build::error::BuildFailure\">BuildFailure</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/plugin/enum.PluginError.html\" title=\"enum sudachi::plugin::PluginError\">PluginError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.117/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/config/enum.ConfigError.html\" title=\"enum sudachi::config::ConfigError\">ConfigError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.117/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.117/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"sudachi/plugin/enum.PluginError.html\" title=\"enum sudachi::plugin::PluginError\">PluginError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"sudachi/dic/build/error/struct.DicBuildError.html\" title=\"struct sudachi::dic::build::error::DicBuildError\">DicBuildError</a>&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"sudachi/dic/lexicon/word_infos/struct.WordInfo.html\" title=\"struct sudachi::dic::lexicon::word_infos::WordInfo\">WordInfo</a>&gt; for <a class=\"struct\" href=\"sudachi/dic/lexicon/word_infos/struct.WordInfoData.html\" title=\"struct sudachi::dic::lexicon::word_infos::WordInfoData\">WordInfoData</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"sudachi/dic/lexicon/word_infos/struct.WordInfoData.html\" title=\"struct sudachi::dic::lexicon::word_infos::WordInfoData\">WordInfoData</a>&gt; for <a class=\"struct\" href=\"sudachi/dic/lexicon/word_infos/struct.WordInfo.html\" title=\"struct sudachi::dic::lexicon::word_infos::WordInfo\">WordInfo</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"sudachi/plugin/enum.PluginError.html\" title=\"enum sudachi::plugin::PluginError\">PluginError</a>"],["impl&lt;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Err&lt;<a class=\"enum\" href=\"sudachi/error/enum.SudachiNomError.html\" title=\"enum sudachi::error::SudachiNomError\">SudachiNomError</a>&lt;I&gt;&gt;&gt; for <a class=\"enum\" href=\"sudachi/error/enum.SudachiError.html\" title=\"enum sudachi::error::SudachiError\">SudachiError</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[8661]}