<h1 align="center">
Goose is your on-machine developer agent, working for you, on your terms
</h1>

<p align="center">
  <img src="docs/assets/goose.png" width="400" height="400" alt="Goose Drawing"/>
</p>
<p align="center">
  Generated by Goose from its <a href="https://github.com/block-open-source/goose-plugins/blob/main/src/goose_plugins/toolkits/artify.py">VincentVanCode toolkit</a>.
</p>

<p align="center">
  <a href="https://block.github.io/goose/">
    <img src="https://img.shields.io/badge/Documentation-goose_docs-teal">
  </a>
  <a href=https://pypi.org/project/goose-ai>
    <img src="https://img.shields.io/pypi/v/goose-ai?color=green">
  </a>
  <a href="https://opensource.org/licenses/Apache-2.0">
    <img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg">
  </a>
  <a href="https://discord.gg/7GaTvbDwga">
    <img src="https://img.shields.io/discord/1287729918100246654?logo=discord&logoColor=white&label=Join+Us&color=blueviolet" alt="Discord">
  </a>
</p>

<p align="center">
<a href="#unique-features-of-goose-compared-to-other-ai-assistants">Unique features</a> 🤖 •
<a href="#what-block-employees-have-to-say-about-goose"> Block Employees on Goose</a> <img src="docs/assets/logo.png" height="15" width="15" alt="Block Emoji"/> •
<a href="#quick-start-guide">Quick start guide</a> 🚀 •
<a href="#getting-involved">Getting involved!</a> 👋
</p>


## Running the CLI

#### OpenAI provider (default):
```
export OPENAI_API_KEY=...

cargo run --bin goose -- --provider open-ai --model gpt-4o
```


#### Databricks provider:
```
export DATABRICKS_HOST=...
export DATABRICKS_TOKEN=...

cargo run --bin goose -- --provider databricks --model claude-3-5-sonnet-2
```
