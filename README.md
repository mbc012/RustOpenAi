<br/>
<p align="center">
  <h3 align="center">[WIP] Rust OpenAI Api Wrapper</h3>

  <p align="center">
    A synchronous rust OpenAi api wrapper
    <br/>
    <br/>
    <a href="https://platform.openai.com/docs/overview"><strong>Explore the openai docs »</strong></a>
    <br/>
    <br/>
  </p>
</p>



## Table Of Contents

* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Coverage](#coverage)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)
* [About the Project](#about-the-project)
* [Acknowledgements](#acknowledgements)

## WIP

This project is currently being built, and in a WIP status. It is currently not ready for use, the documentation is currently incomplete and the source is being reformatted. Please check back later for updates.

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.

* npm

```sh
npm install npm@latest -g
```

### Installation

1. Get OpenAI apikey from [here](https://platform.openai.com/api-keys) and set it up on your local machine according to the instructions [here](https://platform.openai.com/docs/quickstart/step-2-setup-your-api-key).

2. Clone the repo to your directory

```sh
git clone https://github.com/mbc012/RustOpenAi.git
```

3. 

3. Install NPM packages

```sh
npm install
```

4. Enter your API in `config.js`

```JS
const API_KEY = 'ENTER YOUR API';
```

## Coverage

### Stable Endpoints
| Endpoints | Status | Notes |
|-----------|-----|-|
| Audio     |🌑||
| Chat |🌕||
| Embeddings |🌑||
| File-Tuning | 🌑||
| Files |🌕||
|Images|🌑||
|Models|🌕||
|Moderations|🌕||


### Beta Endpoints
| Endpoints | Status | Notes                 |
|-----------|----|-----------------------|
| Assistant |🌕|                       |
| Threads   |🌕|                       |
| Messages |🌕|                       |
| Runs |🌗| Only partial coverage |


#### Legend
| Status | Meaning          |
|--------|------------------|
|🌑| No Coverage      |
|🌗| Partial Coverage |
|🌕| Full Coverage    |

## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_


## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.
* If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/mbc012/RustOpenAI/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
* Please make sure you check your spelling and grammar.
* Create individual PR for each suggestion.
* Please also read through the [Code Of Conduct](https://github.com/mbc012/RustOpenAI/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

## License

Distributed under the Apache 2.0 License. See [LICENSE](https://github.com/mbc012/RustOpenAI/blob/main/LICENSE.md) for more information.

## About The Project

This project was undertaken with the aim of enhancing my Rust skillset. This is my first Rust project. OpenAPI was chosen due to its relevance to GPT models and AI, making it a significant milestone in my future journey with Rust.


## Acknowledgements

* [ShaanCoding - ReadMe Template](https://github.com/ShaanCoding/)

