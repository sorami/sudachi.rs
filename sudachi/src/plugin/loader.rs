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

use libloading::{Library, Symbol};
use serde_json::Value;

use crate::config::{Config, ConfigError};
use crate::dic::grammar::Grammar;
use crate::error::{SudachiError, SudachiResult};
use crate::plugin::PluginError;

/// Holds loaded plugins, whether they are bundled
/// or loaded from DSOs
pub struct PluginContainer<T: PluginCategory + ?Sized> {
    libraries: Vec<Library>,
    plugins: Vec<<T as PluginCategory>::BoxType>,
}

impl<T: PluginCategory + ?Sized> PluginContainer<T> {
    pub fn plugins(&self) -> &[<T as PluginCategory>::BoxType] {
        &self.plugins
    }
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

impl<T: PluginCategory + ?Sized> Drop for PluginContainer<T> {
    fn drop(&mut self) {
        self.plugins.clear();
        self.libraries.clear();
    }
}

struct PluginLoader<'a, T: PluginCategory + ?Sized> {
    cfg: &'a Config,
    grammar: &'a Grammar<'a>,
    libraries: Vec<Library>,
    plugins: Vec<<T as PluginCategory>::BoxType>,
}

impl<'a, T: PluginCategory + ?Sized> PluginLoader<'a, T> {
    pub fn new(grammar: &'a Grammar, config: &'a Config) -> PluginLoader<'a, T> {
        PluginLoader {
            cfg: config,
            grammar,
            libraries: Vec::new(),
            plugins: Vec::new(),
        }
    }

    pub fn load(&mut self) -> SudachiResult<()> {
        let configs = <T as PluginCategory>::configurations(self.cfg);
        for cfg in configs {
            let name = extract_plugin_class(cfg)?;
            self.load_plugin(name, cfg)?;
        }
        Ok(())
    }

    pub fn freeze(self) -> PluginContainer<T> {
        return PluginContainer {
            libraries: self.libraries,
            plugins: self.plugins,
        };
    }

    fn load_plugin(&mut self, name: &str, plugin_cfg: &Value) -> SudachiResult<()> {
        let mut plugin =
            // Try to load bundled plugin first, if its name looks like it
            if let Some(stripped_name) = name.strip_prefix("com.worksap.nlp.sudachi.") {
                if let Some(p) = <T as PluginCategory>::bundled_impl(stripped_name) {
                    p
                } else {
                    return Err(SudachiError::ConfigError(ConfigError::InvalidFormat(
                        format!("Failed to lookup bundled plugin: {}", name)
                    )))
                }
            // Otherwise treat name as DSO
            } else {
                let candidates = self.resolve_dso_names(name);
                self.load_plugin_from_dso(&candidates)?
            };

        <T as PluginCategory>::do_setup(&mut plugin, plugin_cfg, &self.cfg, &self.grammar)?;
        self.plugins.push(plugin);
        Ok(())
    }

    fn resolve_dso_names(&self, name: &str) -> Vec<String> {
        let resolved = self.cfg.resolve_plugin_paths(name.to_owned());
        resolved
    }

    fn try_load_library_from(candidates: &[String]) -> SudachiResult<Library> {
        if candidates.is_empty() {
            return Err(SudachiError::PluginError(PluginError::InvalidDataFormat(
                "No candidates to load library".to_owned()
            )))
        }

        let mut last_error = libloading::Error::IncompatibleSize;
        for p in candidates.iter() {
            match unsafe { Library::new(p.as_str()) } {
                Ok(lib) => return Ok(lib),
                Err(e) => last_error = e
            }
        };
        Err(SudachiError::PluginError(PluginError::Libloading {
            source: last_error,
            message: format!("failed to load library from: {:?}", candidates)
        }))
    }

    fn load_plugin_from_dso(
        &mut self,
        candidates: &[String]
    ) -> SudachiResult<<T as PluginCategory>::BoxType> {
        let lib = Self::try_load_library_from(candidates)?;
        let load_fn: Symbol<fn() -> SudachiResult<<T as PluginCategory>::BoxType>> =
            unsafe { lib.get(b"load_plugin")? };
        let plugin = load_fn();
        self.libraries.push(lib);
        plugin
    }
}

fn extract_plugin_class(val: &Value) -> SudachiResult<&str> {
    let obj = match val {
        Value::Object(v) => v,
        _ => {
            return Err(SudachiError::ConfigError(ConfigError::InvalidFormat(
                "plugin config must be an object".to_owned(),
            )));
        }
    };
    match obj.get("class") {
        Some(Value::String(v)) => Ok(v),
        _ => Err(SudachiError::ConfigError(ConfigError::InvalidFormat(
            "plugin config must have 'class' key to indicate plugin SO file".to_owned(),
        ))),
    }
}

/// A category of Plugins
pub trait PluginCategory {
    /// Boxed type of the plugin. Should be Box<dyn XXXX>.
    type BoxType;

    /// Type of the initialization function.
    /// It must take 0 arguments and return `SudachiResult<Self::BoxType>`.
    type InitFnType;

    /// Extract plugin configurations from the config
    fn configurations(cfg: &Config) -> &[Value];

    /// Create bundled plugin for plugin name
    /// Instead of full name like com.worksap.nlp.sudachi.ProlongedSoundMarkPlugin
    /// should handle only the short one: ProlongedSoundMarkPlugin
    ///
    /// com.worksap.nlp.sudachi. (last dot included) will be stripped automatically
    /// by the loader code
    fn bundled_impl(name: &str) -> Option<Self::BoxType>;

    /// Perform initial setup.
    /// We can't call set_up of the plugin directly in the default implementation
    /// of this method because we do not know the specific type yet
    fn do_setup(
        ptr: &mut Self::BoxType,
        settings: &Value,
        config: &Config,
        grammar: &Grammar,
    ) -> SudachiResult<()>;
}

/// Helper function to load the plugins of a single category
/// Should be called with turbofish syntax and trait object type:
/// `let plugins = load_plugins_of::<dyn InputText>(...)`.
pub fn load_plugins_of<T: PluginCategory + ?Sized>(
    cfg: &Config,
    grammar: &Grammar,
) -> SudachiResult<PluginContainer<T>> {
    let mut loader: PluginLoader<T> = PluginLoader::new(grammar, cfg);
    loader.load()?;
    Ok(loader.freeze())
}
