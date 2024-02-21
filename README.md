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

This project is currently being built, and in a WIP status. It is currently not ready for use, the documentation is currently incomplete and the source code is incomplete and many changes my still occur. Please check back later for updates, or use at your own risk.

## Getting Started

### Prerequisites

* Rust ([Windows](https://www.rust-lang.org/tools/install) | [Other](https://forge.rust-lang.org/infra/other-installation-methods.html))

### Installation

1. Get OpenAI apikey from [here](https://platform.openai.com/api-keys) and set it up on your local machine according to the instructions [here](https://platform.openai.com/docs/quickstart/step-2-setup-your-api-key).

2. Clone the repo to your directory

```sh
git clone https://github.com/mbc012/RustOpenAi.git
```
3. FINISH

```sh
TODO
```



## Coverage
<table>
<tr><td valign="top" width="50%">

### Stable Endpoints
| Endpoints | Status | Notes |
|-----------|-----|-|
| Audio     |⬛️||
| Chat |🔶||
| Embeddings |⬛️||
| File-Tuning | ⬛️||
| Files |✔️||
|Images|⬛️||
|Models|✔️||
|Moderations|✔️||

</td>
<td valign="top" width="50%">

### Beta Endpoints
| Endpoints | Status | Notes                |
|-----------|----|----------------------|
| Assistant |✔️|                      |
| Threads   |✔️|                      |
| Messages |✔️|                      |
| Runs |🔶| |


### Legend
| Status | Meaning          |
|--------|------------------|
|⬛️| No Coverage      |
|🔶| Partial Coverage |
|✔️| Full Coverage    |

</td></tr>
</table>



## Usage

**Model:**

Create a `Model`
```rust
let client = Client::new(None); // Create a client
let model_id = String::from("gpt-4"); // String defining the model id
let model = client.load_model(model_id).unwrap(); // Retrieve model
```

List all models
```rust
let client = Client::new(None); // Create a client
let models = client.list_models().unwrap(); // Get a `ApiList<Model>`
```

**Moderation:** Check if a String violates the OpenAI Content policy. This can be accessed through a `Client` struct
```rust
let client = Client::new(None);
let input_string = String::from("What color is the sky?"); // String to be checked
let model = ...; // Optional param; This can be either None, `Model` or a String representing a model id.
let moderated = client.create_moderation(input_string, model) // Returns a Result<Moderation, OpenApiError>
                      .unwrap();
moderated.is_flagged() // Returns a bool corresponding to the 'flagged' param
```

**Assistants:**

Create an `Assistant`
```rust
let client = Client::new(None);
let model = client.load_model(String::from("gpt-4")).unwrap();
let assistant: Assistant = AssistantBuilder::new(model)
                              .with_name("TestAssistant1")
                              .with_description("This is my test assistant")
                              .with_instruction("You are a helpful and friendly assistant!")
                              .build(client.get_networking())
```

Retrieve an `Assistant`
```rust
let client = Client::new(None);
let assistant_id = String::from("assistant_id_here");
let assistant = client.retrieve_assistant()
                      .unwrap(); 
```

List all assistants
```rust
let client = Client::new(None);
let assistants = client.list_assistants(None)
                       .unwrap();
```

Delete an `Assistant`
```rust

```

Retrieve an `AssistantFile`
```rust
let client = Client::new(None);
let assistant_id = String::from("assistant_id_here");
let file_id = String::from("file_id_here");
let file = client.retrieve_assistant_file(assistant_id, file_id)
                 .unwrap();
```



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

