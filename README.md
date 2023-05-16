<h1>
  <div align="center">
    <br />
    <br />
    <b><em>shai</em></b>
    <br />
    <br />
    &nbsp;
  </div>
</h1>

<!--
<div align="center">
  <a href="https://github.com/sullvn/cowbox/actions/workflows/test.yaml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/sullvn/shai/test.yaml?branch=main&label=Tests&style=for-the-badge&logo=github" alt="Tests status" />
  </a>
</div>
-->
<br />
<br />

Command-line assistant powered by AI.

*Shai* helps you brainstorm commands and
remember strange shell incantations.

```sh
$ shai command "File path of the largest file within ~/Documents"
find / -type f -printf '%s %p\n' | sort -nr | head -1 | cut -d" " -f2-

$ find / -type f -printf '%s %p\n' | sort -nr | head -1 | cut -d" " -f2-
/Users/me/Documents/Zoom/2030-11-12 12.54.55 My Zoom Meeting/video128341984.mp4
```


## Features

- Translate natural language to a shell
  one-liner

That's it! See [As Compared to X](#roadmap)
for more featureful alternatives and
[Roadmap](#roadmap) for where *shai* is
heading.


## Installation
### Installation via Pre-built Binaries

Feel free to grab a pre-built binary on the
[GitHub Releases page][0].

Your platform doesn't have a binary for it?
Check out 
[installing from source](#installation-via-source)
below.


### Installation via Source

Installing from source is pretty easy if you
have [Cargo][1] setup:

```sh
$ git clone https://github.com/sullvn/shai.git
$ cargo install --path shai
```


## Setup

*Shai* currently depends on OpenAI's API to
work. You will have to [make an OpenAI
account][2] and [create an API key][3].

Then pass the API key to *shai*:

```sh
#
# Option A: One-time Configuration
#
$ shai configure --openai-api-key <YOUR API KEY>

#
# Option B: Zero Config via Environment Variable
#
$ OPENAI_API_KEY=<YOUR API KEY> shai "Path of the largest file on the system"
```


## As Compared to X

*Shai* is pretty early along, so you will
probably be better served by one of these:

- [**aicmd**][4] is a very simple CLI
  frontend to ChatGPT, just like the current
  state of *shai*. It requires Node.js to
  run. *shai* is a native binary, so has
  no requirements.
- [**AIChat**][5] is native alternative,
  also made in Rust, that has multiple
  modes. Looks great!
- [**ShellGPT**][6] is also a simple
  frontend to ChatGPT, but has a bunch of
  modes and configuration options. Python
  is required.
- [**Chatblade**][7] is similar to
  *ShellGPT*. It has more of a "graphical"
  text interface. Python is required.
- [**Bot Aquarium**][8] lets an LLM loose
  in a Linux Docker container. Actually
  very similar to the end goal of *shai*.
  The only difference is *shai* will safely
  automate tasks on your actual machine.
- [**Auto-GPT**][9] lets an LLM loose
  on your actual computer, via high-level
  interactions and scripts. *shai* will
  remain focused on lower-level shell
  and program automation.
- [**privateGPT**][10] runs a local LLMs
  through documents on your machine. You
  can ask questions of them in natural
  language.


## Roadmap

1. Integrate [cowbox][11] for *Just Run It*
   <sup>:tm:</sup> capabilities
2. Bundle offline LLMs, like [LLaMA][12]
3. Chat mode with history and refining
   queries
4. Shell mode which integrates plain
   commands with AI queries


## You May Also Like

- [cowbox][11] – Safely run programs without
  your files getting borked
- [pvw][13] – Command-line tool which interactively
  previews command outputs


<div align="center">
  <br />
  <br />
  <br />
  <br />
  <b><em>shai</em></b>
  <br />
  <br />
  <br />
  <br />
  &nbsp;
</div>


[0]: https://github.com/sullvn/shai/releases
[1]: https://doc.rust-lang.org/cargo/
[2]: https://platform.openai.com/signup
[3]: https://platform.openai.com/account/api-keys
[4]: https://aicmd.app
[5]: https://github.com/sigoden/aichat
[6]: https://github.com/TheR1D/shell_gpt
[7]: https://github.com/npiv/chatblade
[8]: https://github.com/fafrd/aquarium
[9]: https://github.com/Significant-Gravitas/Auto-GPT
[10]: https://github.com/imartinez/privateGPT
[11]: https://github.com/sullvn/cowbox
[12]: https://github.com/ggerganov/llama.cpp
[13]: https://github.com/sullvn/pvw
