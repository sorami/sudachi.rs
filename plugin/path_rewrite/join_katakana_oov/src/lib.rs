/*
 * Copyright (c) 2021 Works Applications Co., Ltd.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use sudachi::plugin::path_rewrite::join_katakana_oov::JoinKatakanaOovPlugin;
use sudachi::plugin::path_rewrite::PathRewritePlugin;
use sudachi::plugin::PluginCategory;
use sudachi::prelude::*;
use sudachi::sudachi_dso_plugin;

// Generate DSO for tests
sudachi_dso_plugin!(dyn PathRewritePlugin, JoinKatakanaOovPlugin);